//! `seed/voltaic-g2.wasm` is the compiler in the tree, in wasm.
//!
//! Row 102 (`docs/100 - 102 - The Seed Across the Turn.md`) §3 step 4:
//!
//! > Prove g2 reads generation 2. Run `voltaic-g2.wasm` over the five-program
//! > port set from row 99 §1.2 and over `examples/*.vlt`, and diff against what
//! > the in-tree compiler produces. g2 and the in-tree compiler must agree
//! > byte-for-byte; they are the same compiler, one of them in wasm.
//!
//! # Why this is a different assertion from `fixpoint.rs`
//!
//! `fixpoint.rs` asks whether the compiler reproduces *itself* — stage1 ==
//! stage2 — which is the oldest check in self-hosting and the one that makes a
//! binary stage0 safe to trust. It runs on `voltaic-g1.wasm`, because g1 is what
//! bootstraps the tree today.
//!
//! This asks something narrower and, for one commit only, more useful: whether
//! the blob that was *just built* is faithful to the tree it was built from. The
//! two blobs are the same crate at two commits, so the risk is not that the
//! compiler is wrong — `fixpoint` and the 1,300-test suite cover that — but that
//! the blob is a stale or mis-optimised copy of it. `wasm-opt -Oz` sits between
//! the build and the committed bytes, and nothing else in the repository looks
//! at what it did.
//!
//! So: same input, two compilers that should be one compiler, bytes compared.
//! A `wasm-opt` that miscompiled, a `cargo build` that picked up a stale
//! dependency, or a blob committed from the wrong `target/` directory all fail
//! here and all pass `blob.rs`, which only asserts the bytes match their own
//! documentation.
//!
//! # What it does not assert
//!
//! That generation 2's surface is *complete*. It compares two compilers over the
//! corpus; a construct doc 87 specifies and neither compiler implements is absent
//! from both and invisible here. That is the same blind spot doc 97 §10.2
//! recorded for the inference corpora and doc 97 §10.30 for the grammar — a check
//! that compares an artifact to its source cannot see what is missing from both.
//! `examples/*.vlt` is what closes it, by being the corpus a surface addition is
//! required to extend.

use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

/// The workspace root, from this crate's manifest.
fn workspace_root() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Every `.rs` under `dir`, relative to it, sorted.
fn rust_files(dir: &Path) -> Vec<PathBuf> {
  let mut found = Vec::new();
  collect(dir, dir, &mut found);
  found.sort();
  found
}

fn collect(root: &Path, dir: &Path, into: &mut Vec<PathBuf>) {
  let Ok(entries) = std::fs::read_dir(dir) else {
    return;
  };
  for entry in entries.flatten() {
    let path = entry.path();
    if path.is_dir() {
      collect(root, &path, into);
    } else if path.extension().is_some_and(|e| e == "rs") {
      into.push(
        path
          .strip_prefix(root)
          .expect("under the root being walked")
          .to_path_buf(),
      );
    }
  }
}

/// Transpile `vlt` into a fresh directory under `target/`.
///
/// **No rustfmt**, unlike `fixpoint.rs`'s helper. Both sides here are raw
/// emitter output, and formatting them would hide a disagreement rustfmt happens
/// to normalise away — which is the one class of defect this test exists to
/// catch, since `wasm-opt` is the only thing between the two sides.
fn generate(label: &str, vlt: &Path, run: impl FnOnce(&Path, &Path)) -> PathBuf {
  let out = workspace_root().join("target").join(label);
  let _ = std::fs::remove_dir_all(&out);
  std::fs::create_dir_all(&out).expect("should create the output directory");
  run(vlt, &out);
  out
}

/// Run `seed/voltaic-<generation>.wasm` under `wasmtime` over `vlt` into `out`.
fn by_blob(generation: &str, vlt: &Path, out: &Path) {
  let root = workspace_root();
  let blob = root.join(format!("seed/voltaic-{generation}.wasm"));
  assert!(
    blob.exists(),
    "{} should be committed — seed/SEED.md says why it exists",
    blob.display()
  );
  let run = Command::new("wasmtime")
    .arg("run")
    .arg("--dir")
    .arg(&root)
    .arg(&blob)
    .arg(vlt)
    .arg(out)
    .current_dir(&root)
    .output()
    .expect("should run wasmtime — devenv.nix pins it");
  assert!(
    run.status.success(),
    "the {generation} blob should transpile {}:\n{}",
    vlt.display(),
    String::from_utf8_lossy(&run.stderr)
  );
}

/// Run the **native** `voltaic-seed` — the in-tree compiler — over `vlt` into `out`.
fn by_tree(vlt: &Path, out: &Path) {
  let run = Command::new(env!("CARGO_BIN_EXE_voltaic-seed"))
    .arg(vlt)
    .arg(out)
    .current_dir(workspace_root())
    .output()
    .expect("should run the native voltaic-seed");
  assert!(
    run.status.success(),
    "the in-tree compiler should transpile {}:\n{}",
    vlt.display(),
    String::from_utf8_lossy(&run.stderr)
  );
}

/// Two output trees hold the same files with the same bytes.
fn agree(label: &str, left: &Path, right: &Path) {
  let ours = rust_files(left);
  let theirs = rust_files(right);
  assert!(
    !ours.is_empty(),
    "{label}: the g2 blob emitted nothing, so the comparison would be vacuous"
  );
  assert_eq!(
    ours, theirs,
    "{label}: the g2 blob and the in-tree compiler emitted different file sets"
  );
  for rel in ours {
    let a = std::fs::read(left.join(&rel)).expect("should read the blob's output");
    let b = std::fs::read(right.join(&rel)).expect("should read the tree's output");
    assert!(
      a == b,
      "{label}: seed/voltaic-g2.wasm and the in-tree compiler disagree on {}. They are \
       the same compiler, one of them through `wasm-opt -Oz`, so a disagreement means \
       the committed blob is not the tree — a stale build, the wrong `target/`, or a \
       miscompile. Rebuild with `moon run :seed-refresh-g2` and re-record the hash.",
      rel.display()
    );
  }
}

/// Row 102 §3 step 4, over every crate the blob will be asked to compile.
///
/// `seed/GENERATION` is the input list rather than a hand-written one, for its
/// own comment's reason: a second copy drifts, and the copy that would drift here
/// is exactly the set `bootstrap.sh` dispatches over.
#[test]
fn the_g2_blob_and_the_tree_agree_on_every_ported_crate() {
  let root = workspace_root();
  let list =
    std::fs::read_to_string(root.join("seed/GENERATION")).expect("should read seed/GENERATION");
  let crates: Vec<String> = list
    .lines()
    .map(str::trim)
    .filter(|l| !l.is_empty() && !l.starts_with('#'))
    .filter_map(|l| l.split_whitespace().next().map(str::to_string))
    .collect();
  assert!(
    !crates.is_empty(),
    "seed/GENERATION names no crates, so this test would assert nothing"
  );

  for name in &crates {
    let vlt = root.join("crates").join(name).join("vlt");
    if !vlt.is_dir() {
      continue;
    }
    let mine = generate(&format!("turn-g2-{name}"), &vlt, |v, o| by_blob("g2", v, o));
    let theirs = generate(&format!("turn-tree-{name}"), &vlt, by_tree);
    agree(name, &mine, &theirs);
  }
}

/// The same comparison over `examples/`, which is the corpus a surface addition
/// is required to extend.
///
/// The crates above are all generation 1 today, so they exercise none of
/// generation 2's additions — `examples/ladder.vlt` and `examples/else_tail.vlt`
/// are the only `.vlt` in the repository that do. Without this the test above
/// would pass with a blob that had never seen a ladder.
///
/// # Why the corpus is staged rather than pointed at
///
/// `examples/` holds one `.vlt` that is **not a compilable program**:
/// `lexical.vlt` is a lexer fixture, a run of token shapes — `[0, ...rest, 9]`,
/// `total : u64` — that the parser rejects, and `voltaic build` rejects it too.
/// Running either compiler over the directory therefore fails before any
/// comparison happens, which is how this test failed on its first run.
///
/// The discriminator is **whether the example has a golden**: `tests/golden/`
/// holds a `.vrs` for every example that compiles, and exactly one example has
/// none. That is derived from the tree rather than a second list to keep in step,
/// which is `seed/GENERATION`'s reason as well. It also means an example that
/// stops compiling drops out of this corpus loudly — `examples.rs`'s
/// `every_example_has_a_golden_expectation` fails first.
#[test]
fn the_g2_blob_and_the_tree_agree_on_the_examples() {
  let root = workspace_root();
  let staged = root.join("target/turn-corpus");
  let _ = std::fs::remove_dir_all(&staged);
  std::fs::create_dir_all(&staged).expect("should create the staging directory");

  let mut count = 0;
  for entry in std::fs::read_dir(root.join("examples")).expect("should read examples/") {
    let path = entry.expect("should read an examples/ entry").path();
    if path.extension().is_none_or(|e| e != "vlt") {
      continue;
    }
    let stem = path
      .file_stem()
      .expect("a .vlt has a stem")
      .to_string_lossy()
      .to_string();
    if !root.join(format!("tests/golden/{stem}.vrs")).exists() {
      continue;
    }
    std::fs::copy(&path, staged.join(format!("{stem}.vlt"))).expect("should stage an example");
    count += 1;
  }
  assert!(
    count > 40,
    "staged only {count} examples; the corpus should be every example with a golden, \
     and there are 47 of those"
  );
  for witness in ["ladder", "else_tail"] {
    assert!(
      staged.join(format!("{witness}.vlt")).exists(),
      "examples/{witness}.vlt is the only thing in the repository exercising one of \
       generation 2's additions, and it is not in the staged corpus — so this test \
       would pass with a blob that had never seen it"
    );
  }

  let mine = generate("turn-g2-examples", &staged, |v, o| by_blob("g2", v, o));
  let theirs = generate("turn-tree-examples", &staged, by_tree);
  agree("examples", &mine, &theirs);
}

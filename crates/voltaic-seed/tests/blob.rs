//! The committed `seed/voltaic-g1.wasm` is the blob `seed/SEED.md` claims it is.
//!
//! Plan 72 leaf 3. A checked-in binary is the one artifact in this repository
//! a reader cannot inspect, so the two things that *can* be asserted about it
//! are asserted: its size and its hash, against the table in `SEED.md` that
//! also records the commit and the command it came from.
//!
//! That makes a hand-edited blob and a stale table the same failure, which is
//! the point — a seed whose provenance row disagrees with its bytes is worse
//! than no row at all, because the row reads as verification.
//!
//! ```bash
//! VOLTAIC_UPDATE=1 cargo test -p voltaic-seed --test blob
//! ```
//!
//! # What this cannot assert
//!
//! That the blob is what the recipe *produces*. `wasm-opt` and rustc are
//! deterministic for a fixed input and a fixed version, but the versions are
//! pinned by `devenv.nix` rather than by this test, so a toolchain bump moves
//! the hash legitimately. The assertion is therefore about **agreement**
//! between the blob and its documentation, not about reproducibility — and
//! `tests/fixpoint.rs` is what checks the blob still *behaves*.

use std::path::PathBuf;

/// The workspace root, from this crate's manifest.
fn workspace_root() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The `| blake3 | \`…\` |` and `| size | … bytes |` rows of `SEED.md`.
fn recorded(seed_md: &str) -> (Option<String>, Option<usize>) {
  let mut hash = None;
  let mut size = None;
  for line in seed_md.lines() {
    let line = line.trim();
    if let Some(rest) = line.strip_prefix("| blake3 |") {
      hash = Some(
        rest
          .trim()
          .trim_matches('|')
          .trim()
          .trim_matches('`')
          .to_owned(),
      );
    }
    if let Some(rest) = line.strip_prefix("| size |") {
      size = rest
        .trim()
        .trim_matches('|')
        .trim()
        .trim_end_matches(" bytes")
        .replace(',', "")
        .parse()
        .ok();
    }
  }
  (hash, size)
}

#[test]
fn the_committed_seed_matches_what_seed_md_records() {
  let root = workspace_root();
  let blob = root.join("seed/voltaic-g1.wasm");
  let doc_path = root.join("seed/SEED.md");

  let bytes = std::fs::read(&blob).expect(
    "seed/voltaic-g1.wasm should be committed — `moon run :seed-refresh` builds it, and \
     seed/SEED.md says why it exists",
  );
  let hash = blake3::hash(&bytes).to_hex().to_string();
  let doc = std::fs::read_to_string(&doc_path).expect("should read seed/SEED.md");

  if std::env::var("VOLTAIC_UPDATE").is_ok() {
    let mut out = String::new();
    for line in doc.lines() {
      if line.trim_start().starts_with("| blake3 |") {
        out.push_str(&format!("| blake3 | `{hash}` |\n"));
      } else if line.trim_start().starts_with("| size |") {
        out.push_str(&format!("| size | {} bytes |\n", bytes.len()));
      } else {
        out.push_str(line);
        out.push('\n');
      }
    }
    std::fs::write(&doc_path, out).expect("should write seed/SEED.md");
    panic!(
      "updated seed/SEED.md — a changed hash is a changed compiler, so review the diff \
       rather than absorbing it. Also update the commit row by hand."
    );
  }

  let (want_hash, want_size) = recorded(&doc);
  assert_eq!(
    want_size,
    Some(bytes.len()),
    "seed/SEED.md records a different size than seed/voltaic-g1.wasm has"
  );
  assert_eq!(
    want_hash.as_deref(),
    Some(hash.as_str()),
    "seed/voltaic-g1.wasm does not hash to what seed/SEED.md records. Either the blob was \
     replaced without updating, or the table is stale — and a provenance row that \
     disagrees with its bytes reads as verification while providing none."
  );
}

/// Row 102 leaf 8: `SEED.md`'s provenance is complete and the invariant holds.
///
/// Doc 100 §1.2:
///
/// > A generation's seed must be built by the previous generation's compiler,
/// > before the previous generation's source is replaced. Equivalently: at no
/// > point is the only compiler for a language written in that language.
///
/// A blob built by itself is what that forbids, so the check is that `built by`
/// never names the generation in `compiles`. For `voltaic-g1.wasm` it names no
/// generation at all — the base case, produced by the committed Rust compiler
/// plan 72 then deleted.
///
/// This asserts a *recorded* fact rather than a derived one, and that is the
/// honest shape: nothing in the repository can recompute which compiler built a
/// committed binary. What it can do is refuse a provenance table that states the
/// forbidden thing, and refuse one that states nothing at all — which is the
/// failure that matters, because an absent row reads as "fine" and a present
/// wrong one reads as verified.
#[test]
fn seed_md_records_a_provenance_the_invariant_allows() {
  let doc = std::fs::read_to_string(workspace_root().join("seed/SEED.md"))
    .expect("should read seed/SEED.md");

  let field = |name: &str| -> String {
    doc
      .lines()
      .map(str::trim)
      .find(|l| l.starts_with(&format!("| {name} |")))
      .unwrap_or_else(|| {
        panic!(
          "seed/SEED.md has no `{name}` row. Row 102 leaf 8 added it; without it the \
           invariant in doc 100 §1.2 is stated and unchecked."
        )
      })
      .trim_start_matches('|')
      .trim()
      .trim_start_matches(name)
      .trim()
      .trim_start_matches('|')
      .trim()
      .trim_end_matches('|')
      .trim()
      .to_string()
  };

  let compiles = field("compiles");
  let built_by = field("built by");

  assert!(
    compiles == "g1" || compiles == "g2",
    "seed/SEED.md records `compiles: {compiles}`; expected g1 or g2"
  );
  assert!(
    !built_by.is_empty(),
    "seed/SEED.md's `built by` row is empty, so it records no provenance at all"
  );
  assert!(
    !built_by.split_whitespace().any(|word| word == compiles),
    "seed/SEED.md says this blob compiles `{compiles}` and was built by `{built_by}`, \
     which names the same generation. That is the one thing doc 100 §1.2 forbids: a \
     seed built by the compiler it is supposed to underwrite."
  );
}

/// The generation column is the thing `bootstrap.sh` dispatches on, so a blob
/// some crate is marked for has to exist.
#[test]
fn every_generation_named_in_the_list_has_its_blob() {
  let root = workspace_root();
  let text =
    std::fs::read_to_string(root.join("seed/GENERATION")).expect("should read seed/GENERATION");
  let mut wanted: Vec<String> = text
    .lines()
    .map(str::trim)
    .filter(|l| !l.is_empty() && !l.starts_with('#'))
    .filter_map(|l| l.split_whitespace().nth(2).map(str::to_string))
    .collect();
  wanted.sort();
  wanted.dedup();
  assert!(
    !wanted.is_empty(),
    "seed/GENERATION names no generations, so bootstrap.sh has nothing to dispatch on"
  );
  for generation in wanted {
    let blob = root.join(format!("seed/voltaic-{generation}.wasm"));
    assert!(
      blob.exists(),
      "seed/GENERATION marks a crate as `{generation}` and {} does not exist, so \
       ./bootstrap.sh would fail on a clean clone",
      blob.display()
    );
  }
}

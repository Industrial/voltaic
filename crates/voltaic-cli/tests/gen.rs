//! `voltaic gen` and `voltaic_driver::compile_tree` — leaf v1-gen-tree.
//!
//! `gen` transpiles a whole tree: every `<rel>.vlt` under the source becomes
//! `<rel>.rs` and `<rel>.rs.map` under the output, subdirectories mirrored, so
//! the standard `a.rs` + `a/b.rs` module layout falls out of `module`
//! declarations. `--workspace` does the same for every `crates/*/vlt` →
//! `crates/*/src`, and for `crates/*/tests/**/*.vlt` in place.
//!
//! What these tests pin, in order of how expensive a regression would be:
//!
//! * **Incremental.** An output whose bytes would not change is not written, so
//!   its mtime is stable and cargo does not rebuild what depends on it.
//! * **`--check`.** Writes nothing; exits 1 naming each stale, missing or
//!   orphaned path, and 0 when the tree is fresh.
//! * **Deletion.** Removing a `.vlt` removes its `.rs` and `.rs.map` — only
//!   outputs with a `.rs.map` beside them, so a hand-written `.rs` is never
//!   touched — and any directory that leaves empty.
//! * **Sidecars first.** Every `.vlt.sig` is written before anything is built,
//!   because cross-crate resolution reads them: the consumer below emits
//!   `p.depth()` only because the upstream sidecar says `depth` is a method.
//! * **`--rustfmt`.** Formats each output and re-aligns its map, so a line in
//!   the formatted Rust still resolves to the `.vlt` line it came from.
//!
//! The fixtures under `tests/gen_tree/` are files, per the repository rule;
//! every test copies them into a scratch directory under `target/` first, so
//! nothing here mutates a tracked file.

mod support;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::SystemTime;

fn repo_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .and_then(Path::parent)
    .expect("the crate sits two levels below the repository root")
    .to_path_buf()
}

fn fixtures() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/gen_tree")
}

/// A fresh scratch directory under `target/`, unique to this test.
fn scratch(label: &str) -> PathBuf {
  let dir = repo_root()
    .join("target/gen-tests")
    .join(format!("{label}-{}", std::process::id()));
  let _ = std::fs::remove_dir_all(&dir);
  std::fs::create_dir_all(&dir).expect("should create the scratch directory");
  dir
}

fn copy_tree(from: &Path, to: &Path) {
  std::fs::create_dir_all(to).expect("should create a directory");
  for entry in std::fs::read_dir(from).expect("should read a fixture directory") {
    let path = entry.expect("should read an entry").path();
    let target = to.join(path.file_name().expect("a name"));
    if path.is_dir() {
      copy_tree(&path, &target);
    } else {
      std::fs::copy(&path, &target).expect("should copy a fixture");
    }
  }
}

/// The three-level tree, copied into `<scratch>/tree`.
fn tree(label: &str) -> PathBuf {
  let root = scratch(label);
  copy_tree(&fixtures().join("tree"), &root.join("tree"));
  root
}

/// The two-crate workspace, copied, with its manifest put in place.
///
/// The fixture's manifest is `workspace.toml` rather than `Cargo.toml` so
/// that nothing scanning this repository for manifests mistakes the fixture
/// for a crate.
fn workspace(label: &str) -> PathBuf {
  let root = scratch(label).join("ws");
  copy_tree(&fixtures().join("workspace"), &root);
  std::fs::rename(root.join("workspace.toml"), root.join("Cargo.toml"))
    .expect("should put the manifest in place");
  root
}

fn voltaic(args: &[&str], cwd: &Path) -> Output {
  Command::new(env!("CARGO_BIN_EXE_voltaic"))
    .args(args)
    .current_dir(cwd)
    .output()
    .expect("should run voltaic")
}

fn gen_tree(root: &Path, extra: &[&str]) -> Output {
  let mut args = vec!["gen"];
  args.extend_from_slice(extra);
  args.extend_from_slice(&["tree", "out"]);
  voltaic(&args, root)
}

fn succeeded(out: &Output) -> bool {
  out.status.success()
}

fn stderr(out: &Output) -> String {
  String::from_utf8_lossy(&out.stderr).to_string()
}

/// Every file under `dir`, relative and sorted, with its mtime.
fn files(dir: &Path) -> Vec<(PathBuf, SystemTime)> {
  fn walk(base: &Path, dir: &Path, into: &mut Vec<(PathBuf, SystemTime)>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
      return;
    };
    for entry in entries.flatten() {
      let path = entry.path();
      if path.is_dir() {
        walk(base, &path, into);
      } else {
        let modified = entry
          .metadata()
          .and_then(|m| m.modified())
          .expect("should read an mtime");
        into.push((
          path
            .strip_prefix(base)
            .expect("under the base")
            .to_path_buf(),
          modified,
        ));
      }
    }
  }
  let mut found = Vec::new();
  walk(dir, dir, &mut found);
  found.sort();
  found
}

fn names(dir: &Path) -> Vec<String> {
  let mut found: Vec<String> = files(dir)
    .into_iter()
    .map(|(p, _)| p.to_string_lossy().to_string())
    .collect();
  found.sort();
  found
}

const MIRRORED: &[&str] = &[
  "main.rs",
  "main.rs.map",
  "outer.rs",
  "outer.rs.map",
  "outer/inner.rs",
  "outer/inner.rs.map",
  "outer/inner/deepest.rs",
  "outer/inner/deepest.rs.map",
];

#[test]
fn gen_mirrors_a_three_level_tree_that_rustc_compiles() {
  let root = tree("mirror");
  let out = gen_tree(&root, &[]);
  assert!(succeeded(&out), "gen should succeed:\n{}", stderr(&out));
  assert_eq!(names(&root.join("out")), MIRRORED);
  for source in ["main", "outer", "outer/inner", "outer/inner/deepest"] {
    assert!(
      root
        .join("tree")
        .join(format!("{source}.vlt.sig"))
        .is_file(),
      "gen should write a sidecar beside {source}.vlt"
    );
  }
  assert!(
    !root.join("out/.voltaic-cache").exists(),
    "gen bypasses the compile cache and must leave none in the output"
  );

  if support::skip_without_rustc("gen_mirrors_a_three_level_tree_that_rustc_compiles") {
    return;
  }
  let binary = root.join("main");
  let compiled = Command::new(support::rustc())
    .args(["--edition", "2024", "-o"])
    .arg(&binary)
    .arg(root.join("out/main.rs"))
    .output()
    .expect("should run rustc");
  assert!(
    compiled.status.success(),
    "the `mod` declarations should resolve to the mirrored files:\n{}",
    stderr(&compiled)
  );
  let ran = Command::new(&binary)
    .output()
    .expect("should run the program");
  assert!(ran.status.success(), "the program should run");
}

#[test]
fn compile_tree_mirrors_the_same_layout() {
  let root = tree("driver");
  let compiled = voltaic_driver::compile_tree(&root.join("tree"), &root.join("out"))
    .expect("the tree should compile");
  assert_eq!(compiled.inputs.len(), 4);
  assert_eq!(names(&root.join("out")), MIRRORED);
}

#[test]
fn a_second_run_rewrites_nothing() {
  let root = tree("incremental");
  assert!(succeeded(&gen_tree(&root, &[])));
  let before = files(&root);
  std::thread::sleep(std::time::Duration::from_millis(1100));

  let again = gen_tree(&root, &[]);
  assert!(succeeded(&again), "{}", stderr(&again));
  assert_eq!(
    files(&root),
    before,
    "an unchanged output, sidecar or map must not be rewritten"
  );

  // `compile_tree` labels its maps with the path it was handed, so it gets an
  // output of its own rather than sharing gen's.
  let driver_out = root.join("driver-out");
  voltaic_driver::compile_tree(&root.join("tree"), &driver_out).expect("should compile");
  let first = files(&driver_out);
  std::thread::sleep(std::time::Duration::from_millis(1100));
  let compiled =
    voltaic_driver::compile_tree(&root.join("tree"), &driver_out).expect("should compile again");
  assert_eq!(compiled.outputs.len(), 4);
  assert_eq!(files(&driver_out), first, "nor by compile_tree");
}

#[test]
fn compile_dir_no_longer_rewrites_a_cache_hit() {
  let root = tree("compile-dir");
  let src = root.join("tree");
  let out = root.join("flat");
  voltaic_driver::compile_dir(&src, &out).expect("should compile");
  let before = files(&out);
  std::thread::sleep(std::time::Duration::from_millis(1100));
  voltaic_driver::compile_dir(&src, &out).expect("should compile again");
  assert_eq!(files(&out), before);
}

#[test]
fn check_is_clean_when_fresh_and_fails_after_an_edit() {
  let root = tree("check-edit");
  let fresh = gen_tree(&root, &["--check"]);
  assert!(!succeeded(&fresh), "nothing has been generated yet");
  assert!(stderr(&fresh).contains("missing: "), "{}", stderr(&fresh));
  assert!(!root.join("out").exists(), "--check writes nothing");

  assert!(succeeded(&gen_tree(&root, &[])));
  let clean = gen_tree(&root, &["--check"]);
  assert!(
    succeeded(&clean),
    "a fresh tree checks clean:\n{}",
    stderr(&clean)
  );

  std::fs::copy(
    fixtures().join("edited/deepest.vlt"),
    root.join("tree/outer/inner/deepest.vlt"),
  )
  .expect("should edit the source");
  let before = files(&root.join("out"));
  let stale = gen_tree(&root, &["--check"]);
  assert_eq!(stale.status.code(), Some(1));
  assert!(
    stderr(&stale).contains("stale: ") && stderr(&stale).contains("deepest.rs"),
    "{}",
    stderr(&stale)
  );
  assert_eq!(files(&root.join("out")), before, "--check writes nothing");

  assert!(succeeded(&gen_tree(&root, &[])));
  assert!(succeeded(&gen_tree(&root, &["--check"])));
}

#[test]
fn deleting_a_source_sweeps_its_outputs_and_the_empty_directory() {
  let root = tree("check-delete");
  assert!(succeeded(&gen_tree(&root, &[])));
  std::fs::remove_file(root.join("tree/outer/inner/deepest.vlt")).expect("should delete");

  let orphaned = gen_tree(&root, &["--check"]);
  assert_eq!(orphaned.status.code(), Some(1));
  assert!(
    stderr(&orphaned).contains("orphaned: "),
    "{}",
    stderr(&orphaned)
  );
  assert!(
    root.join("out/outer/inner/deepest.rs").is_file(),
    "--check removes nothing"
  );

  let swept = gen_tree(&root, &[]);
  assert!(succeeded(&swept), "{}", stderr(&swept));
  assert!(
    !root.join("out/outer/inner").exists(),
    "the emptied directory goes too"
  );
  assert!(!root.join("tree/outer/inner/deepest.vlt.sig").exists());
  assert_eq!(names(&root.join("out")), &MIRRORED[..6]);
  assert!(succeeded(&gen_tree(&root, &["--check"])));
}

#[test]
fn a_file_without_a_map_is_never_removed() {
  let root = tree("hand-written");
  assert!(succeeded(&gen_tree(&root, &[])));
  std::fs::copy(
    fixtures().join("workspace/crates/downcrate/tests/hand.rs"),
    root.join("out/outer/hand.rs"),
  )
  .expect("should place a hand-written file");
  let swept = gen_tree(&root, &["--check"]);
  assert!(succeeded(&swept), "{}", stderr(&swept));
}

#[test]
fn workspace_mode_maps_every_crate_and_its_tests() {
  let root = workspace("workspace");
  let out = voltaic(&["gen", "--workspace"], &root.join("crates/downcrate/vlt"));
  assert!(succeeded(&out), "{}", stderr(&out));

  assert!(root.join("crates/upcrate/src/point.rs").is_file());
  assert!(root.join("crates/upcrate/src/point.rs.map").is_file());
  assert!(root.join("crates/downcrate/tests/probe.rs").is_file());
  assert!(root.join("crates/downcrate/tests/probe.rs.map").is_file());
  let sidecar = std::fs::read_to_string(root.join("crates/upcrate/vlt/point.vlt.sig"))
    .expect("gen should write the upstream sidecar");
  assert!(sidecar.contains("Point.depth method"), "{sidecar}");

  let consumer =
    std::fs::read_to_string(root.join("crates/downcrate/src/consumer.rs")).expect("generated");
  assert!(
    consumer.contains("p.depth()"),
    "the consumer resolves against the sidecar written first:\n{consumer}"
  );

  let hand = std::fs::read(root.join("crates/downcrate/tests/hand.rs")).expect("still there");
  let original =
    std::fs::read(fixtures().join("workspace/crates/downcrate/tests/hand.rs")).expect("fixture");
  assert_eq!(hand, original, "a hand-written test is never touched");

  let check = voltaic(&["gen", "--workspace", "--check"], &root);
  assert!(succeeded(&check), "{}", stderr(&check));
}

#[test]
fn workspace_mode_refuses_to_overwrite_a_hand_written_test() {
  let root = workspace("collision");
  let tests = root.join("crates/downcrate/tests");
  std::fs::copy(tests.join("probe.vlt"), tests.join("hand.vlt")).expect("should collide");
  let out = voltaic(&["gen", "--workspace"], &root);
  assert!(!succeeded(&out));
  assert!(
    stderr(&out).contains("refusing to overwrite"),
    "{}",
    stderr(&out)
  );
}

#[test]
fn workspace_mode_outside_a_workspace_says_so() {
  let root = scratch("nowhere");
  let out = voltaic(&["gen", "--workspace"], Path::new("/"));
  assert!(!succeeded(&out));
  assert!(
    stderr(&out).contains("--workspace found no directory"),
    "{}",
    stderr(&out)
  );
  let _ = std::fs::remove_dir_all(root);
}

#[test]
fn rustfmt_keeps_the_map_pointing_at_the_source() {
  if !support::rustfmt_available() {
    eprintln!("skipping rustfmt_keeps_the_map_pointing_at_the_source: no rustfmt");
    return;
  }
  let root = workspace("rustfmt");
  let out = voltaic(&["gen", "--workspace", "--rustfmt"], &root);
  assert!(succeeded(&out), "{}", stderr(&out));

  let rust =
    std::fs::read_to_string(root.join("crates/downcrate/src/consumer.rs")).expect("generated");
  let map = voltaic_diag::parse_map(
    &std::fs::read_to_string(root.join("crates/downcrate/src/consumer.rs.map"))
      .expect("a map beside it"),
  )
  .expect("the re-aligned map parses");
  assert_eq!(
    map.len(),
    rust.lines().count(),
    "one record per formatted line"
  );

  let call = rust
    .lines()
    .position(|l| l.contains("p.depth()"))
    .expect("the call is emitted")
    + 1;
  let source =
    std::fs::read_to_string(root.join("crates/downcrate/vlt/consumer.vlt")).expect("the source");
  let expected = source
    .lines()
    .position(|l| l.contains("p.depth"))
    .expect("the source line")
    + 1;
  assert_eq!(
    map.resolve(call, 5).map(|at| at.0),
    Some(expected),
    "the formatted call still maps to the .vlt line it came from"
  );

  let again = voltaic(&["gen", "--workspace", "--rustfmt", "--check"], &root);
  assert!(succeeded(&again), "{}", stderr(&again));
}

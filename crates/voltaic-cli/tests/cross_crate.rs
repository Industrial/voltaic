//! Doc 98 leaf 2: cross-crate resolution reads sidecars, never upstream source.
//!
//! > Cross-crate resolution reads sidecars, never upstream source. This is the
//! > leaf that keeps separate compilation and per-crate caching working.
//!
//! # What this closes
//!
//! Doc 97 §10.36 made `x.f` a field read or a nullary method call, decided by
//! the solver — and `declarations::collect` runs over **one `Program`**, which
//! is one `.vlt` file. So a consumer could never learn that an upstream type had
//! a method, and doc 101 §10.12 records the rule that guessed instead being
//! withdrawn: "not declared here" is not "foreign", because almost nothing is
//! declared here.
//!
//! The sidecar is what carries it. `voltaic fmt` writes one per `.vlt`
//! (doc 98 leaf 1), `Sidecar` publishes a line per member (doc 98 §10.5), and
//! `voltaic_driver::resolve::upstream` reads the sidecars of every crate an
//! `import` names.
//!
//! # Why a scratch workspace and not this one
//!
//! This repository's own crates have no committed sidecars yet, so resolving
//! against them would assert nothing. A two-crate workspace under `target/` is
//! the smallest thing that can hold a published upstream and a consumer, and it
//! lets the test state the property rather than the project's current state:
//! **a method published upstream emits as a call, and a field published upstream
//! emits as a field.**
//!
//! It also pins the negative, which is the half that matters: with the sidecar
//! **deleted**, the same consumer emits `p.depth` — a field read that does not
//! exist. That is the behaviour before this leaf, and the test that would fail if
//! resolution silently stopped working.

use std::path::{Path, PathBuf};
use std::process::Command;

fn repo_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .and_then(Path::parent)
    .expect("the crate sits two levels below the repository root")
    .to_path_buf()
}

fn voltaic() -> PathBuf {
  repo_root().join("target/debug/voltaic")
}

/// A two-crate workspace: `upcrate` declares, `downcrate` consumes.
fn workspace(label: &str) -> PathBuf {
  let root = repo_root().join("target/cross-crate").join(label);
  let _ = std::fs::remove_dir_all(&root);
  for dir in ["crates/upcrate/vlt", "crates/downcrate/vlt"] {
    std::fs::create_dir_all(root.join(dir)).expect("should create the scratch workspace");
  }
  // `workspace_above` climbs until it finds a directory holding both `crates/`
  // and `Cargo.toml`, so the manifest is what marks the root.
  std::fs::write(root.join("Cargo.toml"), "[workspace]\n").expect("should write the manifest");
  std::fs::write(
    root.join("crates/upcrate/vlt/point.vlt"),
    "data Point = Point { x : f64, y : f64 }\n\n\
     instance Point\n  export depth : &self -> f64\n  depth self = self.y\n",
  )
  .expect("should write the upstream source");
  std::fs::write(
    root.join("crates/downcrate/vlt/use.vlt"),
    "import upcrate::Point\n\n\
     reach : &Point -> f64\nreach p = p.depth\n\n\
     read : &Point -> f64\nread p = p.x\n",
  )
  .expect("should write the consumer");
  root
}

fn run(args: &[&str]) -> bool {
  Command::new(voltaic())
    .args(args)
    .current_dir(repo_root())
    .output()
    .expect("should run voltaic")
    .status
    .success()
}

/// The consumer's generated Rust.
fn consumer_rust(root: &Path) -> String {
  let out = root.join("out.vrs");
  let input = root.join("crates/downcrate/vlt/use.vlt");
  assert!(
    run(&[
      "build",
      input.to_str().expect("a utf-8 path"),
      "-o",
      out.to_str().expect("a utf-8 path"),
    ]),
    "the consumer should transpile"
  );
  std::fs::read_to_string(&out).expect("should read the generated Rust")
}

#[test]
fn a_method_published_upstream_emits_as_a_call() {
  let root = workspace("published");
  let up = root.join("crates/upcrate/vlt/point.vlt");
  assert!(
    run(&["fmt", up.to_str().expect("a utf-8 path")]),
    "`voltaic fmt` should write the upstream sidecar"
  );
  let sidecar = std::fs::read_to_string(root.join("crates/upcrate/vlt/point.vlt.sig"))
    .expect("`voltaic fmt` should have written a sidecar");
  assert!(
    sidecar.contains("Point.depth method"),
    "the sidecar should publish the method:\n{sidecar}"
  );
  assert!(
    sidecar.contains("Point.x field"),
    "the sidecar should publish the field:\n{sidecar}"
  );

  let rust = consumer_rust(&root);
  assert!(
    rust.contains("p.depth()"),
    "a method published upstream should emit as a call:\n{rust}"
  );
  assert!(
    rust.contains("p.x") && !rust.contains("p.x()"),
    "a field published upstream should stay a field read:\n{rust}"
  );
}

#[test]
fn without_the_sidecar_the_same_method_emits_as_a_field_read() {
  let root = workspace("unpublished");
  // Deliberately no `voltaic fmt` on the upstream crate, so nothing is published.
  assert!(
    !root.join("crates/upcrate/vlt/point.vlt.sig").exists(),
    "this case is about the sidecar being absent"
  );

  let rust = consumer_rust(&root);
  assert!(
    rust.contains("p.depth") && !rust.contains("p.depth()"),
    "with nothing published the solver cannot know `depth` is a method, and \
     doc 97 §10.36's restriction says leave it a field — this is the behaviour \
     the sidecar replaces, pinned so a silent regression in resolution shows up \
     as this test passing when the other one fails:\n{rust}"
  );
}

//! `voltaic cargo-check` — leaf v4-cargo-check-remap.
//!
//! `voltaic check` compiles one file with bare rustc. A crate written by
//! `voltaic gen --workspace` is compiled by cargo, and without this command its
//! errors point at the generated `crates/*/src/*.rs`. `cargo-check` runs cargo's
//! check with `--message-format=json`, keeps each `compiler-message`, and remaps
//! its primary span through the `.rs.map` beside the generated file.
//!
//! The fixture under `tests/cargo_check/` is a two-crate workspace: `beta`
//! imports `alpha`, and `beta/vlt/lib.vlt` carries two deliberate `E0308`s —
//! one in ordinary Voltaic (line 7), one inside a `rust """` hatch whose lines
//! all map to the hatch's first line (line 9). Its manifests are stored as
//! `workspace.toml` and `crate.toml` so nothing scanning this repository takes
//! the fixture for a crate; every test copies it under `target/` first.

mod support;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn repo_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .and_then(Path::parent)
    .expect("the crate sits two levels below the repository root")
    .to_path_buf()
}

fn copy_tree(from: &Path, to: &Path) {
  std::fs::create_dir_all(to).expect("should create a directory");
  for entry in std::fs::read_dir(from).expect("should read a fixture directory") {
    let path = entry.expect("should read an entry").path();
    let name = path.file_name().expect("a name");
    let target = if name == "workspace.toml" || name == "crate.toml" {
      to.join("Cargo.toml")
    } else {
      to.join(name)
    };
    if path.is_dir() {
      copy_tree(&path, &target);
    } else {
      std::fs::copy(&path, &target).expect("should copy a fixture");
    }
  }
}

/// The fixture workspace, copied and generated with `voltaic gen --workspace`.
fn generated(label: &str) -> PathBuf {
  let root = repo_root()
    .join("target/cargo-check-tests")
    .join(format!("{label}-{}", std::process::id()));
  let _ = std::fs::remove_dir_all(&root);
  copy_tree(
    &Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/cargo_check"),
    &root,
  );
  let out = voltaic(&["gen", "--workspace"], &root);
  assert!(
    out.status.success(),
    "gen --workspace should succeed:\n{}",
    stderr(&out)
  );
  root
}

/// Run `voltaic` with cargo's and rustc's inherited environment removed, then
/// pointed at a toolchain verified to run — the same reasoning as
/// `cargo_project.rs`'s `cargo()`.
fn voltaic(args: &[&str], cwd: &Path) -> Output {
  let mut command = Command::new(env!("CARGO_BIN_EXE_voltaic"));
  command.args(args).current_dir(cwd);
  for (key, _) in std::env::vars_os() {
    let name = key.to_string_lossy();
    if name.starts_with("CARGO") || name.starts_with("RUST") {
      command.env_remove(&key);
    }
  }
  command.env("RUSTC", support::rustc());
  if let Some(cargo) = support::cargo_path() {
    command.env("CARGO", cargo);
  }
  command.output().expect("should run voltaic")
}

fn stderr(out: &Output) -> String {
  String::from_utf8_lossy(&out.stderr).to_string()
}

fn skip(test: &str) -> bool {
  support::skip_without_rustc(test) || support::skip_without_cargo(test)
}

/// Criteria 1–4 and 6: both errors are reported at their `.vlt` line, no
/// location names a generated `.rs`, and the exit code is cargo's own.
#[test]
fn errors_are_reported_against_the_vlt_with_cargos_exit_code() {
  if skip("errors_are_reported_against_the_vlt_with_cargos_exit_code") {
    return;
  }
  let root = generated("remap");
  let out = voltaic(&["cargo-check", "--plain"], &root);
  let text = stderr(&out);

  assert_eq!(out.status.code(), Some(101), "cargo's exit code:\n{text}");
  assert!(
    text.contains("crates/beta/vlt/lib.vlt:7:"),
    "the ordinary error should be on .vlt line 7:\n{text}"
  );
  assert!(
    text.contains("E0308"),
    "rustc's code passes through:\n{text}"
  );
  assert!(
    !text.contains(".rs:"),
    "no location should name generated Rust:\n{text}"
  );
}

/// Criterion 5: a span inside a `rust """` hatch maps to the hatch's line.
#[test]
fn an_error_inside_a_hatch_is_reported_at_the_hatch() {
  if skip("an_error_inside_a_hatch_is_reported_at_the_hatch") {
    return;
  }
  let root = generated("hatch");
  let out = voltaic(&["cargo-check", "--plain", "-p", "beta"], &root);
  let text = stderr(&out);
  assert!(
    text.contains("crates/beta/vlt/lib.vlt:9:"),
    "the hatch error should be on the hatch's line 9:\n{text}"
  );
}

/// Arguments are forwarded, and a clean check exits 0.
#[test]
fn a_clean_package_exits_zero() {
  if skip("a_clean_package_exits_zero") {
    return;
  }
  let root = generated("clean");
  let out = voltaic(&["cargo-check", "--plain", "-p", "alpha"], &root);
  assert!(out.status.success(), "alpha is clean:\n{}", stderr(&out));
}

/// Criterion 2's other half: with no map beside it, a message is reported
/// against the Rust file, unremapped.
#[test]
fn a_file_with_no_map_passes_through_against_the_rust() {
  if skip("a_file_with_no_map_passes_through_against_the_rust") {
    return;
  }
  let root = generated("passthrough");
  std::fs::remove_file(root.join("crates/beta/src/lib.rs.map")).expect("the map gen wrote");
  let out = voltaic(&["cargo-check", "--plain", "-p", "beta"], &root);
  let text = stderr(&out);
  assert_eq!(out.status.code(), Some(101), "cargo's exit code:\n{text}");
  assert!(
    text.contains("crates/beta/src/lib.rs:"),
    "an unmapped file is reported as itself:\n{text}"
  );
}

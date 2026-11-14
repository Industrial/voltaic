//! Every committed sidecar agrees with its source.
//!
//! Doc 98 §1.3: *a sidecar disagreeing with its source is `VLT0901`.* With the
//! compiler's own 110 sidecars committed, that stops being a rule about user
//! code and becomes a maintenance obligation on this repository: a sidecar is a
//! **published API**, cross-crate resolution reads it instead of the source
//! (doc 98 §10.7), and a stale one makes the solver answer about a program that
//! no longer exists.
//!
//! # Why this is worse than a missing sidecar
//!
//! A missing sidecar is handled: `upstream` finds nothing, the solver claims
//! nothing, and doc 97 §10.36's restriction leaves every `x.f` a field read.
//! `cross_crate.rs` pins exactly that. A *stale* one is the opposite — it answers
//! confidently and wrongly, and the consumer's generated Rust is wrong in a way
//! no gate on the consumer can see, because the consumer is consistent with what
//! it was told.
//!
//! That is the same shape `AGENTS.md` records for the index: *"a forced tool
//! answering from a stale index is worse than no rule, because the answers still
//! look authoritative."*
//!
//! # What this cannot check
//!
//! That the sidecar is *complete* — it compares what `voltaic sig` produces now
//! against what is committed, so a defect in the generator is invisible to it.
//! `voltaic-lattice`'s own tests cover the generator; this covers the drift.

use std::path::{Path, PathBuf};
use std::process::Command;

fn repo_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .and_then(Path::parent)
    .expect("the crate sits two levels below the repository root")
    .to_path_buf()
}

/// Every `.vlt` under a crate listed in `seed/GENERATION`.
///
/// The list rather than a glob, for `seed/GENERATION`'s own reason: it is the
/// only copy of which crates are generated.
fn sources() -> Vec<PathBuf> {
  let root = repo_root();
  let listed =
    std::fs::read_to_string(root.join("seed/GENERATION")).expect("should read seed/GENERATION");
  let mut found = Vec::new();
  for line in listed.lines().map(str::trim) {
    if line.is_empty() || line.starts_with('#') {
      continue;
    }
    let Some(name) = line.split_whitespace().next() else {
      continue;
    };
    collect(&root.join("crates").join(name).join("vlt"), &mut found);
  }
  found.sort();
  found
}

fn collect(dir: &Path, into: &mut Vec<PathBuf>) {
  let Ok(entries) = std::fs::read_dir(dir) else {
    return;
  };
  for entry in entries.flatten() {
    let path = entry.path();
    if path.is_dir() {
      collect(&path, into);
    } else if path.extension().is_some_and(|e| e == "vlt") {
      into.push(path);
    }
  }
}

fn sidecar_of(source: &Path) -> PathBuf {
  let mut shown = source.as_os_str().to_os_string();
  shown.push(".sig");
  PathBuf::from(shown)
}

#[test]
fn every_source_has_a_committed_sidecar() {
  let mut missing = Vec::new();
  for source in sources() {
    if !sidecar_of(&source).exists() {
      missing.push(source.display().to_string());
    }
  }
  assert!(
    missing.is_empty(),
    "these sources have no sidecar, so a consumer resolving against them learns \
     nothing and every `x.f` stays a field read. Regenerate with:\n  \
     for f in crates/*/vlt/**/*.vlt; do voltaic sig --emit \"$f\"; done\n{missing:#?}"
  );
}

#[test]
fn every_committed_sidecar_agrees_with_its_source() {
  let root = repo_root();
  let voltaic = root.join("target/debug/voltaic");
  if !voltaic.exists() {
    eprintln!("skipped: target/debug/voltaic is not built");
    return;
  }

  let mut stale = Vec::new();
  for source in sources() {
    let sidecar = sidecar_of(&source);
    let Ok(committed) = std::fs::read_to_string(&sidecar) else {
      continue;
    };
    // Regenerate into a scratch copy rather than over the committed file, so a
    // failing run leaves the tree exactly as it found it.
    let scratch = root.join("target/sidecar-check");
    std::fs::create_dir_all(&scratch).expect("should create the scratch directory");
    let probe = scratch.join(source.file_name().expect("a source has a file name"));
    std::fs::copy(&source, &probe).expect("should copy the source");
    let run = Command::new(&voltaic)
      .args(["sig", "--emit"])
      .arg(&probe)
      .current_dir(&root)
      .output()
      .expect("should run voltaic sig");
    if !run.status.success() {
      stale.push(format!(
        "{}: `voltaic sig --emit` failed:\n{}",
        source.display(),
        String::from_utf8_lossy(&run.stderr)
      ));
      continue;
    }
    let fresh =
      std::fs::read_to_string(sidecar_of(&probe)).expect("should read the regenerated sidecar");
    if fresh != committed {
      stale.push(format!("{}", source.display()));
    }
  }

  assert!(
    stale.is_empty(),
    "these sidecars no longer agree with their source — doc 98 §1.3's `VLT0901`. \
     A stale sidecar is worse than a missing one: cross-crate resolution reads it \
     instead of the source, so it answers confidently about a program that no \
     longer exists. Regenerate with:\n  \
     for f in crates/*/vlt/**/*.vlt; do voltaic sig --emit \"$f\"; done\n{stale:#?}"
  );
}

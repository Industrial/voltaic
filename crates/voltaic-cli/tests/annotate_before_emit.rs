//! Row 99 wave 1: nothing may call the emitter without annotating first.
//!
//! # Why this needs a gate rather than a convention
//!
//! An unsigned definition's parameters carry `ty = None`, and `voltaic-emit`
//! reads an untyped parameter as **the receiver** — it moves it to position 0
//! and emits it without a type. That convention predates wave 1 and is how
//! `self` works, so an unsigned definition reaching the emitter unannotated
//! does not fail: it produces *wrong Rust*, silently.
//!
//! That is not hypothetical. `crates/voltaic-fuzz/vlt/target.vlt` emitted
//! without annotating, and `the_generated_rust_always_parses` went red the
//! moment `examples/inferred.vlt` entered the fuzz corpus. And this gate then
//! found a *fifth* site nobody had looked at — `voltaic-mcp/vlt/tools.vlt` —
//! which is the whole argument for writing it down instead of remembering it.
//!
//! # What this checks, and what it cannot
//!
//! It reads source text and requires every file that names an emit entry point
//! to also name `annotate`. That is coarse: it cannot tell that the annotate
//! call *precedes* the emit call, or that it is on the same program. It is
//! enough to catch the failure that actually happened — a file that emits and
//! never annotates at all — and the per-site behaviour is covered by
//! `voltaic-driver`'s own tests and the `examples` corpus.
//!
//! [`EXEMPT`] names the files where emitting without annotating is correct,
//! each with its reason, and [`every_exemption_still_emits`] refuses an
//! exemption that no longer calls the emitter — so the list cannot rot into a
//! blanket.

use std::path::{Path, PathBuf};

/// The two ways to reach the emitter.
const EMIT_CALLS: &[&str] = &["voltaic_emit::emit_with_map", "voltaic_emit::emit "];

/// Files that emit without annotating, and why that is right.
const EXEMPT: &[(&str, &str)] = &[(
  "crates/voltaic-bench/vlt/throughput.vlt",
  "measures the emitter's throughput in isolation; annotating would time the \
     solver instead of the thing under test",
)];

fn repo_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .and_then(Path::parent)
    .expect("the crate sits two levels below the repository root")
    .to_path_buf()
}

/// Every `.vlt` the compiler is written in, by repo-relative path.
fn sources() -> Vec<(String, String)> {
  let mut found = Vec::new();
  let crates = repo_root().join("crates");
  let Ok(entries) = std::fs::read_dir(&crates) else {
    panic!("crates/ should be readable");
  };
  for entry in entries.filter_map(Result::ok) {
    collect(&entry.path().join("vlt"), &mut found);
  }
  found.sort();
  found
}

fn collect(dir: &Path, into: &mut Vec<(String, String)>) {
  let Ok(entries) = std::fs::read_dir(dir) else {
    return;
  };
  for entry in entries.filter_map(Result::ok) {
    let path = entry.path();
    if path.is_dir() {
      collect(&path, into);
    } else if path.extension().is_some_and(|ext| ext == "vlt") {
      let Ok(text) = std::fs::read_to_string(&path) else {
        continue;
      };
      let shown = path
        .strip_prefix(repo_root())
        .unwrap_or(&path)
        .display()
        .to_string();
      into.push((shown, text));
    }
  }
}

fn emits(text: &str) -> bool {
  EMIT_CALLS.iter().any(|call| text.contains(call))
}

#[test]
fn every_emit_site_also_annotates() {
  let exempt: Vec<&str> = EXEMPT.iter().map(|(path, _)| *path).collect();
  let mut offenders = Vec::new();
  for (shown, text) in sources() {
    if !emits(&text) || exempt.contains(&shown.as_str()) {
      continue;
    }
    if !text.contains("annotate") {
      offenders.push(shown);
    }
  }
  assert!(
    offenders.is_empty(),
    "these files reach the emitter without annotating first, so an unsigned \
     definition's parameters would be emitted as receivers: {offenders:?}\n\nAdd the \
     annotate call, or add the file to EXEMPT with the reason it is correct."
  );
}

/// The other side of the list: an exemption that no longer emits is stale, and a
/// stale exemption is a hole that looks like a decision.
#[test]
fn every_exemption_still_emits() {
  let all = sources();
  for (path, reason) in EXEMPT {
    let found = all.iter().find(|(shown, _)| shown == path);
    let Some((_, text)) = found else {
      panic!("{path} is exempt and does not exist; the exemption is stale ({reason})");
    };
    assert!(
      emits(text),
      "{path} is exempt from annotating and no longer calls the emitter, so the \
       exemption asserts nothing ({reason})"
    );
  }
}

/// A gate over a list of call patterns is narrow if the patterns stop matching.
#[test]
fn the_emit_patterns_still_match_something() {
  let all = sources();
  for call in EMIT_CALLS {
    assert!(
      all.iter().any(|(_, text)| text.contains(call)),
      "no file contains `{call}`, so this gate is looking for a spelling that no \
       longer exists"
    );
  }
}

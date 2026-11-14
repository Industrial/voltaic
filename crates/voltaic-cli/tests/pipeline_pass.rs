//! Row 103 leaf 2, first transform: `|>` becomes a receiver-first `.`.
//!
//! Doc 97 §1.3 deletion 3 — `|>`, `>>>`, `<<<` and `$` are replaced by `.`
//! chains that break across lines — and the `|>` half is the one with a
//! one-to-one rewrite: `a |> .f` is `a.f` and `a |> .f b` is `a.f b`, because
//! both already lower to `a.f()` and `a.f(b)`.
//!
//! # Why this is run over the compiler's own source and not a fixture
//!
//! §1.1's identity gate is the whole correctness argument for pass 1:
//!
//! > The `.vrs` generated from the migrated source is byte-identical to the
//! > `.xrs` generation 1 generated from the original.
//!
//! A fixture proves the transform does something. The compiler's own 105 `.vlt`
//! files prove it does *only* that, over 8,479 `.`-calls and every shape of
//! pipeline the project actually writes — a corpus nobody wrote to be easy to
//! migrate. That is doc 97 §10.2's finding in a second place: the demanding
//! corpus is the one that was not designed for the test.
//!
//! # What a failure here means
//!
//! A file whose migrated source lowers differently. The gate reports the first
//! differing line of generated Rust, which is the only report that localises a
//! migration defect — "the tests still pass" is compatible with a changed
//! borrow, a changed clone, a different `match` shape.
//!
//! Nothing is written. This asserts the transform *would* be safe; `apply` is
//! what writes, and row 103 leaf 6 is what calls it crate by crate.
//!
//! It lives in `voltaic-cli` rather than `voltaic-driver` because it needs both
//! the driver (the pass and the gate) and the formatter (to print the rewritten
//! AST back to `.vlt`), and the driver does not depend on the formatter. Adding
//! that edge for a test would also mean amending `dependency_sources` and
//! `moon_project_graph.rs` — a real cost for no gain, since `voltaic-cli`
//! already depends on both.

use std::path::{Path, PathBuf};

use voltaic_driver::{Gate, Pass, gate, rewrite_program};

fn repo_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .and_then(Path::parent)
    .expect("the crate sits two levels below the repository root")
    .to_path_buf()
}

/// Every `.vlt` the compiler is written in, read from `seed/GENERATION`.
///
/// The list rather than a glob, for `seed/GENERATION`'s own reason: it is the
/// only copy of which crates are generated, and a second walk would be a second
/// copy free to drift.
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

/// The source with every `|>` collapsed, as text.
///
/// Parse, rewrite, print — the printer is what turns the rewritten AST back into
/// `.vlt`, which is why doc 97 §10.43's rule matters here more than anywhere: a
/// printer that drops a field would silently delete it from every migrated file.
fn migrated(text: &str) -> Option<String> {
  let mut program = voltaic_parser::parse_source(text).ok()?;
  rewrite_program(&mut program, Pass::Pipelines);
  Some(voltaic_fmt::format(&program, text))
}

#[test]
fn the_corpus_is_the_size_generation_one_was() {
  let found = sources();
  assert!(
    found.len() > 100,
    "the compiler is written in 105 `.vlt` files and this found {}; a shrinking \
     corpus means the walk stopped finding them, not that the work got smaller",
    found.len()
  );
}

#[test]
fn collapsing_every_pipeline_changes_no_program() {
  let updating = std::env::var("VOLTAIC_UPDATE").is_ok();
  let mut rejected = Vec::new();
  let mut written = Vec::new();
  let mut touched = 0;
  let mut stages = 0;
  let mut survived = Vec::new();
  for path in sources() {
    let text = std::fs::read_to_string(&path).expect("a listed source should be readable");
    if !text.contains("|>") {
      continue;
    }
    let Some(after) = migrated(&text) else {
      rejected.push(format!("{}: the original does not parse", path.display()));
      continue;
    };
    touched += 1;
    if after.contains("|>") {
      let line = after
        .lines()
        .find(|l| l.contains("|>"))
        .unwrap_or_default()
        .trim();
      survived.push(format!(
        "{}: {}",
        path.display(),
        &line[..line.len().min(90)]
      ));
    } else {
      stages += 1;
    }
    if after == text {
      continue;
    }
    if updating {
      // `apply` gates before it writes, so a rejected file stays untouched even
      // here — leaf 3's "the gate runs before the write, not after".
      match voltaic_driver::apply(&path, &after) {
        Ok(verdict) if verdict.passed() => written.push(path.display().to_string()),
        Ok(verdict) => rejected.push(format!("{}: {}", path.display(), verdict.explain())),
        Err(e) => rejected.push(format!("{}: {e}", path.display())),
      }
      continue;
    }
    let verdict = gate(&path, &text, &after);
    if !verdict.passed() {
      rejected.push(format!("{}: {}", path.display(), verdict.explain()));
    }
  }
  assert!(
    touched > 50,
    "only {touched} files hold a `|>`, which is too few to mean anything — the \
     compiler's own source is written in pipelines"
  );
  // The reach, reported rather than assumed. Until the formatter printed a
  // zero-argument `Dot` as `recv.name ()` the pass had to *decline* every
  // no-argument stage — 450 of 473 of them — because `recv.name` reparses as a
  // field access, so the rewrite was not a respelling. The count is here
  // because "the gate passes" is equally true of a pass that rewrites nothing.
  println!("collapsed every `|>` in {stages} of {touched} files that hold one");
  let quoted: Vec<_> = survived.iter().filter(|s| !s.contains('"')).collect();
  assert!(
    quoted.is_empty(),
    "every `|>` left in a migrated file should be the operator written as data \
     — the AST's spelling, the lexer's table, a diagnostic's help text, a test \
     fixture — and these are on lines with no string literal at all, so they \
     are pipeline *expressions* the pass declined: {quoted:#?}\n\nSurvivors in \
     full:\n{survived:#?}"
  );
  assert!(
    rejected.is_empty(),
    "collapsing `|>` to `.` changed what these files lower to, so the rewrite is \
     not a respelling: {rejected:#?}"
  );
  assert!(
    written.is_empty(),
    "migrated {} files. The generated Rust is byte-identical by construction — \
     `apply` gated each one before writing it — so review the `.vlt` diff and \
     re-run without VOLTAIC_UPDATE:\n{written:#?}",
    written.len()
  );
}

/// The one line that unblocked the no-argument half, pinned where it can be
/// read: a collapsed stage with no arguments must print as a **call**.
///
/// `s.len` reparses as `Expr::Field`, and a field read is not a method call, so
/// the pass had to decline every such stage — 450 of the corpus's 473. Printing
/// `s.len ()` makes the rewritten source say what the AST means, and the
/// surface already admitted that spelling: `recv.name ()` parses as a
/// zero-argument `Dot` and lowers to `recv.name()`.
///
/// This is the whole of the fix. No `std` catalogue, no inference, no
/// classification — `crates/voltaic-driver/tests/std_receivers.rs` measures why
/// none of those would have worked.
#[test]
fn a_collapsed_no_argument_stage_prints_as_a_call() {
  let after = migrated("f : &str -> usize\nf s = s |> .len\n").expect("should parse");
  assert!(
    after.contains("s.len ()"),
    "a no-argument stage must print its parentheses or it reparses as a field \
     access: {after}"
  );
  assert!(
    !after.contains("|>"),
    "and the pipeline must be gone, not merely rewritten alongside: {after}"
  );
}

/// The gate can fail, demonstrated rather than assumed.
///
/// A pass asserted over 105 files that all pass proves nothing unless the
/// assertion can go red. This feeds the gate a migration that *is* wrong — the
/// method renamed — and requires it to say so.
#[test]
fn the_gate_rejects_a_rewrite_that_changes_the_call() {
  let before = "f : u64 -> u64\nf n = n |> .max 3\n";
  let wrong = "f : u64 -> u64\nf n = n.min 3\n";
  let verdict = gate(&PathBuf::from("probe.vlt"), before, wrong);
  assert!(
    matches!(verdict, Gate::Differs { .. }),
    "renaming the method must fail the gate: {}",
    verdict.explain()
  );
}

//! Verbatim macro bodies through the formatter.
//!
//! A body is one Rust token tree, so the printer copies it and never reflows
//! it. What it does canonicalise is the gap: `name! { … }` takes a space, and
//! `name!(…)` / `name![…]` take none, which is what rustfmt writes too. The
//! indented form keeps its block, re-padded one level under the head.
//!
//! The fixture pair is `fmt/macro_bodies.vlt` against
//! `fmt/macro_bodies.canonical.vlt`, a file rather than a literal for
//! AGENTS.md's reason.

use std::path::PathBuf;

fn fixtures() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fmt")
}

fn formatted(stem: &str) -> (String, String) {
  let path = fixtures().join(format!("{stem}.vlt"));
  let source =
    std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("{} should exist", path.display()));
  let program = voltaic_parser::parse_source(&source)
    .unwrap_or_else(|e| panic!("{} should parse: {e:?}", path.display()));
  let printed = voltaic_fmt::format(&program, &source);
  (source, printed)
}

/// A program's debug form with every byte position blanked: spans, and the
/// per-line offsets a verbatim body carries for the source map.
fn without_positions(program: &voltaic_ast::Program) -> String {
  let text = format!("{program:?}");
  let mut out = String::with_capacity(text.len());
  let mut rest = text.as_str();
  loop {
    let span = rest.find("span: (").map(|at| (at, "span: (", ')'));
    let lines = rest.find("lines: [").map(|at| (at, "lines: [", ']'));
    let next = match (span, lines) {
      (Some(a), Some(b)) => Some(if a.0 < b.0 { a } else { b }),
      (a, b) => a.or(b),
    };
    let Some((at, key, close)) = next else { break };
    out.push_str(&rest[..at]);
    out.push_str(key);
    out.push('_');
    rest = &rest[at + key.len()..];
    match rest.find(close) {
      Some(end) => rest = &rest[end + 1..],
      None => break,
    }
  }
  out.push_str(rest);
  out
}

#[test]
fn each_body_reaches_its_canonical_form() {
  let (_, printed) = formatted("macro_bodies");
  let path = fixtures().join("macro_bodies.canonical.vlt");
  if std::env::var_os("VOLTAIC_UPDATE").is_some() {
    std::fs::write(&path, &printed).expect("should write the expectation");
    panic!("updated {} — review the diff", path.display());
  }
  let expected = std::fs::read_to_string(&path)
    .unwrap_or_else(|_| panic!("{} is missing; run with VOLTAIC_UPDATE=1", path.display()));
  assert_eq!(
    printed,
    expected,
    "the output differs from {}",
    path.display()
  );
}

#[test]
fn formatting_preserves_the_program() {
  let (source, printed) = formatted("macro_bodies");
  let before = voltaic_parser::parse_source(&source).expect("the fixture should parse");
  let after = voltaic_parser::parse_source(&printed).expect("the output should re-parse");
  assert_eq!(
    without_positions(&before),
    without_positions(&after),
    "formatting changed a macro body"
  );
}

#[test]
fn formatting_is_idempotent() {
  let (_, once) = formatted("macro_bodies");
  let reparsed = voltaic_parser::parse_source(&once).expect("should re-parse");
  let twice = voltaic_fmt::format(&reparsed, &once);
  assert_eq!(once, twice, "formatting a macro body is not idempotent");
}

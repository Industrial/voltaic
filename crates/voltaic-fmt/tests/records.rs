//! The one canonical layout of a record, declared or built.
//!
//! `{` on the header's line, one field per line with a trailing comma, and `}`
//! dedented back to the header. Since 0.3.0 it is written at every width: a
//! record is never joined onto one line, and the comma-first layout Haskell
//! writes is reflowed rather than kept.

use std::path::PathBuf;

fn fixtures() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fmt")
}

/// The fixture's source and its formatted form.
fn formatted(stem: &str) -> (String, String) {
  let path = fixtures().join(format!("{stem}.vlt"));
  let source =
    std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("{} should exist", path.display()));
  let program = voltaic_parser::parse_source(&source)
    .unwrap_or_else(|e| panic!("{} should parse: {e:?}", path.display()));
  let printed = voltaic_fmt::format(&program, &source);
  (source, printed)
}

/// A program's debug form with every span blanked.
fn without_spans(program: &voltaic_ast::Program) -> String {
  let text = format!("{program:?}");
  let mut out = String::with_capacity(text.len());
  let mut rest = text.as_str();
  while let Some(at) = rest.find("span: (") {
    out.push_str(&rest[..at]);
    out.push_str("span: _");
    rest = &rest[at + "span: (".len()..];
    match rest.find(')') {
      Some(close) => rest = &rest[close + 1..],
      None => break,
    }
  }
  out.push_str(rest);
  out
}

#[test]
fn every_record_reaches_its_canonical_form() {
  let (_, printed) = formatted("records");
  let path = fixtures().join("records.canonical.vlt");
  if std::env::var_os("VOLTAIC_UPDATE").is_some() {
    std::fs::write(&path, &printed).expect("should write the expectation");
    panic!("updated {} — review the diff", path.display());
  }
  let expected = std::fs::read_to_string(&path).unwrap_or_else(|_| {
    panic!(
      "{} is missing; run with VOLTAIC_UPDATE=1 to create it",
      path.display()
    )
  });
  assert_eq!(
    printed,
    expected,
    "the output differs from {}",
    path.display()
  );
}

/// No line opens with a comma, and no line holds a field after a `{`.
#[test]
fn no_record_is_written_comma_first_or_on_one_line() {
  let (_, printed) = formatted("records");
  for line in printed
    .lines()
    .filter(|l| !l.trim_start().starts_with("--"))
  {
    assert!(
      !line.trim_start().starts_with(','),
      "`{line}` opens with a comma"
    );
    if line.contains('"') {
      continue;
    }
    if let Some(at) = line.find('{') {
      assert!(
        matches!(line[at + 1..].trim(), "" | "}"),
        "`{line}` holds a field on the brace's line"
      );
    }
  }
}

#[test]
fn formatting_preserves_the_program() {
  let (source, printed) = formatted("records");
  let before = voltaic_parser::parse_source(&source).expect("the fixture should parse");
  let after = voltaic_parser::parse_source(&printed).expect("the output should re-parse");
  assert_eq!(
    without_spans(&before),
    without_spans(&after),
    "laying out a record changed the program"
  );
}

#[test]
fn formatting_is_idempotent() {
  let (_, once) = formatted("records");
  let reparsed = voltaic_parser::parse_source(&once).expect("should re-parse");
  let twice = voltaic_fmt::format(&reparsed, &once);
  assert_eq!(once, twice, "formatting a record is not idempotent");
}

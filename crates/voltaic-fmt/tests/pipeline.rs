//! Doc 03 §5.3's pipeline, in its one canonical layout.
//!
//! `|>` is the one binary operator with a layout of its own, because it is the
//! one the lexer continues across lines: `next_line_continues` looks ahead for
//! a line opening with `|>` and swallows it before the off-side rule can see
//! it. So a broken pipeline has exactly one spelling that parses — one stage
//! per continuation line, indented beneath the receiver.
//!
//! Since 0.3.0 that is the **only** spelling the formatter writes. It used to
//! break a pipeline only past `PIPELINE_WIDTH`, rustfmt's `chain_width`, and
//! to join a broken one that fitted. A pipeline is a sequence of stages, and
//! the layout now says so at every width.

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

/// Compare against the recorded expectation, or rewrite it and fail.
fn against_canonical(stem: &str, printed: &str) {
  let path = fixtures().join(format!("{stem}.canonical.vlt"));
  if std::env::var_os("VOLTAIC_UPDATE").is_some() {
    std::fs::write(&path, printed).expect("should write the expectation");
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

/// A program's debug form with every span blanked — leaf 5's normaliser.
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

/// Every line that opens a pipeline continuation, with its indentation.
fn continuations(text: &str) -> Vec<&str> {
  text
    .lines()
    .filter(|line| line.trim_start().starts_with("|>"))
    .collect()
}

#[test]
fn every_pipeline_reaches_its_canonical_form() {
  let (_, printed) = formatted("pipeline");
  against_canonical("pipeline", &printed);
}

/// Every pipeline breaks at every stage, whatever its width.
///
/// Thirteen continuation lines: four for the clause-level pipeline, two for
/// `short`, one for `single`, four for `kept` and two for `half`. Until 0.3.0
/// only the pipelines wider than `PIPELINE_WIDTH` broke and this was twelve
/// with the short ones at zero.
#[test]
fn every_pipeline_is_broken_at_every_stage() {
  let (_, printed) = formatted("pipeline");
  assert_eq!(
    continuations(&printed).len(),
    13,
    "every pipeline should break at every stage:\n{printed}"
  );
  for line in printed.lines() {
    let code = line.split("--").next().unwrap_or(line).trim_start();
    assert!(
      !code.contains("|>") || code.starts_with("|>"),
      "`{line}` holds a `|>` that is not the first token of its line"
    );
  }
}

/// A continuation line sits one level in from the statement it continues.
///
/// Two spaces at the top level and four inside `main`, which is §3 row 11's
/// one level and the layout `examples/application.vlt` writes.
#[test]
fn a_continuation_is_one_level_in_from_its_statement() {
  let (_, printed) = formatted("pipeline");
  for line in continuations(&printed) {
    let indent = line.len() - line.trim_start().len();
    assert!(
      indent == 2 || indent == 4,
      "`{line}` is indented {indent} spaces, which is not one level in from a \
       statement at depth 0 or 1"
    );
  }
}

#[test]
fn formatting_preserves_the_program() {
  let (source, printed) = formatted("pipeline");
  let before = voltaic_parser::parse_source(&source).expect("the fixture should parse");
  let after = voltaic_parser::parse_source(&printed).expect("the output should re-parse");
  assert_eq!(
    without_spans(&before),
    without_spans(&after),
    "breaking a pipeline changed the program"
  );
}

/// The half that matters most: a broken pipeline stays broken, and a joined
/// one stays joined. A formatter that oscillates here would rewrite a file on
/// every run.
#[test]
fn formatting_is_idempotent() {
  let (_, once) = formatted("pipeline");
  let reparsed = voltaic_parser::parse_source(&once).expect("should re-parse");
  let twice = voltaic_fmt::format(&reparsed, &once);
  assert_eq!(once, twice, "formatting a pipeline is not idempotent");
}

//! Verbatim macro bodies: `name! { … }`, `name! ( … )`, `name! [ … ]`, and an
//! indented block under a bare `name!`, each lexed as one `MacroBody` token.
//!
//! The body is a Rust token tree, so the scanner balances brackets the way
//! Rust does: a bracket inside a string, a char, a raw string or a comment does
//! not count, and a lifetime is not an unclosed char. Sources are files under
//! `tests/macro_body/`, per AGENTS.md.

use std::path::PathBuf;
use voltaic_lexer::{TokenKind, lex};

fn fixture(name: &str) -> String {
  let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("tests/macro_body")
    .join(name);
  std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("{} should exist", path.display()))
}

/// One `MacroBody` token: its opener, code, form and span.
type Body = (char, String, bool, (usize, usize));

fn bodies(src: &str) -> Vec<Body> {
  lex(src)
    .expect("should lex")
    .into_iter()
    .filter_map(|t| match t.kind {
      TokenKind::MacroBody {
        open,
        code,
        indented,
      } => Some((open, code, indented, t.span)),
      _ => None,
    })
    .collect()
}

#[test]
fn a_braced_body_is_one_token_balanced_as_rust_balances_it() {
  let src = fixture("braced.vlt");
  let found = bodies(&src);
  assert_eq!(found.len(), 1, "one body: {found:?}");
  let (open, code, indented, span) = &found[0];
  let opener = src.find("! {").expect("the fixture has a body") + 2;
  let closer = src.find("}\ndone").expect("the fixture closes its body");
  assert_eq!(*open, '{');
  assert!(!indented);
  assert_eq!(*span, (opener, closer + 1), "the token covers its brackets");
  assert_eq!(
    code,
    &src[opener + 1..closer],
    "the body is copied verbatim"
  );
}

#[test]
fn the_tokens_after_a_body_still_lex() {
  let src = fixture("braced.vlt");
  let toks: Vec<TokenKind> = lex(&src)
    .expect("should lex")
    .into_iter()
    .map(|t| t.kind)
    .collect();
  assert!(toks.contains(&TokenKind::Ident("done".into())), "{toks:?}");
}

#[test]
fn an_indented_body_is_one_token_and_voltaic_comments_inside_are_kept() {
  let src = fixture("indented.vlt");
  let found = bodies(&src);
  assert_eq!(found.len(), 1, "one body: {found:?}");
  let (open, code, indented, span) = &found[0];
  assert_eq!(*open, '{');
  assert!(indented);
  assert!(code.starts_with("div {"), "dedented: {code:?}");
  assert!(
    code.contains("-- not a Voltaic comment"),
    "a `--` is Rust text here"
  );
  let after_bang = src.find("rsx!").unwrap() + 4;
  assert_eq!(span.0, after_bang, "the token starts where the head ends");
  assert_eq!(code.lines().count(), 4, "one line of code per body line");
}

#[test]
fn an_unclosed_body_names_its_opener() {
  let src = fixture("unclosed.vlt");
  let err = lex(&src).expect_err("an unclosed body is an error");
  assert_eq!(err.code, voltaic_diag::Code::UnbalancedDelimiter);
  let opener = src.find("! {").unwrap() + 2;
  assert_eq!(err.span, (opener, opener + 1));
}

#[test]
fn negation_a_spaced_bang_and_a_bare_bang_before_a_dedent_are_not_bodies() {
  let src = fixture("not_a_body.vlt");
  assert!(bodies(&src).is_empty());
}

/// A bare `name!` with no deeper line after it leaves the cursor where it was,
/// so the blank line it looked past is still trivia on the next token.
#[test]
fn a_bare_bang_keeps_the_blank_line_after_it() {
  let src = fixture("not_a_body.vlt");
  let toks = lex(&src).expect("should lex");
  let h = toks
    .iter()
    .find(|t| t.kind == TokenKind::Ident("h".into()))
    .expect("the fixture names `h`");
  assert!(
    h.leading
      .iter()
      .any(|t| t.kind == voltaic_lexer::TriviaKind::BlankLines),
    "the blank line before `h` was swallowed: {:?}",
    h.leading
  );
}

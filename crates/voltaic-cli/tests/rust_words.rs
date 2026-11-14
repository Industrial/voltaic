//! Identifiers that collide with Rust's keywords.
//!
//! Found while measuring row 99 wave 4's keyword cull, and the measurement is
//! why the file exists: **a reserved word has two jobs.** It either spells a
//! Voltaic construct, or it shields the generated Rust by stopping a user
//! naming a binding after one of Rust's own keywords. Doc 97 §1.4's target of
//! eleven counts only the first.
//!
//! `crates/voltaic-cli/tests/keyword_usage.rs` carries that argument and the
//! numbers. This file carries the escaping that makes culling the second job
//! safe — without it, the moment `struct` stops being reserved a program naming
//! a binding `struct` emits invalid Rust.

mod support;

/// A Voltaic identifier that is a Rust keyword is escaped, so the generated
/// line is one rustc can read.
///
/// **This was a live defect**, found while measuring row 99 wave 4's keyword
/// cull. `ref` is a Rust keyword that Voltaic does not reserve, so
///
///     shadowed =
///       ref = 7
///       ref
///
/// emitted `let ref = 7;` — and rustc rejected it with Voltaic's own help text
/// saying *"this is on a line Voltaic generated, so it has no place in your
/// source"*, the compiler admitting it produced something it cannot attribute.
/// A program Voltaic accepts and rustc rejects is what the charter forbids.
///
/// `voltaic_ast::rust_word` is the fix, applied at the two places an identifier
/// reaches the output: `Pattern::rust_text` for a binder and `binding_text` for
/// a read. Both, because escaping one gives `let r#ref = 7; ref`, which does
/// not compile either — a partial fix here is worse than none.
///
/// It is also wave 4's **prerequisite**: the keyword set shields the output as
/// well as spelling constructs, so culling `struct` from it without this would
/// let a program name a binding `struct` and emit invalid Rust.
/// `crates/voltaic-cli/tests/keyword_usage.rs` carries that argument.
#[test]
fn an_identifier_that_is_a_rust_keyword_is_escaped() {
  let rust = support::transpile_source(
    "rust_keyword_binder",
    "shadowed : u64\nshadowed =\n  ref = 7\n  ref\n",
  );
  assert!(
    rust.contains("let r#ref = 7"),
    "the binder must be escaped: {rust}"
  );
  assert!(
    rust.contains("r#ref\n"),
    "and so must the read, or the two disagree and neither compiles: {rust}"
  );
}

/// And an ordinary name that merely *contains* a keyword is left alone.
#[test]
fn an_ordinary_identifier_is_not_escaped() {
  let rust = support::transpile_source(
    "ordinary_binder",
    "plain : u64\nplain =\n  reference = 3\n  reference\n",
  );
  assert!(
    !rust.contains("r#"),
    "`reference` is not `ref`; escaping by substring would be a new bug: {rust}"
  );
}

//! The refinement fragment, and what it refuses.
//!
//! Plan 97 (`docs/95 - 97 - Refinements.md`) leaf 1. Doc 86 §7.3 states the
//! admitted fragment — quantifier-free linear integer arithmetic over
//! parameters, their fields and `.len` — and doc 86 §2 clause 5 makes the bound
//! a *condition of admission* rather than a quality target: an inference that
//! does not terminate on every input is not admissible, so anything outside the
//! fragment must be a diagnostic.
//!
//! # The refusal is structural, not a failed attempt
//!
//! §1.4 of the plan is emphatic that a timeout must never be a pass. The
//! stronger version of that is to make the *representation* the fragment: a
//! `Term` is a map from names to integer coefficients plus a constant, so a
//! product of two variables cannot be built at all. `term_of` therefore refuses
//! on the way in — `VLT0801` before any solve runs — and the refusal depends on
//! the shape of the expression rather than on how a solver behaved.
//!
//! [`a_variable_product_is_refused_rather_than_attempted`] is plan 97 §3's
//! fourth proof target, and that plan calls it *"more important than the three
//! positive cases"*: a fragment checker that quietly accepts a predicate it
//! cannot decide is the clause-5 failure.

use voltaic_lattice::{Comparison, Predicate, Refusal, Term, read_predicate, term_of};
use voltaic_parser::parse_source;

/// Parse a single expression by wrapping it in the smallest definition that holds one.
fn expression(text: &str) -> voltaic_ast::Expr {
  let source = format!("probe : bool\nprobe = {text}\n");
  let program = parse_source(&source).expect("the probe should parse");
  let item = program.items.first().expect("one item");
  let voltaic_ast::Item::Fn(declaration) = item else {
    panic!("the probe should be a function");
  };
  let clause = declaration.clauses.first().expect("one clause");
  let block = clause.body.plain().expect("a plain body");
  block.stmts.first().expect("one statement").expr.clone()
}

fn predicate(text: &str) -> Result<Predicate, Refusal> {
  read_predicate(&expression(text))
}

fn term(text: &str) -> Result<Term, Refusal> {
  term_of(&expression(text))
}

#[test]
fn a_comparison_against_a_literal_is_in_the_fragment() {
  let read = predicate("i < 3").expect("should be admitted");
  assert_eq!(read.to_string(), "i < 3");
}

#[test]
fn a_length_comparison_is_in_the_fragment() {
  let read = predicate("i < xs.len").expect("should be admitted");
  assert_eq!(read.to_string(), "i < len xs");
}

#[test]
fn a_conjunction_of_comparisons_is_in_the_fragment() {
  let read = predicate("i >= 0 && i < xs.len").expect("should be admitted");
  assert_eq!(read.to_string(), "i >= 0 and i < len xs");
  let mentioned = read.mentions();
  assert!(mentioned.contains(&"i".to_string()));
  assert!(mentioned.contains(&"len xs".to_string()));
}

#[test]
fn linear_arithmetic_is_in_the_fragment() {
  assert_eq!(term("i + 1").expect("admitted").to_string(), "i + 1");
  assert_eq!(term("2 * i").expect("admitted").to_string(), "2*i");
  assert_eq!(term("i * 2").expect("admitted").to_string(), "2*i");
  assert_eq!(term("i - i").expect("admitted").to_string(), "0");
}

/// Plan 97 §3's fourth target, and the one that proves the refusal works.
#[test]
fn a_variable_product_is_refused_rather_than_attempted() {
  let refusal = term("i * j").expect_err("a variable product is outside the fragment");
  assert_eq!(
    refusal.rule,
    voltaic_lattice::RULE_PRODUCT,
    "the refusal must name the rule, not just fail"
  );
  assert!(
    refusal.subterm.contains('*'),
    "and it must name the offending subterm: {refusal:?}"
  );
}

#[test]
fn division_is_refused() {
  let refusal = term("i / 2").expect_err("division is outside the fragment");
  assert_eq!(refusal.rule, voltaic_lattice::RULE_DIVISION);

  let modulo = term("i % 2").expect_err("so is the remainder");
  assert_eq!(modulo.rule, voltaic_lattice::RULE_DIVISION);
}

#[test]
fn a_function_call_in_a_predicate_is_refused() {
  // Doc 87 §22.1 declines user function calls in a predicate: inlining them
  // would need a termination proof, and an uninterpreted encoding leaves the
  // fragment. Either way the answer is a diagnostic.
  let refusal = term("f i").expect_err("a call is outside the fragment");
  assert_eq!(refusal.rule, voltaic_lattice::RULE_CALL);
}

#[test]
fn a_non_integer_literal_is_refused() {
  let refusal = term("1.5").expect_err("the fragment is integer arithmetic");
  assert_eq!(refusal.rule, voltaic_lattice::RULE_SHAPE);
}

#[test]
fn a_refusal_always_carries_a_span_and_a_rule() {
  for text in ["i * j", "i / 2", "f i", "1.5"] {
    let refusal = term(text).expect_err("should be refused");
    assert!(
      refusal.span.1 >= refusal.span.0,
      "{text} refused without a span: {refusal:?}"
    );
    assert!(
      !refusal.rule.is_empty(),
      "{text} refused without naming a rule"
    );
  }
}

#[test]
fn a_field_path_is_a_distinct_unknown_from_its_root() {
  let read = predicate("p.x < p.y").expect("should be admitted");
  let mentioned = read.mentions();
  assert!(mentioned.contains(&"p.x".to_string()));
  assert!(mentioned.contains(&"p.y".to_string()));
  assert_eq!(mentioned.len(), 2, "two fields are two unknowns");
}

#[test]
fn a_sequence_and_its_length_are_different_unknowns() {
  assert_ne!(Term::length_of("xs"), Term::variable("xs"));
}

#[test]
fn a_comparison_has_a_negation_and_it_is_its_own_inverse() {
  // There is no `Not` constructor: comparisons are closed under negation, so a
  // negated predicate is expressible without one and the representation stays
  // exactly the fragment.
  for op in [
    Comparison::Below,
    Comparison::AtMost,
    Comparison::Same,
    Comparison::Different,
    Comparison::AtLeast,
    Comparison::Above,
  ] {
    assert_eq!(op.negated().negated(), op);
    assert_ne!(op.negated(), op);
  }
}

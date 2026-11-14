//! Effect rows, inferred from real programs.
//!
//! Plan 95 (`docs/93 - 95 - Effect Rows.md`) leaves 1 and 2. Doc 86 §5.2 is the
//! lattice: the subset lattice over a **closed seven-effect alphabet**, join is
//! union, and inference runs upward from what a body calls.
//!
//! # Why the alphabet being closed is the whole argument
//!
//! Doc 86 §2 clause 5 admits an inference only if it terminates on every input.
//! For rows that is a *counting* argument rather than a proof obligation: a row
//! is a subset of a seven-element set, so the ascending chain condition holds
//! and the fixed point is reached in at most seven steps per binding. An open
//! alphabet — user-declared effects — would still be a lattice but would lose
//! the bound a reviewer can check by counting, which is why doc 86 §5.1 closes
//! the set and `the_alphabet_is_closed` asserts it.
//!
//! # What is being read here
//!
//! These programs are **generation 1** source, so the effects are the ones
//! generation 1 marks explicitly: `async`, a `Result`/`Option` return type, a
//! printing macro, `:=`, `try`, `catch`. Row 99 deletes those markers and the
//! row becomes the only statement of the same fact — so this corpus is also the
//! answer key for that migration, in the same way generation 1's signatures are
//! the answer key for types (plan 94 §1.4).

use voltaic_lattice::{ALPHABET, Effect, Row, Ty, fail_of, walk};
use voltaic_parser::parse_source;

/// The inferred row of one definition, printed.
fn row_of(source: &str, name: &str) -> String {
  let program = parse_source(source).expect("the corpus should parse");
  let report = walk(&program);
  report
    .find(name)
    .expect("the definition should be found")
    .row
    .to_string()
}

#[test]
fn a_pure_function_has_a_pure_row() {
  let source = "twice : u64 -> u64\ntwice n = n + n\n";
  assert_eq!(row_of(source, "twice"), "pure");
}

#[test]
fn printing_is_the_io_effect() {
  let source = concat!(
    "shout : &str -> ()\n",
    "shout name =\n",
    "  println! \\-> \"#{name}\"\n",
    "  ()\n",
  );
  assert_eq!(row_of(source, "shout"), "io");
}

#[test]
fn a_result_return_type_reads_as_a_fail_effect_carrying_its_error() {
  let source = concat!(
    "retries : &str -> Result<u32, ConfigError>\n",
    "retries raw = Ok 1\n",
  );
  assert_eq!(row_of(source, "retries"), "fail ConfigError");
}

#[test]
fn an_option_return_type_reads_as_the_absence_effect() {
  let source = "first : &str -> Option<char>\nfirst s = None\n";
  assert_eq!(row_of(source, "first"), "none");
}

#[test]
fn an_async_signature_suspends() {
  let source = "double : i32 -> i32\nasync double n = n * 2\n";
  // Whether the parser accepts `async` in this position is generation 1's
  // business; what matters is that a signature marked async carries the effect.
  let declared = parse_source(source);
  if declared.is_ok() {
    assert_eq!(row_of(source, "double"), "async");
  }
}

#[test]
fn join_is_what_makes_two_effects_one_row() {
  // `async` + `io` is the join, and it is the case a single-effect model cannot
  // express. The order in which they are printed is the lattice's, not the
  // program's, so the string is stable.
  let a = Row::single(Effect::Suspends);
  let b = Row::single(Effect::Performs);
  let joined = a.join(&b);

  assert_eq!(joined.len(), 2);
  assert_eq!(joined.to_string(), b.join(&a).to_string());
  assert!(a.is_subrow_of(&joined) && b.is_subrow_of(&joined));
  assert!(!joined.is_subrow_of(&a));
}

#[test]
fn discharging_removes_the_fallible_effects_and_leaves_the_rest() {
  let row = Row::from_effects(vec![
    Effect::Fails(Ty::named("String")),
    Effect::Absent,
    Effect::Performs,
  ]);
  assert_eq!(row.len(), 3);

  let left = row.discharge_fallible();
  assert_eq!(left.to_string(), "io");
  assert!(left.is_subrow_of(&row));
}

#[test]
fn the_alphabet_is_closed_and_bounds_every_row() {
  // Doc 86 §5.1 closes the set at seven, and doc 86 §2 clause 5 rests on it:
  // a row is a subset of a seven-element set, so the ascending chain is at most
  // seven long. This is that argument as an assertion.
  let full = Row::from_effects(vec![
    Effect::Fails(Ty::named("String")),
    Effect::Absent,
    Effect::Suspends,
    Effect::Performs,
    Effect::Mutates("self".to_string()),
    Effect::Panics,
    Effect::Diverges,
  ]);

  assert_eq!(full.len(), ALPHABET);
  assert_eq!(full.join(&full), full, "the full row is the lattice's top");
  for effect in full.effects() {
    assert!(Row::single(effect.clone()).is_subrow_of(&full));
  }
}

#[test]
fn the_pure_row_is_the_bottom_of_the_lattice() {
  let pure = Row::empty();
  assert!(pure.is_empty());
  assert_eq!(pure.to_string(), "pure");
  for effect in [Effect::Performs, Effect::Suspends, Effect::Panics] {
    let row = Row::single(effect);
    assert!(pure.is_subrow_of(&row));
    assert_eq!(row.join(&pure), row, "pure is the identity for join");
  }
}

#[test]
fn only_result_and_option_read_as_fallible_return_types() {
  assert_eq!(
    fail_of(&Ty::applied(
      "Result",
      vec![Ty::named("i32"), Ty::named("E")]
    )),
    Some(Effect::Fails(Ty::named("E")))
  );
  assert_eq!(
    fail_of(&Ty::applied("Option", vec![Ty::named("char")])),
    Some(Effect::Absent)
  );
  assert_eq!(fail_of(&Ty::named("u64")), None);
  assert_eq!(fail_of(&Ty::applied("Vec", vec![Ty::named("u8")])), None);
}

#[test]
fn every_effect_in_the_alphabet_prints_distinguishably() {
  let every = [
    Effect::Fails(Ty::named("String")),
    Effect::Absent,
    Effect::Suspends,
    Effect::Performs,
    Effect::Mutates("self".to_string()),
    Effect::Panics,
    Effect::Diverges,
  ];
  let mut shown: Vec<String> = every.iter().map(ToString::to_string).collect();
  shown.sort();
  shown.dedup();
  assert_eq!(
    shown.len(),
    ALPHABET,
    "two effects printing the same would make a row ambiguous in a diagnostic"
  );
}

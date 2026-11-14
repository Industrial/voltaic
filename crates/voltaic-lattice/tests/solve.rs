//! Inference against real programs, with the author's own signature as the oracle.
//!
//! Plan 94 (`docs/92 - 94 - The Lattice Crate.md`) leaf 5. §1.4 says the corpus
//! is built now rather than later for one reason: **generation 1's hand-written
//! signatures are a free answer key, and they disappear at row 99.** Every
//! definition in the corpus states its own type, so for each one there are two
//! numbers to compare — what the author wrote, and what the solver worked out.
//!
//! # What a failure here means
//!
//! A disagreement is a Lattice bug, not a user error. Doc 90 §1.4 makes that the
//! testable form of "inference is not checking": the solver computes what the
//! emitter writes down, so if it computes something the author contradicted,
//! the solver is wrong.
//!
//! # What this corpus does not yet cover
//!
//! Member access is **deferred** by the driver rather than solved, because a
//! field needs a declaration environment and that is row 98's. Anything whose
//! type depends on a field or a method therefore infers as an unconstrained
//! hole, and the tests below say so explicitly rather than asserting a type the
//! engine has no way to know.

use voltaic_lattice::{Ty, solve, walk};
use voltaic_parser::parse_source;

/// The inferred type of one definition, resolved through the solution.
fn inferred(source: &str, name: &str) -> String {
  let program = parse_source(source).expect("the corpus should parse");
  let report = walk(&program);
  let outcome = solve(&report.store);
  assert!(
    outcome.is_solved(),
    "{name} should solve, but {} constraint(s) failed: {:?}",
    outcome.failures.len(),
    outcome.failures
  );
  let found = report.find(name).expect("the definition should be found");
  outcome.resolved(&found.ty).to_string()
}

#[test]
fn a_literal_body_fixes_the_return_type() {
  let source = "greeting : &str\ngreeting = \"hello\"\n";
  assert_eq!(inferred(source, "greeting"), "&<str>");
}

#[test]
fn a_bool_body_infers_bool() {
  // A signature is still written, because generation 1 has no module-scope
  // binding without `const` — `ready = true` alone is VLT0106. Row 99 is what
  // makes the signature optional; until then the oracle and the requirement
  // happen to coincide.
  let source = "ready : bool\nready = true\n";
  assert_eq!(inferred(source, "ready"), "bool");
}

#[test]
fn a_parameter_flows_from_the_written_signature_into_the_body() {
  let source = "echo : u64 -> u64\necho n = n\n";
  assert_eq!(inferred(source, "echo"), "u64 -> u64");
}

#[test]
fn a_comparison_infers_bool_and_ties_its_operands_together() {
  let source = "big : u64 -> bool\nbig n = n > 10\n";
  assert_eq!(inferred(source, "big"), "u64 -> bool");
}

#[test]
fn arithmetic_ties_both_sides_to_the_result() {
  let source = "twice : u64 -> u64\ntwice n = n + n\n";
  assert_eq!(inferred(source, "twice"), "u64 -> u64");
}

#[test]
fn a_guard_ladder_unifies_every_arm() {
  let source = concat!(
    "tier : u64 -> &'static str\n",
    "tier spend\n",
    "  | spend > 10 = \"gold\"\n",
    "  | otherwise = \"basic\"\n",
  );
  // `&'static str` reads as `&<str>`: a lifetime is not part of the type for
  // inference purposes, because Lattice does not infer lifetimes (doc 86 §6.3).
  assert_eq!(inferred(source, "tier"), "u64 -> &<str>");
}

#[test]
fn an_ascription_pins_a_type_the_body_could_not_supply() {
  let source = "answer : u64\nanswer = 42 : u64\n";
  assert_eq!(inferred(source, "answer"), "u64");
}

#[test]
fn the_inferred_type_agrees_with_every_written_signature_in_the_corpus() {
  // The answer key. Each program states its own type; inference must not
  // contradict it. A mismatch is a Lattice bug (doc 90 §1.4).
  let corpus = [
    "f : u64 -> u64\nf n = n\n",
    "g : bool -> bool\ng b = b\n",
    "h : u64 -> bool\nh n = n == n\n",
    "k : char -> char\nk c = c\n",
  ];

  for source in corpus {
    let program = parse_source(source).expect("should parse");
    let report = walk(&program);
    let outcome = solve(&report.store);
    assert!(outcome.is_solved(), "{source:?} should solve");

    for definition in &report.definitions {
      let Some(written) = definition.written.as_ref() else {
        continue;
      };
      let found = outcome.resolved(&definition.ty);
      assert_eq!(
        found.to_string(),
        written.to_string(),
        "inference contradicted the author in {source:?}: \
         wrote {written}, inferred {found}"
      );
    }
  }
}

#[test]
fn a_contradiction_between_body_and_signature_is_reported_not_absorbed() {
  // The body says `bool`; the signature says `u64`. The solver must notice.
  let source = "wrong : u64\nwrong = true\n";
  let program = parse_source(source).expect("should parse");
  let report = walk(&program);
  let outcome = solve(&report.store);

  assert!(
    !outcome.is_solved(),
    "a body contradicting its signature must fail, not resolve"
  );
  let failure = &outcome.failures[0];
  assert!(
    !failure.mismatch.is_cycle(),
    "this is a plain mismatch, not an occurs-check failure"
  );
  // The provenance chain is what row 101 renders; it must exist here already.
  let chain = report.store.chain(failure.at);
  assert!(
    !chain.is_empty(),
    "the failing constraint must carry a provenance chain"
  );
}

#[test]
fn a_member_access_is_an_unconstrained_hole_until_row_98() {
  // Deliberately asserting the *limitation*, so that closing it breaks this
  // test rather than silently changing behaviour.
  let source = "width : u64\nwidth = shape.side\n";
  let program = parse_source(source).expect("should parse");
  let report = walk(&program);
  let outcome = solve(&report.store);

  assert!(
    outcome.is_solved(),
    "a deferred member must not fail the solve"
  );
  let found = report.find("width").expect("should be found");
  let resolved = outcome.resolved(&found.ty);
  assert_eq!(
    resolved,
    Ty::named("u64"),
    "the signature still pins it; what is missing is the field's own type"
  );
}

#[test]
fn an_empty_program_solves_to_nothing() {
  let program = parse_source("").expect("the empty program should parse");
  let report = walk(&program);
  assert!(report.definitions.is_empty());
  assert!(solve(&report.store).is_solved());
}

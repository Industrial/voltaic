//! The declaration environment, and what it unblocks.
//!
//! Plan 98 (`docs/96 - 98 - Structural Classes.md`) leaf 3: a type satisfies a
//! class iff it *has* the members the class requires. That needs somewhere to
//! look members up, and this is it.
//!
//! # It also closes plan 94's deferred obligation
//!
//! Plan 94 §10.9 recorded that `Obligation::Member` was **deferred rather than
//! guessed**, because a field access needs to know a type's fields. That was the
//! right call and this is the thing it was waiting for: with declarations in
//! hand, a field access resolves, and the test that asserted the limitation in
//! `solve.rs` is the one that had to change.
//!
//! The deferral is still the behaviour when the declarations are *absent* —
//! `a_member_constraint_with_no_declarations_is_deferred_rather_than_guessed_at`
//! in `driver.vlt` holds that — because a missing declaration is not a licence to
//! invent a type.

use voltaic_lattice::{Declarations, Ty, collect, solve_with, walk};
use voltaic_parser::parse_source;

fn known(source: &str) -> Declarations {
  let program = parse_source(source).expect("should parse");
  collect(&program)
}

#[test]
fn a_records_fields_are_looked_up_by_owner_and_name() {
  let found = known("data Point = Point { x : f64, y : String }\n");
  assert!(found.knows("Point"));
  assert_eq!(found.member("Point", "x"), Some(&Ty::named("f64")));
  assert_eq!(found.member("Point", "y"), Some(&Ty::named("String")));
  assert_eq!(found.member("Point", "z"), None, "a field it does not have");
  assert_eq!(found.member("Other", "x"), None, "a type it does not know");
}

#[test]
fn a_class_contributes_requirements_rather_than_members() {
  let found = known("class Shape\n  area : &self -> f64\n  sides : &self -> u32\n");
  let required: Vec<&str> = found
    .requires("Shape")
    .iter()
    .map(|r| r.name.as_str())
    .collect();
  assert_eq!(required, vec!["area", "sides"]);
}

/// Leaf 3: satisfaction is structural — having the members *is* satisfying.
#[test]
fn a_type_satisfies_a_class_exactly_when_it_has_the_required_members() {
  let source = concat!(
    "class Shape\n",
    "  area : &self -> f64\n",
    "data Square = Square { side : f64 }\n",
    "data Blob = Blob { size : f64 }\n",
    "instance Shape Square\n",
    "  area : &self -> f64\n",
    "  area self = self.side * self.side\n",
  );
  let found = known(source);

  assert!(
    found.satisfies("Square", "Shape"),
    "Square has `area`, so it satisfies Shape — no `instance` header required \
     to say so, which is the whole of doc 87 §3.2"
  );
  assert!(
    !found.satisfies("Blob", "Shape"),
    "Blob has no `area`, so it does not satisfy Shape"
  );
}

#[test]
fn satisfaction_of_a_class_nobody_declared_is_false_rather_than_vacuously_true() {
  // An empty requirement list would make every type satisfy every unknown
  // class, which is the wrong default: it would let a constraint on a
  // misspelled class name pass silently.
  let found = known("data Square = Square { side : f64 }\n");
  assert!(!found.satisfies("Square", "Shape"));
  assert!(!found.satisfies("Square", "Mispelled"));
}

/// What plan 94 §10.9 deferred, now discharged.
#[test]
fn a_field_access_resolves_once_the_declarations_are_known() {
  let source = concat!(
    "data Point = Point { x : f64, y : f64 }\n",
    "abscissa : Point -> f64\n",
    "abscissa p = p.x\n",
  );
  let program = parse_source(source).expect("should parse");
  let report = walk(&program);
  let outcome = solve_with(&report.store, &report.known);

  assert!(outcome.is_solved(), "{:?}", outcome.failures);
  assert!(
    outcome.deferred.is_empty(),
    "a field of a known type must resolve, not defer"
  );

  let found = report.find("abscissa").expect("should be found");
  assert_eq!(outcome.resolved(&found.ty).to_string(), "Point -> f64");
}

#[test]
fn a_field_of_the_wrong_type_is_a_reported_mismatch() {
  let source = concat!(
    "data Point = Point { x : f64, y : f64 }\n",
    "label : Point -> String\n",
    "label p = p.x\n",
  );
  let program = parse_source(source).expect("should parse");
  let report = walk(&program);
  let outcome = solve_with(&report.store, &report.known);

  assert!(
    !outcome.is_solved(),
    "`p.x` is f64 and the signature says String — the solver must notice"
  );
}

#[test]
fn an_instance_members_receiver_is_bound_to_the_instance_target() {
  let source = concat!(
    "data Square = Square { side : f64 }\n",
    "instance Square\n",
    "  area : &self -> f64\n",
    "  area self = self.side * self.side\n",
  );
  let program = parse_source(source).expect("should parse");
  let report = walk(&program);
  let outcome = solve_with(&report.store, &report.known);

  assert!(outcome.is_solved(), "{:?}", outcome.failures);
  let area = report.find("area").expect("area should be found");
  assert_eq!(
    outcome.resolved(&area.ty).to_string(),
    "Square -> f64",
    "without the target binding the receiver stays an unconstrained hole, and \
     then its field cannot resolve either"
  );
}

#[test]
fn an_empty_environment_knows_nothing_and_claims_nothing() {
  let found = Declarations::empty();
  assert!(found.is_empty());
  assert!(!found.knows("Anything"));
  assert_eq!(found.member("Anything", "at_all"), None);
  assert!(found.requires("AnyClass").is_empty());
}

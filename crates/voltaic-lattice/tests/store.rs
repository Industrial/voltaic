//! The constraint store, exercised as a consumer sees it.
//!
//! Plan 94 (`docs/92 - 94 - The Lattice Crate.md`) leaf 1. These are the
//! public-API tests AGENTS.md wants outside the crate: an inline test can reach
//! private internals and eventually will, so what a downstream crate can
//! actually do is asserted from here.
//!
//! # What leaf 1 is really asserting
//!
//! Doc 86 §2 clause 1 admits a solver-determined lowering **only if the choice
//! can be printed**. That makes provenance a correctness requirement rather
//! than a debugging aid, and the strongest way to hold it is to make it
//! impossible to avoid: `Constraint::new` takes a `Provenance`, so there is no
//! way to put a constraint in the store without a span and a reason.
//! [`a_constraint_cannot_exist_without_provenance`] is that claim as a test —
//! it would not compile if the field were optional.

use voltaic_lattice::{Obligation, Provenance, Reason, Store, Ty, Variable, explain, summarise};

fn u64_ty() -> Ty {
  Ty::named("u64")
}

#[test]
fn a_fresh_store_hands_out_variables_from_zero() {
  let mut store = Store::new();
  assert!(store.is_empty());
  assert_eq!(store.fresh(), Variable::new(0));
  assert_eq!(store.fresh(), Variable::new(1));
  assert_eq!(store.variables(), 2);
}

#[test]
fn a_constraint_cannot_exist_without_provenance() {
  let mut store = Store::new();
  let at = store.require(
    Obligation::Unify {
      left: u64_ty(),
      right: u64_ty(),
    },
    (0, 3),
    Reason::LiteralKind,
  );

  let constraint = store.at(at).expect("the constraint should be there");
  assert_eq!(constraint.provenance.span, (0, 3));
  assert_eq!(constraint.provenance.reason, Reason::LiteralKind);
  assert!(constraint.provenance.is_root());
}

#[test]
fn every_constraint_in_a_store_names_its_span_and_its_reason() {
  let mut store = Store::new();
  store.require(
    Obligation::Unify {
      left: u64_ty(),
      right: u64_ty(),
    },
    (0, 3),
    Reason::LiteralKind,
  );
  store.require(
    Obligation::Member {
      recv: Ty::named("Cursor"),
      name: "pos".to_string(),
      result: Ty::named("usize"),
    },
    (7, 10),
    Reason::FieldAccess,
  );
  store.require(
    Obligation::Callable {
      callee: Ty::arrow(u64_ty(), u64_ty()),
      args: vec![u64_ty()],
      result: u64_ty(),
    },
    (12, 20),
    Reason::CallCallee,
  );

  assert_eq!(store.len(), 3);
  for constraint in store.all() {
    let span = constraint.provenance.span;
    assert!(span.1 >= span.0, "{constraint:?} has no span");
    assert!(
      !constraint.provenance.reason.label().is_empty(),
      "{constraint:?} has no reason"
    );
  }
}

#[test]
fn a_derived_constraint_can_be_walked_back_to_its_root() {
  let mut store = Store::new();
  let root = store.require(
    Obligation::Unify {
      left: u64_ty(),
      right: u64_ty(),
    },
    (0, 3),
    Reason::LiteralKind,
  );
  let derived = store.require_from(
    root,
    Obligation::Unify {
      left: u64_ty(),
      right: u64_ty(),
    },
    (9, 12),
    Reason::ReturnPosition,
  );

  let walked = store.chain(derived);
  assert_eq!(walked.len(), 2);
  assert_eq!(walked[0].reason, Reason::ReturnPosition);
  assert_eq!(walked[1].reason, Reason::LiteralKind);

  let shown = explain(derived, &store);
  assert!(
    shown.contains("returned from here") && shown.contains("literal"),
    "an explanation should name every step in the chain:\n{shown}"
  );
}

#[test]
fn a_hole_is_tracked_through_the_types_it_appears_in() {
  let mut store = Store::new();
  let hole = store.fresh_hole();
  let at = store.require(
    Obligation::Unify {
      left: hole.clone(),
      right: Ty::applied("Vec", vec![hole.clone()]),
    },
    (0, 8),
    Reason::ElementOf,
  );

  let constraint = store.at(at).expect("the constraint should be there");
  assert_eq!(constraint.holes(), vec![Variable::new(0)]);
  assert!(!hole.is_resolved());
}

#[test]
fn an_obligation_summarises_in_terms_a_reader_can_check() {
  let unify = Obligation::Unify {
    left: Ty::named("String"),
    right: Ty::named("&str"),
  };
  assert_eq!(summarise(&unify), "String must equal &str");

  let member = Obligation::Member {
    recv: Ty::named("Cursor"),
    name: "pos".to_string(),
    result: Ty::named("usize"),
  };
  assert_eq!(summarise(&member), "Cursor must have `pos` of type usize");
}

#[test]
fn a_function_type_is_an_ordinary_constructor() {
  let f = Ty::arrow(Ty::named("u64"), Ty::named("bool"));
  assert!(f.is_arrow());
  assert_eq!(f.to_string(), "u64 -> bool");
  assert!(f.is_resolved());
  assert!(f.holes().is_empty());
}

#[test]
fn an_explanation_for_an_index_that_is_not_there_says_so() {
  let store = Store::new();
  assert!(explain(0, &store).contains("no constraint"));
}

#[test]
fn a_provenance_can_be_built_without_a_store() {
  let root = Provenance::at((4, 9), Reason::ParameterUse);
  assert!(root.is_root());
  let derived = Provenance::from(3, (10, 14), Reason::CallArgument(1));
  assert_eq!(derived.derived_from, Some(3));
  assert_eq!(derived.reason.argument_index(), Some(1));
}

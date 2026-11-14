//! Ownership modes, inferred, against the modes the author wrote.
//!
//! Plan 96 (`docs/94 - 96 - Mode Inference.md`) leaves 1–3. Doc 86 §6.1 is the
//! lattice: a three-element chain `shared ⊑ unique ⊑ owned`, lowering to `&T`,
//! `&mut T` and `T`, where the solve assigns the **least** mode every use
//! permits.
//!
//! # The answer key
//!
//! `crates/voltaic-lexer/vlt/cursor.vlt` is plan 96 §3's proof target and it is
//! a good one: twelve members plus two constructors, every receiver
//! hand-annotated, and the annotations are **not uniform** — four mutate and
//! eight do not. So there are fourteen independent answers to check against, and
//! a pass that returned `shared` everywhere or `owned` everywhere would score
//! eight or four rather than looking plausible.
//!
//! Doc 90 §1.4 makes the direction of blame explicit: a disagreement here is a
//! Lattice bug, because the solver's job is to compute what the author would
//! have had to write.
//!
//! # Why this needs the call graph
//!
//! `eat` is `&mut self` only because it calls `.bump self`, and `bump` is
//! `&mut self`. Nothing in `eat`'s own body mutates. So a receiver's mode is a
//! fixed point over the call graph — which is exactly why doc 88 §3 says mode
//! inference is a whole-crate solve and not a per-definition one, and why
//! `raise_receivers` iterates.

use voltaic_lattice::{Mode, walk};
use voltaic_parser::parse_source;

fn report_of(path: &str) -> voltaic_lattice::Report {
  let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
  let source = std::fs::read_to_string(root.join(path)).expect("should read the corpus file");
  let program = parse_source(&source).expect("the corpus should parse");
  walk(&program)
}

/// Criterion 1: every inferred mode agrees with the one the author wrote.
#[test]
fn every_inferred_mode_agrees_with_cursor_vlt() {
  let report = report_of("crates/voltaic-lexer/vlt/cursor.vlt");
  assert!(
    report.definitions.len() >= 14,
    "cursor.vlt should contribute at least fourteen definitions, found {}",
    report.definitions.len()
  );

  // Receivers only — see `a_by_value_copy_parameter_may_legitimately_infer_shared`
  // for why an ordinary parameter is not a sharp oracle.
  let mut checked = 0;
  let mut wrong = Vec::new();
  for definition in &report.definitions {
    for (name, written) in &definition.written_modes {
      if name != "self" {
        continue;
      }
      let Some((_, inferred)) = definition.modes.iter().find(|(n, _)| n == name) else {
        continue;
      };
      checked += 1;
      if inferred != written {
        wrong.push(format!(
          "  {}'s receiver: wrote {written}, inferred {inferred}",
          definition.name
        ));
      }
    }
  }

  assert!(checked >= 12, "only {checked} receivers were comparable");
  assert!(
    wrong.is_empty(),
    "inference contradicted the author on {} of {checked} receivers. \
     A disagreement is a Lattice bug, not a user error (doc 90 §1.4):\n{}",
    wrong.len(),
    wrong.join("\n")
  );
}

/// An ordinary by-value parameter is **not** a sharp oracle, and this records why.
///
/// `slice : Span -> &self -> &'a str` takes its `span` by value, so the written
/// mode is `owned`; inference says `shared`, because the body only reads it. Both
/// are correct: `Span` is `Copy`, so the author's choice between `Span` and
/// `&Span` was free, and `shared` is the *least* mode the uses permit — which is
/// precisely what doc 86 §6.1 asks the solve to find.
///
/// Telling "the author had a free choice" from "the author's choice was forced"
/// needs to know whether the type is `Copy`, which needs a declaration
/// environment — the same thing `Member` is waiting on in row 98. Until then the
/// oracle is the receivers, where `&self` versus `&mut self` is forced by the
/// body and there is a single right answer.
#[test]
fn a_by_value_copy_parameter_may_legitimately_infer_shared() {
  let report = report_of("crates/voltaic-lexer/vlt/cursor.vlt");
  let slice = report.find("slice").expect("slice should be found");

  let written = slice
    .written_modes
    .iter()
    .find(|(n, _)| n == "span")
    .map(|(_, m)| *m);
  let inferred = slice
    .modes
    .iter()
    .find(|(n, _)| n == "span")
    .map(|(_, m)| *m);

  assert_eq!(
    written,
    Some(Mode::Owned),
    "the author wrote `Span`, by value"
  );
  assert_eq!(
    inferred,
    Some(Mode::Shared),
    "and inference finds the least mode the body's uses permit"
  );
  assert!(
    inferred < written,
    "the divergence is always in this direction: inference cannot be stricter \
     than the author, only cheaper"
  );
}

/// The annotations are not uniform, which is what makes the above a real test.
#[test]
fn the_answer_key_is_mixed_rather_than_all_one_mode() {
  let report = report_of("crates/voltaic-lexer/vlt/cursor.vlt");
  let written: Vec<Mode> = report
    .definitions
    .iter()
    .flat_map(|d| d.written_modes.iter().map(|(_, m)| *m))
    .collect();

  let shared = written.iter().filter(|m| **m == Mode::Shared).count();
  let unique = written.iter().filter(|m| **m == Mode::Unique).count();
  assert!(
    shared >= 2 && unique >= 2,
    "cursor.vlt should carry both shared and unique receivers, \
     found {shared} shared and {unique} unique — if this ever becomes uniform, \
     `every_inferred_mode_agrees_with_cursor_vlt` stops being evidence"
  );
}

#[test]
fn a_parameter_that_is_only_read_infers_the_cheapest_mode() {
  let source = "big : u64 -> bool\nbig n = n > 10\n";
  let program = parse_source(source).expect("should parse");
  let report = walk(&program);
  let found = report.find("big").expect("should be found");
  assert_eq!(found.modes, vec![("n".to_string(), Mode::Shared)]);
}

#[test]
fn a_parameter_that_is_assigned_through_infers_unique() {
  let source = concat!(
    "bump_it : &mut Cursor -> ()\n",
    "bump_it c =\n",
    "  c.pos := 1\n",
    "  ()\n",
  );
  let program = parse_source(source).expect("should parse");
  let report = walk(&program);
  let found = report.find("bump_it").expect("should be found");
  assert_eq!(found.modes, vec![("c".to_string(), Mode::Unique)]);
}

#[test]
fn the_lattice_is_a_three_element_chain() {
  assert!(Mode::Shared < Mode::Unique && Mode::Unique < Mode::Owned);
  assert_eq!(Mode::least(), Mode::Shared);
  assert_eq!(Mode::greatest(), Mode::Owned);

  // join is the greater of the two, and `shared` is the identity — the two
  // properties the least-fixed-point solve relies on.
  for a in [Mode::Shared, Mode::Unique, Mode::Owned] {
    assert_eq!(a.join(Mode::Shared), a);
    assert_eq!(a.join(a), a);
    for b in [Mode::Shared, Mode::Unique, Mode::Owned] {
      assert_eq!(a.join(b), b.join(a));
      assert!(a.join(b).permits(a) && a.join(b).permits(b));
    }
  }
}

#[test]
fn each_mode_lowers_to_the_rust_form_doc_86_names() {
  assert_eq!(Mode::Shared.emits(), "&");
  assert_eq!(Mode::Unique.emits(), "&mut ");
  assert_eq!(Mode::Owned.emits(), "");
}

/// A receiver-first call is the **only** spelling whose modes match the callee's.
///
/// `infer_mode` has two walks and both were missing an `Expr::Dot` arm, so a
/// receiver-first call contributed nothing to either: `collect_calls` never
/// recorded the `(method, receiver)` pair, and `walk_expr` never reached the
/// receiver or the arguments, leaving every binder at its default. Found by row
/// 103 leaf 6 migrating `voltaic-lexer`, whose sidecar moved `push_blank`'s
/// `@ start owned, end owned` to `start shared, end shared` while the generated
/// Rust stayed byte-identical.
///
/// # The three spellings disagree, and this is which one is right
///
/// For `take : usize -> &self -> u64`, the callee's own modes are
/// `at owned, self shared` — `at` is a `usize` passed by value, `self` is a
/// shared borrow. Measured for the three ways to call it:
///
/// | spelling | inferred |
/// |---|---|
/// | `.take at self` (receiver-last) | `at owned, self` **`owned`** |
/// | `self \|> .take at` (pipeline) | `at` **`shared`**`, self shared` |
/// | `self.take at` (receiver-first) | `at owned, self shared` |
///
/// Receiver-first agrees with the callee; the other two each get one binder
/// wrong. Receiver-last makes `self` owned because `walk_expr`'s `Call` arm
/// passes the caller's `escaping` to every argument and the receiver *is* the
/// last argument; the pipeline makes `at` shared because `Binary` resets
/// `escaping` to `false` for both sides.
///
/// So this is asserted against the **callee's** modes rather than against
/// another spelling: two spellings agreeing would be satisfied by both being
/// wrong, and here two of the three are.
///
/// # The receiver is borrowed, not consumed
///
/// `walk_expr`'s `Dot` arm passes `false` for the receiver and the caller's
/// `escaping` for the arguments. Passing `escaping` for the receiver too was
/// tried first and is wrong: `recv.method ()` borrows `recv` exactly as
/// `recv.field` does, and `Expr::Field` has always walked its receiver with
/// `false`. The difference is measurable — with `escaping`, the committed corpus
/// moved `voltaic-diag`'s `caret_width` from `@ source shared` to
/// `@ source owned`; with `false`, **all 110 sidecars regenerate unchanged**.
#[test]
fn a_receiver_first_call_infers_the_modes_the_callee_declares() {
  let source = "data H = H { export items : Vec<u64> }\n\ninstance H\n  take : usize -> &self -> u64\n  take at self = .unwrap_or 0 (.copied (.get at self.items))\n\n  export via_last : usize -> &self -> u64\n  via_last at self = .take at self\n\n  export via_pipe : usize -> &self -> u64\n  via_pipe at self = self |> .take at\n\n  export via_dot : usize -> &self -> u64\n  via_dot at self = self.take at\n";
  let program = voltaic_parser::parse_source(source).expect("should parse");
  let found = voltaic_lattice::walk(&program);
  let modes = |name: &str| {
    let d = found
      .find(name)
      .unwrap_or_else(|| panic!("{name} should be inferred"));
    d.modes
      .iter()
      .map(|m| format!("{} {}", m.0, m.1))
      .collect::<Vec<_>>()
      .join(", ")
  };
  assert_eq!(
    modes("via_dot"),
    modes("take"),
    "a receiver-first call must demand of its binders exactly what the callee \
     declares; if it does not, migrating `|>` to `.` rewrites published modes"
  );
  // The other two, pinned as the known-wrong baselines. They are not bugs this
  // test is fixing — they are why the assertion above compares against the
  // callee and not against a sibling spelling.
  assert_eq!(
    modes("via_last"),
    "at owned, self owned",
    "receiver-last over-claims the receiver, because it *is* the last argument"
  );
  assert_eq!(
    modes("via_pipe"),
    "at shared, self shared",
    "the pipeline under-claims the argument, because `Binary` resets `escaping`"
  );
}

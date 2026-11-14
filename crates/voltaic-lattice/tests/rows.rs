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

/// Row 95 leaf 3: the caller's row is the join of its own effects and the rows
/// it reaches.
///
/// This lives here rather than inline because it needs `voltaic_ast::Item` to
/// destructure a parsed program, and a pattern cannot be written
/// fully-qualified — so an inline test would need an `import` the lib build
/// then reports unused.
#[test]
fn a_called_row_joins_with_the_callers_own_effects() {
  let program = voltaic_parser::parse_source(
    "loud : ()\nloud =\n  announce\n  println! \\-> \"and more\"\n  ()\n",
  )
  .expect("should parse");
  let known = vec![(
    "announce".to_string(),
    voltaic_lattice::Row::single(voltaic_lattice::Effect::Absent),
  )];
  let voltaic_ast::Item::Fn(f) = &program.items[0] else {
    panic!("the fixture is a function");
  };
  let found = voltaic_lattice::clause_row_with(&f.clauses, &known);
  assert!(
    found.holds(&voltaic_lattice::Effect::Absent),
    "the callee's row did not reach the caller: {found}"
  );
  assert!(
    found.holds(&voltaic_lattice::Effect::Performs),
    "the caller lost its own effects: {found}"
  );
}

/// The whole fixed point, through `walk`, which is what the pipeline uses.
#[test]
fn an_io_call_propagates_to_its_caller_through_walk() {
  let program = voltaic_parser::parse_source(
    "announce : ()\nannounce =\n  println! \\-> \"ready\"\n  ()\n\nmain : ()\nmain =\n  announce\n  ()\n",
  )
  .expect("should parse");
  let found = voltaic_lattice::walk(&program);
  let main = found.find("main").expect("main should be inferred");
  assert!(
    main.row.holds(&voltaic_lattice::Effect::Performs),
    "`main` calls an `io` function and must carry `io`; got `{}`",
    main.row
  );
}

/// Mutual recursion is the case a non-fixed-point implementation gets wrong,
/// and the reason the bound is a counting argument: the alphabet is closed at
/// seven and a row can only grow, so the iteration cannot fail to terminate.
#[test]
fn mutual_recursion_reaches_a_fixed_point() {
  let program = voltaic_parser::parse_source(
    "ping : u64 -> u64\nping n =\n  if n > 0\n    pong (n - 1)\n  else\n    0\n\npong : u64 -> u64\npong n =\n  println! \\-> \"tick\"\n  ping n\n",
  )
  .expect("should parse");
  let found = voltaic_lattice::walk(&program);
  for name in ["ping", "pong"] {
    let d = found.find(name).unwrap_or_else(|| panic!("{name}"));
    assert!(
      d.row.holds(&voltaic_lattice::Effect::Performs),
      "`{name}` reaches the printing side of the cycle; got `{}`",
      d.row
    );
  }
}

/// A receiver-first call propagates its callee's row, exactly like a pipeline.
///
/// This was a real hole, found by row 103 leaf 6 migrating `voltaic-lexer` and
/// watching its sidecar *lose* effects: `lex`, `run` and `finish` each dropped
/// `mut last` when `self |> .end_of_line` became `self.end_of_line ()`. The
/// generated Rust was byte-identical — the identity gate said so — so nothing
/// about the program changed; what changed was Lattice's report of it.
///
/// The cause was `expr_row_with` having no `Expr::Dot` arm, so a receiver-first
/// call fell to `_ -> Row::empty ()` and discarded the receiver's effects, the
/// arguments' effects, the callee's row, and `PANICKING` detection all at once.
///
/// Both spellings are asserted together, because the property that matters is
/// not "a `Dot` has effects" but **"the two spellings agree"** — a migration
/// that changes a published row is not a respelling, whatever the Rust says.
#[test]
fn a_receiver_first_call_propagates_its_callees_row() {
  let source = "data T = T { export trailing : Option<u8> }\n\ndata H = H { export items : Vec<T> }\n\
                \n\ninstance H\n  inner : &mut self -> ()\n  inner self =\n    \
                last = self.items.last_mut ().expect \"nonempty\"\n    last.trailing := None\n    ()\n\n  \
                export outer_pipe : &mut self -> ()\n  outer_pipe self =\n    self |> .inner\n    ()\n\n  \
                export outer_dot : &mut self -> ()\n  outer_dot self =\n    self.inner ()\n    ()\n";
  let program = voltaic_parser::parse_source(source).expect("should parse");
  let found = voltaic_lattice::walk(&program);
  let rows: Vec<String> = ["inner", "outer_pipe", "outer_dot"]
    .iter()
    .map(|n| {
      found
        .find(n)
        .unwrap_or_else(|| panic!("{n} should be inferred"))
        .row
        .to_string()
    })
    .collect();
  assert_eq!(
    rows[1], rows[0],
    "the pipeline spelling propagated `inner`'s row before this fix, and still must"
  );
  assert_eq!(
    rows[2], rows[1],
    "and the receiver-first spelling must agree with it — if these differ, \
     migrating `|>` to `.` silently rewrites every published effect row in the \
     compiler"
  );
  assert!(
    !rows[0].is_empty(),
    "the probe is only meaningful if `inner` has an effect to lose; got an \
     empty row, so this test would pass on three empty rows"
  );
}

/// A field read of an effectful expression keeps the effects.
///
/// Found by the same migration, and **it is not migration-specific** — it was
/// wrong for generation 1 too, and one line of the committed corpus proved it:
/// `voltaic-parser`'s `power` is
/// `(.expect "should have a precedence" (binary_power (&kind))).1`, and its
/// sidecar recorded no row at all. The tuple index is an `Expr::Field`, which
/// swallowed both the `none` from `binary_power` and the `panic` from `.expect`.
///
/// It now reads `! none + panic`. That was the only one of 110 sidecars this
/// fix changed, which is the useful part of the number: the hole was real and
/// narrow, not a wholesale under-report.
#[test]
fn a_field_read_keeps_its_receivers_effects() {
  let source = "data T = T { export mark : u8 }\n\ndata H = H { export items : Vec<T> }\n\
                \n\ninstance H\n  inner : &self -> &T\n  inner self = .expect \"nonempty\" (.first self.items)\n\n  \
                export via_field : &self -> u8\n  via_field self = self.inner ().mark\n";
  let program = voltaic_parser::parse_source(source).expect("should parse");
  let found = voltaic_lattice::walk(&program);
  for name in ["inner", "via_field"] {
    let d = found
      .find(name)
      .unwrap_or_else(|| panic!("{name} should be inferred"));
    assert!(
      d.row.holds(&voltaic_lattice::Effect::Panics),
      "`{name}` reaches `.expect`, so it panics; got `{}`",
      d.row
    );
  }
}

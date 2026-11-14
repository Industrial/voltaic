//! Doc 18 §2.3's guard ladder.
//!
//! `| cond = body` rows under a clause's pattern row, lowering to an
//! `if`/`else if`/`else` chain. The ladder replaces three constructs doc 03
//! had grown over the same ground — `cond`, `when` arm guards, and §6.2's
//! relational patterns — which is why §2.3 calls the bar the highest-yield
//! token in the redesign.

use voltaic_emit::emit;
use voltaic_parser::parse_source;

fn vrs(src: &str) -> String {
  emit(&parse_source(src).expect("should parse"))
}

/// §2.3's own example, including that `_` becomes a bare `else`.
#[test]
fn a_ladder_becomes_an_if_else_chain() {
  let rust = vrs(
    "tier : u64 -> &'static str\n\
     tier spend\n  \
       | spend > 10_000 = \"platinum\"\n  \
       | spend > 1_000 = \"gold\"\n  \
       | _ = \"basic\"\n",
  );
  assert!(rust.contains("if spend > 10_000 {"), "{rust}");
  assert!(rust.contains("} else if spend > 1_000 {"), "{rust}");
  assert!(rust.contains("} else {"), "{rust}");
  assert!(
    !rust.contains("_ ="),
    "the catch-all spelling is not emitted:\n{rust}"
  );
}

/// The catch-all must not become `if true`. A conditional chain has an implicit
/// `else` of `()`, so a guarded function with a return type would stop
/// compiling — and rustc, not Voltaic, is what notices. Doc 18 §1.
#[test]
fn the_catch_all_does_not_become_a_condition() {
  let rust = vrs(
    "tier : u64 -> &'static str\n\
     tier spend\n  \
       | spend > 10 = \"high\"\n  \
       | _ = \"low\"\n",
  );
  assert!(!rust.contains("if true"), "{rust}");
  // The chain closes, so the whole `if` is one expression with a value.
  // Narrow enough to be the one-line form, which is what rustfmt does to a
  // two-armed `if`/`else` in a function body.
  assert!(
    rust.contains("if spend > 10 { \"high\" } else { \"low\" }"),
    "{rust}"
  );
}

/// A ladder is a tail position exactly as a plain body is: the value of the
/// chain is the function's value, with no `return` the author did not write.
#[test]
fn a_ladder_needs_no_return() {
  let rust = vrs(
    "sign : i32 -> i32\n\
     sign n\n  \
       | n > 0 = 1\n  \
       | _ = 0\n",
  );
  assert!(!rust.contains("return"), "{rust}");
}

/// Guards and several clauses compose: each clause carries its own ladder, and
/// the ladder lands inside that clause's `match` arm.
#[test]
fn a_ladder_lives_inside_a_match_arm() {
  let rust = vrs(
    "step : u64 -> u64\n\
     step 0 = 0\n\
     step n\n  \
       | n > 100 = 100\n  \
       | _ = n\n",
  );
  assert!(rust.contains("match "), "{rust}");
  assert!(rust.contains("0 => 0,"), "{rust}");
  assert!(rust.contains("if n > 100 {"), "{rust}");
}

/// A guard with no catch-all still emits — Voltaic does not decide whether a
/// ladder is total, so the chain reaches rustc and rustc reports the type.
#[test]
fn a_ladder_without_a_catch_all_still_emits() {
  let rust = vrs("noisy : ()\nnoisy\n  | 1 > 0 = println! \\-> \"hi\"\n");
  assert!(rust.contains("if 1 > 0 {"), "{rust}");
  assert!(!rust.contains("else"), "nothing is invented:\n{rust}");
}

/// Row 99 wave 2's additive half: a ladder arm may be a **pattern row**.
///
/// §4 says `case` collapses into "the guard ladder and pattern rows", and two
/// thirds of that already existed — multi-clause dispatch is pattern rows, and
/// `describe (Some n) = …` has lowered to a `match` since generation 1. What had
/// no replacement was `case` used as an *expression* in the middle of a body,
/// because multi-clause dispatch only dispatches on parameters.
///
/// The arm's type was the only thing in the way: it was `Option<Expr>` while
/// `if` already took a `Condition`, which models `x is Some y` and carries
/// `has_pattern`. `emit_condition` then lowers it to `let Some(n) = held`, which
/// is exactly what `if`/`else if` wants — so the machinery was all built for
/// `if` and the change is the type.
#[test]
fn a_pattern_row_becomes_an_if_let() {
  let rust = emit(
    &parse_source(concat!(
      "describe : Option<u64> -> &'static str\n",
      "describe held\n",
      "  | held is Some n = \"some\"\n",
      "  | _ = \"none\"\n",
    ))
    .expect("should parse"),
  );
  assert!(
    rust.contains("if let Some(n) = held {"),
    "a pattern row lowers to `if let`:\n{rust}"
  );
}

/// A pattern row chains like any other arm, so the second one is `else if let`.
#[test]
fn a_second_pattern_row_chains_as_else_if_let() {
  let rust = emit(
    &parse_source(concat!(
      "name : Result<u64, u64> -> &'static str\n",
      "name held\n",
      "  | held is Ok n = \"ok\"\n",
      "  | held is Err e = \"err\"\n",
      "  | _ = \"neither\"\n",
    ))
    .expect("should parse"),
  );
  assert!(
    rust.contains("} else if let Err(e) = held {"),
    "a second pattern row chains:\n{rust}"
  );
}

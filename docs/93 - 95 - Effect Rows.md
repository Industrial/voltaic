# Plan 95 — Effect Rows

Plan ID: `rows`
Depends on: 94
Milestone: 15 — *the solver*
Status: `planned`

---

## 1. What this plan is

`voltaic-lattice/vlt/row/`: a seven-effect alphabet, the subset lattice over it,
implicit propagation, four discharge forms, and rank-1 effect variables.

This is the row that deletes the most constructs. Nine of generation 1's constructs
collapse into it, and five generation-1 plans' worth of design go with them.

### 1.1 What it replaces

| Generation 1 | Plan that built it | Generation 2 |
|---|---|---|
| `try` block | 28 | the row |
| `catch` | 28 | `else` |
| `?` postfix | 02 §6.3 | implicit propagation |
| `??` coalesce | 29 | `else` |
| `?.` optional chaining | retired by 57 | implicit propagation |
| `.await` postfix | 20 | implicit |
| `async` block modifier | 31 | the `async` effect |
| `do .and_then` / `do .flat_map` | 86 | the row, for `std` shapes only — §1.4 |
| `.map_err` as the idiomatic conversion | — | `From` via row unification |

Generation 1's `examples/fallible.sdx` is the measurement: 60 code lines and 670
tokens, with a 30-line header comment explaining why the design is right. Its Voltaic
2 port is 29 lines and 235 tokens — **2.85×**, the largest per-program ratio in the
five-program port set, and the reason this row exists.

### 1.2 The alphabet is closed, and that is the design

```
fail E    none    async    io    mut ρ    panic    diverge
```

Seven, and doc 86 §5.1 closes the set.

A closed alphabet is what makes row unification decidable: a row is a subset of a
seven-element set, so the ascending chain condition holds trivially and the fixed
point is reached in at most seven iterations per binding. That is the clause-5
argument, and it is a counting argument rather than a proof obligation.

An open alphabet — user-declared effects — would be the natural generalisation and is
**not** taken. It would make the lattice the powerset of an unbounded set, which is
still a lattice but no longer gives a bound a reviewer can check by counting. Venue:
a later generation, and only with a measured bound.

### 1.3 Propagation up, discharge down

Inference runs **upward**: a definition's row is the join of its body's rows, minus
what the body discharges. Nothing is written at a call site.

```
parse_it s = s.parse : i32 ! fail String
attempt x y = parse_it x + parse_it y      # fail String, propagated
fallback s = parse_it s else 0             # {}, discharged
first_name u = u.profile.name              # none, propagated
```

The fourth line is the one worth looking at twice. `u.profile` is `?Profile`, so the
`none` effect enters the row, and the field access *after* it is ordinary field
access. Optional chaining is not a construct; it is what happens when a row
propagates through a `.`.

Discharge forms, and doc 86 §5.3 closes this set too:

| Form | Discharges | Lowers to |
|---|---|---|
| `e else v` | `fail`, `none` | `unwrap_or`, or a `match` where the arity needs one |
| `e else x -> body` | `fail` | `unwrap_or_else`, or a `match` |
| `e.must` | `fail`, `none` | `unwrap`; adds `panic`; lint-flagged |
| `~e` | `fail`, `none` | nothing — **reifies** the row into a `Result`/`Option` value |

### 1.4 `else` is not `catch`, and the difference is the whole reversal

Generation 1's `catch` had to lower to a `match` rather than `unwrap_or_else`, and doc
03 §12.1 gives the decisive reason: *that method's closure arity differs between
`Option` and `Result`, so a method lowering would be type-dependent and Voltaic has no
types.*

`else` has types. It lowers to `unwrap_or`, `unwrap_or_else` or a `match`, whichever
fits the row it is discharging — the choice doc 03 §12.1 could not make. This is the
cleanest single example in the generation of a construct that was *worse* than it
needed to be because of the charter, rather than because of its design.

### 1.5 `~` exists because effects are not values

An effect system makes fallibility a property of an expression, not a thing you can
hold. But a program sometimes needs to hold one: generation 1's
`examples/fallible.sdx` has

```
print_outcome : Result<i32, String> -> ()
```

and without reification that function cannot be written at all. `~e` is the operator
that turns a row back into data.

It is one character, and it is also the escape hatch for doc 86 §5.6's second
regression: nested optionals. `u.profile.nickname` flattens to `?String` because an
effect cannot nest; `~u.profile.nickname` is `?(?String)`. **The terser spelling is
now the lossy one**, which is the wrong way round, and this plan does not fix it — it
records it, here and in `docs/MIGRATION.md` as a `manual` migration class.

### 1.6 Effect polymorphism, in one form

A higher-order parameter may carry an effect variable:

```
map f xs = [f x | x in xs]      # row: ε, where ε is f's row
```

Rank-1, like type variables, for row 94 §1.2's reason. This is the minimum that makes
the surface usable: without it, `map` over a fallible function would have to be a
different function from `map` over a pure one.

---

## 2. Preconditions

- Row 94 `done`: store, driver, provenance, unification.
- `fail`'s parameter is a type, so two `fail`s join by `From`-convertibility, which
  means the row engine asks the type engine a question. That is the dependency doc 88
  §1.2 predicted and it is why these are modules rather than crates.

---

## 3. Concrete proof this plan targets

`examples/fallible.vlt`, ported, compiling, and **running with identical output** to
generation 1's `examples/fallible.stdout`. Eleven printed lines, and generation 1's
own file documents what each one catches.

Two of them are the real targets:

- `after_a_failed_try` returns `&'static str`, not a `Result`, and prints `"reached"`
  *after* a failure. A lowering where the failure escaped the discharge would skip the
  print — a case only a run catches, which is why generation 1 wrote it.
- `nickname_of`, which is the nesting case §1.5 regresses. Its port uses `~` and its
  expected output is unchanged.

---

## 4. Leaves

### Leaf 1 — the alphabet and the lattice

The seven effects, join, the ⊑ relation, and the iteration bound asserted in the
driver.

### Leaf 2 — propagation

Upward inference through application, field access, blocks, comprehensions and
`loop`. The `?`-through-`.` behaviour of §1.3 is this leaf's main deliverable and its
main risk: a `.` chain through two optionals must flatten once, not twice.

### Leaf 3 — discharge

The four forms, each with its lowering choice made from the row. `else`'s three
lowerings (§1.4) need a test each, because picking `unwrap_or` where a `match` is
required is a compile error in the output and picking a `match` where `unwrap_or`
would do is a quality regression the zero-cost gate catches.

### Leaf 4 — `From` conversion on join

Two `fail`s with different error types join if a `From` exists. If it does not, the
diagnostic names both types and the two spans. Generation 1 wrote this conversion by
hand with `errors`'s `from … as …` row; this leaf is what deletes that row.

### Leaf 5 — effect variables

§1.6.

### Leaf 6 — `async` without an executor

`async` is inferred and lowered to `async fn` plus `.await`. No executor, no join
construct — doc 86 §5.5, and generation 1's reason, unchanged: every joining form is
runtime-crate-provided and clause 2 forbids implying one.

### Leaf 7 — `tests/solve/` and `tests/ui/` growth

Row expectations in `tests/solve/`; `VLT07xx` fixtures in `tests/ui/`. A diagnostic
range with no fixtures is not shipped (doc 88 §6.1).

---

## 5. Acceptance criteria

1. `examples/fallible.vlt` runs with output byte-identical to generation 1's
   `.stdout`, including the `"reached"` case.
2. The iteration bound is asserted and holds on the fuzz target.
3. `else` picks all three lowerings, one test each, and the zero-cost gate passes on
   each.
4. A `From`-less join produces a diagnostic naming both error types and both spans.
5. A `.` chain through two optionals flattens exactly once, and `~` recovers the
   nested shape. Both asserted, because §1.5 is a semantic regression and an
   unasserted regression becomes an accident.
6. `tests/ui/` has at least one fixture per `VLT07xx` code.

---

## 6. Out of scope, and the venue it names

**`with M` — general monadic comprehension over user types — is not in this
generation.** Doc 86 §5.6 states the loss and doc 87 §22.2 marks it a debt rather
than a decision. The sketch, recorded so the venue is real:

> A block modifier `with M` names a **type** rather than a method. Inside it, `<-`
> binds through whichever of `and_then`, `flat_map` or `bind` `M` provides — chosen by
> the solver from `M`, not by the author from a method name, which is strictly more
> information than generation 1's `do .and_then` had. The final row is still written
> out, because fabricating `pure` would be the inference doc 86 §2 clause 1 forbids.

Why not now: shipping an effect row and a user-monad construct in the same generation
would mean two answers to one question on day one, and the row is the one that pays
for itself (§1.1's 2.85×). The venue is a later generation, and the condition is that
the effect row has been in use long enough to know which of the two people reach for.

Also out of scope: user-declared effects (§1.2), resumable handlers (doc 87 §22.2),
and modes — row 96, even though a discharge creates mode constraints.

---

## 7. Gate

Standing gate, plus `examples/fallible.vlt`'s run expectation, plus the bound
assertion, plus the zero-cost gate on each `else` lowering.

---

## 8. Emerald reuse

None. Emerald had no effect system; its fallibility story was `Result` by hand.

---

## 9. Decision log

| Decision | Why |
|---|---|
| Seven effects, closed | a counting argument for clause 5 that a reviewer can check |
| No user-declared effects | an unbounded alphabet has no checkable bound |
| `none` as an effect rather than `Option` as a type | it is what deletes optional chaining as a construct |
| Accept that nested optionals flatten | the cost of the row; recorded in doc 86 §5.6, asserted in §5.5, and `~` is the recovery |
| `~` as reification | without it `print_outcome` cannot be written at all (§1.5) |
| `else` chooses its own lowering | doc 03 §12.1's problem, solved by the reversal (§1.4) |
| `async` still ships without an executor | clause 2, unchanged from generation 1 |
| Defer `with M` | two answers to one question on day one is worse than a named debt |

---

## 10. Result

*To be filled after execution.*

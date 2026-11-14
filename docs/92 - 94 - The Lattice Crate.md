# Plan 94 — The Lattice Crate

Plan ID: `lattice`
Depends on: 93
Milestone: 15 — *the solver*
Status: `planned`

---

## 1. What this plan is

`voltaic-lattice`, the fourteenth crate: a constraint store, a fixed-point driver,
provenance tracking, and the type engine. Rows 95, 96 and 97 add the three other
engines onto the machinery this row builds.

It also builds `tests/solve/`, the corpus that makes inference reviewable. That
corpus is the deliverable this plan would be worthless without — see §1.3.

### 1.1 One crate, four engines, one solve loop

Doc 88 §1.2 has the argument: the four engines solve *together*, because a mode
constraint can be created by an effect discharge, an effect can be created by a mode
(`mut ρ`), and a refinement's predicate mentions `.len`, which is a type question.
Four crates would be a dependency cycle.

The layout, which satisfies doc 06's module-tree rules:

```
crates/voltaic-lattice/vlt/
  lib.vlt            # declarations and the public entry point only
  constraint.vlt     # the store and the fixed-point driver
  type/              # unification, generalisation, instantiation
  row/               # row 95
  mode/              # row 96
  refine/            # row 97
  explain.vlt        # provenance rendering for `--why`
```

`constraint.vlt` is budgeted at **900 source lines** and splits into `constraint/` if
it exceeds it, because the 1,500-line cap is on the *generated* Rust and the ratio is
about 1.6 (doc 88 §8.2).

`row/` and `mode/` both want a `Join` variant, and two enums sharing a variant name
cannot share a file — every `data` emits `use <Enum>::*`, so the shared name is
`E0659`. They are in separate directories for that reason rather than by taste.

### 1.2 The type system is rank-1 on purpose

Rank-1 Hindley–Milner with row polymorphism over records. No higher-rank types, no
impredicativity, no higher-kinded abstraction.

This is a ceiling chosen rather than a limitation accepted. Rank-1 HM has **complete
and decidable** inference; rank-N does not, and the moment inference is incomplete the
language needs mandatory annotations in positions a user cannot predict — which is
precisely the failure doc 86 §2 clause 1 forbids, since a lowering the author cannot
anticipate is a lowering that cannot be printed back to them.

Doc 87 §22.2 declines higher-kinded abstraction on exactly this ground, and names what
it costs: `Monad m` is not expressible, which is why row 95's effect alphabet is
closed and why generation 1's `do .and_then` over user types is a regression (doc 86
§5.6).

### 1.3 Provenance is a correctness requirement, not a debug feature

Every constraint in the store carries:

- the **span** that created it,
- the **reason** — a structured value, not a string: `ParameterUse`, `FieldAccess`,
  `CallArgument(n)`, `Discharge`, `Ascription`, `ClassRequirement`, …,
- the **constraint it was derived from**, if any.

Doc 86 §2 clause 1 admits a solver-determined lowering *only if the choice can be
printed*. A store that loses provenance cannot satisfy that clause, so a constraint
without a reason is a bug in this crate — asserted, not merely discouraged.

This is also what makes row 101 possible at all. A conflict is reported by walking the
provenance chain of both sides, which is the difference between *"cannot unify `&str`
with `String`"* and *"`peek` is `shared` because line 17 reads it, and `owned` because
line 23 returns it past the frame"*.

### 1.4 What this plan does not do

- No effect rows (95), modes (96) or refinements (97).
- No surface change. The parser still accepts generation 1's surface; this crate runs
  after it and its output is, for this row, **only** checked against `tests/solve/`
  expectations. Nothing in the emitter consumes it yet.
- No incremental solve. §6.

That ordering is deliberate: a solver whose output nothing depends on can be measured
against a corpus before it is trusted with an emitter.

---

## 2. Preconditions

- Row 93 is `done`, so the crate is written in `.vlt` from the first commit rather
  than being renamed later.
- `voltaic-ast` nodes gain a judgement slot. Every node still carries a byte span —
  doc 02's rule, and the one Emerald broke twenty-one plans after its diagnostics plan
  shipped, by which point the fix was judged too large.
- The crate is written in Voltaic 1 (the surface the seed can parse), because row 102
  has not run. Doc 88 §7's invariant: at no point is the only compiler for a language
  written in that language.

---

## 3. Concrete proof this plan targets

```
voltaic sig crates/voltaic-lexer/vlt/cursor.vlt
```

printing, for the generation-1 source of `cursor.vlt`, the judgement generation 1's
author wrote by hand — `peek : &self -> Option<char>` and the other eleven members —
**derived rather than read**. The file's own signature lines are the answer key, which
is what makes `cursor.vlt` the right first target: 12 members, hand-annotated, already
in the tree.

A second target, the one that proves generalisation: `examples/currying.sdx`, whose
whole point is partially applied functions.

---

## 4. Leaves

### Leaf 1 — the constraint store

The store, provenance, and the union-find for type variables. No solving yet; the
deliverable is that a traversal of `examples/hello.vlt` produces a store whose every
constraint names its span and reason.

### Leaf 2 — the fixed-point driver

Worklist, iteration bound, and the ascending-chain argument per engine. The driver
takes an engine's transfer function and iterates to a fixed point, asserting the
bound. Rows 95–97 supply transfer functions; this leaf supplies the loop and the
assertion that it terminates.

### Leaf 3 — unification, generalisation, instantiation

Rank-1 HM. Let-generalisation at definition boundaries, instantiation at use sites,
the occurs check, and row polymorphism over records so that field access on a partially
known record works.

### Leaf 4 — `voltaic sig`

Prints the inferred judgement of every definition in a file. Flags: `--json` for the
LSP, `--why <name>` stubbed to the provenance walk this row can already do for types.

### Leaf 5 — `tests/solve/`

The corpus. One `.vlt` and one `.sig` expectation per program; blessable with
`VOLTAIC_UPDATE=1`, and blessing rewrites and then **fails**, per the standing
convention.

The initial set is every `examples/*.vlt` plus `crates/voltaic-lexer/vlt/cursor.vlt`,
and the expectation is checked against the **hand-written signature lines already in
those files** for as long as generation 1's surface still has them. That is a free
oracle and it disappears at row 99, which is why this corpus is built now.

### Leaf 6 — the fuzz target

`voltaic-fuzz` gains a target that asserts the driver's iteration bound is never
exceeded. An input that makes the solver exceed its bound is a clause-5 violation and
blocks the generation (doc 88 §4.2).

---

## 5. Acceptance criteria

1. `tests/solve/` green over every `examples/*.vlt`.
2. For every definition in generation 1's corpus that carries a hand-written
   signature, **the inferred judgement equals the written one**, or the difference is
   recorded in this document's §10 with the reason. This is the plan's central
   criterion: the answer key exists and it is used.
3. Every constraint in the store carries a span and a structured reason; asserted.
4. The driver's iteration bound holds on the fuzz target.
5. `voltaic sig --why` walks provenance for a type conflict and names both sides.
6. The crate satisfies doc 06: module tree, path-aware naming, no comments, 1,500-line
   cap on the generated Rust.
7. Nothing in the emitter consumes the judgement yet, so the full gate is green with
   **no output diff**. A `.vrs` that changed in this row is this row having done
   something it was not asked to.

---

## 6. Out of scope

- **Incremental solve.** Doc 88 §3's three levels are real work and they belong with
  the row that makes the driver depend on the solve, which is 99. Until then every
  solve is whole-crate and slow, and that is acceptable because nothing is waiting on
  it.
- **Cross-crate inference.** There is none, ever — doc 86 §8.3. The sidecar is row
  100.
- **Lifetime inference.** Lattice does not infer lifetimes (doc 86 §6.3); the emitter
  elides and rustc resolves. Row 96 owns what happens when elision fails.

---

## 7. Gate

Standing push gate, plus `tests/solve/`, plus the new fuzz target, plus the
no-output-diff assertion in §5.7.

---

## 8. Emerald reuse

None available — Emerald had no inference. The relevant Emerald lesson is negative and
is already a repository rule: **no plan implements a library function.** The union-find
and the worklist are small enough to write and `AGENTS.md`'s rule is about *library*
functions rather than algorithms, but the solver interface in row 97 is where that
rule bites, and it is answered there.

---

## 9. Decision log

| Decision | Why |
|---|---|
| One crate, four modules | the engines solve together; four crates is a cycle (doc 88 §1.2) |
| Rank-1 HM | complete and decidable inference; rank-N forfeits both and clause 1 with them |
| Provenance asserted, not optional | clause 1 admits a solver-determined lowering only if it is printable |
| Structured reasons, not strings | row 101 renders them; a string cannot be rendered two ways |
| Build the corpus in this row | generation 1's hand-written signatures are a free oracle that disappears at row 99 |
| Nothing consumes the judgement yet | a solver measured against a corpus before an emitter trusts it |
| `row/` and `mode/` in separate directories | both want `Join`; two enums sharing a variant name cannot share a file |

---

## 10. Result

In progress. Leaf 1 shipped 2026-11-14: the crate, the constraint store, provenance,
variables, a type representation and `explain`. 38 tests — 29 inline, 9 integration.
Gate green at 28 moon tasks and 949 workspace tests, with **no output diff**, per §5.7.

### 10.1 §1.1's layout is impossible as written, and the reason is a keyword

The layout names `type/` for the type engine. **`type` is a Voltaic keyword, and
`module <keyword>` cannot parse** — `AGENTS.md` records it: *"`class`, `data`, `test`,
`type` and `errors` are all keywords, so a module may not be named after one."* The
engine's directory is therefore `unify/`, which is a domain concept rather than a
bucket and so also satisfies doc 06.

The same trap caught a *member* name. `Variable::of` would not parse — `of` is the
keyword in `case x of` — and the diagnostic was exactly right: **`VLT0103`: expected a
name after `::`**. It is `Variable::new`.

Two keyword collisions in one leaf, both in names that read as obviously fine. Worth
checking the 74-keyword list before naming anything in generation 1's surface, which
is the surface rows 94–101 are written in.

### 10.2 Leaf 1's real deliverable is a type, not an assertion

§1.3 asks for provenance on every constraint and §5.3 states it as an assertion. The
stronger form shipped instead: **`Constraint::new` takes a `Provenance`**, so there is
no way to put a constraint in the store without a span and a reason. The integration
test that names this —
`a_constraint_cannot_exist_without_provenance` — would not *compile* if the field were
optional, which is a better guarantee than a test that could be deleted.

### 10.3 `Fun` as a separate variant was wrong

The first `Ty` had `Hole | Con | Fun | Nil`, with `Fun { from : Box<Ty>, to : Box<Ty> }`.
Every pattern match on it then had to destructure through a `Box`, which is friction in
every arm for no gain. Replaced by a uniform `Con { name = "->", args = [from, to] }`,
which is the standard HM representation, removes `Box` from the whole file, and makes
`is_arrow` a one-line predicate. `Ty::arrow` is unchanged at the call site.

### 10.4 Three surface frictions, each costing a round trip

Recorded because rows 95–101 are written in the same surface and will hit them again:

- **`.index *self` is a multiplication section.** It lowered to
  `|__vlt_1| __vlt_1.index() * self` — `*` was read as binary multiply, not deref.
  `AGENTS.md` documents the shape (*"`(* 2)` is a dereference and unary always wins"*)
  and this is the other direction of it. Use the field, or parenthesise.
- **A receiver is passed unborrowed.** `.is_empty (&s)` generates `(&s).is_empty()`,
  which is `clippy::needless_borrow` under `-D warnings`. The corpus convention is
  `.is_empty s` and letting autoref do it; a borrow is written only where the
  *parameter* is a reference.
- **An import used only in a `test` block is an unused import in the lib build**,
  because the tests land in a separate module. Qualify at the use site instead.

### 10.5 Or-patterns are not a generation-2 feature, and two documents said they were

Reading `crates/voltaic-ast/vlt/expr.vlt` for the AST's shape turned up line 85: one
`case` arm whose body serves forty-odd patterns joined by **`or`**, lowering to Rust's
`|` or-pattern. So generation 1 has or-patterns; what it lacks is the `|` *spelling*.

`docs/86 - Voltaic 2.md` §10 claimed three of eight unreachable constructs close in
generation 2. It is **two** — `as` casts and `ref` patterns. Both documents are
corrected in place, and `tests/unreachable/or_pattern.vlt`'s own comment is wrong on
the same point (row 104 owns it, since the fixture still correctly fails on `|`).

This is plan 46's finding recurring — *eleven of sixteen apparent gaps were the probe's
own bug* — and it is what `docs/102 - 104` §1.2 predicted of that table. It was found by
reading the corpus for an unrelated reason, which is the argument for regrading against
fixtures rather than against prose.

### 10.6 Leaves 2 and 3 shipped together, 2026-11-14

`substitution.vlt`, `unify.vlt`, `driver.vlt`, `scheme.vlt`. 72 tests — 63 inline, 9
integration. Clippy clean under `-D warnings`; gate green at 28 tasks.

They shipped as one commit because a driver with no transfer function cannot be
tested and unification is the first transfer function. Splitting them would have meant
a commit whose only assertion was that a loop terminates on an empty worklist.

**What `solve` does and does not do.** `Unify` is discharged by unification;
`Callable` is discharged by building the arrow its use implies (`arrow_of`) and
unifying the callee against it; **`Member` is deferred, not guessed.** A field access
needs to know a type's fields, which needs a declaration environment, which is row
98's. The driver collects deferred indices rather than failing, and the test naming
that behaviour is what stops a later row from quietly making something up.

**The round bound holds by the shape of the lattice, not by hope.** `MAX_ROUNDS` is 16
and the loop exits as soon as a round binds nothing new. A 64-link chain of holes
solves inside the bound, which is the test; the general argument is that each round
either binds at least one variable or is the last.

### 10.7 The occurs check is not enough on its own

`unify` has an occurs check, so unification cannot build `?0 = Vec<?0>`. But
`Substitution::bind` is a raw primitive with no check, and `instantiate` used it — so
when a scheme quantified `?0` and the instantiating store's counter also stood at 0,
`instantiate` bound `?0 → ?0` and `resolve` recursed until the **stack overflowed**.

A real bug, found by a test rather than by review, and fixed at both ends: `resolve`
treats a self-binding as the identity, and `instantiate` skips an identity binding
rather than making one. Two tests now name each half.

The collision is artificial in the test — a real caller generalises and instantiates
against the same store, whose counter is always past the quantified set — but a
primitive that diverges on a reachable input is a primitive to fix, not to document.

### 10.8 `.voltaic-cache` can serve stale output across a `.vlt` edit

This cost more round trips than any other single thing in the leaf. `bootstrap.sh`
appeared to succeed, reported every file, and left `src/` holding the *previous*
generation — so clippy kept reporting findings that the source had already fixed.

The cache is `crates/<crate>/src/.voltaic-cache`, written by `voltaic-driver`. Running
the seed directly over the source directory produced correct output immediately, which
is what isolated it.

**This is the same shape as `AGENTS.md`'s seed-refresh trap** — *"`moon run
:seed-refresh` replays a cached size, so the number it prints can be a previous
run's"* — and the same remedy applies: do not trust the task's report, check the
artifact. For regeneration the check is to read the generated file, and the fix when it
is stale is `rm -rf crates/*/src/.voltaic-cache`.

### 10.9 Constraint generation and the corpus, 2026-11-14

`annotation.vlt` reads a written `TypeExpr` into a `Ty`; `generate.vlt` walks a
`Program` and produces a store plus one `Inferred` per definition. 96 tests — 76
inline, 11 in the corpus, 9 on the store. Clippy clean; gate green at 28 tasks.

**The oracle works.** §1.4 said generation 1's hand-written signatures are a free
answer key that disappears at row 99, and the corpus now spends it: for every
definition that states its own type, inference is compared against what the author
wrote, and a disagreement is a Lattice bug rather than a user error (doc 90 §1.4).
Eleven programs, including a guard ladder, a comparison, arithmetic, an ascription
and a deliberate contradiction that must be *reported* rather than absorbed.

**Lifetimes are dropped when reading an annotation, and a test caught why.**
`&'static str` first read as `&<'static str>`, which then refused to unify with the
`&<str>` inferred from a string literal. Doc 86 §6.3 is the rule — Lattice does not
infer lifetimes — so a lifetime is not part of the type for inference purposes.
`without_lifetime` strips it, and `&'a str` and `&str` are now the same `Ty`.

**Two corpus programs were not valid Voltaic**, which is worth recording because it
will recur: `ready = true` at module scope is **VLT0106** — a module-scope binding
needs `const` or `static` and a type, because Rust forbids `let` there. So a nullary
definition in generation 1 *must* carry a signature. Row 99 is what makes the
signature optional; until then the oracle and the requirement happen to coincide,
and the corpus says so rather than working around it.

**`Member` is asserted to be unsolved.** One test fixes the current limitation in
place: a field access resolves to an unconstrained hole, and closing that in row 98
will break the test rather than silently changing behaviour. A limitation with no
test is a limitation that becomes an accident.

**Two more registration gates, both teaching a real convention.** A dev-dependency
on a workspace crate must be **path-only with no version** — `cargo publish` resolves
a dev-dependency's version against crates.io even when it carries a path, so a
versioned one makes this crate's publish wait for a sibling it does not need in order
to build. And `dependency_sources` must list the dev-dependency too, because a change
to the parser can change this crate's test verdict, which is exactly what the group
exists to invalidate.

### 10.10 Leaves outstanding

| Leaf | Status |
|---|---|
| 1 — the constraint store | `done` |
| 2 — the fixed-point driver | `done` |
| 3 — unification, generalisation, instantiation | `done` |
| 4 — `voltaic sig` | `done` |
| 5 — `tests/solve/` | `done`, as a Rust corpus under `crates/voltaic-lattice/tests/` rather than a blessable `tests/solve/` directory. §10.12 is why. |
| 6 — the fuzz target | `done` |

Leaf 5 landing before leaf 4 is a deviation worth naming: the corpus is what makes
leaf 4 worth building, because `voltaic sig` with no oracle prints numbers nobody can
check.

### 10.11 `voltaic sig` is the seventh subcommand, and the surface gate priced it

Run on `examples/guards.vlt`, with nothing in the file read but its code:

```
tier : u64 -> &<str>
fib : u64 -> u64
step : u64 -> u64
main : ()
```

Those are generation 1's own hand-written signatures, derived rather than read —
modulo the lifetime erasure §10.9 records. `--why fib` prints the provenance chain
behind each, which is what **discharges doc 86 §2 clause 1 for types**: a
solver-determined lowering is admissible only if the choice can be printed, and this
is the thing that prints it. Row 92 §2.3 made that a condition of the charter
reversal rather than a nicety.

`crates/voltaic-cli/tests/cli_surface.rs` is a deliberate second copy of the command
list whose own doc comment prices a seventh subcommand at *"three edits — the
specification, the table, and a line here that says why it is here."* All three were
paid. The specification edit went to **doc 86 §8.1**, not doc 01 §27: doc 01 is
generation 1's record and is not edited, so the gate's prose now names the document
that supersedes it. Doc 02 §3 records Emerald's CLI reaching seventeen subcommands
with nothing in its code to say when to stop; this gate is that something, and it
worked.

### 10.12 The round bound is asserted, not reported

`voltaic-fuzz` gains a `solve_program` target and `sweep.rs` a
`the_solver_never_exceeds_its_round_bound` assertion over the whole fuzz corpus —
the hostile inputs, the deep nesting and the mutations included.

It is asserted rather than reported because **it is a bound, not a timing**. Doc 88
§4.2 draws that line and plan 44's standing decision is the reason: a timing in the
gate is a flaky gate. The counting argument is what makes the bound checkable — each
round either binds a variable or is the last — so an input that exceeded it would mean
the loop can spin without binding anything, which doc 88 §4.2 says blocks the
generation rather than raising the bound.

### 10.13 `tests/solve/` is Rust, not a blessable directory

§4 leaf 5 asks for `tests/solve/*.vlt` with blessable `.sig` expectations. What
shipped is `crates/voltaic-lattice/tests/solve.rs`, which asserts the same property
against inline programs.

The reason is the oracle. A blessable expectation file is only worth having when the
expectation is *expensive to state and cheap to check* — which is true of the golden
`.vrs` corpus, where the expected text is a hundred lines of Rust. A judgement is one
line, and generation 1's own signature already states it, so a `.sig` file would be a
third copy of a fact the source and the corpus both carry. Blessing it would also
make a changed judgement absorbable by re-blessing, where a Rust assertion makes it a
code change someone has to justify.

**When the blessable form becomes the right shape is row 100**, which introduces
`.vlt.sig` as the *published API* — there the file is the artifact rather than a
restatement, and `voltaic sig` is already able to write it.

Committed per leaf rather than per plan: each leaf has its own gate, and a single
commit containing an inference engine is not reviewable. The register row stays
`in-progress` across them, as row 99 §1.1 plans for its nine waves.

# Plan 101 — Diagnostics Without Signatures

Plan ID: `diagnostics-2`
Depends on: 94, 95, 96, 97
Milestone: 17 — *the turn*
Status: `planned` — **approved jointly with row 92**, per that row's §2.3

---

## 1. What this plan is

The four new diagnostic ranges, constraint-provenance rendering, definition-boundary
reporting, `tests/explain/`, and a UI fixture for every new code.

### 1.1 It is the condition on which the charter was reversed

Row 92 §2.3 says it in those words: *"This plan should not be approved without row 101
being approved with it."*

The reversal's entire justification is that the solver can explain itself. Doc 86 §2
clause 1 admits a solver-determined lowering **only if the choice can be printed**. A
generation that shipped inference without provenance rendering, without `--why`, and
without fixtures for the four new ranges would be exactly the language that clause was
written to forbid — and it would be the known failure mode of every language that
inferred everything.

So this row is not a polish pass. It is the second half of row 92.

### 1.2 The failure mode, stated precisely

A type error in a program with no written signatures has nowhere to anchor. The naïve
behaviour is to report at the point the constraint store became unsatisfiable, and that
point is frequently nowhere near the mistake:

- the author writes a wrong field name in a helper,
- unification succeeds locally because the field is row-polymorphic,
- the conflict surfaces three functions away, at the first *use* that pins the row,
- and the message names two types the author never wrote.

This is Haskell's and Scala's reputation, and it is earned. Generation 1 does not have
the problem at all, because every signature is written and every diagnostic has a line
to point at — which is a genuine advantage this row is trying to recover rather than
replace.

### 1.3 The four answers

| | Mechanism | Owner |
|---|---|---|
| 1 | **Constraint provenance is mandatory** — every constraint carries a span and a structured reason | row 94 §1.3, built |
| 2 | **Report at the definition boundary by default** — against the definition's inferred signature and the use that contradicted it, not the innermost expression | this row, leaf 2 |
| 3 | **`--why` for any judgement** — so an author can interrogate rather than guess | row 100 leaf 3, built |
| 4 | **`tests/ui/` grows to match** — a range with no fixtures is not shipped | this row, leaf 5 |

Answer 2 is this row's central technical content and the one that is a real design
decision rather than plumbing.

### 1.4 Definition-boundary reporting

The rule: **a conflict is reported against the nearest enclosing definition's inferred
signature, naming both the constraint that came from inside and the constraint that
came from outside.**

```
VLT0401: `name` cannot be both `String` and `&str`
  --> cursor.vlt:12:16
   |
12 |   pub label self = self.src
   |                    ^^^^^^^^ inferred `&str` here, from the field's type
   |
  --> cursor.vlt:31:9
   |
31 |   store (c.label)
   |         ^^^^^^^^^ required `String` here, because `store`'s parameter escapes
   |
   = `label`'s inferred signature is `self -> &str @ shared`
   = run `voltaic sig --why label` for the full constraint chain
```

Three properties of that shape, each chosen against the naïve alternative:

- **Two spans, never one.** A unification failure has two sides by construction and
  reporting one is what makes inference feel arbitrary.
- **The inferred signature is printed in the message.** The author did not write it,
  so the diagnostic must supply it; otherwise the message refers to a fact the author
  has no way to see.
- **`--why` is named in the message.** The full chain can be long, and a diagnostic
  that inlines it is unreadable while one that omits it is unhelpful. Naming the
  command is the resolution.

---

## 2. Preconditions

- Rows 94–97 `done`: all four engines exist and all four produce constraints with
  provenance.
- `voltaic-diag` already reports against the source the author wrote, with 1-based
  line and column in characters. That machinery is generation 1's and it carries
  forward; what is new is what gets rendered into it.
- Row 93 moved `RDX####` to `VLT####` keeping the numbers. This row is where the
  numbers move into doc 87 §17's ranges, and it is the only row that renumbers — one
  churn instead of two.

---

## 3. Concrete proof this row targets

Three programs, each a genuine instance of §1.2's shape rather than a toy:

1. **A wrong field name in a helper**, surfacing three functions away. The diagnostic
   must name the helper's definition and the use that pinned it, not the innermost
   expression.
2. **A mode conflict** — a value read in one place and returned past the frame in
   another — producing `VLT0601` with both uses named and their modes.
3. **An effect that cannot be discharged** — a `fail` reaching a context with no
   `else` and no propagation path — producing `VLT07xx` naming the raise site and the
   boundary it could not cross.

Each is a `tests/ui/` fixture, and each one's `.stderr` is read rather than blessed,
because the whole content of this row is whether those three messages are good.

---

## 4. Leaves

### Leaf 1 — the ranges

Renumber into doc 87 §17: `VLT01xx` lexical, `02xx` parse, `03xx` resolution, `04xx`
types, `05xx` ambiguity and visibility, `06xx` modes, `07xx` rows, `08xx` refinements,
`09xx` the sidecar.

### Leaf 2 — definition-boundary reporting

§1.4. Includes the rule for nested definitions — a conflict inside a `where` group is
reported against the enclosing definition, because a `where` binding has no published
signature to point at.

### Leaf 3 — provenance rendering

Walk row 94's chain and render it. The structured reasons — `ParameterUse`,
`FieldAccess`, `CallArgument(n)`, `Discharge`, `Ascription`, `ClassRequirement` —
each get a phrasing, and the phrasings are data rather than format strings so that
`tests/explain/` can assert them.

### Leaf 4 — `tests/explain/`

`.vlt` plus `.why` expectation files, blessable. The corpus that keeps `--why` honest:
a change to the solver that makes an explanation worse shows up as a diff.

### Leaf 5 — UI fixtures per range

At least one fixture per new code, and the three in §3 as the anchors. **A range with
no fixtures is not shipped** — doc 88 §6.1, and this leaf is where that is enforced
rather than stated.

### Leaf 6 — the suggestion machinery

Where a diagnostic has an obvious fix, it names it: a mode conflict suggests the
restructure or `@clone`; a missing discharge suggests `else`; a fragment violation
names the subterm. Suggestions are **named, not applied** — `--fix` is not in this
generation, because a suggested edit that is wrong is worse than no suggestion and the
measurement for that does not exist yet.

---

## 5. Acceptance criteria

1. §3's three programs produce diagnostics whose `.stderr` names both sides, prints
   the inferred signature, and names `--why`.
2. Every `VLT04xx`, `VLT06xx`, `VLT07xx` and `VLT08xx` code has at least one UI
   fixture.
3. `tests/explain/` green, with an expectation for each structured reason.
4. No diagnostic in the corpus reports a span inside a definition when the conflict is
   between a definition and its use. Asserted by inspection of all 55-plus fixtures,
   because it is the rule this row exists to establish and a single violation makes it
   a suggestion rather than a rule.
5. Every diagnostic is still reported against the `.vlt` the author wrote, with 1-based
   line and column in characters. Generation 1's property, and the regression this row
   is most able to cause.
6. No message names a type the author did not write **without also naming where it
   came from**. This is §1.2's failure mode in one sentence and it is the criterion
   most worth arguing about in review.

---

## 6. Out of scope

- **`--fix`.** Leaf 6.
- **Improving generation 1's existing diagnostics.** 55 fixtures carry them and they
  are good; this row adds ranges rather than revisiting them.
- **Error recovery.** Generation 1's item-boundary recovery carries forward unchanged,
  including the behaviour `AGENTS.md` records as a trap: a `module <keyword>` failure
  emits one correct diagnostic and then a dozen cascading ones, and the advice is to
  read the first rather than the loudest run. The reversal does not change that and
  this row does not improve it.
- **Localisation.** Not in this generation, not named as a venue.

---

## 7. Gate

Standing gate, plus `tests/explain/`, plus the per-range fixture requirement, plus
§5.4's inspection.

---

## 8. Emerald reuse

One negative lesson, cited because it is the same class of mistake: Emerald found a
whole AST node type with **no span at all**, twenty-one plans after its diagnostics
plan shipped, and by then the fix was judged too large and it settled for a
workaround. `AGENTS.md`'s *every AST node carries a byte span* rule exists because of
it.

The analogue here is provenance: a constraint without a reason is this generation's
node without a span, and row 94 §1.3 asserts against it **in the row that builds the
store** rather than in this one, for exactly Emerald's reason.

---

## 9. Decision log

| Decision | Why |
|---|---|
| Approved jointly with row 92 | the reversal's justification is that the solver can explain itself |
| Definition-boundary reporting as the default | the innermost-expression default is the known failure mode |
| Always two spans | a unification failure has two sides by construction |
| Print the inferred signature in the message | the author did not write it and cannot otherwise see it |
| Name `--why` rather than inline the chain | inlining is unreadable, omitting is unhelpful |
| Structured reasons, not format strings | `tests/explain/` must be able to assert them |
| Suggestions named, not applied | a wrong auto-fix is worse than none, and the measurement does not exist yet |
| Renumber here, not in row 93 | one churn instead of two; this row owns the ranges |

---

## 10. Result

Partial. The **SOLVE stage reaches diagnostics**, which every leaf below assumes and
none of them lists: before this, `voltaic-lattice` was a library with no caller in the
pipeline, and §5's criteria are all unreachable from a solver nothing invokes. Leaves
1 and 3–6 remain, and §10.4 is why leaf 1 has to come before any of them.

### 10.1 What shipped

`crates/voltaic-driver/vlt/judge.vlt` runs `walk` then `solve_with` over a parsed
program and turns each reported contradiction into a `Diagnostic`. It is called from
`lint`, not `check` — `code_coverage.rs` records that a UI case must fail to build and
a lint case must succeed, and a judgement is a **warning**, so it has no home in
`tests/ui`. `check` still reaches it, because `check` calls `lint`.

The severity is not a detail. Doc 86 §2's first corollary makes rustc the authority on
types, so Lattice **advises**: a judgement that turned out to be wrong must not be able
to stop a build that rustc would have accepted.

`crates/voltaic-cli/tests/lint/lattice_conflict.vlt` is the anchor fixture, blessed to:

```
warning[VLT1004]: bool cannot also be u64
  --> lattice_conflict.vlt:19:1
19 | wrong = true
   | ^^^^^
  = help: pinned by an ascription. Run `voltaic sig --why` for the constraint chain.
```

### 10.2 The rule that made the stage shippable: silence where Lattice cannot model

The first wiring reported **every** entry in `Outcome.failures`, and that broke six
tests across `examples`, `run`, `phase1` and `currying` — all of them on programs that
are correct, and two of them in the corpus that exists to prove the compiler works.

The diagnostics were not noise in the sense of being badly worded. They were *wrong*,
and each one was Lattice reporting its own incompleteness as though it were the
programmer's mistake:

| Program | Reported | Actually |
|---|---|---|
| `apply : &dyn Fn(i32) -> i32 -> i32 -> i32` | `?0 must be callable with 1 argument(s)` | a `dyn Fn` is callable in Rust and is not an arrow in this model |
| `and_then : self -> impl Fn(u64) -> Tally -> Tally` | `?8 must be callable with 1 argument(s)` | the same, through `impl Fn` |
| `depth : Meters -> Depth`, `depth m = m.0` | `f64 cannot also be Depth` | `type Depth = f64`, so they are one type |

So the stage needed a rule for **when Lattice is entitled to speak**, and finding it
took three attempts. The two that failed are worth keeping, because each looked
sufficient and was not:

1. **"Report only a mismatch between two ground names."** `bool` against `u64` has no
   structure left to have modelled wrongly, so it cannot be a modelling gap. Correct as
   far as it goes, and insufficient: `unify_constructors` **recurses**, so a `Callable`
   obligation on `impl Fn(u64) -> Tally` compares arrow against arrow, descends, and the
   *inner* comparison is `impl Fn(u64)` against `u64` — two ground names. The mismatch
   that surfaces is ground while the thing that went wrong is three levels up.
2. **"Report only `Unify` obligations."** A `Unify` comes from a type the programmer
   wrote; `Callable` and `Member` are Lattice's own derivations. This is the right
   *line* and still not enough on its own, because a `Unify` between a written alias and
   a written primitive is ground, is the programmer's, and is correct code.

The rule that holds is **both**, and it is `is_contradiction` in `driver.vlt`:

> A failure is a contradiction when its mismatch is between two distinct nullary names
> **and** the obligation it came from is a `Unify` — that is, when the programmer wrote
> both sides and there is no structure left for Lattice to have misunderstood.

`Obligation::is_written` is the second half, and it lives on the obligation rather than
in the driver because it is the obligation's own answer to *"did the programmer write
both sides of this?"*.

Everything else stays in `Outcome.failures`, where `voltaic sig --why` can still show
it. The distinction is between what Lattice *knows* and what Lattice *says*.

### 10.3 Type aliases are transparent, because they are transparent to rustc

The third row of the table above is the one case that was a plain defect rather than a
boundary question. `type Depth = f64` makes `Depth` and `f64` one type and rustc expands
it before comparing anything; Lattice compared the names. `Declarations` now carries
`aliases` from `Item::Type`, `expand` rewrites a type through them, and `apply` expands
both sides of every obligation before unifying — so an alias is transparent everywhere
rather than at the one call site that noticed.

Expansion terminates by refusing to expand a name twice, which means a cyclic alias
reaches a fixed point instead of looping. rustc rejects the cycle itself (`E0391`), so
terminating is the whole obligation: the test asserting it says so.

`newtype` is deliberately **not** an alias — it is a distinct type with a field 0, and
there is a test for that beside the alias tests, because the two declarations look alike
and mean opposite things.

### 10.4 The code is `VLT1004`, in the lint band, and leaf 1 has to move it

Leaf 1 renumbers into doc 87 §17, where a type conflict is `04xx`. That renumbering has
not happened, so this shipped as `VLT1004` in the 1000+ lint band — which is also
*correct* for what it currently is, a warning rather than an error. When leaf 1 lands it
has to decide whether a Lattice judgement is a `04xx` error or stays a lint, and §10.1's
argument says the answer depends on whether the engines have become trustworthy enough
to block a build. They have not yet.

### 10.5 Measured: the stage is silent on the whole corpus

44 examples, 0 unsolved judgements. The full workspace suite is **1,140 / 1,140** with
the stage live. That number is the criterion the rule in §10.2 was built to meet, and
it is a weaker claim than it looks: the stage is quiet because it is *restricted*, not
because inference is complete. Widening what Lattice is entitled to say is leaves 1–6
and rows 95–97, and each widening has to re-measure this.

### 10.7 Leaves 3 and 4

**Leaf 3 — provenance rendering.** `steps` returns the chain as structured
`Step` values and `rendered` prints them with line:column plus the obligation
each step asserts. Three things were wrong with what `explain` did before:

- it printed **byte offsets**, which is not something a reader can act on.
  `line_col` already existed in `voltaic-diag` and was private; it is exported
  rather than reimplemented;
- `Store::chain` hands back provenances, which carry the reason and the span and
  **not** the obligation, so an explanation built from it could say why a
  constraint exists and not what it says. `chain_indices` is the same walk
  returning indices, and `chain` is now written in terms of it so the two
  cannot diverge;
- `CallArgument n` rendered as *"passed as an argument here"*. The index was
  recorded by row 94 and dropped by every renderer. `phrasing` is the
  ordinal-aware form and `label` stays the plain one.

**Leaf 4 — `tests/explain/`.** Six cases, each `.vlt` beside the `.why` it
earns, blessable with `VOLTAIC_UPDATE=1`. There is no way to assert that an
explanation is *good*; there is a way to make a change that makes one *worse*
show up as a reviewable diff, which is what the leaf asks for.

Both streams are captured into the expectation. A contradiction goes to stderr
and the chain to stdout, so a harness reading one stream would have recorded
nothing at all for `wrong.vlt` — the case that exists to show a contradiction
explained.

### 10.8 Measured: half the reason vocabulary cannot appear

Leaf 3 says every structured reason gets a phrasing. Writing them exposed that
**six of the twelve `Reason` variants are constructed by no code**, and two more
are constructed but cannot reach `--why`:

| | Reasons | Why |
|---|---|---|
| reached | `Ascribed`, `EquationBody`, `FieldAccess`, `CallCallee` | emitted and visible; the corpus pins them |
| produced, unreachable | `GuardCondition`, `ClassRequirement` | `touching` selects by hole intersection with the *subject*, and these holes belong to a subexpression |
| never produced | `CallArgument`, `Discharged`, `ElementOf`, `LiteralKind`, `ParameterUse`, `ReturnPosition` | no constructor anywhere |

The last column is the useful part. `Discharged` waits on row 95; `CallArgument`
waits on per-argument constraints rather than one bundled `Callable`; and
`LiteralKind` and `ReturnPosition` are simply not used where the generator could
use them — it reaches for `Ascribed` and `EquationBody` instead.

An unreachable phrasing is **indistinguishable from a working one** until
something makes it reachable, so this is a gate rather than a note:
`every_unreachable_reason_is_still_unreachable` fails if one of the eight starts
appearing, and says in its message that this is progress and the phrasing should
move to the reached list with the case that reached it.
`the_three_categories_cover_every_reason` asserts the three lists still account
for all twelve, so none can be dropped from all three.

This is the same two-sided shape `no_comments.rs` and `port.rs` use, and for the
reason recorded there: a list plus a check that reads it can be narrow in two
ways, and only one of them is visible from the green side.

### 10.9 `&<Point>` was what every judgement showed a reader

`&` and `&mut` are ordinary constructors in row 94's representation, so the
generic arm of `Display` printed `&<Point>` and `&<str>` — in `sig` output, in
`--why`, and in every judgement diagnostic. A reader would not recognise the
type they had just written. Two arms fix it, and four tests pin the forms
including the fallback for a constructor merely *named* `&` with the wrong
arity. Four existing expectations asserted the old spelling and were updated.

Worth noting what found this: the explain corpus. A type printed wrongly is
invisible in a unit test that round-trips it, because the same wrong spelling
appears on both sides of the assertion — which is exactly what
`annotation.vlt`'s tests were doing. It is visible the moment a human reads a
blessed file.

### 10.10 Leaf 2 — definition-boundary reporting

All three of §1.4's properties hold. The anchor fixture now renders:

```
warning[VLT1004]: `wrong` cannot be both bool and u64
  --> lattice_conflict.vlt:18:1
   |
18 | wrong : u64
   | ^^^^^^^^^^^
  --> lattice_conflict.vlt:19:1
   |
19 | wrong = true
   | ^^^^^ required u64 here
  --> lattice_conflict.vlt:19:7
   |
19 | wrong = true
   |       ^^^^^^ inferred bool here, an equation's body returns this
  = help: pinned by an ascription. Run `voltaic sig --why wrong` for the constraint chain.
```

`enclosing` finds the **innermost** definition containing the constraint's span —
innermost, not first, which is what makes §1.4's `where`-group rule fall out
rather than need a special case: a nested definition's span is strictly inside
its parent's, so the tightest containing span is already the right answer.

**The second side is the opposing constraint, not the enclosing definition.**
The first attempt pointed the second caret at the definition and produced a
diagnostic with the *same* snippet twice, because the primary span is already the
definition's. `opposing` scans the store for another constraint sharing a hole
with the failing one at a different span — which is the constraint that bound the
variable the other way, and therefore the place §1.4 means by *"inferred `&str`
here"*. `neither_secondary_span_repeats_the_primary` pins it, because a repeated
snippet reads as two findings.

`Diagnostic` grew `related : Vec<(Span, String)>` and the plain renderer draws
each one the way it draws the primary. A `Vec` rather than an `Option`: a
`Member` or `Callable` obligation can have more than two participants, and a
field that holds only the second would silently choose which to drop.

### 10.11 The caret ran 42 characters past the end of its line, and had for a while

Pointing a diagnostic at a *definition* span is the first thing in this tree that
spans more than one line, and it exposed that `caret_width` never consulted the
line it was drawing on. `clamped_width` stops the run at the line end.

That is not a new defect. `tests/ui/unterminated_attribute.stderr` was blessed
with **57 carets under a 15-character line**, because `#[derive(Debug)`'s span
runs to end of file:

```
1 | #[derive(Debug)
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^   (was)
  | ^^^^^^^^^^^^^^^                                            (now)
```

It is the only UI expectation that changed, and it changed in the right
direction. Worth noting how it survived: a blessable corpus records what the
compiler *does*, so an output that was wrong when first blessed stays wrong and
looks reviewed. The corpus catches *changes*; it does not catch a bad baseline.

A second defect fell out of the same code: `clamped_width` computed
`line_length + 1 - column` and panicked with *"attempt to subtract with
overflow"* on a span whose column is past the line end — which is every
EOF-anchored span, including the one above. The guard is a comparison before the
subtraction. An arithmetic panic inside the *diagnostic renderer* is the worst
place for one, because the compiler then fails while trying to explain a failure.

### 10.6 Registration, again

The driver taking `voltaic-lattice` propagated into the **transitive** closure of four
other crates — `voltaic-bench`, `voltaic-lsp`, `voltaic-mcp`, `voltaic-seed` — and
`moon_project_graph.rs` named all four. `dependency_sources` is a transitive list and a
new edge anywhere above a crate changes it, which is exactly the hole AGENTS.md records
`deps` not closing.

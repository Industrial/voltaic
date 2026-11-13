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

*To be filled after execution. Required: the three anchor diagnostics' final text, and
any fixture where §5.4's rule had to be broken, with the reason.*

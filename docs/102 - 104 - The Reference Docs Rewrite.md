# Plan 104 — The Reference Docs Rewrite

Plan ID: `reference-docs`
Depends on: 103
Milestone: 17 — *the turn*
Status: `planned`

---

## 1. What this plan is

The four gated reference documents, rewritten against generation 2: `GRAMMAR.md`,
`DECLINED.md`, `CONFORMANCE.md`, `STATUS.md`. Plus re-pointing the gates that read
them, and the `dependency_sources` edit row 100 left for this row.

It is the **last** row in the register, and that ordering is a finding rather than
scheduling.

### 1.1 A gated reference document cannot be written ahead of its code

Each of the four carries machine-checked evidence:

| Document | Gate | What it resolves |
|---|---|---|
| `GRAMMAR.md` | `grammar_inventory.rs` | every row's evidence — a corpus file that uses the construct, and for `UNREACHABLE`, a fixture asserted to **still fail** |
| `DECLINED.md` | `declined.rs` | every construct in doc 03 §22 appears here |
| `CONFORMANCE.md` | `conformance.rs` | the blocking-occurrence counts, and `seed/GENERATION` |
| `STATUS.md` | `status.rs` | **generated** — every number measured from the tree, and a drifted number fails the build |

So writing the new `GRAMMAR.md` at the start of generation 2 would produce a document
whose every evidence path points at a file that does not exist, and whose
`UNREACHABLE` fixtures cannot be asserted to fail because they cannot be asserted at
all. **The reference docs are an output of the turn, not an input to it.**

Doc 90 §6 discloses the consequence: for the whole of the generation, `DECLINED.md`
asserts a decline — the type checker — that row 92 reversed. That is a knowing
inconsistency, disclosed at the start, and the alternative is worse: an
evidence-carrying document that cannot resolve its evidence is less trustworthy than
one that is out of date.

### 1.2 Doc 86 §10's table is a prediction and this row is where it is tested

Doc 86 §10 predicts that three of generation 1's eight unreachable constructs become
reachable — or-patterns, `as` casts, `ref`/`ref mut` — as a *consequence* of the
solver rather than as features.

Plan 46 measured what a careless inventory is worth: of sixteen apparent gaps in a
first pass, **eleven were the probe's own bug**. A false `UNREACHABLE` is worse than a
missing row, because it sends a future plan to design a surface for something that
already works.

So this row does not copy doc 86 §10's table. It **regrades from scratch**, construct
by construct, against Rust's grammar as organised by the Rust reference, with a corpus
file or a failing fixture for every row. If the prediction was wrong, this row's §10
says so and doc 86 §10 gets a dated correction.

### 1.3 What row 98 may have added to the unreachable list

Row 98 §6 leaves one question open: whether `examples/template.sdx` used `Fields` or
`FieldNames`. If it did, declaration reflection is a **capability generation 1 had and
generation 2 does not**, and this row grades it `UNREACHABLE` with a fixture asserted
to fail.

That is the honest accounting. Doc 89's *What this register does not claim* says a
register that only counted gains would hide the regressions, and this is the row where
a regression becomes a graded, gated, visible fact rather than a sentence in a plan
document.

---

## 2. Preconditions

- Row 103 `done`: the tree is generation 2 and `fixpoint` is green on g2, so every
  evidence path a row might cite exists and compiles.
- Rows 95–99 `done`, so the constructs to grade are the constructs that ship.
- The three fixtures row 99 wave 9 removed from `tests/unreachable/` are gone, and the
  assertion that they fail was updated in that wave — so this row inherits a
  consistent fixture directory rather than fixing one.

---

## 3. Concrete proof this plan targets

`grammar_inventory.rs` green with every row's evidence resolving, on a tree where
nothing is called `.sdx` and nothing emits `__rdx_`. That is the single check that the
generation is coherent: it reads every claim the project makes about its own coverage
and resolves each one against a real file.

---

## 4. Leaves

### Leaf 1 — `GRAMMAR.md`, regraded from scratch

Organised by the Rust reference's areas, as generation 1's is, so coverage is checked
against the host language rather than against this project's history. Every row carries
evidence. §1.2's discipline: regrade, do not copy the prediction.

Generation 1's grade vocabulary is kept — `PASSTHROUGH`, `SUGARED`, `UNREACHABLE` —
because it is the right vocabulary and changing it would make the two generations'
inventories incomparable.

### Leaf 2 — `DECLINED.md`, rewritten against doc 87 §22

And `declined.rs` re-pointed from doc 03 §22 to doc 87 §22.

Three things this leaf must carry, each a reversal or a regression rather than a new
refusal:

- **the type-checker decline, lifted**, with doc 90 as the argument and doc 86 §1.1 as
  the measurement;
- **UFCS, lifted**, and doc 87 §22.5's note that its *reason* was the charter rather
  than the construct;
- **general monadic comprehension over user types, declined again** — and this is the
  delicate one. Generation 1 declined it, then **lifted** it at plan 86 with the
  argument written down, and generation 2 declines it once more because the effect
  alphabet is closed. A document that recorded only the final state would read as
  though it had never been lifted. The row must carry all three movements and the
  venue doc 93 §6 names.

That construct's entry is the most-travelled row in either generation's decline list,
and the record of its travel is worth more than its current verdict.

### Leaf 3 — `CONFORMANCE.md`

What the surface covers today, recounted. Generation 1's measurement went from 952 of
964 blocking occurrences to 12, and the twelve were `as` casts — which generation 2
makes reachable, so the count changes and the shape of the document does too.

### Leaf 4 — `STATUS.md`, regenerated

`status.rs` is amended for the fourteenth crate, the three new corpora
(`tests/solve/`, `tests/refine/`, `tests/explain/`), the renamed extensions, and
`seed/GENERATION`. Then it regenerates — rewriting and then **failing**, so the diff is
reviewed.

Generation 1's posture is kept verbatim and it is worth restating, because a
generation change is exactly when a project starts quoting numbers at itself:

> **No coverage percentage.** A number here would become a target, and it would mostly
> measure the emitter's match arms. **No test count as a quality measure.** **No
> roadmap and no dates.** **No benchmark numbers** — copying one here would make this
> comparison flake.

To which generation 2 adds one of its own: **no terseness ratio in `STATUS.md`.** Doc
86 §1.1 and row 99 §1.2 measure it, in documents that explain what was measured and
what was equalised. A bare `2.24×` in a status table would be quoted without the
caveat that 1.76× is the grammar-only figure, and the project's own §1.2 calls
claiming the library's share *rounding up*.

### Leaf 5 — `dependency_sources`

Add `.vlt.sig` to each crate's `dependency_sources` file group, and the assertion to
`moon_project_graph.rs`. Row 100 leaf 2 established why: `deps: ['^:check']` sets the
order tasks run in, not the hash they are cached under, so the file group is what makes
per-crate caching correct.

### Leaf 6 — doc 86 §10's correction

If §1.2's regrade contradicts the prediction, doc 86 §10 gets a dated note saying
which rows were wrong and what the fixture showed. Appended, not rewritten.

### Leaf 7 — headers on docs 01–85

A one-line header on each of generation 1's numbered documents pointing at its
successor. Not a rewrite: `docs/01` through `docs/85` stay exactly as they are, because
they are the record of what generation 1 did and editing them falsifies it.

---

## 5. Acceptance criteria

1. `grammar_inventory.rs` green, every row's evidence resolving.
2. `declined.rs` green against doc 87 §22, with the three movements in leaf 2 all
   present.
3. `STATUS.md` regenerated and matching the tree; the four *does not say* clauses
   intact, plus the new fifth.
4. `CONFORMANCE.md` recounted, with `seed/GENERATION` as its crate list.
5. `moon_project_graph.rs` green with `.vlt.sig` in every `dependency_sources`.
6. If row 98 §6 resolved against `Fields`, declaration reflection is graded
   `UNREACHABLE` with a failing fixture.
7. Doc 86 §10 carries a correction if the regrade disagreed with it, or an explicit
   confirmation if it did not. **Silence is not an acceptable outcome** — a prediction
   whose test went unreported is a prediction that will be quoted as fact.
8. Docs 01–85 are unedited apart from the one-line successor header.

---

## 6. Out of scope

- **`README.md` and the book.** Rows 92 and 93 amended the charter sentences and the
  extensions; a full rewrite of the book for generation 2 is its own row in a later
  milestone and is not pretended at here.
- **Deleting docs 01–85.** Never. Doc 04 rule 3.
- **`docs/MIGRATION.md`.** Written with row 93 and maintained by rows 99 and 103 as
  their construct tables settle; this row only checks it is consistent with the final
  grades.

---

## 7. Gate

`grammar_inventory.rs`, `declined.rs`, `conformance.rs`, `status.rs`,
`moon_project_graph.rs`, and the standing push gate. Five of those six read a document
and resolve it against the tree, which is why this row is the one that proves the
generation is coherent rather than merely finished.

---

## 8. Emerald reuse

Emerald's negative example is the whole reason these documents are gated. `GRAMMAR.md`'s
own header states it:

> So this is **risk detection**, not documentation.

A reference document nobody checks drifts, and a drifted inventory is worse than none
because it is still quoted. Generation 2 inherits the mechanism unchanged.

---

## 9. Decision log

| Decision | Why |
|---|---|
| Last row in the register | §1.1 — a gated document cannot be written ahead of its code |
| Regrade from scratch rather than copy doc 86 §10 | plan 46: eleven of sixteen apparent gaps were the probe's bug |
| Keep generation 1's grade vocabulary | changing it would make the two inventories incomparable |
| Carry all three movements of the monadic-comprehension row | the record of its travel is worth more than its verdict |
| Add a fifth *does not say* clause for terseness | a bare ratio would be quoted without the equalisation caveat |
| Grade a lost capability `UNREACHABLE` with a failing fixture | a graded regression is a debt; a silent one is a lie |
| Require doc 86 §10 to be corrected *or* confirmed | an untested prediction becomes a quoted fact |
| Docs 01–85 get a header and nothing else | they are the record; editing them falsifies it |

---

## 10. Result

In progress. **`DECLINED.md` is rewritten and its gate re-pointed**, which is the
one item of this row's five that does not need row 103's migrated tree. Doc 87
§22's own opening paragraph names this row as the owner of that re-pointing, so
it is scoped rather than opportunistic.

### 10.1 The gate reads **both** §22 sections, not the newer one

`declined.rs` checked `DECLINED.md` against doc 03 §22. The obvious reading of
"re-point it at doc 87 §22" is to swap the source, and that would have been
wrong: doc 87 §22.3 and §22.4 carry generation 1's reasons forward *"in
substance"* rather than verbatim, so doc 03 §22 is still the only place several
of them are stated in full. Swapping would have silently stopped asserting them.

Reading both is the only version under which no decline loses its gate. Two
floors keep each section contributing — 15 from doc 03, 14 from doc 87 — because
a single combined floor would let the newer section parse as *empty* while the
gate stayed green on the older one's rows alone.

### 10.2 Seven declines were in one place only

The gate found them the moment it read both sections, which is the whole value of
reading both:

| Decline | Added to `DECLINED.md` as |
|---|---|
| An executor | a new row under *Rust has no target for it* |
| A standard library beyond `say` / `ask` / `die` | the same |
| Const-generic arithmetic | a new section, *needs more than a decidable fragment* |
| Dependent types, quantified refinements, user calls in a predicate | the same section |
| Higher-kinded abstraction, monadic comprehension, effect handlers | a new section, *needs a higher-rank or higher-kinded system* |
| Implicit cloning | a row under *fabricate names or API* |
| Inheritance / `extends` | the same |
| Declaration reflection | the same, with the `macro` venue named |
| `macro` definitions | **not** a decline — see §10.4 |

### 10.3 `UFCS` and the type checker needed no exception, and finding that out mattered

The first implementation carried an allow-list so the two rows §22.5 **lifted**
would not be demanded of a decline list. It excused nothing: §22.5 states both as
prose blockquotes rather than table rows, so the table parser never saw them.

The document's own structure already draws the line the allow-list was trying to
draw, so the allow-list is gone and `the_lifted_rows_are_stated_as_prose` keeps
the structure in place instead. If a later edit moves those blockquotes into a
table, the gate would start demanding that `DECLINED.md` list two constructs that
*ship*, and the failure would read as a missing document entry rather than a
mis-filed row — which is why it is worth catching at the structure rather than at
the symptom.

### 10.4 The *merely absent* section was empty, and that is why a decline had nowhere to go

`DECLINED.md`'s own heading *"Not declined — merely absent"* draws the
distinction the document exists for, and it had no rows. So `macro` definitions —
which doc 87 §22.4 lists in the §22 table and then explicitly calls
`UNREACHABLE` *rather than* declined — had nowhere to be written down, and the
gate reported it as an undocumented decline.

It is filled now, with four constructs and where each stands: `macro`
definitions, the or-pattern `|` **spelling** (the capability ships as `or`), `as`
casts, and `ref` / `ref mut`. Each names the row that unblocks it rather than a
reason it never will.

### 10.5 An exemption that was waiting for this row, removed

`no_living_document_asserts_the_retired_charter` exempted `docs/DECLINED.md`,
with this beside it:

> Until then the repository knowingly ships a decline list naming a decline that
> has been lifted, which is better than an evidence-carrying document whose
> evidence does not resolve.

**This row is the "then".** Neither retired phrasing survives the rewrite, the
exemption excuses nothing, and it is deleted. A disclosed inconsistency is still
an inconsistency; removing the exemption is what closes it rather than admitting
it.

### 10.6 What remains

- **`GRAMMAR.md`'s regrade** and **`CONFORMANCE.md`'s rewrite** need generation
  2's surface to grade against, which is row 99 waves 2–9.
- **`.vlt.sig` in `dependency_sources`** needs the sidecars committed, which is
  the half of row 100 leaf 1 that doc 98 §10.4 enumerates.
- **`STATUS.md`** is regenerated by `VOLTAIC_UPDATE=1 cargo test -p voltaic-cli
  --test status` and has been current at every commit of this generation, so
  there is nothing for this row to do but keep it that way.

*Still to fill when the regrade lands: its disagreements with doc 86 §10, and the
final `UNREACHABLE` count. Declaration reflection **did** join the list — doc 87
§22.4's row, in `DECLINED.md`'s "fabricate names or API" section, with the `macro`
construct named as its venue.*

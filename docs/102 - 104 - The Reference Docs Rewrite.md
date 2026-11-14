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

### 10.6 Leaf 7: eighty-five documents that read as current

Every one of generation 1's numbered documents now opens with one line naming its
successor. `crates/voltaic-cli/tests/generation_headers.rs` holds five assertions
over them.

**The cost this closes is specific.** Doc 04 rule 3 forbids deleting a superseded
document, which is right — it is the record of what was done and why. What it
leaves behind is 85 documents that are *indistinguishable from current
documentation when read alone*. A reader who opens
`docs/03 - Voltaic Surface Design.md` because its title matches their question
gets generation 1's surface stated with no hedging: `.f x` with the receiver
**last**, signatures required, `if` and `case` present. All three are reversed by
generation 2, and nothing in the document said so.

**Seventy-eight of the eighty-five point at doc 89, and that is the honest
answer rather than the lazy one.** A generation-1 plan has no generation-2
counterpart: the work was reorganised into nineteen rows, not renumbered.
Inventing a per-plan mapping would assert a correspondence this project never
made. Doc 89 is where the planning went.

**Six point at something else, and the gate asserted seven.** Generation 1 has
seven non-plan documents, and `docs/04 - Voltaic Plan of Plans.md`'s specific
successor *is* doc 89 — a register's successor is a register. The count was
written from the structure (*seven foundational documents*) rather than measured
from the headers, which is §2.1's shape for the third time in this generation.
The gate now asserts six and says why it is not seven.

The fifth assertion is the one worth keeping: **every path a header names must
exist.** A header pointing at a renamed document is worse than no header, because
it reads as maintained.

### 10.7 Leaf 4: the corpora table, and two of the three predicted corpora never existed

`STATUS.md`'s corpora table now has nine rows where it had five. The three added
are generation 2's: `lint/`, `run/`, `explain/` and `sidecar/` — four, not three,
and none of them where the leaf said.

The leaf named **`tests/solve/`, `tests/refine/` and `tests/explain/`**. Measured:
`tests/solve/` and `tests/refine/` **do not exist**, and `explain/` is under
`crates/voltaic-cli/tests/`. What happened to the other two is not a gap — the
solve corpus became `inferred.rs`'s answer key, which is a *count* rather than a
directory of fixtures, and the refinement corpus became `voltaic-lattice`'s own
inline tests. Both are the right homes; the leaf's list was written before either
existed.

§1.2 says this row is where doc 86 §10's prediction is tested. It turns out to be
where *this row's own* predictions are tested too, and the same discipline applies:
regrade, do not copy.

**Nothing needed amending "for the fourteenth crate".** `status.rs` reads the crate
list from the `crates/` directory, so there is no count in it to go stale — and
`seed/GENERATION` would have been the *wrong* source, because it lists the crates
whose `src/` is generated, which is a different set from the crates that exist. A
crate not yet ported would have vanished from the table rather than appearing with
its line counts. The leaf's instruction to use `GENERATION` is declined, with that
reason.

The fifth *does not say* clause is added, and it carries one number the leaf's
version did not: the character saving over the whole compiler is **1.002×**, which
is doc 90 §1's subject. A status table quoting `2.24×` without that is the exact
rounding-up the project's own §1.2 names.

### 10.8 What remains, and which row blocks each

| Leaf | State |
|---|---|
| 1 — `GRAMMAR.md` regraded | **partly done** — §10.10. Three rows blocked per construct (§10.9) |
| 2 — `DECLINED.md` | **done** (§10.1–§10.5) |
| 3 — `CONFORMANCE.md` | blocked on row 99 wave 6, and smaller than written — §10.9 |
| 4 — `STATUS.md` | **done** (§10.7, extended by §10.9) |
| 5 — `.vlt.sig` in `dependency_sources` | **done** — 110 sidecars committed, not two |
| 6 — doc 86 §10's correction | **done** (§10.9) — it did not need leaf 1 |
| 7 — headers on docs 01–85 | **done** (§10.6) |

> **This table said leaves 5 and 6 were blocked, and both have since been done.**
> Leaf 5's entry rested on *"there are **two** `.vlt.sig` files in the repository,
> both fixtures"*; row 100 leaf 1's other half landed and there are now **110**,
> one per source file, so the glob matches something and the assertion is not
> vacuous. Leaf 6's entry said it waited on leaf 1, and §10.9 is why it did
> not.

§1.1 is why the blocked four cannot be hurried: *a gated reference document cannot
be written ahead of its code.* A `GRAMMAR.md` regraded against waves 2 and 3 with
waves 4–9 outstanding would carry rows whose evidence resolves today and stops
resolving next week, which is worse than a document that says it has not been
regraded yet.

Leaf 5 is blocked differently and more simply: `dependency_sources` already covers
`src/**/*`, so a sidecar under `src/` would be picked up — but a `.vlt.sig` lives
beside its `.vlt` under `vlt/`, and there are **two** `.vlt.sig` files in the
repository, both fixtures in `tests/sidecar/`. Adding the glob now would add a
pattern matching nothing and an assertion passing vacuously, which is the one thing
this project's gates are written to avoid.

*Still to fill when the regrade lands: its disagreements with doc 86 §10, and the
final `UNREACHABLE` count. Declaration reflection **did** join the list — doc 87
§22.4's row, in `DECLINED.md`'s "fabricate names or API" section, with the `macro`
construct named as its venue.*

### 10.9 Superseded — what remained before leaves 4 and 7 landed

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

### 10.9 Leaf 6 without leaf 1, and what leaves 1 and 3 actually wait on

§10.8 had leaf 6 blocked on leaf 1, on the reasonable ground that the regrade is
what would contradict doc 86 §10's prediction. **It is not the only thing that
can.** The prediction does not merely assign grades — it names a *mechanism* for
each closure, and a mechanism is testable on its own with two fixtures.

So leaf 6 is done, and doc 86 §10.1 carries it: **zero of eight** close as a
consequence of the solver, where the table claimed two.

- **`as` casts.** The predicted mechanism was *"`x : u8` is an ascription and the
  solver knows whether it is a coercion or a cast"*. Measured: `n : u8` on an
  `i32` is `E0308`, with Lattice's own help saying *"pinned by an ascription"*. An
  ascription pins a type; it does not insert a conversion — and that is right,
  because a truncating `as` conjured from an ascription is a silent lossy
  conversion, the sibling of the implicit clone doc 01 §25.1 declines.
  `tests/unreachable/as_cast.vlt` now carries both spellings, so the correction is
  **gated** rather than only written down.
- **`ref` / `ref mut`.** The capability is reachable — `case p of (a, _) -> *a += 1`
  on a `&mut (u32, u32)` compiles and emits `match p { (a, _) => … }` — but by
  **match ergonomics**, which `examples/destructuring.vlt:54` says generation 1
  already had. The solver adds nothing. What is unreachable is the `ref` spelling,
  for a lexical reason: `ref` is not a reserved word, so `ref a` lexes as two
  identifiers.

Which makes `ref` the same shape as the or-pattern row this table already
corrected once — **cosmetic, not expressive** — and `as` the opposite shape, a
predicted closure that does not happen at all. The pattern worth keeping is that
the *fixtures* were right and the *prediction* was wrong: both
`tests/unreachable/` comments already said as much, and both are gated where the
table was not.

#### Leaf 1, blocked per construct — and doc 97 §10.11 already said so

§10.8's *"blocked on row 99 waves 4–9"* is true but too coarse to act on. The
per-construct answer is **doc 97 §10.11's table**, which this section rediscovered
by running the fixtures rather than by reading it — and got one of three rows
wrong in the process. §10.11 is the authority; it is reproduced here with
today's diagnostics beside it:

| Row | Diagnostic today | Blocked on |
|---|---|---|
| `\|` or-patterns | `E0308` — `1 \| 2` parses as a **guard**, meaning `1` guarded by `2` | wave **2**. `examples/case.vlt:10` still writes `n \| n > 65`, so `\|` is not free |
| `as` casts | `VLT0101` on `n as u8` | wave **6**, which deletes `import … as` and frees the word — and that closes the *spelling* only, per the measurement above |
| `ref` / `ref mut` | `VLT0101` then `VLT0103` — `ref a` is two identifiers | wave **4**. Reserving `ref` is *"what the keyword cull decides"*, and the fixture argues against reserving it at all |

**The row this section got wrong was `ref`**, which a first draft called *"no
wave's work — a decision nobody has scheduled"*. It is wave 4's, and wave 4 is
scheduled. The mistake is the one `AGENTS.md` names twice: *"I could not find
it" is not "it does not exist"*, and a measurement taken instead of a search
answers a narrower question than the one asked. Running three fixtures told me
the diagnostics; it could not tell me who owns the decision, and doc 97 §10.11
could.

So all three rows wait on a scheduled wave. §1.1's argument for this row being
last holds unchanged — a regrade now would produce rows needing a regrade
again — but it now holds for a nameable wave per row rather than for the surface
in general.

#### Leaf 3 is blocked on wave 6, and is smaller than §4 wrote it

§4's leaf 3 says generation 1 went from 952 of 964 blocking occurrences to 12,
*"and the twelve were `as` casts — which generation 2 makes reachable, so the count
changes and the shape of the document does too"*.

**The count does not change.** `as` casts do not close, per the measurement above,
so the twelve stay twelve until wave 6 frees the spelling. The recount against the
migrated corpus is still worth doing and is a smaller job than the leaf was
written to expect.

#### Leaf 4's extension, which does not reverse §10.7's declination

`status.rs` gained a `## Generations` table reading `seed/GENERATION` and `seed/`,
falsified both ways — removing `seed/voltaic-g2.wasm` turns `status_is_current`
red, and flipping one column back to `g1` does too.

**§10.7 declined `seed/GENERATION` and that declination stands.** It was about
sourcing the *crate list* from it, because `GENERATION` lists the crates whose
`src/` is generated and an unported crate would vanish from the Source table. This
table asks a different question — which generation each crate is *written in* — and
`GENERATION` is the only copy of that answer. A retired generation keeps its row,
built from `voltaic_lexer::GENERATIONS` rather than from the lines that have a
count, so `g1`'s retirement is recorded where a reader looks for the current state
rather than disappearing from it.

### 10.10 Leaf 1: one row regraded, and the example it never had

Leaf 1 asks for a regrade *from scratch*. What row 103's migration actually
changed is narrower than that, and the distinction is the useful part:

**`GRAMMAR.md` grades the accepted surface — what a user *can* write — and row
103 changed what the compiler *is* written in.** Those are different documents'
subjects. `|>` is still accepted; receiver-last `.len xs` is still accepted;
`name : Type` in a declaration is still accepted. So most of the inventory's
~128 rows are unaffected, and the from-scratch re-derivation is what waves 4–9
will necessitate, when the accepted surface actually contracts.

One row did change, because row 99's parser work changed the **grammar** and not
only the idiom: *Method calls, field access*. It is regraded, and the grade did
not move — `SUGARED` was and is right. What it gained is three forms stated
separately (`recv.name` a field read, `recv.name args` a receiver-first call,
`recv.name ()` a zero-argument call), the chaining rule and its two type-free
parser facts, and the bracket an argument-carrying link needs.

#### The row cited the wrong examples, and a witness already existed

It cited `examples/comprehension.vlt` and `examples/application.vlt`. **Neither
contains a receiver-first dot** — the spelling row 103 wrote the entire compiler
in, 8,479 call sites.

**A first attempt at this section said the construct was witnessed by nothing and
added an example for it. That was wrong**, and the way it was wrong is worth more
than the row: `examples/receiver_first.vlt` has existed since row 99 wave 3's
argument half and witnesses the form extensively. The row's *citation* was stale;
the corpus was not short a witness. So the fix is one path in one cell, not a new
file.

What the existing example *did* need is a correction of its own. It said:

> Every step here carries an argument, deliberately. A no-argument step is still
> a field access, so `(s.trim).len` is `E0615` today rather than a chain.

True when written and false now — row 99's zero-argument work made
`recv.name ()` a call and `s.len ().count_ones ()` a chain. The example gains
both forms, run rather than only compiled, because every case in it is about
**grouping**: a chain that binds the wrong way still compiles and prints a
different answer.

#### I overwrote a tracked example, and the near-miss is the lesson

The new file was written to `examples/receiver.vlt`, which **already existed** —
plan 67's marked-receiver example, the sole witness for four graded rows
(*Generic traits*, *Two impls of one trait for one type*, *The receiver, marked
in a type*, *Associated functions*). It was destroyed and restored from `HEAD`
within the same session, and nothing was committed in between.

Three signals said so and all three were read past: the write tool reported the
file *"has been updated"* rather than created; `git status` showed ` M` rather
than `??`; and `construct_coverage` went red immediately, naming all four rows.

**The gate is what caught it**, and it caught it because it checks a property —
*every graded construct appears in some corpus file* — rather than a list of
files. A test that enumerated expected witnesses would have been updated along
with the overwrite and said nothing.

The diagnosis also took three wrong turns worth recording, because each was a
plausible theory that a cheaper check would have skipped: order-dependence
(disproved by renaming the file to sort last), an aborting scanner (disproved by
reading `Source::new`, which skips unlexable files via `filter_map`), and a
corpus-size dependency (disproved by reading `Witness::unmet`, where every arm is
`corpus.iter().any`). `any` is monotone in the corpus, so **adding** a file
cannot remove a witness — which should have pointed at *removal* on the first
step rather than the fourth.

#### What is left of leaf 1

The three rows in §10.9's table, each waiting on its wave, plus the from-scratch
re-derivation once waves 4–9 have moved the accepted surface. §1.1's ordering
argument holds and now has a measured shape: there is no point re-deriving 125
rows that generation 2 has not yet touched.

One thing this pass did **not** do and should be done with the re-derivation:
check every row's citation the way this one's was checked. The gate resolves that
a cited path *exists* and `construct_coverage` checks that a construct appears
*somewhere*, but neither notices a row citing a file that witnesses the construct
less well than another file does. That is how this row kept two stale citations
through a whole generation, and it is a per-row judgement rather than something
to gate.

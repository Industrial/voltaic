# Plan 92 — The Charter Reversal

Plan ID: `charter-reversal`
Depends on: nothing. This is generation 2's first row.
Milestone: 13 — *the reversal*
Status: `planned` — awaiting approval; §2 is measured and §4 writes no code

---

## 1. What this plan is

Doc 02 §12.4 permanently declines a Voltaic type checker and calls it *"the project's
load-bearing constraint"*. Doc 03 §2 clause 1 makes type-freedom an admission test:
*"Its lowering is determined by the source text alone."* This plan reverses both, adds
a new clause 5 in their place, and records the reversal in every document that cited
them.

**It writes no code and changes no behaviour.** Its entire deliverable is that the
repository stops contradicting itself.

### 1.1 The measurement that forces it

Measured 2026-11-13, over all thirteen crates, every source file against the Rust it
generates, comments stripped from both sides:

| | Voltaic 1 source | Generated Rust | Ratio |
|---|---:|---:|---:|
| lines | 11,668 | 18,013 | 1.544× |
| tokens | 129,939 | 153,167 | 1.179× |
| characters | 434,662 | 435,454 | **1.002×** |

The character ratio is the finding. Voltaic 1 saves **two parts in a thousand** of the
characters of its own output. Over the 43-program example corpus the same sweep gives
1.290× / 1.197× / 1.045× — the same shape at a different scale.

What the gap between 1.544 and 1.002 says is mechanical: the language deletes braces,
semicolons, `fn`, `let` and closing-brace lines, and deletes them well. It deletes no
types, and the types are 82% of what is left. The constraint is not a tax on the
terseness claim; **it is the ceiling of the terseness claim**, and the ceiling has
been reached.

Two symptoms from the same sweep:

- **1,132 standalone type-signature lines, 8.7% of 13,043 source lines**, across 122
  files. Every definition writes its own name at least twice.
- **74 keywords and 128 graded grammar rows** buy the 1.179×.

### 1.2 The register predicted this, and left the row open

`docs/04 - Voltaic Plan of Plans.md`, *What this milestone does not claim*:

> A score that high requires inference, higher-kinded abstraction or effect tracking,
> each of which needs the checker doc 02 §12.4 forbids — so the honest ceiling under
> the constraint is **8–9**, and the constraint is worth more than the tenth point.
> Doc 02 §12.4 stays.

Every clause of that is accepted here except the last two. The trade it describes —
the constraint against *the tenth point* — is a trade worth taking. §1.1 measures the
actual stake: not a tenth point but **the whole of the character-terseness claim**,
which is the claim the project's README leads with.

The same section, two paragraphs down, names this plan's scope before this plan
existed:

> One re-cut is worth considering and is **not** taken by any row here, so it is
> recorded as an open question rather than assumed: the constraint is stated broadly
> enough to read as forbidding all static analysis, where what it means to forbid is a
> *type* checker and a *borrow* checker. […] Whether the corollary should say so in
> general is a question for a row that owns the diagnostics surface, not for any of
> these.

This is that row, and the re-cut it describes is **exactly** the reversal: what ships
is a solver that **infers and prints**, not a checker that re-derives what rustc
already decides.

### 1.3 What is reversed, and what is not

| | Status |
|---|---|
| Doc 03 §2 clause 1 — type-freedom | **reversed.** Replaced by doc 86 §2 clause 1: solver-determined, *and printable*. |
| Doc 02 §12.4 — "a Voltaic type checker, permanently declined" | **reversed.** What ships is an inference engine; see §1.4 on why the distinction is load-bearing and not a word game. |
| Doc 03 §2 clause 2 — no implied dependency | kept, verbatim |
| Doc 03 §2 clause 3 — semantics-preserving, never a clone | kept, verbatim |
| Doc 03 §2 clause 4 — reachability-positive | kept, verbatim |
| Doc 01 §25.1 — implicit cloning, non-negotiably declined | **kept, and strengthened.** Doc 86 §6.5: if no mode assignment satisfies every use, the answer is a diagnostic, never a clone. |
| Doc 02 §1.1 — no standard library | kept, with one bounded exception: doc 86 §7.4's three `io` primitives. |
| "Every check rustc can make, rustc makes" | **kept as a corollary**, reworded in doc 86 §2 to say what it always meant: Lattice does not re-implement borrow checking, exhaustiveness checking or monomorphisation. |

### 1.4 Inference is not checking, and the difference is testable

This distinction has to be more than rhetoric, because the whole reversal rests on it.
It is testable, and the test is in the gate:

- A **checker** decides whether a program is well-typed and reports an error when it
  is not. Voltaic 2 does not do this. A program that Lattice accepts and rustc rejects
  is a **Lattice bug**, filed as one, not a user error.
- An **inference engine** decides *what to write down* so that rustc can check it.
  That is all Lattice does: it computes a judgement, and the emitter writes the
  judgement into the output.

The consequence is a property of the `.vrs`, and it is assertable: **the emitted Rust
states every type, borrow and lifetime explicitly** (doc 86 §6.6). rustc is therefore
type-checking a program that declares its own types, exactly as it checks hand-written
Rust, rather than confirming Lattice's reasoning. Doc 88 §1.1 records the
counterintuitive consequence: the emitter's job gets *larger* under the reversal, not
smaller.

### 1.5 What this plan does not get to decide

- Whether the surface should change. That is rows 98 and 99, and they depend on this
  row rather than the reverse.
- Whether refinements are admitted. That is row 97 and doc 87 §22.1 bounds it.
- Whether `--require-signatures` becomes the default. Doc 88 §9 declines to settle it.

---

## 2. Preconditions, measured

### 2.1 Every place the constraint is cited

Found by searching the tree, 2026-11-13. Each one must be amended or must carry a
pointer, or the repository asserts both the old rule and the new one:

| Location | What it says | Action |
|---|---|---|
| `AGENTS.md`, *Conventions that are load-bearing* | *"Voltaic has no type checker, no borrow checker and no exhaustiveness checker, and must never grow one."* | **amend** — §4 leaf 3 |
| `docs/02 - Voltaic Update Blueprint.md` §12.4 | the decline table row | **not edited.** Pointer added by this document's existence; doc 86 and 88 carry the supersession note |
| `docs/03 - Voltaic Surface Design.md` §2 | clause 1 and the first corollary | **not edited.** Superseded by doc 87 |
| `docs/DECLINED.md` | *"A Voltaic type checker — the project's load-bearing constraint"*, and the `UFCS` row | **rewritten by row 104**, not here |
| `docs/04 - Voltaic Plan of Plans.md` | the two paragraphs in §1.2 above | **amend in place** with a dated pointer, per that document's own rule 2 |
| `README.md` | *"because Voltaic has no checker of its own and must never grow one"*, twice | **amend** — §4 leaf 4 |
| `crates/voltaic-mcp` server instructions | *"rustc is the authority on types and borrows — Voltaic has no checker of its own."* | **amend** — the first half stays true, the second does not |

Seven locations. The repository's convention is that documents 01–85 are a historical
record and are **not** rewritten — doc 04 rule 3, and the many *"this bullet used to
say the opposite, and the correction is worth keeping"* notes in `AGENTS.md`. So the
numbered docs get supersession by citation; the living documents get amended.

### 2.2 What is *not* affected

Checked, so the reversal is not read as broader than it is:

- The no-standard-library rule, except doc 86 §7.4's three names.
- The no-implied-dependency rule. Lattice's solver dependency is a **build-time**
  dependency of the compiler, not a dependency of a compiled program; a `.vrs` names
  nothing outside `core`, `alloc` and `std`.
- The hygiene rule. Only the prefix changes, and that is row 93.
- Comments as trivia, preserved into the output.
- The off-side rule.

### 2.3 The one thing the reversal makes worse, stated before it is approved

A language whose signatures are inferred has nowhere to anchor a type error. This is
the known failure mode of Haskell and Scala and it is not hypothetical. Doc 88 §6.1
holds the four mitigations and row 101 is the plan.

**This plan should not be approved without row 101 being approved with it.** The
reversal's whole justification is that the solver can explain itself; a reversal that
ships inference without `--why`, without constraint provenance and without UI fixtures
for the four new diagnostic ranges would be the thing doc 86 §2 clause 1 was written
to forbid.

---

## 3. Concrete proof this plan targets

Not a build — this plan has no build. Three textual properties, each assertable:

1. **No document in the tree asserts both rules.** A search for the old constraint's
   phrasings returns only `docs/01`–`docs/85` (the historical record) and this
   document (the reversal).
2. **Doc 86 §2 has five clauses**, clauses 2–4 word-identical to doc 03 §2's clauses
   2–4, and `crates/voltaic-cli/tests/declined.rs` can resolve every construct in doc
   87 §22 against `docs/DECLINED.md` once row 104 re-points it.
3. **Doc 04's two paragraphs carry a dated pointer** to this document, and the pointer
   says which clause of the prediction held and which did not.

---

## 4. Leaves

### Leaf 1 — the three normative documents

`docs/86 - Voltaic 2.md`, `docs/87 - Voltaic 2 Surface Design.md` and
`docs/88 - Voltaic 2 Blueprint.md`. Authored; this plan's approval is their
acceptance.

### Leaf 2 — the register pointer

Amend `docs/04 - Voltaic Plan of Plans.md`'s *What this milestone does not claim*
section with a dated note: which clause of its prediction held (inference and effect
tracking are indeed what the distance costs), which did not (*"doc 02 §12.4 stays"*),
and the measurement that decided it. **Append; do not rewrite the paragraph.** The
prediction was right about the mechanism and wrong about the price, and both halves
are worth keeping.

### Leaf 3 — `AGENTS.md`

Replace the bullet *"Every check rustc can make, rustc makes"* with:

> - **Every check rustc can make, rustc makes.** Voltaic has no borrow checker and no
>   exhaustiveness checker, and must never grow one. It **does** have an inference
>   engine — `voltaic-lattice`, doc 86 §3 — whose job is to compute what the emitter
>   writes down, not to decide whether a program is well-typed. A program Lattice
>   accepts and rustc rejects is a Lattice bug. The emitted `.vrs` states every type,
>   borrow and lifetime explicitly, which is what keeps rustc an independent check
>   rather than a rubber stamp.
>
>   **This bullet used to forbid a type checker outright, and the reversal is worth
>   keeping.** It was right for 91 plans: the constraint is what made every lowering
>   auditable and kept rustc the authority. What retired it is a measurement — Voltaic
>   1 saves **1.002× of the characters** of the Rust it generates, so the no-checker
>   charter was not a tax on the terseness claim but its ceiling. `docs/90 - 92 - The
>   Charter Reversal.md` is the argument; doc 86 §1.1 is the number.

Also add `voltaic-lattice` to the crate list and the `.vlt`/`.vrs` extensions, both of
which row 93 owns — this leaf only adds the charter bullet and leaves the rest to 93
so that a conflict between the two rows is impossible.

### Leaf 4 — `README.md` and the MCP server instructions

Two phrasings of *"no checker of its own and must never grow one"* in `README.md`, and
one in the MCP server's instruction string. Replace with the accurate version: rustc
is the authority on types and borrows; Voltaic infers and writes them down.

### Leaf 5 — this document's §10

Filled after approval, with what the reversal actually cost.

---

## 5. Acceptance criteria

1. §3's three textual properties hold.
2. `AGENTS.md`'s amended bullet carries the measurement and the supersession note, in
   the style of the other recorded reversals in that file.
3. Doc 04's pointer is appended, dated, and names both halves of its prediction.
4. No numbered document 01–85 is edited.
5. Row 101 is approved alongside this row, per §2.3.
6. Doc 86 §2 clauses 2, 3 and 4 are word-identical to doc 03 §2 clauses 2, 3 and 4. A
   reversal that quietly loosened the other three while reversing one would be the
   failure mode this criterion exists to catch.

---

## 6. Out of scope, and what that discloses

- **No code.** The solver is rows 94–97.
- **No surface change.** Rows 98–99.
- **`DECLINED.md`, `GRAMMAR.md`, `CONFORMANCE.md`, `STATUS.md` are untouched.** They
  are gated, and row 104 rewrites them when there is code to grade. This plan
  therefore leaves `DECLINED.md` asserting a decline this plan reverses, for the
  duration of the generation. **That is a knowing inconsistency and it is disclosed
  here rather than discovered later**: the alternative is a `DECLINED.md` whose rows
  cite doc 87 §22 before doc 87 §22 has any code behind it, and an evidence-carrying
  document that cannot resolve its evidence is worse than one that is out of date.

---

## 7. Gate

Documentation-only, so the gate is the standing one plus one new check:

- the full push gate, unchanged — nothing in this plan touches code, so nothing should
  move
- a new assertion in `crates/voltaic-cli/tests/declined.rs`: **the tree does not
  contain both constraints outside the historical record.** It is the only machine
  check this plan can carry, and it is the one that matters, because the failure mode
  of a documentation reversal is a half-applied one.

---

## 8. Emerald reuse

None. Emerald never reversed a charter constraint; it accumulated 203 plans and 112 of
them building a standard library, which is the cost `AGENTS.md`'s *no plan implements a
library function* rule exists to avoid. The nearest analogue in this project is plan
86's lifting of the general-monadic-comprehension decline — one row, with the argument
written down, and the reopening venue honoured rather than ignored. This plan copies
that shape at a larger scale.

---

## 9. Decision log

| Decision | Why |
|---|---|
| Reverse clause 1 rather than add a sixth clause beside it | two admission tests that contradict each other admit everything |
| Add clause 5 (*decidable, or diagnosed*) in the same plan | the reversal creates the hang-risk that clause 5 forbids; adding it later would mean a window in which an unbounded solver was admissible |
| Keep clauses 2–4 verbatim | a reversal that reworded the untouched clauses would be impossible to audit |
| Do not edit docs 01–85 | doc 04 rule 3, and the repository's standing habit of keeping corrections visible |
| Leave `DECLINED.md` stale for the generation | §6; a gated document cannot be written ahead of its evidence |
| Require row 101 to be approved with this row | §2.3; the reversal's justification is that the solver can explain itself |
| Name the solver *Lattice* and keep the language *Voltaic* | the language's identity does not change; the thing inside it is new, and naming it separately is what lets doc 86 §1 say so in one sentence |

---

## 10. Result

Shipped 2026-11-14. Gate green: `moon run :lint` and `moon run :test`, 13 tasks each.

### 10.1 §2.1 undercounted the citations, and by most of them

The table in §2.1 listed **seven** locations. A full sweep for the two phrasings —
`"no type checker"` and `"has no checker of its own"` — found **twelve living ones**,
and the five this plan missed are not obscure:

| Missed location | Why it matters |
|---|---|
| `crates/voltaic-cli/README.md` | **the published crate's front page on crates.io** |
| `www/src/introduction.md` | the book, which `README.md` points at first |
| `www/machine.sh` | the book's machine-readable summary, written for agents |
| `SECURITY.md` | the document defining what counts as a vulnerability here |
| `nix/nixpkgs-package.nix` | the nixpkgs description |
| `editors/vim/README.md`, `editors/vim/syntax/sdx.vim`, `editors/vscode/syntaxes/voltaic.tmLanguage.json` | three highlighters arguing *from* the constraint |
| `crates/voltaic-mcp/tests/tools.rs` | a doc comment citing the `AGENTS.md` bullet this plan rewrote |

`AGENTS.md` already carries the rule this broke: *"I could not find it" is not "it does
not exist", and this repository's own layout rules put things where a careless search
will not look.* The planning sweep searched a handful of phrasings by hand; the gate in
§7 walks the tree. **The gate found five of the twelve that this plan's own
reconnaissance did not** — which is the argument for writing the gate before believing
the table, rather than after.

The three highlighter sites turned out to need a different kind of edit. They argue
*"Voltaic has no type checker, so a syntax file certainly cannot tell a type from a
constructor"*, and the conclusion survives the reversal while the premise does not: a
TextMate grammar or a `syn match` carries no type information whatever Lattice knows.
They were reworded to claim what they actually depend on — the grammar's ignorance, not
the language's.

### 10.2 The gate caught generated output first, which it should not police

Its first run failed on `crates/voltaic-mcp/src/server.rs` and
`crates/voltaic-mcp/sdx/server.xrs` — the generated tree and the transient build
artifact, both still carrying the old string because `server.sdx` had just been amended
and not yet regenerated.

Both were *correct* findings about a stale tree and *wrong* findings about the charter.
Flagging generated output duplicates `port.rs`, the gate that compares a `.sdx` against
the Rust it produces, so `is_generated` now skips `crates/*/src/` and any `.xrs`. The
artifacts were then regenerated by the port harness, which rewrote them and failed by
design so the diff was reviewed.

### 10.3 §6 was wrong about `STATUS.md`, and could not have been right

§6 says the four gated reference documents are untouched by this plan. Three of them
are. **`STATUS.md` is not, and cannot be:** it is *generated*, its `Source` table counts
lines per crate, and this plan adds 143 lines to `crates/voltaic-cli/tests/declined.rs`.
`status_is_current` failed exactly as designed — *"a commit that changes them must
update it"* — and `voltaic-cli`'s test count moved 10,974 → 11,117, the total 21,700 →
21,843.

So the sentence to correct is §6's, not the design's. A **hand-written** gated document
cannot be written ahead of its code, which is row 104's reason for existing. A
**generated** one is the opposite: it must be regenerated by *every* commit that moves a
number it measures. Conflating the two produced the wrong claim.

### 10.4 Criteria

| | Criterion | Result |
|---|---|---|
| 1 | §3's three textual properties | met; §7's gate asserts the first mechanically |
| 2 | `AGENTS.md` carries the measurement and the supersession note | met, in the style of that file's other recorded reversals |
| 3 | Doc 04's pointer appended, dated, naming both halves of its prediction | met — the mechanism held, the price did not, and the open question it left is the one taken |
| 4 | No numbered document 01–85 edited | met |
| 5 | Row 101 approved alongside | met; its header records the joint approval |
| 6 | Doc 86 §2 clauses 2–4 word-identical to doc 03 §2's | met |

### 10.5 What this plan got wrong

Two things, both above: **§2.1's count** (seven against twelve, §10.1) and **§6's claim
about `STATUS.md`** (§10.3). Neither changed the design. Both were caught by a gate
rather than by review, which is the only reason to prefer a gate to a table.

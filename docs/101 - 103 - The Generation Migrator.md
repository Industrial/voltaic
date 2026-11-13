# Plan 103 — The Generation Migrator

Plan ID: `migrate`
Depends on: 99, 102
Milestone: 17 — *the turn*
Status: `planned` — awaiting approval; §1.3 is the gate and it is the whole plan

---

## 1. What this plan is

`voltaic migrate`: mechanical rewriting of generation-1 source into generation 2,
classified into **auto**, **review** and **manual**, gated on producing **byte-identical
generated Rust**.

Then it migrates this repository's own 11,668 code lines across 122 files and thirteen
crates — and after that, in a second pass, it simplifies what the first pass only
translated.

### 1.1 Two passes, and the order is the finding

The naïve plan is one pass: rewrite generation-1 source into idiomatic generation-2
source. That plan has no gate, because the output is a different program by every
textual measure and the only check available is "does it still pass the tests", which
for 122 files is a check that tells you *something broke* and nothing about where.

So: two passes.

**Pass 1 — migrate for identity.** Translate the surface, and **preserve every
explicit mode and type as an ascription**. `&self` becomes `self @shared`, `&mut self`
becomes `self @unique`, a signature line becomes an inline ascription. The output is
ugly generation-2 source that says exactly what the generation-1 source said.

**Pass 2 — simplify.** Delete the ascriptions the solver would infer identically, one
crate at a time, gated by the same identity check.

The gate for both passes is the same and it is strong:

> **The `.vrs` generated from the migrated source is byte-identical to the `.xrs`
> generation 1 generated from the original.**

### 1.2 Why that gate is strong

Byte-identical generated Rust means the migration changed the *spelling* of the
program and nothing else. Not "the tests still pass" — the tests passing is compatible
with a changed borrow, a changed clone, a changed derive list, a different `match`
shape. Byte-identity is compatible with none of those.

It is available because generation 1 kept its generated output as a reviewable
artifact rather than an intermediate, which doc 02 §10.1 required and which this row is
the largest beneficiary of.

And it is what makes pass 2 safe: deleting an ascription is correct if and only if the
solver infers the same thing, and byte-identity is exactly that test.

### 1.3 The three classes

The migrator **refuses to write a file it cannot classify**, and the classes are
determined by construct, not by confidence:

| Class | Meaning | Constructs |
|---|---|---|
| **auto** | mechanical, with the identity gate as proof | signature lines, `otherwise`, `\|>`/`>>>`/`<<<`/`$`, receiver order, field `:`, `deriving`, `newtype`, `errors`, `import`/`export`/`module`, `try`/`catch`/`?`/`??`, `async`/`await`, `&`/`&mut`/lifetimes, `instance` headers |
| **review** | mechanical but the result wants a human | `do .and_then` over a `std` shape, `_` holes whose position changes with receiver order, sections whose lowering was the method form, `where` groups that relied on parse order |
| **manual** | no mechanical answer exists | `do` over a **user** type; nested `Option<Option<T>>`; `deriving template` using `Fields`/`FieldNames`; anything in a `rust` block that names a generated identifier |

Three constructs in the **manual** class, and each is a loss doc 86 or doc 87 already
records rather than something discovered here:

- **`do` over a user type** — doc 86 §5.6; the effect alphabet is closed.
- **nested optionals** — doc 86 §5.6; an effect cannot nest, and `~` is the recovery.
- **`Fields` / `FieldNames`** — doc 87 §22.4 and row 98 §6; declination, with the
  `macro` construct named as the venue.

A `manual` file is reported with the construct and the line, and the migration of that
file stops. **The count of outstanding `manual` files is a release-blocking number**,
and doc 89's milestone 17 test is that it reaches zero.

### 1.4 What the migrator is not

It is not a formatter and it is not a linter. It translates. Pass 2's simplification is
deletion of now-redundant ascriptions, not restructuring — it will not merge files,
rewrite a loop as a comprehension, or make anything point-free.

That restraint is deliberate and it is plan 91's lesson. Plan 91 rewrote the compiler's
architecture, deleted its comments and made it point-free in one row of thirteen waves,
and its own §10.6 and §10.8 record what the merge tooling cost and what the plan got
wrong. This row has a harder correctness obligation and a weaker claim: it moves a
program between two surfaces and proves it did not change it.

---

## 2. Preconditions, measured

### 2.1 The corpus

| | Files | Code lines |
|---|---:|---:|
| `crates/*/vlt/**` | 122 | 11,668 |
| `examples/*.vlt` | 44 | — |
| `tests/`, `benchmarks/` fixtures | ~120 | — |

The examples and fixtures were migrated by row 99 **by hand**, wave by wave. This row
migrates the compiler, and the examples are its test corpus: a migrator that cannot
reproduce row 99's hand-migration of `examples/*.vlt` is not trusted with
`crates/`.

That is this row's first acceptance criterion and it is a free oracle — row 99 produced
the answers already.

### 2.2 Row 102's `GENERATION` file is what makes this incremental

Thirteen crates, dependency-ordered. `bootstrap.sh` reads `GENERATION` and runs the
blob each crate is marked for (row 102 leaf 7), so a crate can move from `g1` to `g2`
**on its own**, with the tree building at every intermediate state.

Without that, this row would be one commit touching 122 files, which is a commit no
one can review.

### 2.3 Two traps from prior plans that apply directly

Both from `AGENTS.md`, both recorded after costing real time:

- **A guard that exists to stop a destructive step must be checked before that step
  runs.** Plan 91 wave 11: a driver script piped a merge tool into `grep`, took grep's
  exit status, and deleted the crate's subtrees after the tool had refused. `set -o
  pipefail`, or capture and test the status. This row's driver script deletes nothing,
  but it *writes* over source, which is the same hazard.
- **`git checkout -- <path>` restores from the index, not from HEAD.** After a `git add
  -A` that staged a half-finished state it restores the half-finished state. The
  rollback for a failed crate migration is `git checkout HEAD -- <path>` plus a
  separate removal of untracked leftovers.

---

## 3. Concrete proof this plan targets

Three, in order:

1. **`voltaic migrate` reproduces row 99's hand-migration of all 44 examples**, modulo
   formatting, with the identity gate green on every one.
2. **`crates/voltaic-diag` migrated end to end** — 524 code lines, the smallest crate
   with real content and the one every other crate depends on. Its `.vrs` byte-identical
   to its generation-1 `.rs`.
3. **`crates/voltaic-parser` migrated** — 3,184 code lines across 13 files, the
   largest crate, and the one holding the 1,478-line `parser.rs` that
   `crates/voltaic-cli/tests/line_cap.rs` already constrains to under 1,500. If the
   migration *grows* that file past the cap, the cap is what catches it, and the
   resolution is a split rather than a cap change.

---

## 4. Leaves

### Leaf 1 — the classifier

Walk a generation-1 file and classify every construct into auto/review/manual. Report,
and write nothing. Run over all 122 files **before any migration happens**, so the
`manual` count is known at the start rather than discovered at file 97.

### Leaf 2 — pass 1, the auto class

Mechanical translation with ascription preservation (§1.1). The identity gate per file.

### Leaf 3 — the identity gate

Generate from the migrated source with the g2 compiler, diff against the generation-1
`.xrs` for the same file. Byte-identical or the file is rejected and left untouched.

**The gate runs before the write, not after.** §2.3's first trap: a check that exists
to prevent a destructive step must be checked before the step runs. The migrator
writes to a temporary path, gates, then moves.

### Leaf 4 — the review class

Translate, mark, and emit a report naming every file and construct needing a human
read. A `review` file is written — it is mechanically correct — but the row is not done
until each has been read.

### Leaf 5 — the manual class

Report and stop. Per-construct guidance in `docs/MIGRATION.md`, which is this leaf's
documentation deliverable.

### Leaf 6 — crate-by-crate migration

Thirteen crates in `GENERATION`'s dependency order, each one its own commit, each one
flipping its `GENERATION` line from `g1` to `g2`, with the full gate green at every
step.

### Leaf 7 — pass 2, simplification

Delete redundant ascriptions, crate by crate, identity-gated. Reports how much of each
crate's annotation was load-bearing — which is the measurement doc 87 §18's
`unused inference` lint exists to produce, and the number this generation will most
want to quote.

### Leaf 8 — the user-facing migrator

`voltaic migrate` as a shipped subcommand, because users have `.sdx` files too (row 93
§3.4). Same three classes, same gate, same report.

---

## 5. Acceptance criteria

1. All 44 examples reproduced from row 99's hand-migration, identity gate green.
2. All thirteen crates migrated, each in its own commit, full gate green at every one.
3. **Zero outstanding `manual` files** at the end of the row. Each one resolved by
   hand, and each resolution named in §10 with the construct that forced it.
4. Every `review` file read by a human, recorded as read.
5. Pass 2 complete, with the load-bearing-annotation percentage per crate reported.
6. `fixpoint.rs` green on g2 after the migration (row 102 §3 step 6 depends on this
   row finishing).
7. `line_cap.rs` green — no generated file over 1,500 lines (§3.3).
8. `no_comments.rs` green — the compiler's own source still carries zero comments.
9. The test count per crate is unchanged. **`AGENTS.md` records why this is its own
   criterion:** plan 74 dropped `scan_number.rs`'s seven inline tests by reading a
   file's body and stopping above its `mod tests`, and **every gate passed** — the
   crate compiled, every test that existed ran, clippy was clean. A check that compares
   generated output to its source *cannot see something absent from both*. So the test
   count is compared before and after, per crate, by hand.

Criterion 9 is the one this row would otherwise repeat. The identity gate in §1.2 is
strong, and it is strong about the thing it compares; a test deleted from the source is
absent from both sides of that comparison.

---

## 6. Out of scope

- **Restructuring.** §1.4. No file merges, no point-free rewriting, no architecture
  change. Plan 91 did that, in generation 1's surface, and its results section is the
  argument for not doing two things at once.
- **The seed.** Row 102, which this row sits between steps 4 and 6 of.
- **The reference docs.** Row 104.
- **Deleting generation 1's examples' `.stdout` files.** They are the oracle; they
  survive the migration unchanged, which is what makes row 99 §3's byte-identical-stdout
  claim checkable after this row too.

---

## 7. Gate

Per file: the identity gate. Per crate: the full push gate plus the test count. Per
row: `fixpoint`, `bootstrap`, `line_cap`, `no_comments`, `construct_coverage`, and the
zero `manual` count.

---

## 8. Emerald reuse

Emerald's syntax cutover is the direct precedent and its failure is the one this row is
structurally protected from: **113 fixtures lived as string literals inside test
modules and were missed.** `AGENTS.md`'s *every `.sdx` fixture is a file* rule and
`no_literal_expectations.rs` exist because of it, and this row is where the saving is
collected — a migrator that walks files reaches every fixture there is.

---

## 9. Decision log

| Decision | Why |
|---|---|
| Two passes, identity first | pass 1 has a gate; a one-pass idiomatic migration does not |
| Byte-identical generated Rust as the gate | §1.2; "tests still pass" is compatible with a changed borrow |
| Gate before write, to a temp path | §2.3's first trap, from plan 91 wave 11 |
| Classify all 122 files before migrating any | the `manual` count should be known at the start |
| Refuse rather than guess | a half-migrated file that compiles is worse than one that was skipped |
| Crate at a time, each its own commit | row 102's mixed-generation `GENERATION` file is what makes this possible |
| Reproduce row 99's hand-migration first | a free oracle, and a migrator that fails it is not trusted with the compiler |
| Compare test counts by hand, per crate | plan 74's loss was invisible to every gate; §5.9 |
| No restructuring | plan 91's results section |
| Ship `voltaic migrate` to users | they have `.sdx` files and row 93 promised them one release |

---

## 10. Result

*To be filled after execution. Required: the `manual` file list and how each was
resolved; the load-bearing-annotation percentage per crate from pass 2; the
before-and-after test counts; and whether the identity gate ever had to be relaxed,
with the reason.*

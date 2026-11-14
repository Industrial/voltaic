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
| `crates/*/vlt/**` | **105** | **15,264** |
| `examples/*.vlt` | 45 | — |
| `tests/`, `benchmarks/` fixtures | ~120 | — |

**The first row said 122 files and 11,668 lines, and both were wrong — in
opposite directions.** Measured 2026-11-14 while building leaf 1:

- **17 fewer files.** Plan 91 replaced one-construct-per-file with
  one-cohesive-group-per-file and merged 572 files down; the figure here predates
  that merge.
- **3,596 more lines.** This generation added `voltaic-lattice`, which did not
  exist when the figure was taken.

A sizing table that is wrong in both directions is worse than one that is merely
stale, because the two errors partly cancel in any single-number summary. The
same pair of figures is repeated in `AGENTS.md` and was repeated again in
`crates/voltaic-cli/tests/inferred.rs` this session, which is how a number
survives: each copy is quoted from another copy rather than measured.

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

In progress. **Leaf 1 is done**, and §2.1's corpus table is corrected.

### 10.1 The classifier, and the number leaf 1 exists to produce

Leaf 1's own sentence is the reason it comes first:

> Run over all 122 files before any migration happens, so the `manual` count is
> known at the start rather than discovered at file 97.

`crates/voltaic-driver/vlt/classify.vlt` is the classifier and
`crates/voltaic-cli/tests/classify_corpus.rs` runs it over the whole corpus. The
count is reported by the gate rather than asserted, because the point is that it
is *known*, and a test is where this tree reports numbers.

### 10.2 What the classifier is, stated so the number is not over-read

It matches construct **spellings**, line by line, skipping comments. It is not a
parse. A needle inside a string literal counts.

That is deliberate for this leaf and wrong for row 103's pass 1, which translates
from the AST. The question leaf 1 answers is *"does a human have to look at this
file"*, and a `do` inside a string is something a human has to look at in order
to rule out. So the counts are an **upper bound on the human effort**, and the
classifier erring towards `manual` is the safe direction.

The opposite error — a file classified `auto` that needs a human — is the one
that ships a silently wrong migration, and it is why §1.3 says the classes are
determined "by construct, not by confidence". Two tests pin that the extremes are
both reachable: a classifier that can never say `manual` makes doc 89's milestone
17 vacuously true, and one that can never say `auto` makes the count carry no
information either.

### 10.3 §2.1's table was wrong on both axes

105 files and 15,264 code lines, against the recorded 122 and 11,668. The account
is in §2.1 and the short version is that the file count predates plan 91's merge
while the line count predates `voltaic-lattice`. Both numbers were also in
`AGENTS.md`, and both were quoted into a new test file *this session* before being
measured — which is the mechanism by which a wrong number survives a rewrite.

### 10.5 The measured classes, and three rounds of getting there

| | Files |
|---|---:|
| auto | 88 |
| review | 14 |
| **manual** | **3** |
| | 105 |

The number started at 21 and the two tightenings that moved it are each a
finding rather than a tuning:

**Substring to word boundary: 21 → 12.** `Fields` matched `ConstructorFields`,
`where` matched `elsewhere`, and `do` matched inside identifiers. The best of
them was `classify.vlt` classifying **itself** as `manual`, because its marker
table holds the needles as string literals. A needle made of identifier
characters now has to land on a word boundary; one made of punctuation — `|>`,
`>>>`, `.await` — cannot have one and keeps substring matching, because `|>`
inside an identifier is not a thing.

**Word boundary to end-of-line anchor for `do`: 12 → 3.** Nine of the twelve were
the **English word** *do*, in test names:

```
test "two names that are the same type through an alias do not conflict"
```

and in the lexer's own keyword table, `"do" -> Some Do`. A `do` block introduces
a block, so it is the last token on its line. `do` is the only construct in the
table whose *position* distinguishes it from a word meaning something else, which
is why the third matching mode exists and why there is not a fourth.

The anchor also removed the self-match, so the exclusion added to hide that is
deleted. The exclusion list is kept, **empty**, with the note that an exemption is
a permanent hole with a comment beside it while a rule is a rule.

### 10.6 Every `manual` hit was the compiler talking *about* a construct

This is the finding that matters for leaf 1's method, and it is not a caveat
arrived at by reasoning — it is 21 for 21, then 12 for 12, then 3 for 3. The three
that remain:

| File | Hit | What it actually is |
|---|---|---|
| `voltaic-driver/vlt/classify.vlt:26` | a nested optional | the marker table's own `"Option<Option<"` literal |
| `voltaic-emit/vlt/expr.vlt:168` | a generated identifier | the emitter **producing** the `__vlt_` prefix |
| `voltaic-emit/vlt/pattern.vlt:41` | `FieldNames` | the emitter **implementing** the substitution |

A compiler's source names, tabulates and generates every construct of its
language, so text matching on it finds the vocabulary rather than the usage. The
honest reading of **3** is therefore *"at most 3, and all three are known to be
the classifier reading the compiler's own dictionary"* — which plausibly makes the
true count **zero** and doc 89's milestone 17 already satisfied for `crates/`,
though that cannot be asserted from a text classifier and is not asserted here.

Two consequences:

- the classifier is a triage tool for **user** code, which is what `voltaic
  migrate` will be pointed at, and it is sound there;
- the trustworthy count for the compiler's own source comes from **pass 1**, which
  §1.1 already specifies as translating from the AST. Leaf 1's value was never the
  number alone — it was knowing the number's *shape* before migration starts
  rather than at file 97, and the shape turns out to be "nearly everything is
  mechanical".

### 10.4 What remains

Leaves 2–8 need row 99's surface: there is nothing to translate *to* until the
generation-2 spelling exists. Leaf 3's identity gate is the one that could be
built next without it — the gate is "generate from the migrated source, diff
against the generation-1 output, byte-identical or reject" — and an identity
transform is a valid degenerate case to test it against. It is not built here
because a gate with nothing to gate invites being weakened later to let the first
real migration through, which is the opposite of §2.3's first trap.

*Still to fill when leaves 2–8 land: the `manual` file list and how each was
resolved; the load-bearing-annotation percentage per crate from pass 2; the
before-and-after test counts; and whether the identity gate ever had to be relaxed,
with the reason.*

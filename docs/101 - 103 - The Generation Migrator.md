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

### 10.5 The measured classes, and four rounds of getting there

| | Files | …after wave 2's constructs were added |
|---|---:|---:|
| auto | 88 | **20** |
| review | 14 | **83** |
| **manual** | **3** | **3** |
| | 105 | 106 |

**The right-hand column is the real one, and the left-hand one measured the corpus
with the generation's largest change missing from the table.** The fourth round is
§10.5.1; the first three are below it and stand.

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

#### 10.5.1 Round four: the table named none of wave 2, and 88 `auto` was the dangerous direction

The twenty-two needles covered §1.3's auto list — pipelines, composition,
`otherwise`, `newtype`, `errors`, `deriving`, `instance`, modules, `async`,
`catch`, `&mut` — the `review` pair, and the `manual` four. **None of them was
`if`, `while`, `for`, `case` or `guard`**, which is doc 97 §1.3's *wave 2*:

> `if`/`else if`/`match`/`case`/`cond`/`switch`/`when`/`iff`/`unless`/`guard`/`then`
> collapse into the guard ladder and pattern rows. `otherwise` becomes `_`.
> `for`/`while`/`until` become comprehensions, iterator methods, and `loop`/`break`.

So a file whose only generation-2 work was control flow classified **`auto`** —
"mechanical, with the identity gate as proof". §10.2 names that error and names it
as the serious one:

> The opposite error — a file classified `auto` that needs a human — is the one
> that ships a silently wrong migration.

It was live, in 83 files, while §10.2 was asserting the counts were "an upper
bound on the human effort". They were a **lower** bound by a factor of 5.9 on
`review`.

**The classes assigned, and why three of five are `review`.** `guard` is `auto`:
it is one-to-one with doc 87 §5.6's `else` tail, which row 99 wave 2 has now
built, and the translation moves no code. The other four restructure. Collapsing a
statement `if` into a ladder hoists the rest of the body into the catch-all arm;
`for` and `while` become a comprehension or an iterator method, which is §1.4's own
example of what the migrator does **not** do mechanically; `case … of` moves its
scrutinee into every arm as a pattern test, which is arm-for-arm but changes every
arm. Erring towards the human is §10.2's stated safe direction.

**A fourth matching mode, and §10.5's "there is not a fourth" is retired.** That
sentence was true of a table with no control flow in it: `do` was the only needle
whose *position* told it from a word meaning something else. `if`, `while`, `for`
and `guard` all open a statement, so they take an `AtStart` anchor for exactly
`do`'s reason — `if` appears in English in test names, and `test "it fails if the
input is empty"` is not an `if` statement. `case` keeps `Anywhere`, because
`y = case held of` is a value position and the construct is not statement-initial.

`Marker`'s `tail : bool` became a `Position` enum in the same change. Two booleans
would have admitted `tail && head`, which has no meaning; three variants cannot be
combined wrongly.

**The count agrees with an independent measurement, which is why it is trusted.**
86 of 106 `.vlt` files hold a statement-initial `if`/`while`/`for`/`guard` or a
`case`, counted by a script that shares no code with the classifier. The classifier
says 83 `review` + 3 `manual` = 86 non-`auto`. Exact.

**Two gates, because the omission had none.** `classify_corpus.rs` reported the
counts and asserted only a floor on the corpus *size*, so a marker table missing a
third of the generation changed a printed number and failed nothing.

- `every_wave_two_construct_is_in_the_marker_table` asks the **table**: each of the
  five needles must match something.
- `no_file_holding_control_flow_classifies_auto` asks the **tree**: no file holding
  one of the constructs may come back `auto`.

Both, and not either, for a reason the session supplied directly. The first
attempt at `opens_a_statement` **did not transpile** — `&body[.len needle ..]`, an
open-ended range in an index, which this surface does not parse. The port refused
and left `crates/voltaic-driver/src/` untouched, which is exactly what it is built
to do; `cargo build` then succeeded against the **stale** generated tree, and the
survey printed the old 88/14/3 as though the new markers had been measured. A
table-only gate reads the table it compiled against and would have passed that.
Only a gate that reads the files catches a marker that is present and not firing.

That is also `AGENTS.md`'s *"do not grep compiler output for `^error`"* in a new
costume: the port's refusal says *"should transpile"*, not *"error"*, so a filter
built to catch `^error|E0[0-9]+` showed nothing and the silence read as success.

### 10.7 Leaf 3: the identity gate, and it needs no wasm

`crates/voltaic-driver/vlt/migrate.vlt` and
`crates/voltaic-driver/tests/identity_gate.rs`. `gate` transpiles both sides and
compares the generated Rust line by line; `apply` writes **only** on
`Gate::Identical`, and `Gate` is one of `Identical`, `Differs { line, was, now }`
or `Unreadable { side, problems }`.

**The gate does not run the g2 blob**, though §3 step 4's wording says "with the
g2 compiler". It does not have to: the compiler in the tree reads **both**
generations for the duration of the turn, so it transpiles the generation-1
original and the generation-2 candidate itself. That is the same comparison with
one fewer moving part, and it costs nothing in trust because row 102's
`across_the_turn.rs` separately proves blob == tree over all fourteen ported
crates and all forty-seven compiling examples. The two gates compose: that one
says *blob is the tree*, this one says *tree(original) == tree(migrated)*.

**The write-after-gate claim is falsified, not asserted.** Inverting the two
statements in `apply` — write, then gate — turns
`a_migration_the_gate_rejects_leaves_the_file_untouched` and
`a_migration_the_compiler_cannot_read_leaves_the_file_untouched` red and leaves
the other three green. §2.3's first trap says a guard before a destructive step
has to be checked before the step; that is the check, and it was run in both
directions.

Five integration tests rather than more inline ones, because the claim is about
the **filesystem**: a pure test cannot observe a write that did not happen.
`migrate.vlt`'s own four tests stay pure and cover the decision —
including the one that matters most for pass 1, *a spelling change with the same
lowering passes*: `n |> .max 3` and `.max 3 n` are different text and identical
Rust, which is precisely what the gate has to accept.

One of the five is not in the plan and should be: `an_already_migrated_file_gates_
against_itself`. Row 103 migrates crate by crate with `seed/GENERATION`'s column
tracking progress, so a second run over a finished crate must be a no-op. Without
it the gate would report every completed crate as failing, which is a migration
tool that cannot be run twice.

### 10.9 Leaf 2: the visitor, and the first transform it carries

`crates/voltaic-driver/vlt/visit.vlt` traverses a `Program` and applies a pass to
every expression, bottom-up. It reaches all 51 `Expr` variants plus `Block`,
`Stmt`, `Condition`, `ConditionOperand`, `CaseArm`, `FieldInit`, `Guard`,
`Qualifier`, `Section`, `StrInterpPart`, `Binding` and nested `Item`s.

**It does not reach patterns, and that is a property rather than an omission.** A
pass rewriting expressions must not edit a pattern: `|>` cannot appear in one, and
a pass that touched one would change what a program *matches* rather than what it
computes. `a_numeral_in_a_pattern_is_left_alone` pins it, and the test that found
it is the one that listed a pattern literal among the expression numerals and
failed.

**The pass is an enum, not a closure, because this surface cannot spell the
closure.** `&dyn Fn(&mut Expr) -> ()` in a signature loses its second parameter —
`capture_type` is greedy, so the `-> ()` reads as the *closure's* return and the
arity check reports 1 where 2 was written. A type alias hid the arrow and
defaulted to `+ 'static`, which no closure over a local satisfies; a generic alias
(`type Step<'a> = …`) does not parse at all. Three attempts, and the enum is
better anyway: no lifetime to thread, a closed list a reader can enumerate, and
`Copy`, so every `rewrite_*` takes it by value.

### 10.10 The first transform, and the identity gate paid for itself twice

`Pass::Pipelines` turns `a |> .f b` into `a.f b`. Run over the compiler's own
105 `.vlt` files — 8,479 `.`-calls, a corpus nobody wrote to be easy to migrate —
with leaf 3's gate on every file. Nothing is written; the assertion is that the
rewrite *would* be safe.

**It rejected twelve files on the first run, and named the construct.** A
no-argument stage — `self.args |> .iter` — collapses to `self.args.iter`, which
**reparses as `Expr::Field`**: a field access. The solver cannot say otherwise,
because doc 97 §10.36's `std` boundary means it knows no methods on `Vec` or
`Path`. So the pass is restricted to stages that carry arguments, where the
reparse gives a `Dot` and the lowering is a call.

That is §1.2's whole argument demonstrated: *"byte-identical generated Rust means
the migration changed the spelling of the program and nothing else"*. A
tests-still-pass gate would have shipped twelve files emitting field reads that do
not exist. The no-argument half unblocks when `std` reaches the declaration
environment, which §10.8 records.

**Then it rejected one more, and that one was the formatter.** Reprinting
`voltaic-lattice/vlt/infer_mode.vlt` turned `&**callee` into `&(*(*callee))` — the
same AST, which is why `formatting_preserves_meaning` was content, and different
*text*, which the emitter passes through for a unary operand. The printer
parenthesised **any** nested prefix unary; parentheses are needed only where the
two operators would lex as one token, and `&` after `&` is the only such pair —
`&&` is in the operator table and `**`, `--`, `!!` are not.

So byte-identity is **stricter than "the same program"**, and this is where that
pays: a semantically-null text difference in the migrated output is still a
difference a reviewer would have to account for, and the gate found the one file
in 105 where the formatter introduced one.

### 10.11 The pass is applied and then reverted, and the number is why

`VOLTAIC_UPDATE=1` on `collapsing_every_pipeline_changes_no_program` applies the
migration through `apply`, which gates each file before writing it. It ran, it
wrote **71 files**, every one gated — and the diff was then reverted.

The number that decides it, measured over the 61 files holding a `|>`:

| | Stages |
|---|---:|
| no arguments | **450** |
| with arguments | 23 |
| | 473 |

**The pass reaches 5% of the stages, and mid-chain it reads worse than what it
replaces.** A chain is `self.args |> .iter |> .map (…)`, and only the `.map`
stage carries arguments, so collapsing it alone yields
`(self.args |> .iter).map (…)` — a parenthesised pipeline inside a receiver-first
call, where the original was three aligned lines. Two spellings of application in
one expression is exactly what doc 87 §1.2 deletes `|>` to stop.

So the migration waits for the no-argument half, when a whole chain collapses at
once. That is doc 97 §10.36's `std` boundary, and §10.8 is where it is recorded.

**What is kept is the part that keeps its value with no source change**: the
visitor, the transform, and the assertion that the rewrite *would* be safe across
all 105 files. When `std` reaches the declaration environment the pass widens by
one condition — `if .is_empty found` goes — and the gate is already there to
prove the result.

**And the revert needed `git checkout HEAD --`, not `git checkout --`.**
`AGENTS.md` records why: the latter restores from the *index*, and `git add -A`
had already staged part of this session. The 71 files came back from HEAD, which
is what was wanted; from the index they would have come back migrated.

### 10.12 The `std` boundary is not a `std` boundary, and that changes which row owns it

Doc 97 §10.36 left the no-argument half of wave 3 waiting on *"`std` reaching the
declaration environment"*. The obvious rule was tried and is **withdrawn**, and
the reason relocates the dependency.

The rule: *a receiver the solver resolved to a type not declared here has no
fields the environment knows, so `x.name` is a method call.* Numeric names stay
fields (`x.0` is a tuple index on any type), an unresolved receiver still claims
nothing, and a declared type is still asked.

It worked on the case it was written for — `v.len` on a `Vec` became `v.len()` —
and the port then **refused `voltaic-lattice`**, naming one line:

```
-  from_text(written.text.trim())
+  from_text(written.text().trim())
```

`written : &TypeExpr`, and `TypeExpr` is a `data` with a `text` field. It is
simply declared in **`voltaic-ast`**, not in the program being compiled.

**`declarations::collect` runs over one `Program`, and a `Program` is one `.vlt`
file.** So the environment knows the types declared in that file and nothing
else — not `Vec`, and not any type from another crate in this project either.
"Not declared here" is therefore not "foreign": the environment's ignorance is
the *normal* case, and a rule keyed on it reclassifies every cross-crate field
read as a method call.

No table of `std` names fixes that, because `std` was never the boundary. What
the no-argument half needs is **cross-crate declaration knowledge** — the
`.vlt.sig` sidecars carrying each crate's declared types, which is row 100 leaf
1's other half.

That is the same dependency row 104 leaf 5 waits on (doc 102 §10.8), which makes
it the one unblocking move with two consumers: wave 3's no-argument half, and
therefore leaf 2's receiver-order transform, leaf 6's crate-by-crate run, row 102
step 5, and `.vlt.sig` in `dependency_sources`.

**The port is what caught it, and it caught it the way it is built to.** It staged
`voltaic-lattice`, compiled the staged tree, refused it, and left
`crates/voltaic-lattice/src/` untouched — so `cargo build` stayed green on the
previous generation while the mistake was diagnosed from
`target/voltaic-port-staging/`. A migrator that wrote first would have left the
workspace unbuildable, which doc 97 §10.34 records as the state the port cannot
rescue itself from.

### 10.8 Wave 3 is the next prerequisite, and the solver could not answer its question

Leaf 2's largest `auto` construct is doc 87 §1.2's inversion of `.`:

> | `f x` | `f(x)` — a free function |
> | `x.f y` | `f(x, y)` — the same function, receiver first |

That is **not implemented**: `"ab".to_uppercase` emits `"ab".to_uppercase` with no
call, because plan 57 §1.1 made `.` field access and nothing else, and rustc
answers `E0615` *"method, not a field"*. Row 99 wave 3 owns the inversion. Until it
lands, pass 1's receiver-order rewrite has no target surface to rewrite *to* —
which is §5.7's and §5.6's shape for a third time, found the same way.

What could be closed here was closed. Doc 87 §2.2 states:

> Field access is `.` too, and a bare field reference is `(.name)` — resolved by
> the solver, **which knows whether `name` is a field or a method.**

**It did not know.** `declarations::collect` pushed a `data` declaration's fields
and an `instance` block's methods into one `members` list with nothing telling
them apart, so `.member "Cursor" "pos"` and `.member "Cursor" "bump"` were
indistinguishable. `Member` now carries a `MemberKind`, and `kind_of`, `is_field`
and `is_method` are the queries wave 3's emitter will ask — `x.pos` is `x.pos`,
`x.bump` is `x.bump()`.

`kind_of` returns `Option<MemberKind>` rather than defaulting, because *"neither"*
is a real third answer and the one that will matter most: a receiver whose type
the solver does not know is **not** a field access, and saying that is different
from saying it is a method. Over a `std` type the solver knows no fields at all,
so every `.name` on one is a call — correct for `"ab".len`, and wrong for a public
field like `Range::start`, which rustc will reject loudly. That is the charter
working rather than a gap, and it is the kind of case wave 3 has to measure rather
than reason about.

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

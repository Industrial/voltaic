# Plan 102 — The Seed Across the Turn

Plan ID: `seed-g2`
Depends on: 99
Milestone: 17 — *the turn*
Status: `planned` — awaiting approval; §1.2 is the invariant and §3 is the ordering

---

## 1. What this plan is

Every crate under `crates/` is written in Voltaic. `seed/voltaic1.wasm` is a previous
Voltaic compiler, and it **cannot parse a single `.vlt` file**. So a surface change of
generation 2's size cannot be self-applied, and this row is the ordering that gets
from one generation's seed to the next without ever breaking the bootstrap.

It also introduces the generation concept into the seed's naming and into
`seed/PORTED`.

### 1.1 Why this is the hardest row in the register

`seed/SEED.md` states the dependency that makes it hard:

> `voltaic-cli` is built from `voltaic-fmt`, which is built from `voltaic-ast` — so on
> a clean clone with those trees untracked, nothing exists that could generate them.

Thirteen crates, dependency-ordered in `seed/PORTED`, every one of them with an
untracked generated `src/`. The blob is the only thing that breaks the cycle. A row
that changes the surface the blob must parse, without first producing a blob that
parses it, leaves a repository that cannot be built from a clean clone — and the failure
is invisible locally, because the generated trees are still on disk.

**Plan 68 proved this within the hour.** `AGENTS.md` records it: plan 68 untracked the
generated trees, *"breaking the bootstrap within the hour"*, and plan 72 replaced the
committed trees with the blob.

### 1.2 The invariant

`seed/SEED.md` states it, in the context of Zig's `zig1.wasm`, and it is the rule this
whole row serves:

> **the seed was built from the Rust implementation before that implementation was
> replaced.** Zig's C++ compiler produced the first `zig1.wasm` and was deleted
> afterwards. A seed built the other way round is verified by a compiler the seed is
> supposed to underwrite.

Generalised to a generation change:

> **A generation's seed must be built by the previous generation's compiler, before
> the previous generation's source is replaced.**

Equivalently, and this is the form to check a plan against: *at no point is the only
compiler for a language written in that language.*

### 1.3 The only ordering that satisfies it

There is exactly one, and it is counterintuitive:

> **Generation 2's front end is written in generation 1.**

Rows 94–101 build `voltaic-lattice`, the new lexer token set, the new parser and the
new emitter — and they write all of it in **`.vlt` files using generation 1's
surface**, because that is the surface `voltaic1.wasm` can compile. Row 94 §2 already
states this as a precondition.

The result is a compiler that *reads* generation 2 and is *written in* generation 1.
That compiler builds the generation-2 seed. Only then does the tree's own source
migrate (row 103).

The alternative orderings and why each fails:

| Ordering | Fails because |
|---|---|
| Write generation 2's front end in generation 2 | nothing can compile it; the cycle has no entry point |
| Migrate the source first, then build the front end | the migrated source cannot be compiled by `voltaic1.wasm` and the new front end does not exist yet |
| Commit the generated Rust again as a bridge | plan 68's mistake exactly, undone by plan 72 |
| Hand-write a generation-2 parser in Rust as a bootstrap | a second implementation to keep in sync, which is rustc's `cfg(bootstrap)` cost — and `AGENTS.md` records that this project chose two commands over that precisely because its seed is a build artifact rather than a release |

### 1.4 Generation-tagged names

| | From | To |
|---|---|---|
| the current blob | `seed/voltaic1.wasm` | `seed/voltaic-g1.wasm` |
| the new blob | — | `seed/voltaic-g2.wasm` |
| the crate list | `seed/PORTED` | `seed/GENERATION` |

`voltaic1.wasm`'s name encodes *version 1 of the seed*, which was right when there was
one generation and becomes ambiguous the moment there are two. Row 93 deliberately
left this rename alone, because naming a generation before the generation exists is
putting the conclusion in the premise.

`seed/PORTED` becomes `seed/GENERATION`: one line per crate, dependency-ordered, with
**the plan that ported it and the generation it is written in**. The file's own comment
is the reason the shape matters and it is kept:

> **This is the only copy.** It was four — `port.rs`, `conformance.rs`, `bootstrap.rs`
> and `bootstrap.sh` — and three had drifted when `voltaic-diag` landed, including the
> pair that proves a clean clone rebuilds every generated tree. A duplicated list
> drifts consistently: the check and the thing checked agree while both are wrong.

During the turn the file carries **both** generations, and that is the point: it is
what tells `bootstrap.sh` which blob to run for which crate.

---

## 2. Preconditions, measured

- Row 99 `done`: the compiler accepts generation 2. The tree is still written in
  generation 1 — row 99 §5.8 asserts `crates/*/vlt/**` has no diff from that row.
- `seed/voltaic1.wasm` is 1,005,131 bytes with its blake3 recorded in `seed/SEED.md`
  and checked by `crates/voltaic-seed/tests/blob.rs`, which *recomputes* the hash and
  fails on a mismatch — so the blob cannot be hand-edited and the row cannot be
  stale.
- Three tests keep the bootstrap honest and all three must stay green through the
  turn: `blob.rs` (provenance), `bootstrap.rs` (a clone without the generated Rust
  rebuilds it byte-identically), `fixpoint.rs` (stage1 == stage2).

### 2.1 One trap, already paid for

`AGENTS.md`:

> **`moon run :seed-refresh` replays a cached size, so the number it prints can be a
> previous run's.** […] Measured 2026-11-12: it reported `1035089 bytes` with
> 1,005,152 bytes on disk. That is what made plan 91 wave 5 misread its own refresh as
> a no-op. **Check `git status seed/` and `stat` the file; do not read the task's
> output.**

This row refreshes the seed twice. Both times the verification is `stat` plus a status
check, never the task's stdout.

---

## 3. The ordering, as a sequence

Six steps. Steps 3 and 5 are the ones that cannot be reordered.

1. **Rename.** `voltaic1.wasm` → `voltaic-g1.wasm`; `PORTED` → `GENERATION`, every
   crate marked `g1`. Update `bootstrap.sh`, `blob.rs`, `bootstrap.rs`,
   `conformance.rs`, `port.rs` and `SEED.md`. No behaviour change; `blob.rs`'s hash is
   unchanged because the bytes are.
2. **Verify g1 still bootstraps.** Delete every generated `src/`, run the bootstrap
   from `voltaic-g1.wasm`, build, test. This is the baseline, and it is taken *before*
   anything interesting happens so that a later failure is attributable.
3. **Build `voltaic-g2.wasm` from the generation-1-written front end.** The compiler
   in the tree reads generation 2 and is written in generation 1; compile it to
   `wasm32-wasip1` exactly as plan 72 compiled the first blob. Record provenance in
   `SEED.md`: commit, size, blake3, **and the generation of the source it was built
   from**, which is the field §1.2's invariant makes necessary and which `SEED.md`
   does not currently have.
4. **Prove g2 reads generation 2.** Run `voltaic-g2.wasm` over the five-program port
   set from row 99 §1.2 and over `examples/*.vlt`, and diff against what the in-tree
   compiler produces. g2 and the in-tree compiler must agree byte-for-byte; they are
   the same compiler, one of them in wasm.
5. **Row 103 migrates the tree.** Not this row. The tree's source becomes generation 2,
   compiled by `voltaic-g2.wasm`.
6. **Fixpoint on g2, twice, then retire g1.** `fixpoint.rs` asserts stage1 == stage2
   against `voltaic-g2.wasm`. Run it on a clean clone twice. Only then is
   `voltaic-g1.wasm` deleted and `GENERATION` reduced to one generation.

### 3.1 Why g1 is retired last, and not when it stops being needed

Doc 89's completion section says it: *until `fixpoint` is green on g2 twice, g1 is the
only thing that can rebuild the tree.* Deleting g1 at step 4 — when it is no longer
*used* — would mean that a defect found in g2 at step 6 leaves nothing able to rebuild
the repository, and the defect would be found on a clean clone, which is where it
matters and where the generated trees are absent.

The cost of keeping it is one megabyte for the duration of the turn.

---

## 4. Leaves

One per step in §3, plus:

### Leaf 7 — `bootstrap.sh` across two generations

For the duration of the turn the script reads `GENERATION` and runs the blob the crate
is marked for. A crate marked `g1` gets `voltaic-g1.wasm`; one marked `g2` gets
`voltaic-g2.wasm`.

This is what lets row 103 migrate crates **one at a time** rather than in one commit,
and that is the whole reason the two-generation `GENERATION` file exists. A migration
that had to land thirteen crates atomically would be a migration with no reviewable
intermediate state.

### Leaf 8 — `SEED.md` gains a generation field

Provenance for a seed now includes which generation's source built it, because §1.2's
invariant is checkable only if that is recorded. A blob whose source generation equals
the generation it compiles is the error the invariant forbids, and `blob.rs` can assert
it.

---

## 5. Acceptance criteria

1. Step 2's baseline bootstrap is green before step 3 begins, recorded in §10.
2. `voltaic-g2.wasm` and the in-tree compiler produce byte-identical output over
   `examples/*.vlt` and the port set.
3. `SEED.md` records g2's commit, size, blake3 and **source generation**, and
   `blob.rs` recomputes the hash and asserts the source generation is *earlier* than
   the generation the blob compiles.
4. `bootstrap.sh` handles a `GENERATION` file with mixed generations, with a fixture
   proving a mixed tree bootstraps.
5. `fixpoint.rs` green against g2 on a clean clone, **twice**.
6. `voltaic-g1.wasm` is deleted only after §5.5, and `GENERATION` reduced to one
   generation in the same commit.
7. Every seed verification in this row uses `stat` and a status check, never the
   refresh task's stdout (§2.1).

---

## 6. Out of scope

- **Migrating the tree's source.** Row 103.
- **Publishing a seed.** The blob is a build artifact, not a release, and that is what
  makes version skew cost two commands instead of rustc's `cfg(bootstrap)` or Go's
  version chain. `AGENTS.md` records that this is only cheap while the seed is
  unreleased, which is also the window in which Voltaic's surface still moves. Nothing
  here changes that.
- **Reducing the blob's size.** 1 MB is fine and `-Oz` experiments are recorded in
  `.tmp` from a previous plan; not this row's business.

---

## 7. Gate

`blob.rs`, `bootstrap.rs`, `fixpoint.rs`, plus the mixed-generation bootstrap fixture,
plus the standing push gate. The from-scratch bootstrap — generated trees deleted — is
required at steps 2, 4 and 6 and is the only check that can catch this row's failure
mode, because every other check passes against a tree whose `src/` is already on disk.

---

## 8. Emerald reuse

None — Emerald was not self-hosting. The reuse here is from outside: Zig's
`zig1.wasm`, whose model plan 72 adopted including the part §1.2 quotes, and whose
discipline this row generalises from a bootstrap to a generation change.

Also relevant and declined: rustc's `cfg(bootstrap)` double implementations and Go's
version chain, both of which are expensive because their seeds are *releases*. The
survey is `docs/64 - 70 - The Source Beside the Crate.md` §1.

---

## 9. Decision log

| Decision | Why |
|---|---|
| Write generation 2's front end in generation 1 | the only ordering satisfying §1.2; §1.3 has the alternatives and why each fails |
| Generation-tagged blob names | one blob needs no generation in its name; two do |
| `PORTED` → `GENERATION`, carrying both generations | lets row 103 migrate one crate at a time with a reviewable intermediate state |
| Keep it a single list | its own comment: a duplicated list drifts consistently |
| `SEED.md` gains a source-generation field | §1.2's invariant is checkable only if recorded |
| Retire g1 after two clean fixpoints, not when it stops being used | §3.1 |
| Verify by `stat`, never by the refresh task's output | `AGENTS.md`'s measured cache trap |
| No second hand-written implementation | rustc's and Go's cost, declined for the reason the seed exists |

---

## 10. Result

In progress. **§1.4's renames and leaves 7 and 8 are done**, and they are done
*before* row 99 rather than after it — which is a reordering of this register,
with a reason.

### 10.1 This row's infrastructure is a prerequisite for row 99, not a consequence

§2 lists `Row 99 done` as this row's first precondition, and for the *seed
generation* step that is right: there is no generation-2 front end to build a
generation-2 blob with until row 99 ships.

But the **two-generation plumbing** has the dependency the other way round. Doc
97 §10.13 records what row 99 wave 1 found: the moment the accepted surface
changes, every gate that parses the compiler's own source with the freshly built
compiler breaks — `inferred.rs`, `no_comments.rs`, `port.rs`,
`directory_structure.rs`, `line_cap.rs`, `construct_coverage.rs`,
`grammar_inventory.rs`. The tree keeps *building*, because `bootstrap.sh`
generates `src/` with the generation-1 seed. The gates are what break.

Resolving that needs a per-crate notion of which generation a crate is written
in, and that is exactly leaf 7. So leaf 7 ships now and the blob it will one day
dispatch to does not exist yet — which is the correct order, and
`every_generation_named_in_the_list_has_its_blob` is what keeps it honest.

### 10.2 The renames

| | From | To |
|---|---|---|
| the blob | `seed/voltaic1.wasm` | `seed/voltaic-g1.wasm` |
| the crate list | `seed/PORTED` | `seed/GENERATION` |

`voltaic1.wasm` encoded *version 1 of the seed*, which was right with one
generation and ambiguous with two. Nine code and config files cited the old
paths; `docs/01`–`docs/85` still cite them and are left alone, because they are
the record of what those plans did and row 104 owns reference-doc regrading.

### 10.3 Leaf 7: `bootstrap.sh` dispatches per crate

`GENERATION` carries `crate plan generation`, and the script reads the third
column:

```
bootstrap: voltaic-seed regenerated from crates/voltaic-seed/vlt with seed/voltaic-g1.wasm
```

Every crate is `g1` today. The first `g2` line is row 103's first commit; the
last `g1` line leaving is §3.1's retirement step. **This is what makes row 103 a
migration rather than one commit** — fourteen crates landing atomically would
have no reviewable intermediate state at all.

Three refusals, each naming the thing a bare failure would not:

- a generation that is neither `g1` nor `g2` is refused by name, rather than
  producing `seed/voltaic-.wasm`;
- a missing blob is reported **with the crate that wanted it**, because
  `wasmtime: no such file` names the blob and not the reason one was asked for;
- `generations()` refuses a line with no third column, because a line without one
  would otherwise silently pick a blob. It also refuses a *fourth* column, since
  nothing reads one.

`ported()` keeps its name and shape, so the thirteen call sites that want
`(crate, plan)` are untouched. `generations()` is a separate function rather than
a widened tuple for that reason: this row adds a column, not a new contract for
every existing reader.

### 10.4 Leaf 8: the invariant is checked, not merely stated

`SEED.md` gained two rows:

| | |
|---|---|
| compiles | g1 |
| built by | the pre-Voltaic Rust implementation, deleted by plan 72 |

§1.2's invariant — *at no point is the only compiler for a language written in
that language* — is now a test. `blob.rs` refuses a blob whose `built by` names
the generation in its `compiles`, and refuses a table missing either row.

For `voltaic-g1.wasm`, `built by` names no generation at all. That is the base
case and the same shape as Zig's: the compiler that produced it was the committed
Rust one, which plan 72 deleted afterwards. `voltaic-g2.wasm` will record
`compiles: g2` and `built by: g1`, because §1.3's ordering writes generation 2's
front end **in generation 1**.

Worth being exact about what this checks. Nothing in the repository can recompute
which compiler built a committed binary, so this asserts a *recorded* fact. What
it can do is refuse a provenance table that states the forbidden thing, and refuse
one that states nothing — and the second is the failure that matters, because an
absent row reads as fine while a present wrong one reads as verified.

### 10.5 Steps 2, 3 and 4 are done, and the precondition was the ladder

Leaves 1–6 needed row 99's surface to exist. It does now: doc 87 §5.7's ladder
expression and §5.6's `else` tail were the last two pieces, and the two commits
that added them are what unblocked this row rather than anything in it.

**Step 2, the baseline.** Every generated `src/` deleted, `./bootstrap.sh` from
`voltaic-g1.wasm`, build, test: **0.58s** to regenerate fourteen crates,
**2.88s** to `cargo build`, **1301/1301**. Taken before anything interesting
happened, which is §3's whole reason for ordering it second — a later failure is
now attributable.

The generated trees were confirmed untracked **in the tree** first, not in
`.tmp/`: `git ls-files "crates/*/src/**/*.rs"` reports zero. `AGENTS.md` records
why that check is not optional — a scratch probe answers a different question
about git status than the repository does, optimistically.

**Step 3, the g2 blob.** `seed/voltaic-g2.wasm`, 1,084,471 bytes, 79,301 more
than g1 — the two surface additions, nothing surprising, since both blobs are the
same crate at two commits. `seed/SEED.md` now carries an identity table per
generation and `moon.yml` a task per blob.

Two tasks rather than one with an argument, and the reason is the invariant: g1's
blob is frozen at its own provenance for the duration of the turn and g2's tracks
the tree, so a single parameterised task would put refreshing the wrong one one
typo away — and refreshing g1 from a generation-2 tree is exactly what §1.2
forbids, caught by `blob.rs` only *after* the bytes were replaced.

`blob.rs` was generalised to match, and one hazard in doing so is worth keeping:
**a line-wise pass over `SEED.md` would have stamped one blob's hash onto both
tables.** With one table, "rewrite every `| blake3 |` row" was correct; with two
it is a bug that leaves the test *passing* while one blob's provenance is wrong.
The blessing is per-section, keyed by each table's `compiles` row — which is also
what determines the filename, so there is no second copy of the mapping.

It gained a fourth test for the other direction. The size-and-hash check iterates
the *tables*, so it is blind to a `.wasm` in `seed/` with no table —
an orphaned blob from an abandoned generation, tracked and a megabyte, with
nothing asserting anything about it. `every_committed_blob_has_a_provenance_table`
iterates the *directory*. Two-sided, for the reason this repository keeps
rediscovering: a list plus a check that reads the list is narrow in one direction
only.

**Step 4, byte-identity, and it held on the first attempt** — once the corpus was
right. `crates/voltaic-seed/tests/across_the_turn.rs` runs the g2 blob and the
native in-tree compiler over the same input and compares bytes: all fourteen
ported crates, and all forty-seven examples that compile. No rustfmt on either
side, deliberately, since formatting both would normalise away the one class of
disagreement the test exists to catch.

It failed on its first run, and not on a disagreement: **`examples/lexical.vlt` is
not a compilable program.** It is a lexer fixture — `[0, ...rest, 9]`,
`total : u64`, a run of token shapes — and `voltaic build` rejects it exactly as
the seed does. Pointing a compiler at `examples/` therefore fails before any
comparison happens. The corpus is now staged from the examples that **have a
golden**, which is derived from the tree rather than a second list: `tests/golden/`
holds a `.vrs` for every example that compiles, and exactly one example has none.

The falsification is worth recording because it is also the first hard evidence
the two generations are distinct. Run g1 over the same staged corpus and it
rejects **four of forty-seven** — `ladder`, `else_tail`, `inferred`, `rows` —
emitting 43 files where g2 emits 47. So the comparison is not vacuous, g1 cannot
read generation 2, g2 can, and the four-file gap names precisely what the turn has
added so far.

### 10.6 Step 6's first half: `fixpoint` on g2, green twice from a clean tree

`fixpoint.rs` now runs against `seed/voltaic-g2.wasm`, and it is green twice over
a tree whose every generated `src/` was deleted and rebuilt by `./bootstrap.sh`
in between.

**§3 orders step 6 after row 103's migration, and that ordering turns out to be
about something else.** It is about the *source* becoming generation 2. Fixpoint
does not ask that: it asks whether the compiler reproduces itself through the
blob, and g2 reads the generation-1 source the tree still has —
`across_the_turn.rs` proves it byte-for-byte over all fourteen crates. So the
first half of step 6 was available as soon as g2 became the bootstrapping blob.

**And it had to move, not merely could.** Doc 97 §10.41 made `bootstrap.sh` prefer
the newest blob that reads a crate's generation, because `voltaic-g1.wasm` is
immutable and an emitter change made its output diverge from the tree's
permanently. g2 is therefore what a clean clone runs. Leaving `fixpoint.rs`
pointed at g1 would have asserted stage1 == stage2 for a blob nothing runs any
more — a green test about nothing, which is worse than a red one.

An earlier version of this subsection said *"`fixpoint.rs` still runs against g1,
which is correct while every crate is `g1`"*. That was right when it was written
and stopped being right the moment the dispatch changed, which is the ordinary
way a correct sentence goes stale: the fact it rested on was somewhere else.

### 10.8 Step 6's second half: green on two genuinely clean clones

**Acceptance criterion 5 is met.** `fixpoint.rs` is green against g2 on a clean
*clone*, twice, where "twice" is read as **two independent clones** rather than
two runs in one — two runs in one clone would be a cached pass and would not
notice a non-deterministic blob.

Each clone was made with `git clone --branch … --single-branch` from the local
repository into `.tmp/`, which is the right instrument for exactly the reason
`AGENTS.md` warns about `.tmp/` in general: the warning is that the *parent*
ignores `.tmp/` wholly, so git answers a different question about paths inside
it. A clone has **its own `.git` and its own committed `.gitignore`**, and git
commands inside it resolve against those, so the question being answered is the
clone's. Confirmed rather than assumed — `git status --porcelain` inside each
clone after `./bootstrap.sh` reports **0 lines**, with the generated `src/`
present and ignored by the clone's own rules.

The sequence, per clone: clone (no `src/` anywhere — `ls crates/voltaic-lexer/`
is `Cargo.toml moon.yml tests vlt`), `./bootstrap.sh`, then the suite. All
twelve `voltaic-seed` tests pass, and the four that matter here are
`stage1_equals_stage2`, `the_g2_blob_and_the_tree_agree_on_every_ported_crate`,
`a_clone_without_the_generated_rust_bootstraps_from_the_seed`, and
`the_generated_trees_are_ignored`.

#### The dispatch the third column was supposed to do, it does not do

Found while confirming that retirement was still blocked: **`bootstrap.sh`
regenerated every one of the fourteen crates with `voltaic-g2.wasm`**, although
all fourteen `GENERATION` lines read `g1`. That is correct behaviour and
deliberately documented in the script — g1 is immutable, so once the emitter
moved past plan 72 its output differs from the tree's permanently, and g2 reads
generation 1 byte-for-byte. The script validates the column against `g1 | g2`
and then takes the newest blob present.

What was wrong is `seed/GENERATION`'s own header, which claimed *"`bootstrap.sh`
runs `voltaic-g1.wasm` for a crate marked `g1` and `voltaic-g2.wasm` for one
marked `g2`"* — and `blob.rs`'s doc comment, which said the column is *"the thing
`bootstrap.sh` dispatches on"*. Both are corrected in place. The column is
**validated, not dispatched on**, and its three live jobs are: telling a reader
which surface a given `.vlt` may use, keeping the blob set honest via
`every_generation_named_in_the_list_has_its_blob`, and carrying `blob.rs`'s
invariant that a blob's `built by` may not name its own `compiles`.

**The reviewability the column exists for survives intact**, which is why this is
a comment fix and not a design problem: moving one crate's `.vlt` to the new
surface and flipping its line is still one reviewable commit. That property came
from the commit granularity, never from which blob ran.

### 10.7 What remains

- **Step 5**, which is row 103's: migrating the tree's source to generation 2,
  crate by crate, flipping each `GENERATION` line to `g2`. Blocked, and not on
  this row: doc 97 §1.3's deletions 3 and 4 cannot be written as specified, which
  doc 101 §10.14 measures and doc 97 §1.3 now records.
- **Step 6 is done**, both halves — §10.6 for the clean tree, §10.8 above for the
  two clean clones.
- **Then** `voltaic-g1.wasm` is deleted and `GENERATION` reduced to one
  generation, which is §3.1's retirement step and still last. It is **not**
  unblocked by step 6: all fourteen lines still read `g1`, so
  `every_generation_named_in_the_list_has_its_blob` would fail the moment the
  blob were deleted. Retirement needs step 5 first, exactly as §3 orders it.

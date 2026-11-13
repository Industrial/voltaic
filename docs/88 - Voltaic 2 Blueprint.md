# Voltaic 2 — Blueprint

Status: normative for the implementation. Supersedes `docs/02 - Voltaic Update
Blueprint.md` where the two disagree; doc 02 stays as the record of generation 1's
architecture and is not edited, **including §12.4**, whose reversal is recorded in
`docs/90 - 92 - The Charter Reversal.md` rather than by editing the line.

Doc 86 is the language, doc 87 the surface. This document is what has to change in
the tree, what it costs, and what measurement would prove the design wrong.

---

## 1. The pipeline gains one stage

```
generation 1:   .sdx → lex → parse → emit → .xrs → rustc
generation 2:   .vlt → lex → parse → SOLVE → emit → .vrs → rustc
                                       ▲
                                  voltaic-lattice
```

Everything before and after the new stage keeps its job. The lexer still emits spans,
trivia and whitespace adjacency; the parser is still hand-written recursive descent
with a Pratt loop; the emitter still produces ordinary, reviewable, fully-annotated
Rust; rustc is still the authority on everything it can check.

What the new stage does is answer four questions the emitter used to be unable to
ask: *what is this expression's type, what can it do, how is it held, and is its
precondition satisfied.* The emitter then writes the answers down.

### 1.1 The emitter's job gets larger, not smaller

This is the counterintuitive consequence and it is worth stating first. Generation 1
transcribed: the author wrote `&self -> u32` and the emitter wrote
`fn pos(&self) -> u32`. Generation 2's author writes neither, so the emitter writes
both — every signature, every `&`, every `&mut`, every lifetime elision cannot
supply, every `?`, every `.await`, every `#[derive(…)]`.

A `.vrs` is therefore *more* explicit than generation 1's `.xrs`, not less, and that
is the property that keeps rustc an independent check rather than a rubber stamp:
rustc type-checks a program that states its own types.

### 1.2 Why one crate and not four

`voltaic-lattice` holds the type engine, the effect-row engine, the mode engine and
the refinement engine in one crate, because the four solve **together**. A mode
constraint can be created by an effect discharge (`else` on a `fail` whose error
escapes), an effect row can be created by a mode (`mut ρ` is a mode constraint that
reads as an effect), and a refinement's predicate mentions `.len`, which is a type
question. Splitting them into four crates would make the dependency graph a cycle.

They are four *modules* with one solve loop over a shared constraint store:

```
crates/voltaic-lattice/vlt/
  lib.vlt            # module declarations and the public entry point
  constraint.vlt     # the store, and the fixed-point driver
  type/              # unification, generalisation, instantiation
  row/               # effect rows — the subset lattice
  mode/              # the three-element chain
  refine/            # predicate extraction and the solver interface
  explain.vlt        # `--why`: constraint provenance, doc 86 §8.1
```

`explain.vlt` is not an afterthought. Doc 86 §2 clause 1 admits a solver-determined
lowering **only if the choice can be printed**, so provenance tracking is a
correctness requirement of the constraint store, not a debugging feature bolted on
later. Every constraint carries the span and the reason that created it.

---

## 2. Crate deltas

| Crate | Change |
|---|---|
| `voltaic-lexer` | token set shrinks: 74 keywords to 11, four operators deleted (`\|>`, `>>>`, `<<<`, `$`), `?` and `!` gain row meanings, `@` is new. The off-side rule is untouched. |
| `voltaic-ast` | nodes gain a judgement slot. Every node still carries a byte span — doc 02's rule, and the one Emerald broke twenty-one plans after its diagnostics plan. |
| `voltaic-parser` | **loses the `class_heads` map.** Generation 1's parser learns a member's signature shape from the `class` declaration in the same file, which is what forces doc 87 §3.3's one-file trap. Resolution moves to `voltaic-lattice` and the trap goes with it. |
| `voltaic-lattice` | **new.** §1.2. |
| `voltaic-emit` | emits signatures, borrows, lifetimes, `?`, `.await` and derives from the judgement. Largest single change in the tree. |
| `voltaic-diag` | four new code ranges (doc 87 §17), and constraint-provenance rendering for `--why`. |
| `voltaic-driver` | per-crate solve, sidecar maintenance, and the cache change in §3. |
| `voltaic-fmt` | writes `.vlt.sig`; loses the signature-line printer and the four deleted operators. |
| `voltaic-cli` | `voltaic sig` is new; `voltaic check --semver`, `--why`, `--require-signatures`, `--pedantic` are new flags. |
| `voltaic-lsp` | **the generation's biggest user-visible win.** Inlay hints for every inferred judgement, hover showing `--why`, and a code action that writes an inferred signature into the source. The LSP is what makes "no signatures in the file" tolerable. |
| `voltaic-mcp` | `voltaic_sig` joins `voltaic_build`, `voltaic_check`, `voltaic_fmt`, `voltaic_source_map`. |
| `voltaic-bench` | gains a solve-time axis; §4. |
| `voltaic-fuzz` | gains a target per engine. A fuzz target that makes the solver *hang* is the one failure clause 5 forbids, so this is a correctness gate, not hardening. |
| `voltaic-seed` | generation-tagged blobs; `docs/100 - 102 - The Seed Across the Turn.md`. |

Thirteen crates become fourteen. `docs/06 - Voltaic Code Architecture.md`'s rules —
one cohesive group per file, `src/` a module tree at most three deep, path-aware
naming, the 1,500-line cap, no comments in the compiler's own source — apply to
`voltaic-lattice` unchanged, and its layout above is written to satisfy them.

---

## 3. Incrementality is the real architectural cost

Generation 1's driver caches **per file**, and that is sound because a file's
lowering depends on its own text and nothing else. Generation 2's solve is
**per crate**: mode inference is a fixed point over the crate's call graph, so
editing one function can change another function's inferred borrow.

This is the reversal's largest engineering consequence and it is not optional.

### 3.1 The shape of the fix

Three levels, in order of cost:

1. **Signature-level fingerprint.** After a solve, each definition's judgement is
   hashed. An edit that does not change any judgement in a file cannot affect any
   other file, so the solve stops at the file boundary. In practice most edits are
   of this kind — a body change that keeps a signature.
2. **Reverse-dependency solve.** If a judgement hash changes, re-solve the
   definitions that reference it, transitively, and no others. The call graph is
   already built for `voltaic-driver`'s lints.
3. **Whole-crate solve.** The fallback, and what a cold build does.

### 3.2 Cross-crate is not incremental, it is *published*

Doc 86 §8.3 is the rule: a downstream crate type-checks against the upstream
crate's `.vlt.sig`, not its source. So the dependency graph between crates carries
*signatures*, exactly as Rust's does, and the per-crate caching that
`.moon/tasks/tag-crate.yml` already relies on keeps working.

This is why the sidecar is committed rather than cached, and it is the whole reason
`docs/98 - 100 - The Signature Sidecar.md` is its own plan. A cached sidecar would
make a clean clone's build depend on a build artifact, which is the mistake plan 68
made with the committed `.rs` trees and plan 72 undid.

### 3.3 The `dependency_sources` group still holds

`AGENTS.md` records that `deps: ['^:check']` is **not** what makes moon's per-crate
caching correct — the `dependency_sources` file group is, because it makes a crate's
transitive dependencies real inputs to its hash. Generation 2 adds `.vlt.sig` files
to that group. `crates/voltaic-cli/tests/moon_project_graph.rs` must gain the
assertion, and plan 104 owns it.

---

## 4. The compile-time budget

Generation 1 has no solver and transpiles at 5.7 MB/s in a debug build. Generation 2
adds a fixed-point solve and, on some functions, an SMT query. The budget, and the
policy that keeps it:

| Stage | Policy |
|---|---|
| type unification | linear in program size with the union-find; no budget needed |
| effect rows | at most 7 iterations per binding (doc 86 §5.4); bounded by construction |
| modes | at most 2 iterations per binding (doc 86 §6.1); bounded by construction |
| refinements | **the only unbounded stage, and the only one with a policy** |

### 4.1 The refinement policy

1. The solver runs **only** on definitions that have a refinement in scope — a
   parameter predicate, or a call to something with one. A crate with no refinements
   pays nothing.
2. Queries are **cached by predicate hash plus call-site context hash**, in the same
   cache `voltaic-driver` already keeps.
3. A query has a **timeout**, and a timeout is a **diagnostic** — `VLT0802`, naming
   the predicate and the call site. It is never a pass. Clause 5 is satisfied by the
   timeout being an answer rather than a hang.
4. `voltaic check --no-refine` skips stage 4 entirely and is what CI uses for the
   fast gate; the full gate runs it.

### 4.2 What would falsify this

A stated threshold, so the design can be wrong rather than merely optimistic:

- **Cold full-workspace solve above 10× the generation-1 transpile time** is a
  design failure, not a tuning problem. That is 89 ms of solve against 8.9 ms of
  transpile over the example corpus.
- **A refinement cache hit rate below 80%** on a warm incremental edit means the
  context hash is too fine and stage 4 is being re-run for nothing.
- **Any fuzz input that makes any engine exceed its iteration bound** is a clause-5
  violation and blocks the generation.

`crates/voltaic-bench` gains these three axes, and `benchmarks/REPORT.md` reports
them. Per plan 44's standing decision, they are **reported, not asserted** — a
timing in the gate is a flaky gate — except the third, which is a bound rather than
a timing and therefore *is* asserted.

---

## 5. Testing architecture

Generation 1's five corpora carry forward with the extension rename. Three are added,
one per new failure mode.

| Corpus | Asserts |
|---|---|
| `examples/*.vlt` | the CI-checked source of truth for what works |
| `tests/golden/*.vrs` | the generated Rust, as exact text |
| `crates/voltaic-cli/tests/ui/*.vlt` | the diagnostic a bad program gets |
| `tests/unreachable/*.vlt` | the remaining gaps, asserted to **still fail** |
| `benchmarks/pairs/*.vlt` | generated and hand-written Rust, as identical IR |
| **`tests/solve/*.vlt` + `*.sig`** | **the judgement inferred for every definition** |
| **`tests/refine/*.vlt`** | **a predicate violation is rejected, with the right code** |
| **`tests/explain/*.vlt` + `*.why`** | **`--why`'s output for a mode and a row** |

`tests/solve/` is the important one. It is the corpus that makes inference
*reviewable*: an expectation file holds the judgement of every definition, blessed
with `VOLTAIC_UPDATE=1`, so a change to the solver that silently changes a borrow
shows up as a diff. Without it the solver is the one part of the compiler whose
behaviour has no golden corpus, which is exactly the part that most needs one.

All three are blessable, and blessing rewrites the expectation and then **fails the
run**, as generation 1's three corpora do, so a changed expectation is always
reviewed rather than absorbed into a green build.

### 5.1 The zero-cost gate still holds, and gets harder

`crates/voltaic-bench/tests/zero_cost.rs` asserts that generated and hand-written
Rust produce **identical optimised LLVM IR**. Generation 2 keeps it, and it now
carries more weight: it is the standing proof that mode inference chose the borrow a
competent Rust author would have written. A mode solve that inserts a needless
indirection fails this gate, which is the only automatic check that the inference is
not merely *correct* but *good*.

---

## 6. The three risks, and what answers each

Doc 86 §8 lists the mechanisms. This section states the risks as risks.

### 6.1 Diagnostics degrade when nothing is written down

This is the known failure mode — Haskell's and Scala's — and it is the one that
decides whether the generation is usable. A type error in a program with no
signatures has nowhere to anchor: the solver reports a conflict at the point the
constraint store became unsatisfiable, which is frequently nowhere near the mistake.

Four answers, and `docs/99 - 101 - Diagnostics Without Signatures.md` is the plan:

1. **Constraint provenance is mandatory** (§1.2) — every constraint carries the span
   and reason that created it, so a conflict can name both sides.
2. **Report at the definition boundary by default.** A conflict inside a definition
   is reported against that definition's inferred signature and the use that
   contradicted it, not against the innermost expression.
3. **`--why` for any judgement**, so an author can interrogate rather than guess.
4. **`tests/ui/` grows to match.** Generation 1 has 55 UI fixtures. Generation 2's
   four new code ranges need their own, and a range without fixtures is not shipped.

### 6.2 Compile time

§4, with stated falsification thresholds.

### 6.3 Separate compilation and API stability

§3.2 and the sidecar. The honest residual risk: an inferred `pub` type can narrow
without the author noticing, and `voltaic check --semver` is a tool rather than a
guarantee. `--require-signatures` exists for crates that want the stricter
discipline, and the compiler's own crates **turn it on** — the project dogfoods the
strict mode on its published surface and the relaxed mode everywhere else.

---

## 7. The bootstrap, in one paragraph

Every crate is written in Voltaic, so a surface change this large cannot be
self-applied: `seed/voltaic1.wasm` cannot parse a single `.vlt` file. The ordering
that works is the only one available — **write generation 2's front end in generation
1**, compile it with the existing seed, use the result to build a generation-2 seed,
then migrate the crates. At no point is the only compiler for a language written in
that language. Plan 68 proved within an hour what happens when that invariant
breaks; `docs/100 - 102 - The Seed Across the Turn.md` is the plan.

---

## 8. Code architecture deltas

`docs/06 - Voltaic Code Architecture.md` holds the rules and they are unchanged in
substance. Three consequences of generation 2 that touch it:

1. **`voltaic-lattice` is a fourteenth crate** and its layout (§1.2) satisfies the
   module-tree rule: `lib.vlt` holds declarations only, four grouping directories
   named for domain concepts (`type`, `row`, `mode`, `refine`), nothing deeper than
   three levels, no bucket directories.
2. **The 1,500-line cap applies to the generated `.rs`**, which runs about 1.6 lines
   per line of source. The solve loop is the file most at risk; `constraint.vlt` is
   budgeted at 900 source lines and splits into `constraint/` if it exceeds it.
3. **The two-enums-sharing-a-variant-name constraint still holds** — every `data`
   emits `use <Enum>::*`, so two enums with a common variant name cannot share a
   file. `row/` and `mode/` both want a `Join` variant, and they are in different
   files for that reason rather than by taste.

---

## 9. What this blueprint does not decide

- **The SMT solver pin.** `docs/COMPILER.md` owns toolchain decisions and plan 97
  amends it. The requirement here is only that the solver be vendorable, offline, and
  have a stable QF_ALIA front end.
- **Whether `--require-signatures` becomes the default in a later generation.** It is
  a policy question that wants a year of use, not an argument.
- **The `with M` block modifier** that would recover generation 1's `do` over user
  types. Doc 87 §22.2 names the venue; `docs/93 - 95 - Effect Rows.md` §6 holds the
  sketch. It is deliberately out of this generation: shipping an effect row and a
  user-monad construct together would mean two answers to one question on day one.

---

## 10. Citations

- `docs/86 - Voltaic 2.md` — the language.
- `docs/87 - Voltaic 2 Surface Design.md` — the surface.
- `docs/02 - Voltaic Update Blueprint.md` — generation 1's architecture; §12.4 is the
  line plan 92 reverses, and it is not edited.
- `docs/06 - Voltaic Code Architecture.md` — the file and module rules, unchanged.
- `docs/89 - Voltaic 2 Plan of Plans.md` — the register for plans 92–104.

# Plan 99 — The Surface Contraction

Plan ID: `contraction`
Depends on: 95, 96, 98
Milestone: 16 — *the surface*
Status: `planned` — awaiting approval; §4 is nine waves

---

## 1. What this plan is

The largest row in either register. Generation 1's surface becomes generation 2's: 74
keywords become 11, signature lines are deleted, four operators go, the application
order is fixed, fields are juxtaposed, derives are computed, imports are deleted, and
three of the eight unreachable Rust constructs become reachable.

Rows 94–98 built a solver that nothing consumes. This row is where the emitter starts
consuming it and the language the author types changes.

### 1.1 Why one row and not nine

A surface is not a feature. Splitting this into nine rows means shipping, nine times,
a language that is part of each generation — and `docs/04 - Voltaic Plan of Plans.md`
records what that costs: plan 13 was one plan that crossed nineteen register rows,
moving eight to `done`, three to `superseded` and eight to `partial`, and the
`partial` status exists in the legend *because of it*.

So it is one row, decomposed into nine **waves** in §4. A wave is a working tree that
builds and whose gate is green; the register row stays `in-progress` across all nine.

### 1.2 The measured target

The five-program port set, hand-ported and measured 2026-11-13, comments stripped:

| | Rust | Voltaic 1 | Voltaic 2 | V2 vs V1 |
|---|---:|---:|---:|---:|
| guards | 231 t | 195 t | 99 t | 1.97× |
| class | 295 t | 247 t | 137 t | 1.80× |
| fallible | 764 t | 670 t | 235 t | 2.85× |
| errors | 416 t | 217 t | 109 t | 1.99× |
| comprehension | 324 t | 219 t | 110 t | 1.99× |
| **total** | **2,030 t** | **1,548 t** | **690 t** | **2.24×** |

Against Rust: generation 1 is **1.31×**, generation 2 is **2.94×**.

One caveat, measured rather than assumed: generation 2 writes `say x` where generation
1 writes `println! \-> "#{x}"`, and that is a *library* difference. Crediting
generation 1 with `say` and recounting gives **1.76× in tokens and 1.75× in
characters** — the grammar-and-solver figure. The character column is the one that
matters, because doc 86 §1.1's finding is that generation 1's character ratio against
its own output is 1.002×.

**Acceptance is measured against the 1.75× figure, not the 2.24× one** (§5.2). A row
that claimed the library's share of the win would be rounding up, which doc `STATUS.md`
forbids in exactly those words: *none of this is rounded up*.

### 1.3 The nine deletions

| | Deleted | Replaced by | Owner wave |
|---|---|---|---|
| 1 | standalone signature lines | inference (row 94) | 1 |
| 2 | 63 keywords | the guard ladder, comprehensions, the row, the manifest | 2, 4, 6 |
| 3 | `\|>`, `>>>`, `<<<`, `$` | `.` chains — one line, see below | 3 |
| 4 | receiver-last `.f x` | receiver-first `x.f` (UFCS) | 3 |
| 5 | `name : Type` in declarations | `name Type` juxtaposition | 5 |
| 6 | `deriving (…)`, `newtype`, `errors` | computed derives, `data` | 5 |
| 7 | `import`, `export`, `module`, `mod`, `use`, `island` | manifest resolution and `pub` | 6 |
| 8 | `try`, `catch`, `?`, `??`, `async`, `await`, `do` | the effect row (row 95) | 7 |
| 9 | `&`, `&mut`, `'a`, `.clone()` | mode inference (row 96) | 8 |

Wave 9 is the three gaps that close.

#### Deletion 3 works, bar the words "across lines"

Row 103's migrator built the transform, gated it to byte-identical Rust across 73
of the 74 files that hold a `|>`, and applied it. **The result reads better than
what it replaced** — `self.cur |> .peek |> .expect "…"` becomes
`self.cur.peek ().expect "…"` — so the deletion is executable. One clause of its
replacement is not.

**A chain cannot break across lines.** This is `VLT0103`:

```
two h = (h.s.to_string ())
  .len ()
```

A `|>` leads its continuation line with an explicit operator, so the lexer knows
the line continues — `lex.vlt`'s `next_line_continues` holds the list, and it is
`>>>`, `<<<`, `|>`. A bare `.` has no marker, the off-side rule ends the
expression at the newline, and the continuation parses as a receiver-last
application of its own. So the phrase *"across lines"* in the table above
describes nothing the surface admits, and the table now says *"one line"*.

**Adding `.` to that list is not a one-liner**, which is why this stays wave 3's
work rather than being done here. A continuation line beginning with `.` is
ambiguous with a receiver-last statement, and `lex.vlt` itself writes those at
block level (`.push TokenKind::Dedent (start, start) false self`). Indentation
can separate them — a continuation is deeper than the statement's block — but the
rule then has to handle a `.`-leading statement that is the *first* line of a
deeper block, where the old block's indent is the one on the stack. A one-line
chain is longer, not wrong, so this is an improvement to want rather than a
blocker.

**The zero-argument parenthesis problem is fixed, in the parser.** An earlier
version of this subsection said every zero-argument link needs a parenthesis
permanently, because juxtaposition application cannot tell a zero-argument call's
`()` from a unit first argument. The diagnosis was right; *permanently* was not.
`postfix` now declines every postfix on a **unit literal** — `().name`, `()?`,
`()[i]` and `(){…}` are nonsense in every program — and `application` continues
its own postfix chain, without which the first rule breaks `x.ok ()?`. Both are
type-free facts, so neither crosses §10.36's restriction, and the change is
**byte-neutral over the whole corpus**.

What remains is one parenthesis per link that *carries* arguments:
`(xs.iter ()).map (\a -> …)` must be bracketed before `.collect`, because the
`.collect` would otherwise attach to the lambda's closing paren and `(g x).y` is
a legitimate expression. **That** is what **deletion 4's** classification would
remove — `a.f` meaning a call when `f` is a method lets a chain compose with
nothing between the links. It works today for declared methods and not for `std`
(`h.twice.to_string` emits `h.twice().to_string`), and doc 86 §4.3 clause 2 keeps
`std` outside Lattice's environment deliberately. So deletions 3 and 4 are
**related but separable**: 3 lands without 4, and 4 is what makes the last
parentheses go.

Four gates in `crates/voltaic-cli/tests/pipeline_pass.rs` hold these
measurements: `a_zero_argument_chain_composes_without_parentheses`,
`a_dot_with_arguments_still_needs_its_parenthesis`,
`a_dot_chain_cannot_break_across_lines` and
`a_declared_method_classifies_and_a_std_method_does_not`. The last two are
written so that **failing is the good outcome**. Doc 101 §10.14 is the full
account, including the two opposite misreadings it took to get here.

### 1.4 Eleven keywords, and the count is a correction

```
data  class  where  pub  mut  self  test  loop  break  rust  else
```

The design dossier that argued for this generation said **ten**, and listed ten,
omitting `break`. `break` is not optional: `loop` without it is a non-terminating
construct, and `cursor.vlt`'s `eat_while` needs both.

The count is eleven. Doc 86 §9.1 records the correction in place, per doc 04 rule 2,
and it is repeated here because a keyword count is the kind of number that gets quoted
from the wrong document.

### 1.5 What does not change

Doc 87 §20 is the list, and it is long: the off-side rule, pattern rows, guard
ladders, `where` groups, sections, holes, pattern synonyms, optics, `:=`, operator
declaration, attributes, tests, comments-as-trivia, and the output being ordinary
reviewable Rust.

This matters for the waves: a wave that touches something on that list has gone
outside its scope.

---

## 2. Preconditions, measured

### 2.1 The corpus this row rewrites

| Corpus | Files | Rewritten by |
|---|---:|---|
| `examples/*.vlt` | 44 | this row, wave by wave |
| `tests/golden/*.vrs` | 43 | re-blessed, every wave |
| `crates/voltaic-cli/tests/ui/*.vlt` | 55 | this row; many diagnostics stop existing |
| `tests/unreachable/*.vlt` | 7 | **three are deleted** (wave 9) |
| `benchmarks/pairs/*.vlt` | 10 | this row; the zero-cost gate must stay green |
| `crates/*/vlt/**` | 122 files, 11,668 code lines | **row 103**, not this one |

The last line is the important one. **This row does not migrate the compiler's own
source.** It changes what the compiler accepts; row 103 changes what the compiler is
written in. Doing both at once would mean editing a compiler in a language that
compiler does not yet accept.

### 2.2 The bootstrap exposure

Every crate is written in Voltaic, so the moment this row changes the accepted surface
the seed can no longer compile the tree — except that the tree is still Voltaic 1, and
the seed still compiles Voltaic 1. The surface the *compiler is written in* and the
surface the *compiler accepts* diverge for the duration of this row and row 102, and
that divergence is deliberate: it is the only ordering in which the invariant holds
that at no point is the only compiler for a language written in that language.

Doc 88 §7 is the paragraph; row 102 is the plan; plan 68 is the precedent — it broke
the bootstrap within the hour by violating exactly this.

### 2.3 Which UI fixtures stop existing

55 fixtures assert the diagnostic a bad program gets. A diagnostic for a construct
this row deletes has nothing to assert. Counted by inspection before wave 1 starts,
and each one is resolved in one of three ways:

- **ported** — the construct survives with a new spelling;
- **deleted** — the construct is gone, and so is its error;
- **re-pointed** — the program is now wrong for a *different* reason, usually a row
  diagnostic (`VLT07xx`) or a mode diagnostic (`VLT06xx`).

The third class is the one to be careful with: a fixture that still fails, for a new
reason, with an unexamined `.stderr`, is a fixture that has stopped testing what it
was written to test.

---

## 3. Concrete proof this row targets

All 44 examples compiling and running with **byte-identical `.stdout`** to generation
1, plus the five-program port set hitting §1.2's measured figures within tolerance,
plus `benchmarks/pairs/` green on the zero-cost gate.

Byte-identical stdout across a total surface change is the strongest available
statement that the contraction preserved meaning, and it is available only because
generation 1 built run expectations as files beside their programs rather than as
string literals in tests.

---

## 4. Leaves — nine waves in dependency order

Each wave ends with a green gate and re-blessed goldens.

### Wave 1 — signature deletion

Signature lines become optional, then rejected. The emitter writes the inferred
signature. This wave alone should move the corpus by about 8.7% of its lines.

**Ordered first** because it is the only deletion whose correctness the tree can check
against an answer key: generation 1's hand-written signatures are still in the files,
so wave 1 can assert inferred-equals-written before deleting them. Row 94 §5.2 set
this up; this is where it is spent.

### Wave 2 — control flow

`if`/`else if`/`match`/`case`/`cond`/`switch`/`when`/`iff`/`unless`/`guard`/`then`
collapse into the guard ladder and pattern rows. `otherwise` becomes `_`.
`for`/`while`/`until` become comprehensions, iterator methods, and `loop`/`break`.

### Wave 3 — application order

Receiver-first dots; `|>`, `>>>`, `<<<`, `$` deleted; chains continue across lines on a
leading `.`. The largest reader-facing change in the generation and the one most
likely to surface parser ambiguities, which is why it is early.

### Wave 4 — the keyword cull

Everything in §1.3 row 2 not already removed by waves 2 and 3. Ends with an assertion
that `voltaic-lexer`'s keyword set has exactly eleven members.

### Wave 5 — declarations

Juxtaposed fields; `newtype` and `errors` fold into `data`; derives computed;
`data !Copy` opt-out; error display strings.

### Wave 6 — modules

`import`/`export`/`module`/`mod`/`use`/`island` deleted; manifest resolution; `pub`;
`VLT0501` on ambiguity. Expected to delete about 18% of the compiler's eventual source
— plan 91 measured 2,358 lines of `import` and `module` ceremony — though that saving
lands in row 103, not here.

### Wave 7 — effects on the surface

`try`/`catch`/`?`/`??`/`async`/`await`/`do` deleted; `else`, `.must`, `~` wired to row
95's engine. `examples/fallible.vlt` and `examples/bind.vlt` are this wave's targets,
and `bind.vlt` is the one that cannot fully port — doc 86 §5.6.

### Wave 8 — modes on the surface

`&`, `&mut`, lifetimes and `.clone()` deleted; `@` ascription wired to row 96.
`benchmarks/pairs/` and the zero-cost gate are this wave's gate, not just its test.

### Wave 9 — the three gaps

Or-patterns, `as` casts, and `ref`/`ref mut` patterns become reachable. Their three
fixtures leave `tests/unreachable/`, which **breaks the test that asserts they still
fail** — deliberately, and that is how a closed gap is recorded. `docs/GRAMMAR.md` is
regraded by row 104, not here.

> **Wave 9 is now blocked by row 103 having run first, and this is the ordering
> §4 meant by "nine waves in dependency order".** Recorded 2026-11-14, after
> attempting the `as` cast out of order and reverting it.
>
> Wave 9 is the only **additive** wave — three constructs *become reachable*. An
> addition the seed cannot parse triggers `AGENTS.md`'s version-skew rule, *"a
> surface row that the seed cannot parse must refresh the seed in the same
> commit"*. After row 103's migration there is no legal refresh: the tree is
> written in generation 2, so rebuilding `voltaic-g2.wasm` from it is
> `built by: g2`, which `blob.rs` refuses and doc 100 §1.2 forbids. The only
> conforming route is a **generation 3** — a `voltaic-g3.wasm` built by the
> generation-2 tree, with crates migrating again.
>
> So the two rules conflict for an addition, and the conflict is created by the
> ordering rather than by either rule. Row 103 *depends on* row 99 in the
> register, and this is what that dependency was protecting.
>
> **Waves 4, 6, 7 and 8 are unaffected**, because they are *deletions*. A frozen
> blob that accepts a superset still parses a `.vlt` using less of it, so the
> compiler's source can shed constructs without the seed knowing.
>
> Measured rather than reasoned: the `as` cast was implemented end to end — a
> contextual postfix costing no keyword, `n as u8` and `(n as u8) as u64`
> emitting correctly, `examples/cast.stdout` showing 44 against 300 — and the
> only thing that broke was `the_g2_blob_and_the_tree_agree_on_the_examples`.
> The bootstrap itself stayed sound: `every_ported_crate`, `stage1_equals_stage2`
> and `a_clone_without_the_generated_rust_bootstraps_from_the_seed` all passed,
> because the seed only has to read `crates/*/vlt/`. It was reverted anyway,
> because shipping it meant either weakening that gate or violating §1.2 — and
> the gate is right.

---

## 5. Acceptance criteria

1. All 44 examples compile, run, and produce byte-identical `.stdout` to generation 1.
2. The five-program port set reaches **at least 1.70× in characters and 1.70× in
   tokens** against generation 1 with the print library equalised (§1.2). The measured
   design figure is 1.75×/1.76×; the criterion is set just below it so that a small
   loss is a pass and a large one is a failure to explain in §10.
3. `voltaic-lexer`'s keyword set has exactly **eleven** members, asserted.
4. `benchmarks/pairs/` green on the zero-cost gate, all ten.
5. Every one of the 55 UI fixtures is explicitly ported, deleted, or re-pointed, with
   the re-pointed ones' `.stderr` **read** rather than blessed (§2.3).
6. Three fixtures leave `tests/unreachable/` and the assertion that they fail is
   updated in the same wave.
7. `construct_coverage` green with an **empty** `ALLOW` list — generation 1's standing
   proof that no construct reads as one thing and emits another.
8. The compiler's own source is untouched. `crates/*/vlt/**` has no diff from this row.

---

## 6. Out of scope, and what that discloses

- **The compiler's own source.** Row 103. §2.1.
- **The seed.** Row 102.
- **`GRAMMAR.md`, `DECLINED.md`, `CONFORMANCE.md`, `STATUS.md`.** Row 104, because a
  gated reference document cannot be written ahead of the code it grades.
- **`examples/bind.vlt` cannot fully port.** Its `Tally` case is a user monad and doc
  86 §5.6 records the loss. The example is ported to the explicit `.and_then` chain
  and its three `do`-only functions are deleted, with a note in §10 naming them. This
  row will therefore reduce the example corpus's demonstrated expressiveness in one
  place, and that is disclosed here rather than noticed later.
- **`examples/template.vlt`** may not port at all — row 98 §6.

---

## 7. Gate

The standing gate at every wave boundary, plus: the eleven-keyword assertion, the
zero-cost gate, `construct_coverage` with an empty `ALLOW`, the terseness measurement
in §5.2, and `tests/solve/` — which by wave 8 is the corpus doing the most work,
because by then nothing in the source states a type or a borrow.

---

## 8. Emerald reuse

One lesson, already paid for and collected here: **every fixture is a file.** Emerald's
one syntax cutover missed 113 fixtures that lived as string literals inside test
modules. This row is a far larger cutover than Emerald's and it reaches every fixture,
because `no_literal_expectations.rs` has been enforcing the rule since plan 08.

---

## 9. Decision log

| Decision | Why |
|---|---|
| One row, nine waves | a surface is not a feature; plan 13 is the precedent and `partial` exists because of it |
| Wave 1 is signature deletion | the only deletion with an answer key in the tree |
| Wave 3 early | receiver-first is the change most likely to surface parser ambiguity |
| Wave 8 gated by zero-cost | the only automatic check that mode inference is *good* and not merely correct |
| Compiler source untouched | editing a compiler in a language it does not yet accept |
| Accept the 1.70× criterion, not 2.24× | the library's share of the win is not this row's to claim |
| Re-pointed UI fixtures must be read | a fixture that fails for a new reason has stopped testing what it was written for |
| Disclose the `bind.vlt` regression here | the corpus loses demonstrated expressiveness in one place and that should be in the plan, not the postmortem |

---

## 10. Result

In progress. **Wave 1 is complete**: the signature is optional, the emitter writes
the inferred one, and the answer key that licensed the deletion is gated at
1,165/1,165. Waves 2–9 are open.

### 10.1 Inferred equals written, 809 out of 809

| Corpus | Written signatures | Inferred equals written |
|---|---:|---:|
| `examples/` (44 programs) | 231 | **231** |
| `crates/*/vlt/` (123 files) | 934 | **934** |
| | **1,165** | **1,165** |

**These were 215 and 594 when this section was first written**, and the
difference is not growth — it is 330 definitions the gate had been skipping.
`signature_type` returned `None` for every method with a `self` receiver, because
it required a written type on each parameter and `self` has none. So a fully
annotated `skip_layout : &mut self -> ()` read as unannotated and was never
compared. 809/809 was a true statement about a set a third smaller than it
appeared to describe. Found by row 100 leaf 5, whose account is doc 98 §10.7;
`the_compared_set_has_not_shrunk` now puts a floor under both counts.

`voltaic sig --verify` is the comparison and
`crates/voltaic-cli/tests/inferred.rs` is the gate that keeps it true. The gate
matters more than the number: once wave 1 deletes the written signatures there is
nothing left to compare against, so the comparison has to be *gated* before the
deletion rather than run once beside it.

`examples/lexical.vlt` is excluded because it is a lexical catalogue rather than a
program and does not parse by design. A test asserts that it still does not parse,
so the exclusion cannot quietly widen to cover a file that stopped parsing for a
real reason.

### 10.5 Wave 1 is complete: the signature is optional and the emitter writes it

`examples/inferred.vlt` is the proof, and it is in the CI-checked corpus:

```
scaled : u64 -> u64
scaled n = n * 2

doubled n = scaled n

quadrupled n = doubled (doubled n)
```

emits

```rust
fn doubled(n: u64) -> u64 { scaled(n) }
fn quadrupled(n: u64) -> u64 { doubled(doubled(n)) }
```

The pipeline is now **PARSE → SOLVE → ANNOTATE → EMIT**. ANNOTATE
(`voltaic-driver/vlt/annotate.vlt`) writes the inferred judgement back into the
AST, so `voltaic-emit` stays a pure AST-to-Rust function and the source map,
the goldens and everything downstream are untouched. The alternative — teaching
the emitter to call the solver — would have made emission depend on inference
and given two crates a reason to disagree about one answer.

### 10.6 `ty = None` on a parameter already meant something, and it is a trap

`voltaic-emit` reads an untyped parameter as **the receiver**: it moves it to
position 0 and emits it without a type. That is how `self` works and it predates
this row. So an unsigned definition that reaches the emitter unannotated **does
not fail** — it silently emits `fn across(p) -> f64`, with `p` in the receiver
slot.

`ret = None` is therefore the discriminator for "unsigned", because a signed
declaration always has a return type. And the rule *"annotate before you emit"*
needed a gate rather than a convention, which the evidence settled twice over:

- `crates/voltaic-fuzz/vlt/target.vlt` emitted without annotating, and
  `the_generated_rust_always_parses` went red the moment `examples/inferred.vlt`
  entered the fuzz corpus. Four call sites, one missed.
- `crates/voltaic-cli/tests/annotate_before_emit.rs`, written in response, then
  found a **fifth** site nobody had looked at — `voltaic-mcp/vlt/tools.vlt`.

### 10.7 A call to a sibling contributed nothing, which only mattered once

`lookup` saw a clause's own parameters and handed back a fresh hole for every
other name, so `doubled n = scaled n` inferred `?3 -> ?6`. That was invisible
while every definition carried a signature — the types came from the annotations
either way — and it is the whole game once they do not.

`written_globals` seeds the environment with the types the programmer *wrote*,
and `grown_globals` then grows it to a least fixed point: each round solves, adds
back every definition whose judgement came out **hole-free**, and re-infers.
`quadrupled` needs the second round, because `doubled` is itself unsigned.

Two details are load-bearing:

- **Only hole-free types are added.** A partly resolved type in the table would
  make the next round depend on how far the previous one happened to get, which
  is how inference stops being a function of its input.
- **Methods are excluded.** `grown_globals` first added every definition,
  instance members included, and `Store::push` then shadowed every `.push` in
  the crate — including `Vec::push`, which is not a definition at all. A method
  is not in scope by its bare name. The answer key caught it as one disagreement
  out of 934.

### 10.8 What is *not* inferable, and why that is Rust's answer too

`across p = p.x` is refused. A field access on an unannotated binder does not
determine the receiver's type, and rustc cannot infer it either — the signature
there is load-bearing rather than ceremony. `VLT0128` says so and names the
command that explains the chain:

```
error[VLT0128]: cannot infer a signature for `across`
  --> signature_not_inferred.vlt:8:1
  |
8 | across p = p.x
  | ^^^^^^^^^^^^^^
  = help: Lattice reached `?0 -> ?2`, which has no Rust spelling. Write the
    signature, or annotate a parameter so inference can reach one.
    `voltaic sig --why across` shows the chain.
```

This is an **error**, not a lint, and the distinction from row 101 §10.1 is
exact: there, rustc is the authority and can overrule a judgement about a signed
program. Here there is nothing to overrule — the alternative to refusing is
emitting a parameter in the receiver slot.

`write_type` is the refusal's mechanism, and it refuses two things `Display`
happily prints: a **hole**, which has no Rust spelling, and an **arrow**, which
Rust writes three incompatible ways (`impl Fn`, `fn`, `&dyn Fn`) and Lattice does
not choose between. A refusal anywhere inside refuses the whole type, because a
partly written type would not compile.

### 10.9 Three diagnostics changed, and one was deleted

- **Deleted: "this equation defines X, but the signature above it is for Y."**
  The program it reported — `add : i32 -> i32` then `subtract a = a` — is now a
  signature for `add` and an unsigned definition of `subtract`. §2.3's *deleted*
  category, exactly as written.
- **Added: a signature with no equations.** This replaces it and catches strictly
  more: the old message fired only when a mistyped name happened to follow a
  signature; this fires on the missing body however the file continues.
- **Kept: `VLT0106`, the module-scope binding.** `main =` with an indented body
  is a nullary function; `main = 1` is a `let` at module scope, which Rust
  forbids. Making the signature optional must not make that legal, and the
  distinction is the newline.

Three tests that used "a definition with no signature" as their example of a
*malformed item* had to be repointed, which is worth noting as a pattern: a test
whose fixture is "something invalid" silently stops testing its axis when the
thing becomes valid. All three were about recovery, not about signatures.

### 10.10 The answer key after the widening

| Corpus | Written signatures | Congruent with inferred |
|---|---:|---:|
| `examples/` | 231 | **231** |
| `crates/*/vlt/` | 934 | **934** |
| | **1,165** | **1,165** |

Comparison is now **congruence** rather than equality, for one narrow reason:
Lattice has no module resolver — imports are wave 6 — so a type written
`voltaic_diag::Diagnostic` and the same type inferred from a call returning the
imported `Diagnostic` compared unequal while naming one type. A qualified name
is congruent with **its own final segment** and nothing else; `a::T` and `b::T`
stay distinct. Nothing is rewritten, so no diagnostic or sidecar line ever shows
a name the author did not write.

### 10.11 Wave 9's three gaps all depend on earlier waves, which §4 had right

Wave 9 reads as additive — three constructs *become reachable* — so it looked
like the one wave that could be taken out of order. It cannot, and each reason is
a different earlier wave:

| Gap | Blocked on | Why |
|---|---|---|
| or-pattern `\|` | **wave 2** | `1 \| 2` currently means `1` guarded by `2`, unambiguously. The guard ladder has to replace guards before `\|` is free in a pattern. |
| `as` cast | **wave 6** | `as` is reserved for `import … as`, and wave 6 deletes imports. |
| `ref` / `ref mut` | **wave 4** | `ref` is not a reserved word, and reserving it is what the keyword cull decides. The fixture argues against reserving it at all. |

§4 calls the waves "nine waves in dependency order" and this is the check that it
meant it.

### 10.12 One of the three gaps was already closed, and had been for four plans

`tests/unreachable/or_pattern.vlt` said:

> The nearest working thing is separate arms, which match the same values. What
> has no spelling is Rust's *binding* case, where one arm body serves several
> patterns.

One arm body serving several patterns **is** spelled, as `or`:

```
13 or 14 or 15 -> "young"
```

That is `examples/case.vlt:12`, witnessed and green, emitting
`13 | 14 | 15 => "young"`. The capability was never missing; the *punctuation*
was, and the punctuation is a strictly smaller gap.

The fixture has been rescoped to say so and to cite its witness. Nothing went red
for four plans because the fixture fails either way — `1 | 2` still parses as a
guard and still fails at rustc — so the corpus was asserting the right outcome
for the wrong reason. That is the two traps `AGENTS.md` records, together: *a doc
comment is not an oracle for its own code*, and *"I could not find it" is not "it
does not exist"*.

### 10.13 Waves 2–8 share one coordination problem, and it is sharper than §2.2 says

§2.2 says the surface the compiler is *written in* and the surface it *accepts*
diverge for this row's duration, and that the divergence is deliberate. Measured
against the tree as it now stands, the divergence has a specific consequence §2.2
does not name:

**every gate that parses the compiler's own source with the current compiler
breaks the moment the accepted surface changes.** That is not one test. It is
`inferred.rs` (which parses all 123 files), `no_comments.rs`, `port.rs`,
`directory_structure.rs`, `line_cap.rs`, `construct_coverage.rs` and
`grammar_inventory.rs` — and the count grew by one this session, because
`inferred.rs` is new.

The tree itself keeps building, because `bootstrap.sh` generates `src/` with
`seed/voltaic1.wasm`, which is generation 1 and stays generation 1. What breaks is
the *gates*, which use the freshly built compiler to read generation-1 source.

So wave 2 cannot be a parser change plus a corpus re-bless. It needs a decision
about which parser each gate uses — the seed's or the tree's — and that decision
belongs with row 102, which owns generation-tagged seeds. Recording it here
because the discovery is this row's, and because the next session should not
re-derive it.

**Resolved in §10.22**, and the decision went the third way: neither parser. The
gates **defer** for the duration, gated on one constant, because a
generation-switched parser is the dual implementation `AGENTS.md` already refused
for this class of problem.

### 10.14 Wave 2 is half the size §4 states: six of its eleven constructs were never keywords

§4's wave 2 reads:

> `if`/`else if`/`match`/`case`/`cond`/`switch`/`when`/`iff`/`unless`/`guard`/`then`
> collapse into the guard ladder and pattern rows.

Measured against `crates/voltaic-lexer/vlt/keyword.vlt`, **six of those eleven are
not keywords and never were**: `match`, `cond`, `switch`, `iff`, `unless`, `then`.
Five are: `if`, `else`, `case`, `when`, `guard`.

`keyword.vlt`'s own `leaves_ordinary_identifiers_alone` test already listed
`iff`, `cond`, `unless`, `switch` and `then` as free identifiers — so the
repository has asserted for some time that they are not keywords, while this
plan's wave list named them as constructs to delete. Nothing was wrong; the list
was written from the *languages being drawn from* rather than from Voltaic's
lexer, which is the right way to design a surface and the wrong way to size the
work of changing one.

So wave 2's real targets are `if`, `else`, `case`, `when`, `guard`, `otherwise`
and the loop set (`while`, `until`, `for`, `loop`, `in`). That is substantially
less than eleven-into-one.

### 10.15 `otherwise` → `_` is done, and it cost eight follow-throughs

The first of wave 2's real deletions. A guard ladder's catch-all arm is spelled
`_`, which is the wildcard it already is in a `case` arm, a parameter pattern and
a binding. One spelling for one meaning; the keyword set drops from 58 to 57.

The surface corpus was **three lines** — `examples/guards.vlt` twice and
`examples/testing.vlt` once — so this did not need wave 1's two-step
accept-then-reject. A deprecation window that exists for nobody is a second
spelling to carry.

What it did need was eight consequences, and they are the honest measure of what
deleting a keyword costs here:

| | |
|---|---|
| `voltaic-emit/tests/guards.rs` | two tests whose fixtures used it, and whose names said `otherwise` |
| `voltaic-fmt/vlt/item_text.vlt` | the formatter *printed* the keyword when a guard had no condition |
| `voltaic-lexer`'s round-trip test | the word moves from the reserved list to the free-identifier list |
| `editors/vim/syntax/vlt.vim` | coloured it |
| `editors/vscode/syntaxes/voltaic.tmLanguage.json` | the same |
| `tree-sitter-voltaic/grammar.js` | `choice("otherwise", $._expression)` |
| `construct_coverage.rs` | the *Guards* row's witness tokens |
| `editors.rs` | `otherwise` was one of six canaries proving the keyword scan still works |

The last one is the interesting one. A canary list exists to catch a scan that has
silently stopped working, and one of its six members was a construct being
deleted — so the deletion broke the test that watches the tests. `newtype`
replaced it, and **not** `guard`, for a reason worth keeping: `grammar.js` also
has `field("guard", …)`, and the extractor skips any literal that is also a field
name, so `guard` is not in the derived set at all.

### 10.16 The retired spelling gets a diagnostic, because rustc's answer was right and useless

Deleting the keyword left `otherwise` a free identifier, so `| otherwise = …`
still *parses* — as a guard whose condition is a variable of that name — and
fails at rustc:

```
E0425: cannot find value `otherwise` in this scope
```

That is rustc being the authority, which is the charter, and it is a poor
migration message: the program is not wrong about a value, it is written in the
previous spelling. `VLT0118` already exists for exactly this and already had
precedent in `item.vlt`, so:

```
error[VLT0118]: `otherwise` is no longer the catch-all arm of a guard ladder
  --> retired_otherwise.vlt:13:5
   |
13 |   | otherwise = "small"
   |     ^^^^^^^^^
  = help: write `_`, which is the same wildcard a `case` arm and a parameter
    pattern use. Row 99 wave 2 deleted the keyword: one spelling for one meaning
```

A parse error rather than a lint, because the construct is gone and the program
cannot mean anything else. `tests/ui/retired_otherwise.vlt` is the fixture.

**This is the shape every remaining wave-2 deletion should take**, and it is
cheap: the keyword's removal is three lines, and the retired-spelling diagnostic
is what turns an unrelated rustc error into an instruction.

### 10.17 `until` deleted, and the construct argued for its own removal

**58 → 55 keywords** across wave 2 so far: `otherwise`, `until`, `when`.

`until c` was `while !(c)` and nothing else — `Expr::While` carried a `negated`
flag because that is exactly how it lowered. One surface use in the whole tree,
`examples/control.vlt`.

What makes it a clean deletion rather than a judgement call is `VLT0107`.
`until` **declined a pattern test**, because `while !(let Some(v) = x)` is not
Rust and the nearest thing that compiles — `while !matches!(x, Some(_))` — binds
nothing. So the keyword meant as sugar for `while` could do **strictly less than
`while`**, and it needed a diagnostic to say so. A construct whose own error
message reads *"write `while` with the opposite pattern, or a `loop` with a
`break`"* is a construct arguing for its own deletion. `VLT0107` is retired with
it.

Three things went beyond the keyword, and each is part of the deletion rather
than tidying after it:

- **`Expr::While.negated` is gone.** With `until` deleted nothing constructs it
  as `true`, and a field that can only be false is a lie about the AST — the
  emitter and the formatter both still branched on it.
- **The parenthesisation tests became one refusal test.** They were plan 55's
  whole point: `until a && b` is `while !(a && b)`, not `while !a && b`, and
  nothing but the parentheses distinguished a correct lowering from a silently
  wrong one. The trap is now the author's to see — `examples/control.vlt` carries
  it as a comment beside the `while !(…)` it spells out — which is strictly more
  visible than a lowering that hid it.
- **`tests/ui/pattern_until.vlt` was re-pointed, not re-blessed.** §2.3 warns
  that a fixture which still fails for a *new* reason with an unexamined
  `.stderr` has stopped testing what it was written to test. It is
  `retired_until.vlt` now and its comment records both the old decline and why
  the asymmetry retired the keyword.

### 10.18 `when` was a reserved word with no construct

`Keyword::When` appeared in the lexer and in **no parser arm at all**. It had
taken an identifier from every program since generation 1 and reached nothing.

Deleting it cost no migration, because nothing was written in it — and it is the
only deletion in this wave with no corpus at all. Worth noticing how it survived:
`grammar.js` listed it, all three highlighters coloured it, and the lexer's
round-trip test asserted it round-trips. Four things agreed it was a keyword and
none of them asks what it *parses*.

`construct_coverage.rs` is the gate that would catch this class — a construct
graded reachable and witnessed by nothing — and `when` was never graded in
`docs/GRAMMAR.md`, so it was invisible to the one check designed to find it. An
*ungraded* keyword is outside every gate at once.

### 10.19 Pattern rows: wave 2's additive half, and two thirds already existed

§4 says `case` collapses into "the guard ladder and pattern rows". Measured
before writing anything, **two thirds of that was already the language**:

```
describe (Some n) = "some"
describe None     = "none"
```

lowers to a `match` and has since generation 1. `examples/case.vlt` witnesses the
`case` form. What had **no** replacement was `case` used as an *expression* in
the middle of a body, because multi-clause dispatch only dispatches on
parameters.

A guard ladder is the right home for it, and the only thing in the way was the
arm's type. `Guard.condition` was `Option<Expr>`; `if` already took a
`Condition`, which models `x is Some y` and carries `has_pattern`. Widening the
arm to `Condition` is the whole feature:

```
describe held
  | held is Some n = "some"
  | _ = "none"
```

→ `if let Some(n) = held { "some" } else { "none" }`

`emit_condition` already lowered a pattern test to `let Some(n) = x`, which is
exactly what `if` / `else if` wants, so a pattern row chains as `else if let`
with no new machinery. `examples/rows.vlt` witnesses three shapes: a row with a
catch-all, rows chained with an ordinary condition *between* them, and a binding
reaching the body. **Additive**, so nothing is deleted and no gate that parses
the compiler's own source is disturbed.

### 10.20 A pattern row's scrutinee is not a `bool`, and one obligation would have said it was

The arm used to carry one `expr_type … = bool` obligation over its whole
condition. A `Condition` is a list of operands, and a *pattern* operand's source
is **matched rather than tested** — `held is Some n` says nothing about `held`
being a boolean. Keeping the single obligation would have unified every
scrutinee with `bool`.

`Condition::sources` is the accessor the three `voltaic-lattice` consumers want,
and it is on `Condition` rather than open-coded three times for a specific
reason: each copy would have to decide independently what a pattern operand
contributes, and they would not have agreed. Two tests pin both directions — a
pattern row's scrutinee is unconstrained, and an ordinary arm still contradicts
when handed a `u64`.

### 10.21 Two things the gates caught that review would not have

**`condition`'s legacy check false-positived on the ladder's own separator.**
Plan 60 moved a pattern test off `=` onto `is`, and `condition` still reports the
old spelling helpfully — so `| spend > 10_000 = "gold"` was told *"a pattern test
is written `scrutinee is pattern`"*. `ladder_condition` is the same parse without
that check, and `condition` is now the check plus a delegation: factored rather
than copied, because the operand loop and the `||` refusal are the parts that
must not drift.

**I encoded my own taste and `generated_rust_is_rustfmt_clean` refused it.** The
single-line `if … { … } else { … }` form declined a pattern row on the grounds
that a binding is harder to read on one line. rustfmt collapses a short
`if let … else` itself, so the emitter was simply wrong and the gate said so on
the first run. The taste being encoded was mine; rustfmt's is the one that
counts, which is the whole reason that gate exists.

A third, smaller: `highlights.scm` in **two** places — `tree-sitter-voltaic` and
`editors/zed` — named `otherwise` and `until` as *node types*, so the queries
failed to load rather than merely mis-colouring. `grammar.js` itself also needed
the widened `condition`, and per `AGENTS.md`'s precedence trap it was **run** on
a pattern row rather than reasoned about: zero `ERROR` nodes.

### 10.22 §10.13's blocker is resolved: one switch, and the gates defer

§10.13 recorded the problem and left it open: the moment the accepted surface
changes, every gate that parses the compiler's own source with the freshly built
compiler breaks. That is `inferred.rs`, `no_comments.rs`, `port.rs`,
`directory_structure.rs`, `line_cap.rs`, `construct_coverage.rs`,
`grammar_inventory.rs` and `classify_corpus.rs` — the count grew by two over this
row's own work.

**`crates/voltaic-lexer/vlt/generation.vlt` holds `ACCEPTS`**, the generation the
compiler accepts, and `support::mid_turn()` is

> `ACCEPTS != "g1"` **and** some crate in `seed/GENERATION` is still marked `g1`

so the condition is *the surface is ahead of the tree*, computed from two facts
the repository already records rather than from a flag somebody remembers to set.

#### Why deferring, and not a second parser

A generation-switched parser is the obvious alternative and it is the one
`AGENTS.md` already refused for this class of problem: rustc pays for it with
`cfg(bootstrap)` double implementations and Go with a version chain, and *"both
are expensive because their seeds are releases"*. This seed is a build artifact,
so the recorded choice is two commands rather than a dual implementation.
Building the dual implementation here would be reversing that decision quietly,
in a test helper.

So the gates defer, and row 103 re-arms them as each crate crosses. That is a
real loss of coverage for the duration, and it is the loss §2.2 already accepted
when it called the divergence deliberate.

#### Why this is safe to have built before it is needed

`mid_turn()` is **`false` today and provably so**:
`the_surface_has_not_moved_yet` asserts `ACCEPTS == "g1"` *and* that `mid_turn()`
is false. Flipping the constant turns that red and says in its message what the
flip costs — so eight gates cannot begin deferring without somebody reading the
sentence that explains it.

Verified by doing it rather than by reasoning: with `ACCEPTS` set to `g2`, the
review gate failed, the four compiler-parsing tests in `inferred.rs` and
`classify_corpus.rs` deferred, and the deferral printed

```
deferred: the accepted surface is ahead of the tree's source, so this gate
cannot parse the compiler's own generation-1 files. Doc 97 §2.2 and
`support::mid_turn`; row 103 re-arms it crate by crate.
```

Then the constant was restored. **The skip is scoped, too**: within
`inferred.rs`, `the_excluded_example_is_the_lexer_catalogue` and
`both_corpora_are_the_size_the_tree_says_they_are` do not parse the compiler and
stay armed. A file-level skip would have switched off more than the turn
requires, which is how a deferral becomes a hole.

This is what waves 2–9 were waiting on. `if`, `else` and `case` can now be
deleted — 85 of the compiler's 105 files use one of them — because the gates that
would have broken have a defined behaviour for the window in which they must.

### 10.23 Wave 2's deletion list contradicts doc 87, and doc 87 is the surface

> **Withdrawn by §10.27.** This section's conclusion — that doc 87 does not
> delete `if`, `else` or `case`, and that wave 2 is therefore complete — is
> wrong. §1.4's eleven surviving keywords exclude `if` and `case`, which settles
> it the other way. The section is kept rather than deleted, per doc 04 rule 3,
> because §10.29 is about how it went wrong.

§4's wave 2 reads:

> `if`/`else if`/`match`/`case`/`cond`/`switch`/`when`/`iff`/`unless`/`guard`/`then`
> collapse into the guard ladder and pattern rows.

Doc 87 is the **normative surface** for generation 2 — this document is a plan for
reaching it — and doc 87 says the opposite twice.

**§2.2, "Pattern rows and guards, unchanged":**

> Several equations become a `match`; a `|` ladder becomes an `if`/`else if`
> chain; a ladder inside an equation lands inside that clause's arm. All exactly
> as generation 1 lowered them — **this part was already right and is carried
> over without change.** […] `otherwise` is deleted. The catch-all is `_`.

**§20, "What this surface does not change":**

> - pattern rows, guard ladders, `where` groups

So the only control-flow deletion doc 87 specifies is **`otherwise`**, and that is
done. It never deletes `if`, `else` or `case`, and it never shows an `if` in a
Voltaic code block at all — zero in the whole document.

### 10.24 The deletion `if` would need has no specified replacement

This was found by starting to build it rather than by reading, which is the part
worth recording.

A guard ladder is a **clause** form: it can be a whole function body and nothing
smaller. An `if` nested inside a loop has no ladder equivalent, so deleting `if`
requires the ladder to reach **expression** position — a construct that does not
exist. An `Expr::Ladder` variant was started here and then reverted, because
adding it would have been a surface-design decision taken in a parser, which is
the thing doc 87 exists to prevent.

Doc 87 §5 has no subsection on conditionals. §21 says:

> **Open items.** None.

That is not true while `if`'s fate is undecided. Either:

- **`if` stays**, and §4's wave 2 line is simply wrong — the reading §20 supports,
  and the one under which **wave 2 is complete**; or
- **`if` goes**, and doc 87 is missing the expression-position conditional that
  replaces it, which is a §21 open item and a surface decision for doc 87 to make
  rather than for this row to invent.

Taking the first reading until doc 87 says otherwise, because §20 is explicit and
a plan does not overrule a specification.

**Superseded by §10.30.** The first reading is the one §10.27 withdrew, and the
second is what happened: doc 87 gained §5.7 and the `Expr::Ladder` reverted here
was built against it. What stands from this subsection is the *reason* for the
revert — a surface decision does not get taken in a parser — and that reason is
why §5.7 was written before the variant was re-added rather than after.

### 10.25 What that makes wave 2

> **Superseded by §10.28.** The table below is right about the six constructs
> that were never keywords and about the three that are done; it is wrong that
> `if`, `else` and `case` are *"not doc 87's to delete"*.

| Item | Status |
|---|---|
| `otherwise` → `_` | **done** — doc 87 §2.2's one named deletion |
| `when` | **done** — a reserved word with no construct (§10.18) |
| `until` | **done** — argued for its own removal (§10.17) |
| pattern rows | **done** additively, and §20 says the ladder was already right |
| `match`, `cond`, `switch`, `iff`, `unless`, `then` | **never keywords** (§10.14) |
| `if`, `else`, `case` | **not deleted by doc 87** (§10.23) |
| `for`/`while` → comprehensions and iterator methods | doc 87 §5.4 and §5.6 **add** those; neither section deletes `for` or `while` |

Eleven named constructs: six were never keywords, three are done, and two are not
doc 87's to delete. **Wave 2 is complete on doc 87's terms.**

The general lesson is worth more than the wave. §4's nine wave descriptions were
written from the *languages being drawn from* and from an earlier reading of the
surface; doc 87 was written later and more carefully. **Every remaining wave
should be measured against doc 87 §20 and §5 before it is sized**, because two of
the three things wave 2 looked like turned out not to be there: six constructs did
not exist, and two were never scheduled for deletion.

### 10.26 Wave 3 measured against doc 87: it is specified, and it inverts `.`

Unlike wave 2, wave 3 **is** doc 87's. §5.3 deletes four operators with named
replacements, and §1 specifies the application form that replaces them:

| Spelling | Means |
|---|---|
| `f x` | `f(x)` — a free function |
| `x.f y` | `f(x, y)` — the same function, **receiver first** |

What §4's phrase "receiver-first dots" does not say is that this **inverts the
meaning of `.`**. Generation 1's own diagnostic says so in as many words —
`VLT0115`, raised on `xs.iter.map(f)`:

> method-call syntax is retired […] a method is a function whose last argument is
> its receiver, so write `.f args recv` or `recv |> .f args`. **`.` is field
> access and nothing else** (plan 57 §1.1)

So `x.f y` today is a *field access* `x.f` applied to `y`, and under doc 87 §1 it
is a *method call* `f(x, y)`. The token is not being extended; its meaning is
being replaced. `AGENTS.md` already records the near-miss version of this —
*"`(.f)` is a field section, not a method reference"* — which is the same
distinction seen from generation 1's side.

Two consequences this row should not discover at wave 3 leaf 4:

- **Doc 87 §5.1's chain example is not generation 1 syntax.** It reads
  `self.src.get(self.pos .. self.src.len).unwrap_or("")`, which is exactly the
  parenthesised method call `VLT0115` refuses. The chain form is new syntax to be
  *built*, not an existing form to be preferred.
- **Doc 87 §5.3's `>>>` replacement does not reach the position that matters
  yet.** `shout = (.trim.to_uppercase)` against `shout : &str -> String` is
  `VLT0103`, an arity disagreement, because the signature's spine takes one
  parameter and the equation binds none. A point-free *definition* needs either
  wave 1's optional signature or a rule that a chain section saturates the spine;
  neither is written down.

#### The size, measured

| Corpus | `.`-calls | Operator uses |
|---|---:|---:|
| `examples/` | 357 | 43 — `\|>` 32, `>>>` 9, `<<<` 2, `$` 0 |
| `crates/*/vlt/` | **8,479** | `\|>` in 79 of 105 files |

The second row is row 103's to migrate, not this one's — but it is the number
that makes the ordering matter. §10.22's deferral mechanism is what makes wave 3
*possible*: with `ACCEPTS = g2` the eight gates that read the compiler's own
generation-1 source stand down, and the tree keeps building because `bootstrap.sh`
runs the generation-1 seed. Without that, inverting `.` would break the build and
every gate at once.

`$` appears **zero** times in `examples/`, which is worth noting against §5.3's
paragraph justifying it at length: the operator it takes most care to explain is
the one the corpus does not use.

#### Why this row stops here

Wave 3 is one coherent change — the application rule — and §4 already calls it
*"the largest reader-facing change in the generation and the one most likely to
surface parser ambiguities"*. Landing half of it would leave the parser accepting
two incompatible readings of `.`, which is strictly worse than either. The two
gaps above are doc 87's to close first: the chain form has to be built before the
operators it replaces can go, and the point-free definition needs a stated rule.

### 10.27 §10.23 was wrong, and §1.4 is the evidence that settles it

§10.23 concluded that doc 87 does not delete `if`, `else` or `case`, and that
wave 2 was therefore complete. **That conclusion is withdrawn.** The reasoning
rested on doc 87 §20 listing *"pattern rows, guard ladders"* under what the
surface does not change, and §20 does not mean what §10.23 took it to mean.

§1.4 of this document lists the eleven keywords that survive:

```
data  class  where  pub  mut  self  test  loop  break  rust  else
```

`if` is not there. Neither is `case`, `while`, `for`, `in` or `guard`. `else`
**is** — it survives for the `… else` tail. So the eleven-keyword target does
delete `if` and `case`, and §1.3 row 2's *"63 keywords"* is the same statement
counted the other way.

The two readings reconcile, and the reconciliation is the useful part:

> §20's *"guard ladders unchanged"* is about the **clause** ladder, which is not
> changed by **adding** an expression form beside it.

An addition leaves the existing construct exactly as it was, so a ladder
expression is inside §20 rather than outside it. §10.24's revert of
`Expr::Ladder` was therefore over-cautious on that specific point.

### 10.28 What survives §10.23, and it is the thing that actually blocks wave 2

One half of §10.24 stands, and it is the half that matters:

> Doc 87 §5 has no subsection on conditionals.

That is still true, and §1.4's keyword list does not fix it. A list of surviving
keywords says `if` goes; it does not say what replaces it in expression position.
The ladder expression's **syntax** is undetermined — where it may appear, how it
interacts with a `case` arm's body, whether it needs a delimiter when nested —
and doc 87 §21 still says *"Open items. None."*

So wave 2 is **not** complete, and the blocker is narrower than §10.23 claimed:
it is one unwritten subsection of doc 87, not a contradiction between documents.
`Expr::Ladder` is the right construct and inventing its surface in a parser is
still the wrong way to get it.

What is genuinely settled by this pass, and stands:

- six of wave 2's eleven named constructs were never keywords (§10.14);
- `otherwise`, `when` and `until` are deleted (§10.15, §10.17, §10.18);
- pattern rows are added and the clause ladder is untouched (§10.19);
- `if`, `case`, `while`, `for`, `in` and `guard` are scheduled to go, and the
  first two need doc 87 to specify the expression form first.

### 10.29 The correction's own lesson

§10.23 read one list in doc 87 and did not read §1.4 of the document it was being
written into. The list it did read is normative about *change*; the list it
missed is normative about *the end state*, and they answer different questions.

`AGENTS.md` records this shape twice already — *"a doc comment is not an oracle
for its own code"* and *"a declaration read correctly still does not tell you the
behaviour, because another rule can intercept the same input"*. This is the
document-level version: **a section read correctly does not settle the question
when a second section answers it more directly.** §20 says what is carried over;
§1.4 says what is left. For *"is `if` deleted"*, §1.4 is the one being asked.

### 10.30 §5.7 is written, and the ladder reaches expression position

The blocker §10.28 named is closed. Doc 87 gained **§5.7 Conditionals**, and
`Expr::Ladder` is implemented against it: `voltaic-ast`, `value_or_ladder` in the
parser, `emit_ladder` in the emitter, the formatter's printer and precedence arm,
`ladder_expression` in `grammar.js` with three corpus cases, and
`examples/ladder.vlt` with its golden and `.stdout`.

**The premise that had this deferred was wrong, and that is the finding.** §10.24
declined to invent the construct, correctly. §10.28 then treated the specification
as someone else's to write — but doc 87 is one of the eighteen documents written in
*this* project, not part of the 01–85 record that is off-limits. A document being
a specification makes it authoritative over the parser; it does not make it
unwritable by the row that found the hole in it. The rule that applies is the one
§10.24 already stated: the surface decision goes in doc 87 first, then the parser
follows. Both halves of that were available the whole time.

**§5.7's hardest rule took three drafts, and the first two were wrong the same
way.** The construct's own shape is `_indent`, the arms, `_dedent` — specified
once and right once. Nesting is where it went wrong:

1. *A ladder nested inside an arm, indented past it.* Written from the shape of
   the construct. An arm's body is `inline_body` — one expression on one line —
   so this does not parse, and `examples/ladder.vlt` said so on the first run.
2. *A ladder in a `where` group attached to the guarded clause.* Written from
   §2.2's shape, which does have a `where`. It does not parse either, and the
   reason is worth keeping: **a `where` group cannot follow a guard list at any
   indentation.** `clause` consumes a `Dedent` to close the guard list where
   `where_locals` needs an `Indent` to open, so the guarded equation is the one
   equation shape with no `where` available to it. Both the inline-body and
   indented-block forms call `where_locals` in a position where it can fire; the
   guarded form calls it in a position where it structurally cannot.
3. *A ladder bound to a name in the enclosing block.* Parses, compiles, and runs.

The third draft is also the better rule, which is the part that makes this more
than a correction. §2.2's clause ladder **could never nest** — an arm takes one
expression and has no `where` to escape to. So admitting the ladder as a *value*
is not merely one more position for an existing construct: it is what makes
ladders nest at all, and it does so through the ordinary binding every other value
nests through. The subsection earns its place rather than tidying up after a
deletion.

What the two wrong drafts have in common is that each described a nesting read off
the shape of the construct instead of tried. `AGENTS.md` already carries the rule
for this — *"for anything involving precedence, conflicts, or a flat `repeat1` …
run the input"* — and the general form is wider than grammars: **a specification
can be wrong about the language it specifies, and the cheapest check is to write
the program.** A doc 87 subsection costs nothing to write and nothing to be wrong
in; `examples/ladder.vlt` is what made it answerable.

The asymmetry found in draft 2 is real and is **not** fixed here. A guarded
equation having no `where` is a gap in §2.2, not in §5.7 — closing it means
reworking `clause`'s indent bookkeeping and giving the emitter a place to put
`wheres` on a `Guarded` body, with its own gate surface, and §5.7's nesting needs
none of it. It is recorded for whoever wants it rather than folded into this row.

### 10.31 §5.6's `else` tail, and the second specification defect found the same way

§5.7 names the replacement for `guard … else` as *"a binding with an `else`
tail"*, pointing at §5.6. That construct did not exist either, so the deletion
§5.7 unblocked was still blocked one keyword further on. It exists now:
`Binding` carries `r#otherwise : Option<Block>`, `else_tail` parses it,
`control.vlt` lowers it to Rust's `let … else`, `else_tail` is in `grammar.js`
with two corpus cases, and `examples/else_tail.vlt` witnesses four shapes.

Three things it needed beyond the obvious, each found by running a program:

- **The inline tail cannot go through `parse_expr`.** §5.6 says *"`break` is an
  expression of type `!` … so it is a legal `else` arm and the two constructs
  compose without a rule saying they may"* — true of the language, and silent
  about this parser, where `break` and `continue` come from `control_flow` and
  `return` from `stmt`'s own branch. `else break` — §5.6's own example — does not
  parse until the tail asks all three. That is the subsection being right about
  the language and incomplete about the implementation, which is the ordinary
  case and a defect in neither.
- **A binding's pattern had to become refutable.** It never was, because
  `guard … is p else …` was the refutable form; with the keyword gone the tail has
  to carry refutability or the replacement cannot express what it replaces.
  Admitted exactly where an `else` tail follows — `reaches_an_else_tail` — so
  `Some v` with no tail is still not a binding and nothing that parsed before
  parses differently.
- **A multi-statement tail cannot be inlined.** `inline_body` joins with `\n`
  after stripping the pad, so inlining two statements puts the second at the wrong
  indentation. The formatter inlines a one-statement tail (§5.6's spelling, and
  the call `two_armed` already makes for a two-armed ladder) and keeps the block
  otherwise.

**§5.6's example was wrong, and in the same way §5.7's nesting rule was.** It read
`ch = self.peek else break`, with a plain name — which parses, emits
`let ch = self.peek() else { break; };`, and draws rustc's *"unreachable `else`
clause"*, because a name matches everything and the tail can never run. The
pattern has to be a constructor for the construct to do anything, so the example
is now `Some ch = self.peek else break`.

That is **two specification defects in two subsections, both found by writing the
program and neither findable by reading**. §5.7's was a nesting the parser rejects;
§5.6's parses and compiles and is merely pointless. The second is the worse kind: a
gate that only checks programs compile would have passed it, and `voltaic check`
reported it as a *warning*, which is exactly what §19's charter promises and
exactly what a green build swallows.

The general rule, which §10.30 stated for grammars and which is wider than that:
**a specification is a claim about a language, and the cheapest way to check a
claim is to run it.** Both subsections were written in the same sitting, by the
same reading, and both were wrong about the construct they introduced. What
separated them was not care — it was `examples/ladder.vlt` and
`examples/else_tail.vlt`.

### 10.32 Doc 87 cited a section of itself that does not exist, twice

§5.6's and §5.7's text attributed the eleven surviving keywords to *"§1.4"* of doc
87. **Doc 87 has no §1.4** — §1 runs §1.1, §1.2, then §2. The eleven are **doc 86
§9.1**, which is normative, and §1.4 of *this* document repeats them with the count
correction. Both citations are repointed.

Three things make this worth a subsection rather than a quiet fix.

**It is the same error as §5.6's and §5.7's examples, in a third form.** Those two
described constructs the parser rejects; this cites a section that is not there.
All three were written in one sitting, all three were written from recollection of
where a thing lives rather than from looking, and all three read as authoritative.
A citation is the cheapest claim in a document to make and the most expensive to
check, which is exactly the combination that produces this.

**The conclusion it was used to support is still right.** §10.27 withdrew §10.23
on the strength of *"§1.4's eleven surviving keywords exclude `if` and `case`"*,
and that is true — of doc 86 §9.1 and of §1.4 of this document, both of which list
the same eleven. So the repointing changes no conclusion, which is the trap: a
wrong citation supporting a right conclusion has nothing downstream to fail, and
survives indefinitely.

**A gate for this was measured and declined.** The obvious response is a test that
resolves every `§N.M` in docs 86+ against a heading. Three rounds of probe said
not to build it:

| | Dangling reported |
|---|---:|
| first probe | 92 distinct / 150 occurrences |
| case-insensitive, `plan`/`row` attribution added | 61 / 104 |
| inspected by hand | **1 real** (this one, ×2) |

The residue is not tunable away, because the false positives are *contextual*
references that are correct English and unresolvable text: `doc 87 §22.4 lists in
the §22 table` — the second `§22` is bare and means doc 87's; `Doc 86 §4.2–§4.4` —
the far end of a range inherits the attribution; `97 §10.13 records` — a document
named by bare number. And doc 89's headings are `## Milestone 13 —`, unnumbered, so
the probe reported every reference in it as dangling, which is the probe being
wrong about a document rather than the document being wrong.

A gate at that precision is worse than none: 60 false positives is a gate nobody
reads, and a gate nobody reads is how `:check-docs` ended up in the push gate with
514 errors in it (`AGENTS.md`). **Declined, with the numbers, so the next reading
of this does not re-derive them.** What does hold is the habit: a `§` is a claim,
so check it the way §10.30 and §10.31 say to check the others — by looking.

### 10.33 Wave 3's argument half: `x.f y` is `f(x, y)`

Doc 87 §1.2's inversion, in the half that needs no solver. `Expr::Dot
{ recv, name, args }`; `application` builds it where it used to return
`VLT0115`; the emitter writes `recv.name(args)`; the formatter prints
`recv.name args`. `examples/receiver_first.vlt` witnesses one argument, two
arguments, a parenthesised argument, a chain and a hole.

**Arguments settle field-versus-method without types, which is why this half
could land first.** A field access takes no arguments, so `n.max 3` can only be
a call — the solver is needed only for the *no-argument* case, where `"ab".len`
could be either. That half is still open; doc 101 §10.8 has what it needs, and
`MemberKind` is the query it will ask.

**`VLT0115` shrank from four spellings to one, and its help text was a lie.**
`s.len (1)`, `s.len ()` and `s.find "h"` are now the surface; only `s.len(1)` —
Rust's own, no space before the `(` — stays refused, because application here is
juxtaposition and a parenthesised argument list would make `.` mean two things
depending on whitespace. The help still said *"write `.len args recv` or
`recv |> .len args`"*, which is generation 1's receiver-last form that this wave
deletes: a diagnostic telling the author to write the retired spelling.

**Three defects, each caught by a different gate, and none by the one before it.**

- **rustc caught two missing match arms.** Adding an AST variant is exhaustively
  checked, so `voltaic-emit` and `voltaic-fmt` both failed to compile until they
  handled `Dot`. That is the charter paying for itself: the compiler's own source
  is Rust, so rustc audits a surface change to the compiler.
- **`formatting_preserves_meaning` caught a printer that produced plausible
  text.** `(n.max 3).min 8` printed as `n.max 3.min 8`, which reparses as
  `n.max (3.min) 8` — a different program. The cause was passing `u8::MAX` as the
  argument context instead of using `argument`, the helper `Call` uses. **Nothing
  but a round-trip could have caught it**: the text is well-formed, it compiles,
  and it is wrong.
- **`takes_a_postfix` and `stands_alone` needed the shape split, not widened.** A
  `Dot` with arguments is an application and needs its parentheses as a receiver
  or an argument; a `Dot` with none is a postfix access like `Field`. One arm for
  both is what dropped them. Two arms, with the args case first.

**The seed had to be refreshed in the same commit, and the gate that said so was
this session's own.** `across_the_turn.rs`'s
`the_g2_blob_and_the_tree_agree_on_the_examples` went red the moment
`receiver_first.vlt` existed, because the blob predated the inversion.
`AGENTS.md`'s rule — *a surface row the seed cannot parse must refresh the seed
in the same commit* — was enforced rather than remembered.

### 10.34 Adding an AST variant makes the workspace unbuildable, and the port cannot fix it

Worth its own subsection because the recovery is not the obvious one.

`Expr::Dot` lands in `voltaic-ast`, and every crate matching exhaustively on
`Expr` stops compiling until it is regenerated too. The port regenerates in
dependency order and stages each crate, compiling before it writes — so it is
built for exactly this. But `cargo test -p voltaic-cli --test port` has to **build
`voltaic-cli` first**, and `voltaic-cli` depends on the half-regenerated tree. So
the port cannot run, and the tool that exists to move the tree forward is locked
out by the state it exists to resolve.

**`./bootstrap.sh` is the way through**, because it needs no working workspace:
`seed/voltaic-g1.wasm` under `wasmtime` regenerates all fourteen crates in one
pass from the `.vlt`, and the seed parses generation-1 source regardless of what
the tree's Rust currently looks like. That is the seed earning its keep for a
reason plan 72 did not list — not a clean clone, but a mid-change tree.

So the loop for an AST change is **edit `.vlt` → `./bootstrap.sh` → `cargo build`**,
and `VOLTAIC_UPDATE=1 cargo test --test port` only once the workspace compiles
again.

**And the silence that hid it was a filtered grep.** `cargo build 2>&1 | grep -E
"^error"` reported nothing while the build was failing, because cargo colourises
`error` and the line starts with an ANSI escape. `AGENTS.md` records that trap
verbatim — *"do not grep compiler output for `^error` — it reads as green when it
is not"* — and it was walked into anyway, twice in one session, because a filter
written to keep output short is invisible once it is written. **Check exit codes.**

### 10.35 The no-argument half's missing primitive, and the two routes left

`voltaic_lattice::receiver_owner (&store) (&outcome) span` answers the one
question wave 3's remaining half needs: *at this span, what type is the
receiver?* With `MemberKind` from doc 101 §10.8 beside it, the pair is
`is_field (receiver_owner …) name` — which is doc 87 §2.2's *"resolved by the
solver, which knows whether `name` is a field or a method"*, now actually
answerable.

**Keyed by span**, because that is the only thing an `Expr::Field` node and a
`Member` obligation have in common: the obligation carries the provenance of the
access it came from, so the AST and the constraint store meet there and nowhere
else.

**Three ignorances collapse into one `None`, and that is the design.** No
obligation at the span; a receiver that never resolved past a hole; a resolved
type with no owner. All three mean *do not claim it is a field* — an authority
that guesses where it does not know is worse than one that declines, and the
caller's correct response to all three is to leave the node as it was. Two of
the four tests exist only to pin that: a span with no obligation, and a receiver
that is still a hole.

`owner_of` was exported rather than copied, because seeing through `&` and
`&mut` to the owning type is the kind of two-line rule that drifts the moment it
exists twice.

**What remains, and the two routes are not equal.** The rewrite needs to reach
every `Expr::Field` with no arguments and turn it into a `Dot`. Either:

- **a `&mut Expr` walker in the driver**, run after `annotate`'s solve — which
  means a match over all ~50 `Expr` variants, none of which exists today
  (`annotate` walks items and methods, never into expressions); or
- **thread the resolution into the emitter**, which already walks every
  expression — `emit_expr`'s `Field` arm is where the decision belongs, and
  `indent` is already threaded through that whole call graph, so one more
  parameter follows the same path.

The second is smaller and the first is more honest: an AST that still says
`Field` where the program means a method call is an AST that lies to every other
consumer — the formatter, the LSP, `voltaic sig`. §10.33's own lesson argues for
the walker, since the formatter is what caught the last defect and it reads the
AST rather than the emitter.

Not decided here, because it is a structural choice and this subsection exists to
record that the primitive is in place and the choice is the next thing, not to
make it in passing.

### 10.36 Wave 3's no-argument half: the solver decides, and the emitter asks

`c.pos` emits `c.pos`; `c.depth` emits `c.depth()`. Doc 87 §2.2's *"resolved by
the solver, which knows whether `name` is a field or a method"*, working.
`examples/dot_resolution.vlt` witnesses a field, a method, both in one
expression, and the two boundaries below.

**The emitter asks an ambient scope, and the default is `field`.**
`MethodScope`/`MethodTable` hold the spans the solver resolved to method calls;
`emit_expr`'s `Field` arm consults them. Outside a scope the answer is always
*field*, so a caller that never solved emits exactly what generation 1 emitted.
That is the same restriction the SOLVE-stage judge follows — speak only where
sure — and it is what makes a missing scope safe.

An ambient scope rather than a threaded parameter because the emitter already
receives two answers that way: `BindingScope` and `ArityScope`. §10.35 argued for
a `&mut Expr` walker instead, on the grounds that an AST saying `Field` where the
program means a call lies to the formatter and the LSP. **That argument was
wrong, and the correction is the useful part**: the formatter prints `x.f` either
way, because `x.f` *is* what the author wrote. `Expr::Field` names the syntax, and
the interpretation is the solver's — so there is nothing to rewrite and no lie to
tell. The only consumer that needs the answer is the one that emits Rust.

**The safe default is also what made the bug invisible, and that cost a gate.**
The no-argument half was wired into `compile_dir.vlt`, verified end to end by
`voltaic-driver`'s own tests, and did nothing at all through `voltaic build` —
because `build_command.vlt` has its own emit path, as do `check.vlt`,
`migrate.vlt`, `voltaic-fuzz/vlt/target.vlt` and `voltaic-mcp/vlt/tools.vlt`. Six
emit sites, one wired, nothing failing, because an absent scope is
indistinguishable from a scope that found no methods.

`annotate_before_emit.rs` already existed for exactly that shape — its header
records finding a *fifth* site nobody had looked at — so it gained
`every_emit_site_also_installs_the_method_scope`, and that gate **found two more
on its first run**: the fuzz target and the MCP tool. The same two. A gate written
once for a class of defect earns its keep the second time the class recurs.

**Two boundaries, both measured rather than assumed.**

- **`std` is outside the declaration environment.** It knows the fields and
  methods of types declared in the program and nothing of `Vec` or `str`, so a
  receiver-first `v.len` is not resolved and emits `v.len` — `E0615`.
  Receiver-last `.len v` is unaffected, because it was never an `Expr::Field`.
- **A `self` receiver is not resolved either, and that is the sharper limit.**
  `self.depth` inside an `instance Cursor` method emits `self.depth`, because the
  solver does not bind `self` to the enclosing instance's target and
  `receiver_owner` has nothing to answer with. A method calling a sibling method
  still needs `.depth self`.

Both are pinned by tests that assert the *absence* —
`a_self_receiver_is_not_resolved_so_a_method_call_on_self_stays_a_field_read`
asserts `analysis.methods` is **empty** for a program whose only method call is on
`self`. A test that asserts a limitation is how the limitation stops being a
surprise, and it is the test that will fail when the limitation is lifted, which
is the point.

**Six `needless_borrow` and three `collapsible_if` errors, all in new test
bodies.** A method call's receiver auto-borrows, so `.is_empty (&table)` emits
`(&table).is_empty()` where clippy wants `table.is_empty()`. Worth one line
because it is a *Voltaic* surface fact with a *Rust* lint attached: the
receiver-last spelling makes the `&` look necessary, and it is not.

### 10.37 Wave 5's juxtaposition, and the formatter is what migrated the examples

Doc 87 §4.1: `name Type` rather than `name : Type`, the headless record
`data Point { x f64, y f64 }`, and the newtype shape `data Tag String`. The `:`
form still parses, because the compiler's own 105 `.vlt` files are written in it
and row 103 is what migrates them.

**The canonical form is generation 2's, so `voltaic fmt` is the migrator.** That
is the finding worth the subsection. Doc 101 §2.1 says *"the examples and
fixtures were migrated by row 99 **by hand**, wave by wave"* — and for this wave
no hand was needed: making the printer emit §4.1's form and running `voltaic fmt`
over `examples/` migrated **fourteen files** in one pass, and
`tests/golden/` did not change by a byte.

That last clause is the whole proof. A spelling change that leaves every
generated `.vrs` identical is exactly doc 101 §1.1's identity gate, satisfied
over the examples corpus without the migrator existing yet. Wave 5 is the wave
where the formatter *is* leaf 2, for the one construct whose two spellings parse
to the same AST.

It does not generalise. The formatter can migrate a construct whose spelling the
AST does not record — a field's `:` is punctuation, so printing it or not is the
printer's choice. It cannot migrate `if` into a ladder, because those are
different nodes.

**Two restrictions found by the round-trip.**

`newtype` keeps its `=`. The headless form is `data`'s only: §4.1 *deletes* the
`newtype` keyword, so `newtype Meters f64` is not a generation-2 spelling of
anything and printing it produced a file that did not round-trip.

A constructor named something other than its type keeps its `=` too.
`data Shape = Circle f64` is one constructor whose name is load-bearing, so the
condition is on the *name* rather than on the count — and `data Point = Point {…}`
drops the head because `Point` there carries no information.

### 10.38 `formatting_preserves_meaning` was wrong about its own normaliser, again

The round-trip reported that formatting **changed the program** in
`examples/data.vlt`, naming a `CaseArm` sixty lines from anything this wave
touched. It had not changed. `Pattern::Wildcard(Span)` carries its span
*positionally*, so it prints as `Wildcard((2132, 2133))` and the normaliser's scan
for `span: (` walks straight past it — and shortening
`data Celsius = Celsius f64` to `data Celsius f64` shifted every later byte offset
by ten.

The test's own doc comment already records this, for the *first* version:

> The first version of this test failed for that reason and not because any
> program had changed, which is worth recording: a property test whose
> normaliser is wrong reports a defect in the subject.

Same test, same class, second occurrence. The normaliser now blanks a labelled
span **and** any bare `(integer, integer)` — safe because a span is the only
two-integer tuple this AST prints; every other tuple holds nodes, which print as
`Name {` or `Name(`.

### 10.39 A formatting change is a fresh draw for a byte-indexed fuzzer

`voltaic-fuzz`'s sweep mutates a byte and requires that the parser's acceptance
imply the emitter's output parses. Reformatting fourteen examples moved every
later byte, so the sweep explored a different set — and immediately found a hole
that had been there all along.

**`import std::iter8:once` was accepted and emitted `use std::iter8:once;`**,
which is not Rust. `import_path` concatenates every token until a newline, so
anything on the line reached the generated `use` verbatim. A lone `:` is
unambiguously wrong there — `::` lexes as `ColonColon` — so it is now a
diagnostic. One deleted character in `std::iter::once` is what found it.

The sweep's own `assert_eq!(fenced, 15)` then failed at **14**, and that one was
not a defect either: the mutation positions are byte indices, so a mutation that
used to land beside a fence introducer now lands on it. The assertion is a range,
`12..=18`, and both ends are structural — below 12 the corpus has lost its inline
Rust or the sweep stopped reaching it, and above 18 the skip is excusing something
with no fence, because 2 files × 9 mutations is every input there is.

An exact count there turned every reformatting into a false failure. Its comment
argued for exactness because catching 18 was the case for having it; catching 14
is the case against, and the range keeps both signals the count had.

### 10.40 Wave 5's computed derives, and the two things that had to opt out

Doc 86 §4.3: `Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash` derived
for every `data` whose fields all support them, `deriving (…)` never written,
`data !Copy` to opt out. Generation 1 wrote `deriving` on **109 of its 128**
declarations, every one restating what the fields already imply.

Six cases, all in `examples/derives.vlt`: all eight for `u64` fields, seven for a
`String` (no `Copy`), five for an `f64` (no `Eq`, `Ord` or `Hash` — the
intersection doing the work rather than a float special case), seven for
`!Copy`, and **nothing** twice.

**The nothings are the design.** The emitter sees a field's type as *text*, so it
cannot reason about a type it has no table for. Returning the empty set there is
the only sound answer: deriving `Eq` for a field with no `Eq` is a compile error,
and the conservative direction costs a `deriving` line where the aggressive one
costs a build. Clause 2, as §4.3 states it — Lattice derives only what `core` can
derive.

**An explicit `#[derive(…)]` attribute suppresses computation, and the example
written to document that hatch is what proved it.** §4.3 says a trait no table
could know is "written where it always was, as an attribute". The first attempt
computed on top of one, and `examples/attributes.vlt` — which exists to witness
exactly that escape — failed with `E0119`, *conflicting implementations of trait
`Debug` for type `Point`*. A declaration carrying `#[derive(…)]` has said what it
derives.

**And the formatter dropped the `!Copy`**, because `no_copy` is a field on
`DataDecl` and the printer did not know it. Caught by formatting the new example
and reading the result, which is the same check §10.38's round-trip makes and the
reason to run `fmt` on an example before blessing it.

### 10.41 A frozen seed cannot survive an emitter change, and that is structural

`a_clone_without_the_generated_rust_bootstraps_from_the_seed` went red on
`voltaic-driver/cache.rs`: the in-tree compiler now emits a `#[derive(…)]` line
that `seed/voltaic-g1.wasm` does not, so a clean clone would have built
**compiling but different** code. The test's own message says why that matters —
on a clean clone the seed's output *is* the crate.

`AGENTS.md`'s rule covers the parser case: *a surface row the seed cannot parse
must refresh the seed in the same commit.* This is not that. g1 parses the source
fine; it emits differently. And **g1 cannot be refreshed, ever**: rebuilding it
from the tree would make its `built by` and `compiles` both `g1`, which
`blob.rs` refuses by doc 100 §1.2's invariant. So the divergence is permanent from
the first emitter change after plan 72 froze the blob, and no refresh closes it.

Three ways out, and only one is honest. Refreshing g1 is forbidden. Flipping
`seed/GENERATION`'s column to `g2` would make `bootstrap.sh` use the current blob
but would also claim row 103's migration had happened, and the column is
documented — `bootstrap.sh` line 33 and doc 100 §1.4 — as *the generation the
`.vlt` is written in*.

**So the column keeps its meaning and the dispatch gains one: the newest blob that
reads that generation.** g2 reads generation 1 as well as generation 2 — mid-turn
the compiler accepts both and g2 was built from that compiler, which
`across_the_turn.rs` proves byte-for-byte over all fourteen crates — so a `g1`
crate is regenerated by g2 once g2 exists. Verified by deleting every `src/` and
bootstrapping: the clean clone now builds the derives.

g1 stays, unused, for doc 100 §3.1's reason read exactly: *retired last, and not
when it stops being needed.* It stops being needed here; it is kept anyway, so
that a defect found in g2 still leaves something able to rebuild the repository.

### 10.42 Wave 5 is complete: `errors` is deleted as a construct and kept as a shape

Doc 86 §4.4. A `data` whose variants carry a trailing string literal **is** an
error type, and the emitter it reaches is the one `errors` already had — enum,
`#[derive(Debug)]`, `Display`, `Error`. `examples/error_data.vlt` is the
generation-2 spelling of `examples/errors.vlt`'s type, and both are kept until
wave 4 deletes the keyword.

**Delegation, not reimplementation.** `is_error_type` asks whether any
constructor carries a message and `as_errors` builds an `ErrorsDecl` from the
`DataDecl`; `emit_errors` does the rest. That matters because `errors` is the one
Voltaic construct emitting more than one Rust item, and a second copy of it would
be a second place for the `Display` arms to drift from the variants they
describe.

**The parity claim is exact, and the gap is §4.4's own decision.** Generation 1's
`from std::num::ParseIntError as Parse` variant has no `data` form, because §4.4
says `From` conversions "are derived from the effect row rather than declared" —
row 95's work, not a hole here. A `data` error type reaches `errors`' parity minus
the `from` variants, and `examples/error_data.vlt` says so in a comment rather
than leaving a reader to notice.

### 10.43 The formatter dropped a new AST field three times in one wave

`!Copy`, then the trailing message, and before them the `Ladder`'s arms. Each
time the parser read a field, the AST carried it, the emitter used it, and **the
printer silently did not print it** — so `voltaic fmt` deleted the construct from
the source file.

Three for three is a pattern worth naming: **adding a field to an AST node is
four edits, not three.** Parser, AST, emitter, *printer* — and the printer is the
one with no compile error to force it, because a struct field nobody reads is not
an error in Rust.

`formatting_preserves_meaning` catches it: parse, print, reparse, compare. It
would have caught all three. What caught them *faster* was running `voltaic fmt`
on the new example and reading the result before blessing anything — one command,
and the construct is either still there or it is not.

So: after adding an AST field, `fmt` a file that uses it. §10.37's
formatter-as-migrator is what makes that unavoidable rather than tidy — a printer
that drops a field does not merely format badly, it **migrates the field away**.

### 10.2 Both corpora were necessary, and that was not the expectation

The plan treats the compiler's own source as the demanding corpus — 11,668 lines
nobody wrote to be easy to infer, against 43 small examples. The measurement says
the opposite: **the compiler's source found neither defect, and the examples found
both.** 594/594 was already true at the first run.

The reason is that the compiler's source happens never to return an interpolated
string from an annotated function, and happens never to write the same type alias
twice in one signature. A gate on the large corpus alone would have been green
while inference was wrong in two ways — which is the same shape as plan 66's
grammar gates passing before `grammar.js` knew `>>>`.

So the gate runs both, and `both_corpora_are_the_size_the_tree_says_they_are`
asserts neither has quietly shrunk.

### 10.3 The two defects

**An interpolated string is `String`, not `&str`** — 15 of the 16 disagreements.
`"#{tier n}"` lowers to `format!`, which returns `String`; a plain `"abc"` is
`&str`. `expr_type` typed both as `&str`, so every function written
`f : i32 -> String` whose body ends in an interpolation read as inferring
`i32 -> &str`. `Expr::StrInterp` was already a distinct variant, so the fix is one
arm.

**The alias cycle guard was per-type instead of per-path** — the 16th, and a defect
in the alias support this row's SOLVE-stage commit had just added.
`double : Depth -> Depth` expanded the parameter, pushed `Depth` onto the shared
`seen` list, and then declined to expand the return as a cycle, yielding
`f64 -> Depth`. Each branch now carries its own path. The cyclic-alias test was
green throughout, because one `Depth` is all it ever had.

That is the general shape and it is worth stating on its own: **an occurs-style
guard that is shared across sibling branches under-expands rather than looping, so
it fails quietly.** The looping it prevents is loud; the over-reach it causes is
not.

### 10.4 What wave 1 still owes

The agreement is a statement about *inference*, not about *emission*. Wave 1 also
has the emitter write the inferred signature, and deleting the signature lines
changes the parser's accepted surface — neither has happened. The number above is
the licence to do that work, not the work.

### 10.44 An inherent `instance` needs no signature, and the refusal was about the wrong component

Doc 63 §6 held that an inherent `instance` member **cannot** elide its
signature, and `tests/ui/elided_without_a_class.vlt` pinned the refusal with its
reasoning written out: *"an inherent `instance` has no class, so there is
nothing for an elided signature to be taken from. That is a property of the
construct and not a limit of the lookup — widening the resolver would not
help, because no class is named."*

Every clause of that is true, and the conclusion does not follow. It is a fact
about the **resolver**, which *looks a signature up*; it says nothing about the
**solver**, which *computes one*. Nothing was being taken from anywhere:

- the receiver's type is already pinned from the instance header — `generate.vlt`
  emits `Unify { hole, Ty::named target }` for the `self` binder;
- each parameter's type follows from the body, the same way it does for a
  top-level unsigned definition;
- the mode on `self` is what `infer_mode` already computes for every other
  parameter.

The parser was simply never wired to the path that existed: `unsigned_equations`
in `parser.vlt` had been handling top-level unsigned definitions since wave 1,
and the inherent-instance branch raised `VLT0101` instead of falling through to
it. The fix is that fallthrough plus two small things the unsigned path had no
reason to do before — `receiver` now recognises an **untyped** parameter named
`self` (so it is moved to Rust's first position), and `annotate.vlt`'s `fill`
decorates that parameter with the inferred mode rather than giving it a type.

Measured on `tests/run/inherent_inference.vlt`, with no signature written
anywhere: `fn current(&self) -> u32`, `fn bump(&mut self)`, and
`fn plus(&self, extra: u32) -> u32` — both receiver modes, the parameter type,
all three return types, and the reorder.

**Three things this did not do, each worth more than the capability.**

**It did not make the signatures deletable.** Re-measuring `signature_pass`
across the compiler's 1,001 signature lines moved **31** of them out of *"needs
it to compile at all"* (974 → 943) and into *"compiles without it, but the Rust
changes"* (16 → 48). Removable went 4 → 3. So the relaxation is real and it is
not the binding constraint: inference now *reaches* those 31, and the identity
gate still rejects them because the inferred mode or type spelling is not the
written one. **The remaining work is fidelity, not reach** — which is wave 8's,
and is a different claim from the one row 99 wave 1 licensed.

**It cannot be used in `crates/*/vlt/**` at all.** Not by policy — measured.
Running `seed/voltaic-g2.wasm` under `wasmtime` over an unsigned inherent member
still prints `VLT0101 ... has no class to take one from`, and the blob is what
builds `src/` on a clean clone. Deleting one of these signatures from the
compiler's own source would therefore break the bootstrap, and the refresh that
would fix it is illegal: `SEED.md` records g2 as `built by: g1`, g1 is retired,
and `blob.rs` refuses a blob whose `built by` names the generation in its
`compiles`. The capability is available to user code today and to the compiler's
own source at g3.

**The same seed wall moved the witness out of `examples/`.**
`the_g2_blob_and_the_tree_agree_on_the_examples` stages every example that has a
golden and requires the blob to emit what the tree emits, so an `examples/`
witness fails for exactly the reason above. It lives in
`crates/voltaic-cli/tests/run/` instead, and the file says so in its own header
rather than leaving the placement to look like a judgement about how narrow the
case is.

**One case is still inferred wrong, and it is a Lattice bug.** A member that
moves a non-`Copy` field out of `self` — `tag_of self = self.tag` — is emitted
with `&self` and rejected by rustc as `E0507`. Per `AGENTS.md`, a program
Lattice accepts and rustc rejects is filed as a Lattice bug, and this one is
filed here: `infer_mode` has no notion of a field *move* demanding an owned
receiver, because deciding that needs the field's type and `mode_of` is
syntactic. It is **ungated** — there is no corpus for "Voltaic emits it, rustc
rejects it", `tests/unreachable/` is read against `GRAMMAR.md`'s rows and this
is not a grammar gap, and inventing a corpus for one bug was declined. So it is
recorded and not pinned, which is the weaker outcome and is stated as such.

### 10.45 The third lever is declined on arithmetic, and the second was disproved

§10.44 is one of three levers proposed for *"almost no signatures"*. All three
were measured, and only one survived, which is worth recording because the two
that failed did so for unrelated reasons.

**Lever: teach `expr_type` the `Expr::Dot` form.** `generate.vlt`'s `expr_type`
covers 15 expression shapes and falls through to `.fresh_hole store`, so a
method call contributes nothing to inference — which looked like the obvious
reason signatures cannot be dropped. Adding the arm made it **strictly worse**:
removable fell 4 → 0, and the sidecars changed only by hole renumbering
(`?42` → `?50`), meaning the new constraints carried no information and merely
perturbed the numbering the gate compares. Reverted. A hole that resolves to
nothing is not better than no hole.

**Lever: move the method-shaped free functions into instances.** 159 of the
compiler's free functions take a receiver-shaped first or last parameter and
could be `instance` members, which §10.44 would then let go unsigned. Declined,
on two independent grounds:

- it **cannot** be identity-preserving. `fn f(x: T, …)` becoming
  `impl T { fn f(&self, …) }` changes the generated Rust by construction, and
  changes every call site with it. So it is a refactor needing its own review,
  not a respelling a gate can bless — the identity gate that carried row 103's
  migration does not apply;
- its entire payoff is unavailable. The signature can only be *dropped* once
  the seed accepts an unsigned inherent member, and §10.44 measured that it does
  not. An instance member with a written signature costs exactly what a free
  function with a written signature costs: one line.

So at generation 2 the lever is 159 mechanical refactors, a large diff, zero
signatures removed, and a second review at g3 when the deletions become
possible. It is deferred to g3, where it is done **with** the deletion in one
reviewable step instead of twice.

The honest summary of the three: one shipped a capability and no deletions, one
was disproved by measurement, one is arithmetic. What binds *"almost no
signatures"* is not the parser's willingness to accept them missing — that is
now settled for inherent instances — but mode and type-spelling fidelity, and
the seed. Neither is a surface question.

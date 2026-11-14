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
| 3 | `\|>`, `>>>`, `<<<`, `$` | `.` chains across lines | 3 |
| 4 | receiver-last `.f x` | receiver-first `x.f` (UFCS) | 3 |
| 5 | `name : Type` in declarations | `name Type` juxtaposition | 5 |
| 6 | `deriving (…)`, `newtype`, `errors` | computed derives, `data` | 5 |
| 7 | `import`, `export`, `module`, `mod`, `use`, `island` | manifest resolution and `pub` | 6 |
| 8 | `try`, `catch`, `?`, `??`, `async`, `await`, `do` | the effect row (row 95) | 7 |
| 9 | `&`, `&mut`, `'a`, `.clone()` | mode inference (row 96) | 8 |

Wave 9 is the three gaps that close.

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

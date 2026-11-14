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

In progress. Wave 1's **answer key is established and gated**, which is the thing
§4 ordered wave 1 first in order to get. Waves 2–9 are open.

### 10.1 Inferred equals written, 809 out of 809

| Corpus | Written signatures | Inferred equals written |
|---|---:|---:|
| `examples/` (43 programs) | 229 | **229** |
| `crates/*/vlt/` (122 files) | 910 | **910** |
| | **1,139** | **1,139** |

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

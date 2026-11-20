# Conformance

**Can the compiler's own source be expressed in Voltaic, without the
inline-Rust escape hatch?** Doc 02 §4.8 phrases the question that way and
calls it *"the property actually worth proving"*.

**Generated.** Measured from the working tree by
`crates/voltaic-cli/tests/conformance.rs` and compared in the gate. Rewrite
it with:

```bash
VOLTAIC_UPDATE=1 cargo test -p voltaic-cli --test conformance
```

Plan 48 (`docs/42 - 48 - Conformance and Self-Hosting.md`) built it, and
plan 64 changed its answer.

---

## The answer

**Yes — no blocking occurrence is left.**

**0 blocking occurrences**, down from 964 — and the construct that was 952 of them is gone. Plan 48 measured macro invocation at 99% of the obstacle and wrote *"spell it, and what remains is the `as` casts below"*; plan 64 spelled it, as `name! \-> e`, and this is what remained.

That is the whole arc of the measurement in three rows: plan 45 §1.5 **found** the gap and declined to spell it because it needed a surface decision in doc 03; plan 48 **priced** it, which is what made the decision worth taking; plan 64 **took** it. A count is what turned an unreachability note into a priority.

**The figure is zero, and this document said it never would be.** It read *"the figure is not zero and will not be"* while the table beneath it was zeroes, which is what a generated number is for and what a written-down prediction is worth. It can still move: it is read off `docs/GRAMMAR.md`'s `UNREACHABLE` rows and counted over the compiler's own `.vlt`, so any commit introducing one of those constructs raises it.

---

## Blockers

Each is graded `UNREACHABLE` in `docs/GRAMMAR.md`.

| Blocker | Occurrences |
|---|---:|
| `as` cast | 0 |
| `unsafe trait` | 0 |
| `unsafe impl` | 0 |
| `extern "C" fn` | 0 |
| Macro definition | 0 |
| `\|` or-pattern | *not counted* |

**All five remaining gaps are zero**, which is the result doc 02 §4.8 did not expect: `as` casts, `unsafe trait`, `unsafe impl`, `extern "C" fn` and `macro_rules!` are real gaps and were never *this* obstacle. Only a count could say so — and the same count is what identified the one that was. This sentence said *four* of five, naming the `as` cast as the exception, which stopped being true when the compiler's own source lost its last cast.

`\|` or-patterns are **not counted** rather than estimated. The counter is line-based, and no line-based pattern separates a pattern alternative from a bitwise or; a wrong number would be worse than an empty cell.

---

## By crate

| Crate | Files | With a blocker | |
|---|---:|---:|---:|
| `voltaic-ast` | 8 | 0 | 0% |
| `voltaic-bench` | 5 | 0 | 0% |
| `voltaic-cli` | 7 | 0 | 0% |
| `voltaic-diag` | 5 | 0 | 0% |
| `voltaic-driver` | 14 | 0 | 0% |
| `voltaic-emit` | 11 | 0 | 0% |
| `voltaic-fmt` | 6 | 0 | 0% |
| `voltaic-fuzz` | 2 | 0 | 0% |
| `voltaic-lattice` | 23 | 0 | 0% |
| `voltaic-lexer` | 9 | 0 | 0% |
| `voltaic-lsp` | 4 | 0 | 0% |
| `voltaic-mcp` | 5 | 0 | 0% |
| `voltaic-parser` | 13 | 0 | 0% |
| `voltaic-seed` | 2 | 0 | 0% |
| **total** | **114** | **0** | **0%** |

---

## Fourteen crates are no longer a count

The table above measures what *could* be ported. Fourteen crates **have** been, and for them the question is settled by rustc rather than by a blocker count: `voltaic-diag` (plan 73), `voltaic-lexer` (plan 74), `voltaic-ast` (plan 68), `voltaic-lattice` (plan 94), `voltaic-parser` (plan 82), `voltaic-fmt` (plan 69), `voltaic-fuzz` (plan 75), `voltaic-bench` (plan 76), `voltaic-mcp` (plan 77), `voltaic-lsp` (plan 78), `voltaic-emit` (plan 81), `voltaic-driver` (plan 80), `voltaic-cli` (plan 79) and `voltaic-seed` (plan 83) are written in Voltaic, their `.vlt` living in `crates/<crate>/vlt/` beside the `src/` it generates, and the generated Rust is compiled as part of the workspace.

So **the compiler is now compiled from Voltaic** — 114 files of it. That corrects what this document said until plan 70: *"nothing in the build depends on `conformance/`, and the compiler is not compiled from it"*, which was true when plan 48 lifted one 45-line module out of a crate and checked it with `syn`, and stopped being true the moment a whole crate's `src/` was generated.

Plan 48's probe is retired with that sentence. `conformance/clause.vlt` was `clause.rs` expressed in Voltaic and checked by `syn::parse_file`, because one module lifted out of a crate cannot resolve `crate::Stmt`; the real port has no extraction to concede to, so the weaker oracle expired. Its no-escape-hatch assertion now runs over all 114 files in `port.rs`.

What it found still stands, and is the reason doc 02 §4.8 expected a port to be worth doing: every field doc comment was silently discarded, and `voltaic fmt` deleted them from the source, because the lexer classified a comment opening a continuation line as *trailing* the previous line and then held it for a `Newline` a continuation never produces. Plans 68 and 69 found three more of that class and two formatter defects, and plan 73 four more in the toolchain that writes the port.

---

## The Rust implementation is permanent

Doc 02 §4.8 corrects doc 01 §34's own wording: keep the Rust implementation *"permanently rather than as a 'fallback'"*. **No crate's `src/` is hand-written Rust any more.** What stays Rust is what             plan 72 §6 named: the integration tests, which `AGENTS.md` requires to be             Rust asserting over Voltaic-generated code, and `crates/voltaic-driver/build.rs`,             which cargo runs on the host before anything exists.

**The generated Rust is not committed, and this section said it was.** It read *"the fourteen that are ported keep their generated Rust **committed**"*, citing doc 02 §4.8's *"commit the generated `.vrs` so the bootstrap never breaks"*. Plan 72 reversed that: each crate's `src/` is `.gitignore`d, and `voltaic-seed/tests/bootstrap.rs`'s `the_generated_trees_are_ignored` now asserts the opposite of what it once did.

The *reason* §4.8 gave was sound and is what the reversal had to answer: a self-hosting language needs its previous-version compiler present on a clean clone, and untracking thirteen generated trees left nothing able to build the compiler that would regenerate them — plan 68 proved it by breaking the bootstrap within the hour. `seed/voltaic-g2.wasm` is that compiler, stored **once** instead of fourteen times, and row 102 §3.1 retired its predecessor only after `fixpoint` was green on it from clean clones. So the requirement held and the mechanism changed.

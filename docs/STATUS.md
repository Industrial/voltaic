# Status

**Generated.** Every number here is measured from the working tree by
`crates/voltaic-cli/tests/status.rs` and compared in the gate, so a drifted
number fails the build. Rewrite it with:

```bash
VOLTAIC_UPDATE=1 cargo test -p voltaic-cli --test status
```

which rewrites and then **fails**, so the diff is reviewed rather than
absorbed into a green build.

Plan 47 (`docs/41 - 47 - Status and Declines.md`) built it. The posture is
doc 02's, taken from Emerald: *none of this is rounded up*.

---

## Plans

| Status | Rows |
|---|---:|
| `done` | 87 |
| `superseded` | 4 |
| outstanding | 0 |
| **total** | **91** |

`docs/04 - Voltaic Plan of Plans.md` is the register and the authority. **`done` is a register status, not a guarantee**: several rows record a later plan finding something they got wrong, which is why the register carries the correction and this table only counts.

---

## Source

| Crate | `src` | `tests` |
|---|---:|---:|
| `voltaic-ast` | 1653 | 0 |
| `voltaic-bench` | 355 | 217 |
| `voltaic-cli` | 1416 | 13659 |
| `voltaic-diag` | 1055 | 0 |
| `voltaic-driver` | 3676 | 316 |
| `voltaic-emit` | 3974 | 4647 |
| `voltaic-fmt` | 1604 | 1569 |
| `voltaic-fuzz` | 65 | 378 |
| `voltaic-lattice` | 6013 | 1335 |
| `voltaic-lexer` | 2232 | 1196 |
| `voltaic-lsp` | 880 | 756 |
| `voltaic-mcp` | 464 | 776 |
| `voltaic-parser` | 6013 | 787 |
| `voltaic-seed` | 146 | 1130 |
| **total** | **29546** | **26766** |

---

## Generations

| Generation | Crates | Seed |
|---|---:|---|
| `g1` | 0 | retired |
| `g2` | 14 | `seed/voltaic-g2.wasm` |

`seed/GENERATION` is the only copy of which crate is written in which generation, and `seed/` is the only place this repository tracks a binary. A crate's generation is **not** a dispatch — `bootstrap.sh` validates the column and runs the one blob — which doc 100 §10.8 records after the comment there claimed otherwise.

A generation with no seed has been **retired**, which row 102 §3.1 permits only once no crate names it and `fixpoint` is green on its successor from two clean clones. `g1` is in that state; its name survives because `SEED.md` records g2's `built by: g1`, and that is how this blob came to be rather than where a file is.

---

## Corpora

| Corpus | Files | What it asserts |
|---|---:|---|
| `examples/*.vlt` | 53 | the CI-checked source of truth for what works |
| `tests/golden/*.vrs` | 52 | the generated Rust, as exact text |
| `crates/voltaic-cli/tests/ui/*.vlt` | 57 | the diagnostic a bad program gets |
| `crates/voltaic-cli/tests/lint/*.vlt` | 7 | the warning a questionable program gets |
| `crates/voltaic-cli/tests/run/*.stdout` | 2 | what a program prints |
| `crates/voltaic-cli/tests/explain/*.why` | 6 | the constraint chain behind an inference |
| `crates/voltaic-cli/tests/sidecar/*.sig` | 2 | the published signature, as exact text |
| `tests/unreachable/*.vlt` | 7 | the gaps below, asserted to **still fail** |
| `benchmarks/pairs/*.vlt` | 10 | generated and hand-written Rust, as identical IR |

**53 examples and 52 goldens is not an error.** `examples/lexical.vlt` deliberately does not parse — it is the error-path example, and `benchmarks/REPORT.md` excludes it from the throughput sweep for the same reason. Reporting "53 examples, all green" would be rounding up by exactly one file.

---

## Gaps

8 Rust constructs a Voltaic program cannot express. Read out of `docs/GRAMMAR.md`'s `UNREACHABLE` rows rather than listed again here, so the two cannot disagree; each has a fixture that the gate asserts **still fails**, so closing a gap breaks the build until both documents are updated.

| Construct | Fixture |
|---|---|
| Macro definitions | `tests/unreachable/macro_def.vlt` |
| Macro invocation with a non-expression argument | `tests/unreachable/macro_item_argument.vlt` |
| `as` casts | `tests/unreachable/as_cast.vlt` |
| `extern "C" fn` (export) | `tests/unreachable/extern_fn_abi.vlt` |
| `ref` and `ref mut` | `tests/unreachable/ref_pattern.vlt` |
| `unsafe impl` | `tests/unreachable/unsafe_class.vlt` |
| `unsafe trait` | `tests/unreachable/unsafe_class.vlt` |
| `|` or-patterns | `tests/unreachable/or_pattern.vlt` |

Full grading, with the nearest working alternative for each: `docs/GRAMMAR.md`. Deliberate refusals are a different thing and live in `docs/DECLINED.md`.

---

## What this document does not say

- **No coverage percentage.** `moon run :coverage` produces one. A number here would become a target, and it would mostly measure the emitter's match arms.
- **No test count as a quality measure.** The corpora above are counted because they are corpora; a total would invite reading it as a score.
- **No roadmap and no dates.** What is left is the register's outstanding rows. When they land is not a measurement.
- **No benchmark numbers.** `benchmarks/REPORT.md` has them, and plan 44 §1.2 deliberately keeps them out of the gate because they are timings. Copying one here would make this comparison flake.
- **No terseness ratio.** Doc 86 §1.1 and doc 97 §1.2 measure it, in documents that say what was measured and what was equalised. A bare `2.24×` in a status table would be quoted without the caveat that 1.76× is the grammar-only figure, and that the character saving over the whole compiler is **1.002×** — which doc 90 §1 is about. The project's own §1.2 calls claiming the library's share *rounding up*.

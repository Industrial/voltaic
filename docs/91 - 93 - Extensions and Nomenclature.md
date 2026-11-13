# Plan 93 — Extensions and Nomenclature

Plan ID: `extensions`
Depends on: 92
Milestone: 14 — *nomenclature*
Status: `planned` — awaiting approval; §2 and §3 are measured

---

## 1. What this plan is

Four renames, applied across the whole tree in one generation:

| | From | To | Why |
|---|---|---|---|
| source extension | `.sdx` | **`.vlt`** | `sdx` means nothing in a project called Voltaic |
| output extension | `.xrs` | **`.vrs`** | `.rs` with a letter in front; the letter should be the language's |
| source map | `.xrs.map` | **`.vrs.map`** | follows the output |
| per-crate source dir | `crates/<c>/sdx/` | `crates/<c>/vlt/` | follows the source |
| hygiene prefix | `__rdx_` | **`__vlt_`** | `rdx` is the pre-rename identity, in every generated binder |
| diagnostic codes | `RDX####` | **`VLT####`** | the same fossil, in the one place users read it |

And one addition, which has no predecessor: `.vlt.sig`, the signature sidecar (doc 86
§8.2, row 100).

### 1.1 These are fossils, not names

`sdx`, `xrs` and `rdx` all date from before the project was called Voltaic. `rdx` is
the clearest case: it is an abbreviation of the old name, and generation 1 carries it
in **every generated identifier** (`__rdx_a0`), **every diagnostic code** (`RDX0101`),
and the lifetime names the emitter invents. A user who never reads this repository
still reads `RDX0117` when they write `?.`, and `__rdx_a0` the first time they open a
generated file.

A generation that renamed the file extensions and left the prefix would move the
fossil from the filesystem, where only contributors see it, into the output, where
everyone does.

### 1.2 Why this row comes before any language work

The rename is **mechanical** and the surface change is not. Doing them together makes
every merge conflict ambiguous between the two causes, and makes the one gate that
matters here — *the tree still builds and every test still passes* — unable to
distinguish "the rename broke it" from "the surface moved".

It is also the right row to prove row 103's migration tooling on: a rename is a
migration with a known-correct answer, so a migrator that cannot do this one cannot be
trusted with a surface change.

### 1.3 What does not change

- The language's name. It is Voltaic, in both generations.
- The binary's name, `voltaic`. The crate names, `voltaic-*`. The published crates on
  crates.io.
- The MCP tool names — `voltaic_build`, `voltaic_check`, `voltaic_fmt`,
  `voltaic_source_map`. They are already correct.
- `seed/SEED.md`'s role. The blob is renamed by row 102, which owns generations; this
  row does not touch it, for the reason in §4.6.

---

## 2. Preconditions, measured

Counted 2026-11-13 over tracked files.

### 2.1 The scale

| | Occurrences |
|---|---:|
| `.sdx` or `.xrs`, whole tree | **2,644** |
| the same, excluding `docs/` | **1,158** |

`docs/` is the larger half and is **not** rewritten: docs 01–85 are the historical
record of generation 1 and refer to `.sdx` files correctly, because that is what those
files were called. Rewriting them would falsify the record. The 1,158 outside `docs/`
are this plan's work.

### 2.2 Where they are

| Area | Files |
|---|---:|
| `crates/voltaic-cli/tests/ui` | 61 |
| `tools/port/batches` | 55 |
| `crates/voltaic-cli/tests` | 25 |
| `www/src/language` | 15 |
| `examples` | 15 |
| `tests/golden` | 14 |
| `crates/voltaic-fmt/tests/fmt`, `crates/voltaic-cli/tests/lint` | 7 each |
| `crates/voltaic-fmt/tests`, `crates/voltaic-emit/tests` | 6 each |
| `crates/*/sdx` (nine crates) | 2–4 each |
| `editors/*` | 2 each across vim, helix, vscode, zed, neovim |
| `.github/workflows`, `tests/unreachable`, `seed`, `www` | 2–3 each |

### 2.3 The thirty extension literals in code

A search for the string literals `"sdx"` and `"xrs"` finds **30 call sites** that
branch on an extension. They are the ones a path rename alone would not fix, and they
are in two groups:

- **Compiler source** — `voltaic-cli`'s `build_command` and `watch_command`,
  `voltaic-driver`'s `compile_dir`, `voltaic-bench`'s `main` and `pair`. Five files,
  all of them in `crates/*/sdx/`, so they are themselves renamed in the same pass.
- **Test harnesses** — `examples.rs`, `port.rs`, `run.rs`, `ui.rs`, `lint.rs`,
  `site.rs`, `editors.rs`, `tree_sitter.rs`, `no_comments.rs`, `hello.rs`,
  `expression_printer.rs`, `corpus.rs`. Twelve files.

### 2.4 The configuration surface

Found, and each one is a place a missed rename is silent rather than loud:

| File | What it holds |
|---|---|
| `.gitignore` | six patterns naming `.xrs`, `.xrs.map` and `crates/*/sdx/**` |
| `.gitattributes` | the language association for `.sdx` |
| `moon.yml`, `crates/voltaic-cli/moon.yml`, `crates/voltaic-seed/moon.yml` | task inputs |
| eight `crates/*/Cargo.toml` | `include` keys naming `sdx/**` |
| `devenv.nix`, `flake.nix`, `nix/nixpkgs-package.nix` | source filters |
| `.github/workflows/ci.yml`, `pages.yml`, `publish.yml` | path filters and globs |
| `.github/scripts/publish-crate.sh`, `scripts/validate-crate-packaging.sh` | packaging checks |
| `bootstrap.sh`, `sdx-fmt.sh`, `design/*.sh` | scripts; the second is renamed too |
| `editors/vim/ftdetect/sdx.vim`, `editors/vim/syntax/sdx.vim` | file names, not just contents |
| `editors/vscode/package.json` plus `icons/sdx-light.svg`, `icons/sdx-dark.svg` | file names |
| `editors/helix/languages.toml` | `file-types = ["sdx"]` |
| `editors/zed/languages/voltaic/config.toml`, `editors/neovim/lua/voltaic/init.lua` | the same |
| `tree-sitter-voltaic` | grammar file-type association |
| `www/src/**` | 20 occurrences across the book |

---

## 3. The four traps, found before doing it

Each of these is a way this plan silently half-lands. They are the reason it is a plan
and not a `sed` invocation.

### 3.1 `.gitignore` and the directory rename must be one commit

`.gitignore` names `crates/*/sdx/**/*.xrs` and the per-crate `src/` ignores. Rename the
directory in one commit and the patterns in another, and in between **every generated
tree becomes tracked** — thirteen crates of generated Rust staged into history.

The ordering rule: the ignore patterns and the `git mv` land together, and the commit
is verified by a porcelain status check **before** anything else runs.

`AGENTS.md` already records the adjacent trap in the other direction: a probe under
`.tmp/` cannot answer a git-status question, because `.tmp/` is wholly ignored and git
declines to discuss it. The verification for this leaf must therefore run **in the
tree**, not in scratch.

### 3.2 The seed emits the old prefix, and cannot be asked not to

This is the trap that makes the hygiene rename non-mechanical.

`seed/voltaic1.wasm` is a *previous compiler*. It emits `__rdx_a0`, because that is
what it was built to emit. A clean clone bootstraps with the seed, so on a clean clone
the generated Rust contains `__rdx_` while every golden, fixture and test in the tree
expects `__vlt_`.

`AGENTS.md` states the rule for the parse direction — *"a surface row the seed cannot
parse must refresh the seed in the same commit"* — and this is the **emit** direction
of the same rule. The order is the one `AGENTS.md` already gives, and it is not
negotiable:

1. **build** — the tree, with the new prefix, using the old seed
2. **update** — re-bless every corpus against the new prefix
3. **seed-refresh** — rebuild the blob from the new tree, and record its hash

A fourth step is this plan's own addition, and it is the one that proves the rename:

4. **bootstrap from scratch** — delete every generated `src/`, run the bootstrap, and
   assert the result contains no `__rdx_` anywhere.

### 3.3 Re-blessing writes and then fails, by design

61 UI fixtures carry `RDX####` in their `.stderr` expectations; the goldens carry
`__rdx_` in generated identifiers. Both are re-blessed, and blessing **rewrites the
expectations and then fails the run** — generation 1's deliberate design, so a changed
expectation is reviewed rather than absorbed into a green build.

So a bless step is never the last step of a leaf. The leaf is: bless, read the diff,
confirm every change is a prefix rename and nothing else, then run the gate clean.
A diff in a blessed fixture that is *not* a prefix rename is this plan having changed
behaviour, which it must not.

### 3.4 Users have `.sdx` files, and a hard break is avoidable

Out in the world there are `.sdx` files this repository does not own. The compiler
accepts both extensions for one release:

- `voltaic build foo.sdx` works, emits to `foo.vrs`, and prints a deprecation note
  naming `voltaic migrate` (row 103).
- `.sdx` is removed from the accepted set in the release after the one that lands
  generation 2, and `docs/MIGRATION.md` says which release that is.

The repository's *own* files have no transition period: a tree with both extensions is
a tree where every glob is wrong twice.

---

## 4. Leaves

### Leaf 1 — the ignore patterns and the directory moves

`.gitignore` and `.gitattributes` first, then `git mv` for `crates/*/sdx` →
`crates/*/vlt` and every `*.sdx` → `*.vlt` in `examples/`, `tests/`, `benchmarks/`,
`crates/*/tests/`. `git mv`, not delete-and-write: history survives a move and does
not survive a rewrite, and the blame on 572 source files is worth keeping.

Gate: a porcelain status check showing no generated tree staged (§3.1).

### Leaf 2 — the extension literals

The 30 call sites in §2.3. The five in compiler source accept both extensions per
§3.4; the twelve test harnesses accept `.vlt` only, because the corpora are this
repository's own.

### Leaf 3 — the configuration surface

Every file in §2.4, including the file *names*: `sdx-fmt.sh` → `vlt-fmt.sh`,
`editors/vim/ftdetect/sdx.vim` → `vlt.vim`, `editors/vim/syntax/sdx.vim` → `vlt.vim`,
`icons/sdx-{light,dark}.svg` → `icons/vlt-{light,dark}.svg`.

The eight `Cargo.toml` `include` keys matter more than they look: they are what makes
the generated-but-untracked `src/` get packaged, and `docs/RELEASING.md` records that
packaging any crate here needs `--allow-dirty` because of it. A missed `include` key
publishes a crate with no source.

### Leaf 4 — the hygiene prefix and the diagnostic codes

`__rdx_` → `__vlt_`, `'__rdx_` → `'__vlt_`, `RDX####` → `VLT####`. Then §3.2's
four-step order, ending in a from-scratch bootstrap that asserts the old prefix is
gone.

The code *numbers* do not change in this leaf. `RDX0117` becomes `VLT0117`, same
number, same meaning. Re-numbering into doc 87 §17's ranges is row 101's, because that
row owns the diagnostic surface and a renumber without the new ranges would be two
churns where one will do.

### Leaf 5 — the website and the book

`www/src/**`, 20 occurrences, plus every code fence labelled `sdx`. The book is the
documentation (`README.md` points at it first), so a stale extension there is the most
visible possible miss.

### Leaf 6 — the reserved-prefix gate

`crates/voltaic-cli/tests/` has an assertion that user identifiers may not use the
hygiene prefix. It must now reject **both** `__rdx_` and `__vlt_`: the old prefix stays
reserved forever, because a user program that adopted it while it was unreserved would
otherwise start colliding with nothing and reading as if it might.

---

## 5. Acceptance criteria

1. A tree-wide search for `sdx`, `xrs`, `__rdx_`, or `RDX` followed by a digit returns
   hits **only** in `docs/01`–`docs/85`, `docs/90 - 92`, this document, and
   `docs/MIGRATION.md`.
2. The full gate is green, with **no behavioural diff**: every blessed change is a
   prefix or extension rename and nothing else (§3.3).
3. A from-scratch bootstrap — generated trees deleted, blob run, tree built — produces
   Rust containing no `__rdx_` (§3.2 step 4).
4. `voltaic build foo.sdx` still works and warns; `voltaic build foo.vlt` works and
   does not.
5. Every editor integration still highlights, and `crates/voltaic-cli/tests/editors.rs`
   proves it rather than a manual check.
6. Packaging validation passes for all eight published crates, which is what proves
   the `include` keys followed the directory.
7. `git log --follow` on a moved source file shows its pre-rename history.

---

## 6. Out of scope

- **The seed blob's own name.** `seed/voltaic1.wasm` → `seed/voltaic-g1.wasm` is row
  102's, because that row introduces the generation concept the new name encodes.
  Renaming it here would mean naming a generation before the generation exists.
- **Diagnostic renumbering** into doc 87 §17's ranges. Row 101.
- **Any language change.** Nothing in this plan alters what the compiler accepts.

---

## 7. Gate

The standing push gate, plus:

- the from-scratch bootstrap assertion (§3.2 step 4) — new, and the only one of these
  checks that can catch the seed trap
- the packaging validation across all eight crates
- `editors.rs`
- a check that the tree contains no file whose name still carries `sdx`

---

## 8. Emerald reuse

Emerald's one syntax cutover missed **113 fixtures** that lived as string literals
inside test modules, which is why `AGENTS.md` requires every fixture to be a file and
`no_literal_expectations.rs` enforces it. This plan is the first to benefit: the
fixtures are files, so a rename reaches them, and the 30 literals in §2.3 are
*extension* checks rather than embedded programs. That cost was paid once, by Emerald,
and this plan is where the saving shows up.

---

## 9. Decision log

| Decision | Why |
|---|---|
| `.vlt` and `.vrs` | name the language and the language's Rust; three letters, no collision with a common extension |
| Rename the hygiene prefix too | §1.1 — it is the fossil users actually read |
| `git mv`, never delete-and-write | blame on 572 files is worth more than a tidy diff |
| Ignore patterns in the same commit as the moves | §3.1 — the alternative stages thirteen generated trees |
| Accept `.sdx` for one release, with a warning | §3.4 — users' files are not this repository's to break |
| Keep `.sdx`'s *own* transition period at zero inside the tree | a tree with both extensions is a tree where every glob is wrong twice |
| Keep diagnostic numbers, change only the prefix | one churn instead of two; row 101 owns the ranges |
| Keep `__rdx_` reserved after the rename | a program that adopted it while unreserved must keep failing, not start passing |
| Do not touch `docs/01`–`docs/85` | they describe files that were called `.sdx`; rewriting them falsifies the record |

---

## 10. Result

*To be filled after execution.*

# `voltaic-g1.wasm` — the bootstrap seed

Plan 72 (`docs/66 - 72 - The Seed Compiler.md`) leaf 3.

This directory holds **one binary**, and it is the only binary this repository
tracks. It is `crates/voltaic-seed` compiled to `wasm32-wasip1`: `.vlt` in, `.rs`
out, and nothing else — no `check`, no `fmt`, no `watch`, no colour.

## What it is for

`crates/voltaic-ast` and `crates/voltaic-fmt` are **written in Voltaic**. The Rust
under each one's `src/` is generated from the `.vlt` beside it. Something has
to break the resulting cycle, because `voltaic-cli` is built from `voltaic-fmt`,
which is built from `voltaic-ast` — so on a clean clone with those trees
untracked, nothing exists that could generate them.

This blob is that something. Every self-hosting language needs one; what
differs is the form. rustc downloads the previous beta, Go requires a previous
toolchain in a documented chain, OCaml commits bytecode, and Zig commits
`zig1.wasm` — which is the model this follows, including the part that matters
most: **the seed was built from the Rust implementation before that
implementation was replaced.** Zig's C++ compiler produced the first
`zig1.wasm` and was deleted afterwards. A seed built the other way round is
verified by a compiler the seed is supposed to underwrite.

`docs/64 - 70 - The Source Beside the Crate.md` §1 is the full survey.

## Identity

| | |
|---|---|
| built from commit | `667e727` **plus plan 93's rename**, which lands in the same commit as this row |
| compiles | g1 |
| built by | the pre-Voltaic Rust implementation, deleted by plan 72 |
| size | 1005170 bytes |
| blake3 | `0cbf2b3fab61987ae909388209cd4e5b7f53eca130c104447d40af212dd9fc2d` |

`compiles` and `built by` are row 102 leaf 8, and they exist so that §1.2's
invariant is *checkable* rather than only stated:

> A generation's seed must be built by the previous generation's compiler, before
> the previous generation's source is replaced. Equivalently: at no point is the
> only compiler for a language written in that language.

A blob built by itself is exactly what that forbids, so
`crates/voltaic-seed/tests/blob.rs` asserts `built by` never names the generation
in `compiles`. For this blob it names no generation at all, which is the base
case: the compiler that produced it was the committed Rust one, and plan 72
deleted it afterwards — the same shape, and the same order, as Zig's C++ compiler
producing the first `zig1.wasm`.

`voltaic-g2.wasm` will record `compiles: g2` and `built by: g1`, because doc 100
§1.3's ordering writes generation 2's front end **in generation 1**. That is the
one ordering that satisfies the invariant, and the table is where a future reader
can check that it was followed rather than take it on trust.

`crates/voltaic-seed/tests/blob.rs` recomputes the hash and fails if it does not
match this table, so a hand-edited blob and a stale row are the same failure.
It is updated by the test, not hand-edited:

```bash
VOLTAIC_UPDATE=1 cargo test -p voltaic-seed --test blob
```

The update rewrites this table and then **fails**, which is deliberate and is how
every other expectation in this repository behaves: a changed hash is a changed
compiler and has to be looked at rather than absorbed into a green build.

## How to rebuild it

```bash
moon run :seed-refresh
```

which is exactly:

```bash
cargo build --release -p voltaic-seed --target wasm32-wasip1
wasm-opt -Oz --enable-bulk-memory \
  -o seed/voltaic1.wasm \
  target/wasm32-wasip1/release/voltaic-seed.wasm
```

`wasm-opt` is not optional and not cosmetic: the release build is 1,351,423
bytes and `-Oz` takes it to 913,549, so a blob built without it is a third
larger for no reason. `devenv.nix` pins both `binaryen` (for `wasm-opt`) and
`wasmtime` (to run the result).

## When to rebuild it

**Whenever the seed can no longer parse this repository's own `.vlt`.** That is
the version-skew rule, and `AGENTS.md` carries it:

> A surface row that the seed cannot parse must refresh the seed in the same
> commit.

rustc answers the same problem with `cfg(bootstrap)` double implementations and
Go with a version chain bumped about once a year. Both are expensive because
their seeds are *releases*. This one is a build artifact, so the answer is one
command and one re-committed file — and that is only cheap while the seed is
unreleased, which is also the window in which Voltaic's surface still moves.

Refreshing it for any other reason is churn: a new blob is ~900 KB of history
that does not delta-compress, so the rule is a trigger and not a cadence.

## Running it

```bash
wasmtime run --dir . seed/voltaic1.wasm crates/voltaic-ast/vlt <out-dir>
```

`--dir .` is required — WASI grants no filesystem access by default, so without
a preopened directory the seed cannot read a single `.vlt`.

The output is **not** what is committed until rustfmt has run on it. Voltaic
targets rustfmt's defaults and this repository sets `tab_spaces = 2`, so the
pipeline is seed, then the native rustfmt with the repo's config. `bootstrap.sh`
does both.

# voltaic

The Voltaic compiler. `.vlt` in, `.vrs` containing ordinary Rust out.

```bash
cargo install voltaic-cli     # installs the `voltaic` binary
voltaic build hello.vlt       # transpile to hello.vrs
voltaic check hello.vlt       # run rustc, report against the .vlt
voltaic fmt hello.vlt         # canonical formatting
```

Diagnostics are reported against the `.vlt` you wrote, with 1-based line and
column in characters. rustc is the authority on types and borrows — Voltaic
infers them and writes them down, and has no borrow checker or exhaustiveness
checker of its own, by design.

Voltaic is written in Voltaic: the `.vlt` under each crate's `vlt/` is the
source, and the Rust under `src/` is generated from it. A clean clone has no
generated Rust at all and bootstraps from a checked-in seed compiler; see
`seed/SEED.md` in the repository.

- Repository, documentation and the language specification:
  <https://github.com/Industrial/voltaic>
- Releasing and the publish pipeline: `docs/RELEASING.md`

MIT licensed.

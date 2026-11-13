# Diagnostic codes

Every code Voltaic can emit, by band.

> The prefix is `VLT`, from the project's former name, and has not yet been
> renamed to `VLT`. The rename is queued: the codes appear in 73 test
> expectation files, and changing them needs its own
> build-regenerate-reseed cycle.

| Band | Range | What it covers |
|---|---|---|
| Lexical | `VLT0001`–`VLT0013` | Indentation, unterminated literals, bad escapes, malformed numbers, unexpected characters |
| Structural | `VLT0039`–`VLT0063` | Delimiters, module-scope bindings, nesting depth, path separators, newtype and declaration shapes |
| Parse | `VLT0099`–`VLT0127` | Expected token, expression, name or type; empty bodies; retired spellings; operator and clause shapes |
| Lints | `VLT1000`–`VLT1004` | `unwrap` usage, comment-form `allow` pragmas, shadowed bindings, `return` in a callback tail |

The authoritative list is the `Code` enum in `crates/voltaic-diag`, and
`crates/voltaic-cli/tests/ui/` holds one fixture per code with its exact
expected output. That corpus is the specification of what each code *says*,
regenerated with `VOLTAIC_UPDATE=1` so a changed message is always reviewed
rather than absorbed into a green build.

## The ones worth knowing before you hit them

| Code | Says | What it usually means |
|---|---|---|
| `VLT0001` | tab in indentation | Voltaic indents with spaces, with no configurable width |
| `VLT0002` | match an enclosing indentation level | A dedent landed between two open columns — a half-finished refactor |
| `VLT0101` | expected `:` after the name | Very often an `instance` in a different file from its `class` |
| `VLT0103` | expected the module's name | A module named after a keyword — `test`, `data`, `class`, `type`, `errors` |
| `VLT1001` | `unwrap` used | Intended? `#[voltaic::allow(unwrap)]` on the line above |
| `VLT1003` | shadowed binding | Meant to reassign? `:=` and `mut`. Meant to shadow? `#[voltaic::allow(shadow)]` |

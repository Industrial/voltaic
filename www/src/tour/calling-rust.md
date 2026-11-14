# Calling Rust

Everything in Voltaic becomes Rust, so calling Rust is not interop — it is the
normal case. There are three levels, and you will almost always want the first.

## Just call it

Rust paths, types, traits and macros are available as themselves. `import`
brings a path into scope exactly as `use` does.

```voltaic
{{#include ../../../examples/reachability.vlt}}
```

That example is the repository's own map of what a Voltaic program can reach,
and it is in CI, so it is the honest answer rather than an aspiration.

## Method calls put the receiver first

`x.max 3` is `max(x, 3)` — the receiver leads, so a chain reads left to right.
For a call with no arguments write the `()`: `x.len ()`. Those parentheses are
the whole difference from a field read, since `x.len` on its own would select a
field called `len` — unless inference can tell that `len` is a method you
declared, which it can for your own types but not for `std`'s.

A `.` in **head** position is the older receiver-last form and still works:
`.len xs` is `xs.len()` too. That is what lets a method compose with parenless
application and with the pipeline operator.

A bare `(.f)` is a *field* section — it emits a closure reading the field `f` —
and is not a method reference, which is a distinction worth knowing before you
try to eta-reduce a call.

## The escape hatch

When a construct has no Voltaic spelling, write Rust directly:

```voltaic
rust """
  pub fn escaped() -> String {
      String::from("two\nlines")
  }
"""
```

The body is emitted **verbatim and undecoded** — the `\n` above stays two
characters. Nothing inside a fence is parsed, which means nothing inside it is
checked by Voltaic either: `rustc` is the only authority on a fence's contents,
and `voltaic check` is how you hear its verdict.

Use it for what has no surface yet. The eight constructs that currently have
none are listed in [Declined and unreachable](../reference/declined.md), each
with a fixture that CI asserts **still fails** — so when a gap closes, the
build breaks until the documentation is updated too.

## What you get back

Nothing is wrapped. A Voltaic `data` becomes a Rust `enum` or `struct` with the
same name and the same fields; a Voltaic function becomes a Rust `fn`. Other
Rust code can call it without knowing Voltaic was involved, and
`examples/cargo-project/` is a worked example of exactly that — a normal cargo
crate whose build script transpiles `.vlt` into `OUT_DIR`.

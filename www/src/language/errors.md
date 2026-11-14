# Errors

Rust's `Result` and `Option`, with the three operators that remove the
boilerplate around them.

```voltaic
{{#include ../../../examples/errors.vlt}}
```

## `?` propagates

```voltaic
{{#include ../../../examples/fallible.vlt}}
```

As Rust's. One trap, found the hard way: **`?` binds inside a parenthesised
argument**, so where you put the parentheses changes what is propagated.

## `??` and `??=`

```voltaic
{{#include ../../../examples/coalesce.vlt}}
```

`a ?? b` is `a` if it is present and `b` otherwise. `??=` assigns only if
absent. Note `Some *lint` multiplies — a prefix `*` after a constructor needs
parentheses, which is the kind of thing the formatter will fix for you once and
you will then never think about again.

## `catch`

Handles an error in place, as an expression, rather than unwinding a call
stack. Voltaic has no exceptions; `catch` is sugar over `Result`.

## `try` blocks

Scope a run of `?` operations so that the first failure leaves the block rather
than the whole function.

## Typed domain errors are `data`

```voltaic
{{#include ../../../examples/error_data.vlt}}
```

A `data` declaration whose variants carry a **trailing string literal** is an
error type: the message is the literal, and the enum, its `Display` and its
`Error` implementation all come from the shape rather than being written out.
`Invalid String u32 "invalid value #{0} at line #{1}"` numbers its payload
positionally. The `Error` impl is what makes the type usable as a
`Box<dyn Error>`, and the example asserts it by compiling — a missing one does
not coerce.

Generation 1 spelled this as a dedicated `errors` declaration, the only
construct in the language that emitted more than one Rust item. It is deleted as
a *construct* and kept as a *shape*: identical output, one construct fewer. The
keyword is still accepted while both forms are exercised, and
`examples/errors.vlt` above is the older spelling of this same type.

## `unwrap` is discouraged, not forbidden

`.unwrap` raises `VLT1001`. If the panic is intended, say so on the line above
with `#[voltaic::allow(unwrap)]` — an attribute the compiler consumes, so it
never reaches rustc, which would reject it as an unknown lint.

## Optional chaining and the panic family are retired

Both existed and both are gone. The reasoning is in
[Declined and unreachable](../reference/declined.md).

---

The full statement, with every decision and its reasoning, is `docs/03 - Voltaic Surface Design.md`
section 12, *Errors*.

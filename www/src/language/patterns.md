# Patterns

Rust's patterns, plus the forms that let a pattern be used where Rust needs a
match arm.

```voltaic
{{#include ../../../examples/case.vlt}}
```

## `case … of`

The multi-arm form, shown above. Arms are indented under the scrutinee and the
off-side rule ends the construct — there is no closing brace and no comma
between arms.

For the same dispatch **in the middle of a body**, a guard ladder arm may be a
pattern row: see [Bindings and control flow](bindings.md). Multi-clause
definitions are pattern rows on a function's parameters, and have lowered to a
`match` since generation 1.

## Relational and logical patterns

A pattern may be a comparison (`> 0`) or a combination (`> 0 and < 10`), which
removes the most common reason to write a guard. Guards remain for the cases
that need a whole expression:

```voltaic
{{#include ../../../examples/guards.vlt}}
```

## Record patterns and destructuring

```voltaic
{{#include ../../../examples/destructuring.vlt}}
```

Field punning works: naming a field binds it to a variable of the same name.
The `...` rest pattern is **declined** — see
[Declined and unreachable](../reference/declined.md) — as are `ref` and
`ref mut`, and `|` or-patterns.

## Pattern synonyms

```voltaic
{{#include ../../../examples/synonym.vlt}}
```

A pattern given a name and reused. Rust has no pattern abstraction at all — a
pattern cannot be named, reused or parameterised, and `macro_rules!` does not
reach the ergonomics — so this is expressiveness Rust has no counterpart for,
and it costs **no type knowledge whatsoever**. The lowering is textual
substitution at emit time, so rustc performs every check it would have on the
written-out pattern.

## Exhaustiveness is rustc's

Voltaic does not check that a `case` covers its scrutinee, and will not grow a
checker that does. Generation 2 added an inference engine, which computes what
the emitter writes down; deciding whether a program is *well-typed* stays
rustc's, and so does exhaustiveness. Rust already errors on a non-exhaustive
match, against your `.vlt` through the diagnostic remap, so a second checker
could only disagree with the first.

---

The full statement, with every decision and its reasoning, is `docs/03 - Voltaic Surface Design.md`
section 6, *Patterns*.

# Bindings and control flow

```voltaic
{{#include ../../../examples/bindings.vlt}}
```

## Binding, and rebinding

`x = e` binds. `x := e` reassigns, and requires `mut` on the original binding.
Shadowing a name with `=` is a warning (`VLT1003`), suppressed with
`#[voltaic::allow(shadow)]` on the line above — the attribute is consumed by
the compiler and never reaches rustc.

## `guard`

The early-return form. A `guard` whose condition fails runs its `else` branch,
which must diverge. It is the construct that keeps the happy path at one
indentation level instead of nesting every precondition.

## The guard ladder

```voltaic
{{#include ../../../examples/ladder.vlt}}
```

A multi-branch conditional is a **guard ladder** — `| condition = value` rows
with `| _ =` as the catch-all — and it is admissible **wherever a value is**,
not only as a clause body. Nothing delimits it: the dedent ends it, which is
why a construct that needs `if`/`else`/`end` in other languages needs neither
bracket nor keyword here.

## Pattern rows

```voltaic
{{#include ../../../examples/rows.vlt}}
```

A ladder arm may be a **pattern row** — `| held is Some _n = "some"` — which is
what an inline `case` was for. Multi-clause definitions already give you
pattern rows on a function's parameters; this is the same dispatch in the
middle of a body, where multi-clause dispatch cannot reach.

## A binding with an `else` tail

```voltaic
{{#include ../../../examples/else_tail.vlt}}
```

`Some v = .pop (&mut rest) else break` is Rust's let-else, stated explicitly
like everything else Voltaic emits. The tail takes any diverging statement, and
`return` is the one that makes this the form for an early exit:
`Some head = .first xs else return None`.

## `with` — receiver-implicit mutation

Inside a `with` block the receiver is implicit, so a run of mutations on one
value reads as a list of operations rather than as the same name repeated.

## Named bind — callback flattening

```voltaic
{{#include ../../../examples/bind.vlt}}
```

`do .method` is a block modifier, and `<-` inside it binds through the method
the modifier names — so a pyramid of callbacks becomes a sequence of steps.
This is the construct [Declined and unreachable](../reference/declined.md)
refused as a *general monadic comprehension*, reopened on the terms that
document set: the **modifier** is the type-free discrimination, so Voltaic
chooses nothing between `flat_map` and `and_then` — it transcribes the method
you named. The final row is written out for the same reason, rather than having
a `pure` inserted for you.

## Blocks, labels and loops

```voltaic
{{#include ../../../examples/local.vlt}}
```

Block expressions have values. Labels name a loop or block so a `break` can
leave a specific one, carrying a value out if it has one.

---

The full statement, with every decision and its reasoning, is `docs/03 - Voltaic Surface Design.md`
section 7, *Bindings and control flow*.

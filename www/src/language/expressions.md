# Expressions and operators

The chapter with the most surface in it, and one idea underneath: application
should be cheap to write and cheap to read, so parentheses are for grouping
rather than for calling.

```voltaic
{{#include ../../../examples/expressions.vlt}}
```

## Parenless calls are canonical everywhere

`f x y` is `f(x, y)`. Not an alternative spelling — the canonical one, which
`voltaic fmt` will produce. Parentheses group; they do not call.

The consequence worth internalising: **an argument extends as far as it can**.
`escaped ().len ()` reads as `escaped(().len())`, not as
`escaped().len()`. Receiver-last method calls exist partly to remove that
ambiguity, because there is then no stray `()` for the parser to attach
somewhere surprising.

## Zero-argument calls

A nullary function is declared `name : ReturnType` and called as `name`. A
signature of `: () -> T` declares a function taking *unit*, which is a
different thing — and is the mistake to expect if you come from Rust.

## A bare `.name` is resolved by the solver

```voltaic
{{#include ../../../examples/dot_resolution.vlt}}
```

`c.pos` is a field read and `c.depth` is a nullary method call, and **nothing
syntactic separates them** — inference decides. `x.f y` needs no help, because a
field access takes no arguments and so the form is a call whatever the types
are; `x.f` is the ambiguous one.

Generation 1 could not choose, which is why `.` was made field-access-only and
[Declined and unreachable](../reference/declined.md) recorded UFCS as *requiring
type knowledge to choose a shape*. Inference has that knowledge, and the decline
is lifted.

One limit worth knowing: this resolves for a method **declared in your
program**. For a `std` method, write the `()` — `s.len ()` — which needs no
solver at all.

## The pipeline operator

```voltaic
{{#include ../../../examples/compose.vlt}}
```

`x |> f` passes `x` as `f`'s **receiver**, not as its first argument. That is
what makes a pipeline of method calls read left to right without the receiver
jumping position at every step.

## Operator sections

```voltaic
{{#include ../../../examples/section.vlt}}
```

`(+ 1)` is a closure adding one. `(.f)` is a **field** section — it emits
`|x| x.f`, with no call — which is the one trap here: `.f x` is the method call
`x.f()`, so `(.f)` and `.f` are different constructs and eta-reducing a method
call to a field section is a type error, not a simplification.

## Operator declarations

```voltaic
{{#include ../../../examples/operator.vlt}}
```

`infixl 6 <+> = merge` gives an **existing** function a spelling and a fixity,
so `a <+> b` is `merge(a, b)`. Nothing is declared and nothing is renamed — no
item is emitted — which is what keeps go-to-definition pointing somewhere real.

The spelling set is **closed**: twelve `<` · *char* · `>` forms, which is what
keeps the lexer static. An undeclared use is `VLT0123`. Precedence is Haskell's
0–9, mapped so every user operator binds looser than arithmetic, and `infix` is
non-associative by diagnostic (`VLT0125`) because a Pratt loop's binding powers
can only group one way.

A Haskell-style *definition* stays out of reach: no Rust function can be named
`<+>`, and inventing one would be a fabricated API.

## `is`, and chained comparison

`x is Pattern` tests a pattern and yields a bool, which is what lets a pattern
appear in a condition without a `switch`. And `a < b < c` means what it reads
as, rather than comparing a bool against `c`.

---

The full statement, with every decision and its reasoning, is `docs/03 - Voltaic Surface Design.md`
section 5, *Expressions and operators*.

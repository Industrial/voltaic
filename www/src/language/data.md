# Data declarations

```voltaic
{{#include ../../../examples/data.vlt}}
```

## Fields are `name Type`

```voltaic
{{#include ../../../examples/juxtaposed.vlt}}
```

A field is `name Type`, juxtaposed, with no `:` — and a record needs no
constructor head, so `data Point { x f64, y f64 }` is the whole declaration.
The constructor takes the type's own name.

Two reasons, and the second is the one that pays. It removes a token per field,
which over a corpus is the largest repeated-token class after signature lines.
And it frees `:` for **ascription**, which generation 2 needs wherever a type is
stated against an *expression* rather than declared beside a name.

The `:` form still parses — `data Point = Point { x : f64, y : f64 }` emits the
same struct — because the compiler's own sources are written in it. The accepted
surface widens first and narrows once the tree catches up.

## One keyword for both

`data` declares a struct or an enum — which one depends on whether it has
variants. Every `data` declaration emits `use <Name>::*` so that its
constructors are in scope unqualified, and that single fact has a consequence
worth knowing before you organise files: **two enums sharing any variant name
cannot share a module.** `TokenKind::Or` and `Keyword::Or` in one file is
`E0659`, and a pattern match on the bare name then binds instead of matching.

## Records

```voltaic
{{#include ../../../examples/record.vlt}}
```

Field punning in initialisation as well as in patterns: a field whose value is
a variable of the same name needs naming only once.

## Updating through a path

`over` updates a nested field and returns the updated value, which is the
construct that replaces a chain of intermediate bindings. It is also the only
place the grammar reaches a method reference.

```voltaic
{{#include ../../../examples/optic.vlt}}
```

## Derives are computed

```voltaic
{{#include ../../../examples/derives.vlt}}
```

`Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash` are derived for every
`data` whose fields all support them, and `deriving (…)` is **not written**.
`data Point { x u64, y u64 }` gets all eight. `data Named { label String }` gets
seven, because `String` has no `Copy`. `data Scaled { ratio f64 }` gets five,
because an `f64` has no `Eq`, `Ord` or `Hash` — an intersection over the fields
rather than a special case for floats.

`data !Copy Token { kind u64 }` opts out by name, because move semantics are
sometimes the contract.

The direction is deliberately conservative: the emitter reads a field's type as
**text**, so a type it has no table for gets **nothing**. That is when you write
`deriving (Debug, Clone)` or a Rust `#[derive(…)]` yourself, both of which are
still accepted. Voltaic adds no derive macros of its own.

## Visibility

```voltaic
{{#include ../../../examples/visibility.vlt}}
```

`export` is `pub`; `export(crate)` is `pub(crate)`. Default is private, and
private means *visible in the defining module and its descendants* — the same
rule as Rust's, which matters when flattening a module tree moves an item away
from the code that used it.

## Constants, aliases, unit and tuple structs

```voltaic
{{#include ../../../examples/constants.vlt}}
```

---

The full statement, with every decision and its reasoning, is `docs/03 - Voltaic Surface Design.md`
section 9, *Data declarations*.

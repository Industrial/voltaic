# Types, generics, async and traits

All four are Rust's, spelled for the off-side rule. Voltaic adds no type
system of its own and infers nothing Rust would not.

```voltaic
{{#include ../../../examples/generic.vlt}}
```

## Type expressions

Rust's type grammar: paths, references, generics, tuples, slices, `impl Trait`,
`dyn Trait`. Written in signatures after `:` and between `->`s.

```voltaic
{{#include ../../../examples/ascription.vlt}}
```

Ascription pins a type at an expression. The parenthesis in
`(Box::new h) : T` is load-bearing in the *source* — it ascribes the call
rather than `h` — and is not needed in the output.

## `where`, in its other role

In a signature, `where` carries Rust's predicates verbatim:
`where I::IntoIter: Clone`. Position is what distinguishes this from the
local-definition `where` of [Functions](functions.md), and the two cannot
appear in the same place.

## `given` — the verbatim generic list

```voltaic
{{#include ../../../examples/binder.vlt}}
```

Normally the generic list is **synthesised**: a lowercase type name is a type
variable, and the list is the order of first appearance. That covers every
signature in every other example here, and it is still the default.

`given <…>` is the hatch for the shapes synthesis cannot reach —
`given <const N: usize> [u8; N] -> u8`, `given <T: ?Sized>`,
`given <U: From<T>, T>` where the declaration order is not the appearance
order. The clause is Rust's own parameter list, verbatim, so every present and
future Rust binder form is reachable and nothing can drift.

The rule is **all or nothing per signature**: a signature carrying `given` gets
no synthesised generic list, no `=>` bounds and no type rewrite, so its
lowercase atoms are ordinary verbatim Rust types. That is what makes the `N` in
`[u8; N]` the one the author declared.

## Classes and instances

```voltaic
{{#include ../../../examples/class.vlt}}
```

`class` is `trait` and `instance` is `impl`. The one rule that is not Rust's:
**an `instance` must be in the same file as its `class`.** The parser records a
class's member signatures when it parses the declaration and reads them back
when it parses an instance member, and each file gets a fresh parser. An
instance elsewhere fails with `VLT0101` on every name in the line, which looks
nothing like the real cause.

It is a constraint rather than a defect: the parser needs a member's *shape*,
not its types. An **inherent** `instance` has no class to read, so its members
infer instead and may omit their signatures entirely — see
[Functions](functions.md).

## What inference does, and what it does not

Voltaic infers types, parameter modes and effect rows, and that is what lets a
signature be omitted. What it computes is **what the emitter writes down** — the
output states every type, borrow and lifetime explicitly, which is what keeps
rustc an independent check rather than a rubber stamp.

Deciding whether a program is *well-typed* is not inference's job and stays
rustc's. There is no borrow checker and no exhaustiveness checker here, and
there will not be one. A program inference accepts and rustc rejects is a **bug
in Voltaic**, filed as one — not a mistake on your part.

## Async

```voltaic
{{#include ../../../examples/async.vlt}}
```

`async` and `await` as Rust's, with no executor supplied. Pick one from
crates.io — the same answer Voltaic gives for every library question.

---

The full statement, with every decision and its reasoning, is `docs/03 - Voltaic Surface Design.md`
section 14, *Types, generics, async and traits*.

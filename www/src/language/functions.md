# Functions

```voltaic
{{#include ../../../examples/application.vlt}}
```

## Signature, then definition — and the signature is optional

```voltaic
{{#include ../../../examples/inferred.vlt}}
```

`name : A -> B` then `name a =`. The signature is **optional**: omit it and the
judgement is inferred, which `voltaic sig` prints back to you —

```text
scaled : u64 -> u64 @ n shared
doubled : u64 -> u64 @ n owned
main : () ! io
```

— with the parameter modes after `@` and the effect row after `!`.

What decides whether you may omit one is **inferability, not visibility**: an
`export`ed definition needs no signature either, so long as inference can reach
a type. Each unsigned definition has to reach a *written* type somewhere. Above,
`scaled` is annotated, so `doubled` learns `u64 -> u64` from the call alone and
`quadrupled` learns it from `doubled`. A definition that reaches no written type
anywhere is `VLT0128`, *"cannot infer a signature"* — not a gap, because rustc
needs an annotation to break the same cycle.

An **inherent `instance` member** may omit its signature too, receiver mode
included: `current self = self.value` emits `fn current(&self) -> u32`, and
`bump self = self.value += 1` emits `fn bump(&mut self)`. A member of a *class*
instance instead takes its signature from the class declaration, which is why
**an `instance` must sit in the same file as its `class`** — the parser learns
the member's shape there, and each file gets a fresh parser.

## Multi-clause definitions

A function may be defined by several clauses whose patterns select between
them, which is the form that makes a recursive definition read as its own base
case plus its step.

## `where` — local definitions

```voltaic
{{#include ../../../examples/wheres.vlt}}
```

`where` after a body introduces definitions local to that body. This is the
construct that flattens nesting: a helper used twice goes below the thing that
uses it, at one level of indentation, rather than above it at the top level or
inside it as a closure.

`where` has a **second, unrelated role** in a signature, where it carries
Rust's own predicates verbatim. The two are told apart by position, which is
the kind of overload the surface design admits only when the positions cannot
collide.

## Method calls

```voltaic
{{#include ../../../examples/receiver_first.vlt}}
```

`x.f y` is `f(x, y)` — **the receiver first** — so a chain reads left to right
and needs none of `|>`, `>>>`, `<<<` or `$`. Three forms, and position decides
which: `recv.name` with nothing after it is a **field read**; `recv.name args`
is a method call; `recv.name ()` is a method call taking **no** arguments, the
parentheses being the whole difference from the field read, since `h.items.len`
would select a field called `len`.

Zero-argument links chain with nothing between them —
`h.items.len ().count_ones ()`. A link that *carries* arguments needs its
receiver bracketed, because `(g x).y` is a legitimate expression and there is
nothing type-free to decline: `((h.items.iter ()).map (\n -> *n * 2)).collect ()`.

A `.` in **head** position is still a call with the receiver **last**, so
`.len xs` and `.insert k v m` remain live. Generation 2 inverted the postfix `.`
and left the prefix form alone: generation 1 chose receiver-last because it was
the only way to make partial application and pipelines share a spelling
*without types*, and inference removed that constraint.

A method whose name is a Voltaic keyword is unspellable, which is the one sharp
edge here.

## The receiver in a signature

```voltaic
{{#include ../../../examples/receiver.vlt}}
```

`self` in a signature is the receiver **wherever in the signature it is
written**, and the borrow prefix on the marker chooses between Rust's three
forms — `self`, `&self`, `&mut self`. Omit the signature and the mode is
inferred instead.

## Currying

```voltaic
{{#include ../../../examples/currying.vlt}}
```

An under-applied call is a closure, and the signature's arrows state the arity:
`add 1` is one argument short of two, so it lowers to a closure of the rest.
`add 1 2` is saturated and stays a single call — full application is not
currying and does not pay for it.

## Closures

`\x -> e` is a closure. A parameter is a **bare name** — not a pattern and not
an annotated binding — which is worth knowing because the Rust you are
translating from often annotates its closure parameters for inference.

## Default arguments remain declined

Labelled arguments exist; defaults do not. The reasoning is in
[Declined and unreachable](../reference/declined.md).

---

The full statement, with every decision and its reasoning, is `docs/03 - Voltaic Surface Design.md`
section 8, *Functions*.

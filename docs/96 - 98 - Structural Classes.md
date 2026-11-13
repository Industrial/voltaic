# Plan 98 — Structural Classes

Plan ID: `classes`
Depends on: 94
Milestone: 16 — *the surface*
Status: `planned`

---

## 1. What this plan is

`class` becomes a requirement list. The `instance` header is deleted, the restated
member signature with it, the one-file trap with it, and `deriving template` with it.

```
class Shape
  SIDES u32
  area self -> f64
  describe self = "a shape"

SIDES Square = 4
area (s Square) = s.side * s.side
describe (s Square) = "a square"

SIDES Circle = 0
area (c Circle) = 3.0 * c.radius * c.radius
```

A type satisfies `Shape` iff it has the members `Shape` requires. A parameter pattern
`(s Square)` attaches the equation to the type; which class it satisfies is a
consequence rather than a declaration.

### 1.1 Generation 1 documented this as its own deviation

`examples/class.sdx`'s header comment:

> An instance method **restates its signature**, which doc 18 §2.5's own example does
> not. Rust's `impl` restates it too, and Voltaic has no name resolution to copy one
> from the class — AGENTS.md forbids growing any. Doc 18 §2.5 records the deviation.

So generation 1 knew the restatement was a cost, named the cause — no name resolution
— and recorded it as a deviation from its own specification rather than defending it.
Row 92 removed the cause. This row removes the deviation.

Measured: `examples/class.sdx` is 29 code lines and 247 tokens; its port is 22 lines
and 137 tokens. **1.80×**, and the restated signatures plus the two `instance` headers
are most of it.

### 1.2 The one-file trap is deleted, not documented

`AGENTS.md` records it:

> **An `instance` must be in the same file as its `class`.** `Parser` carries
> `class_heads: BTreeMap<(String, String), SignatureHead>`, filled by `record_head`
> when a `class` is parsed and read by `borrowed_head` when an instance member is
> parsed — so the parser learns a member's signature from the class declaration, and
> **each file gets a fresh `Parser`**. An instance in another file parses as a run of
> top-level declarations and fails with `RDX0101` *"expected `:` after the name"* on
> every name in the line, which looks nothing like the actual cause.

It is honestly labelled there as *"a constraint, not a defect"*, and under the old
charter that was right: the parser needed the member's shape, not its types, so it did
not cross the no-type-checker line.

With resolution in `voltaic-lattice` the constraint evaporates. An equation and the
class it satisfies may live in different files, and `class_heads` leaves the parser
(doc 88 §2). This is the clearest case in the generation of a trap that was never a
design choice — it was the absence of resolution showing through, and the plan that
found it said so.

### 1.3 Deriving templates are subsumed

Generation 1's `deriving template` is an `instance` over the reserved type `Target`,
substituted into every declaration that lists it. It exists so one file and four lines
can replace a proc-macro crate.

Structural classes do the same thing directly: an equation whose receiver pattern is a
**class constraint** rather than a concrete type applies to every type satisfying it.

```
fmt (s impl Display) f = Display::fmt (s.Field0) f
```

What the template could do and this cannot: reach `Fields` / `FieldNames` — every
field of a declaration as a `self` access and as a string literal. That is reflection
over a declaration's shape, and doc 87 §22.4 declines it, naming the venue as the
`macro` construct that is itself out of this generation.

**So this row loses a capability**, and it is a real one: a template visiting
declarations of any width is what let generation 1 write one `Display` for many types.
The honest accounting is in §6.

---

## 2. Preconditions

- Row 94 `done`. Resolution and the type engine; a receiver pattern is attached by
  type, so this row cannot exist before inference.
- `examples/class.vlt` renamed by row 93 and green under generation 1.

---

## 3. Concrete proof this plan targets

`examples/class.vlt`, ported, running with output byte-identical to generation 1's
`examples/class.stdout`. Seven printed lines: two areas, two `describe`s — one
overridden, one defaulted — a `perimeter` from an inherent member, and two associated
constants reached through the type.

The defaulted `describe` on `Circle` is the case that matters: a class default that
did not reach the type would print the wrong word rather than fail to compile, which
is why generation 1 checked this file by running it.

---

## 4. Leaves

### Leaf 1 — `class` as a requirement list

Associated constants, method signatures, and optional default equations. **The one
place in Voltaic 2 where a signature is written**, because the requirement is the
contract and there is nothing to infer it from.

### Leaf 2 — receiver patterns

`(s Square)` in a parameter position: the binder, the type, and the attachment rule.
Includes the inherent case — an equation with a receiver pattern and no class — which
is generation 1's `instance Square` with no class name.

### Leaf 3 — satisfaction

A type satisfies a class iff it has every required member with a compatible
judgement. Produces the `impl Shape for Square` block in the emitted `.vrs`. A type
that has *some* of a class's members and is used where the class is required gets a
diagnostic naming the missing ones — `VLT03xx`, resolution.

### Leaf 4 — class constraints as receiver patterns

§1.3's `(s impl Display)` form, which is what subsumes the template.

### Leaf 5 — remove `class_heads` from the parser

§1.2. The parser stops learning signatures; resolution moves to `voltaic-lattice`.
Includes a fixture proving an equation and its class in **different files** now work,
because a deleted trap with no test is a trap waiting to come back.

### Leaf 6 — supertraits

`class Debug => Tally` becomes a constraint list on the class. Unchanged in meaning
from generation 1; the spelling follows whatever row 99 settles for constraint lists,
and this leaf defers to it rather than inventing a second syntax.

---

## 5. Acceptance criteria

1. `examples/class.vlt` runs with output byte-identical to generation 1's `.stdout`,
   the defaulted `describe` included.
2. An equation and its class in different files work, with a fixture.
3. `class_heads` is gone from `voltaic-parser`, and `RDX0101`-on-every-name is no
   longer reachable from a cross-file instance.
4. The emitted `impl` blocks are byte-identical to generation 1's for every corpus
   program, so this row changes the source and not the output.
5. A type missing a required member produces a diagnostic naming the missing members,
   not a unification failure.
6. `deriving template` is removed from the lexer, the parser, the emitter and the
   grammar, and `examples/template.vlt` is either ported to §1.3's form or moved to
   `tests/unreachable/` with the capability loss recorded. §6 decides which.

---

## 6. Out of scope, and the capability this row loses

**Declaration reflection.** `Fields` and `FieldNames` — every field of a declaration
as an access and as a string — have no replacement in this generation. Doc 87 §22.4
declines it; the venue is a `macro` construct that is itself out of this generation.

The accounting, stated plainly rather than buried:

- generation 1's `deriving template` can write one `Display` impl for declarations of
  any width, with no proc macro and no second compilation unit;
- generation 2's class-constraint receiver pattern can write one impl for every type
  satisfying a constraint, but cannot enumerate a type's fields;
- so a template that only uses `Field0` ports directly, and a template that uses
  `Fields` or `FieldNames` **does not port**.

`examples/template.sdx` must be read before this row is approved, and if it uses
`Fields` then criterion §5.6 resolves to "moved to `tests/unreachable/`" and the loss
is recorded in `docs/GRAMMAR.md` by row 104 as a construct generation 2 cannot
express. **A capability regression that is graded is a debt; one that is quietly
dropped is a lie**, and generation 1's own habit of asserting unreachable fixtures to
*keep failing* is the mechanism for saying so.

Also out of scope: trait objects (`dyn`) beyond what passthrough already gives; blanket
impls with where-clauses more complex than a constraint list; specialisation, which
rustc does not have on stable.

---

## 7. Gate

Standing gate, plus `examples/class.vlt`'s run expectation, plus the cross-file
fixture, plus §5.4's identical-output assertion.

---

## 8. Emerald reuse

None. Emerald's trait story was nominal and single-file, and this row's whole content
is the consequence of not being either.

---

## 9. Decision log

| Decision | Why |
|---|---|
| Structural satisfaction, not a declared instance | deletes the header and the restated signature; generation 1 called the restatement its own deviation |
| Keep signatures in the `class` body | the requirement is the contract; there is nothing to infer it from |
| Receiver pattern `(s Square)` carries the attachment | one notation serves the class case and the inherent case, as generation 1's `instance` did |
| Delete `class_heads` with a cross-file fixture | a deleted trap with no test is a trap waiting to come back |
| Delete `deriving template` | subsumed, except for reflection, which is declined with a venue |
| Record the reflection loss as an unreachable fixture | a graded regression is a debt; a silent one is a lie |
| Defer the constraint-list spelling to row 99 | two syntaxes for one idea is what generation 1 did with application order |

---

## 10. Result

*To be filled after execution. Including: whether `examples/template.sdx` used
`Fields`, and therefore which way §5.6 resolved.*

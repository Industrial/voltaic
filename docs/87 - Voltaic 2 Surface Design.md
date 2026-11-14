# Voltaic 2 — Surface Design

Status: normative for the surface. Supersedes `docs/03 - Voltaic Surface Design.md`
where the two disagree; doc 03 stays as the record of generation 1's surface and is
not edited.

Doc 86 specifies the language. This document is the surface: every construct, its
lowering, the admission test it passes, and — in §22 — what is refused and why.

The admission test is doc 86 §2's, five clauses. Every row below cites the clause it
turns on when the choice was close.

---

## 1. The shape of a program

A `.vlt` file is a sequence of declarations. There is no module header, no import
block and no visibility block. Layout is significant; there are no braces and no
semicolons.

```
data Cursor { src String, pos usize }

new src = Cursor { src, pos 0 }

pub peek self = self.src.get(self.pos .. self.src.len).unwrap_or("").chars.next

pub bump self =
  ch = self.peek
  self.pos += ch.len_utf8
  ch

test "bump advances by one character"
  c = Cursor.new "ab"
  c.bump
  c.pos == 1
```

Five things in that file are the whole of the generation's change, and each has its
own section below: no signatures (§2), fields by juxtaposition (§4), no `&self`
(§7), receiver-first dots (§5), and `ch = self.peek` carrying the `none` effect
implicitly (§6).

### 1.1 The off-side rule is kept, and it was measured

Generation 1's layout rule survives unchanged: indentation opens blocks, a
continuation line indents, and a guard ladder and a `where` group read as outlines.
It is kept because §1.1 of doc 86 measured where generation 1's terseness actually
came from, and layout was the half that worked — 1.544× in lines against 1.002× in
characters. Braces were the thing worth deleting and they are still deleted.

### 1.2 One application order, not three

Generation 1 has three spellings for applying a function to an argument:

| Spelling | Means |
|---|---|
| `f x` | `f(x)` |
| `.f x` | `x.f()` — the receiver is **last** |
| `x \|> .f` | `x.f()` — the receiver is **first** |

So `.len self.src` reads right-to-left in a language whose pipelines read
left-to-right, and the same idea has two spellings that disagree about direction.
Generation 1 chose this because a receiver-last method was the only way to make
partial application and pipelines share a spelling *without types* — doc 03 §2
clause 1 forbade UFCS for exactly that reason, and doc `DECLINED.md` records it:
*"UFCS (Nim) — requires type knowledge to choose a shape"*.

Lattice has types. So generation 2 has **two** spellings and they agree:

| Spelling | Means |
|---|---|
| `f x` | `f(x)` — a free function |
| `x.f y` | `f(x, y)` — the same function, receiver first |

A free function and a method are the same thing, resolved by the receiver's inferred
type. `|>`, `>>>`, `<<<` and `$` are all deleted (§5.3), because a `.` chain that
breaks across lines is what every one of them existed to spell.

---

## 2. Definitions

### 2.1 No signature line

```
tier s | s > 10k = "platinum"
       | s > 1k  = "gold"
       | _       = "basic"
```

No `tier : u64 -> &'static str`. The judgement is inferred and printed on demand
(doc 86 §8.1). This is the single largest deletion in the generation: 8.7% of
generation 1's source lines were signature lines, and every one of them wrote its
definition's name a second time.

Clause 1 (solver-determined) is what admits it, and clause 1's second sentence is
what bounds it: the inferred signature must be printable, which is why
`voltaic sig` and the sidecar are not optional extras but the condition of
admission.

### 2.2 Pattern rows and guards, unchanged

```
fib 0 = 0
fib 1 = 1
fib n = fib (n-1) + fib (n-2)

step 0 = 0
step n | n > 100 = 100
       | _       = n
```

Several equations become a `match`; a `|` ladder becomes an `if`/`else if` chain; a
ladder inside an equation lands inside that clause's arm. All exactly as generation 1
lowered them — this part was already right and is carried over without change.

`otherwise` is deleted. The catch-all is `_`, which is already the wildcard pattern,
so the language has one spelling for "anything" instead of two.

### 2.3 Or-patterns

```
is_space ' ' | '\t' | '\n' = true
is_space _ = false
```

Generation 1 grades `|` or-patterns `UNREACHABLE` and holds
`tests/unreachable/or_pattern.vlt` asserted to keep failing. The reason is positional:
`|` was the guard token in the same syntactic slot, and without types the parser
could not tell a second pattern from a guard expression. With the solver present a
pattern position is known to be a pattern position, so `|` inside a pattern and `|`
starting a guard are unambiguous.

> **Corrected 2026-11-14, during plan 94 leaf 1.** This section read as though
> or-patterns were a generation-2 *addition*. They are not. **Generation 1 already has
> them, spelled `or`** — `crates/voltaic-ast/vlt/expr.vlt` line 85 is a single `case`
> arm whose body serves forty-odd patterns joined by `or`, and it lowers to Rust's `|`
> or-pattern with one body. `or` is in generation 1's keyword list for exactly this.
>
> So what this row delivers is the **`|` spelling** — the same construct, written the
> way a Rust reader expects. That is still worth having, because plan 93 §10.2 measured
> what its absence costs: `Some "vlt" | Some "sdx" ->` silently became
> `Some("vlt") if Some("sdx")`, and only a type error caught it. A spelling that looks
> available and means something else is worse than one that is plainly absent.
>
> `docs/86 - Voltaic 2.md` §10 carries the same correction and drops its closure count
> from three to two. `tests/unreachable/or_pattern.vlt`'s own comment is wrong on the
> same point — it claims *"what has no spelling is Rust's binding case, where one arm
> body serves several patterns"* — and row 104 owns it.

Clause 4 (reachability-positive) is therefore **not** what admits this row; clause 1
is. The construct is reachable either way, and what the solver buys is that the obvious
spelling stops meaning something else.

### 2.4 Local bindings and `where`

```
retries raw =
  n = raw.parse : u32
  n + 1

grid = pairs.collect
  where pairs = [(x, y) | x in 1..3, y in 1..3]
```

A bare `name = expr` line inside a body is a `let`. `mut name = expr` is `let mut`.
`where` attaches a group of bindings after the body, as in generation 1, and the
bindings are independent — a `where` group is not a `let` chain and may not refer to
itself.

### 2.5 Holes

The application hole survives:

```
m.insert k _          # |v| m.insert(k, v)
(.len)                # |x| x.len()
```

`_` marks the argument a closure still awaits, in any position. With receiver-first
dots the two generation-1 subtleties disappear: `(.trim)` is unambiguously
`|x| x.trim()` because a method is a function and the receiver is the first argument,
so there is no selector-versus-method distinction to get wrong. Generation 1's own
README records that it had this table wrong until plan 85 measured it; the
receiver-first order is what removes the trap rather than documenting it.

Field access is `.` too, and a bare field reference is `(.name)` — resolved by the
solver, which knows whether `name` is a field or a method. Clause 1.

---

## 3. Classes

### 3.1 Structural, with one written signature

```
class Shape
  SIDES u32
  area self -> f64
  describe self = "a shape"
```

A `class` body is a requirement list: associated constants, method signatures, and
optionally a default equation. **This is the one place in Voltaic 2 where a signature
is written**, and it is written because there is nothing to infer it from — the
requirement *is* the contract.

### 3.2 No `instance` header

```
SIDES Square = 4
area (s Square) = s.side * s.side
describe (s Square) = "a square"

SIDES Circle = 0
area (c Circle) = 3.0 * c.radius * c.radius
```

A type satisfies `Shape` iff it has the members `Shape` requires. There is no
`instance Shape Square` header and no restated member signature. A parameter pattern
`(s Square)` names the receiver's type, and that is what attaches the equation to the
type; which class it satisfies is then a consequence rather than a declaration.

Generation 1's `examples/class.sdx` flags its own version of this as a deviation
forced by the absence of a checker:

> An instance method **restates its signature**, which doc 18 §2.5's own example does
> not. Rust's `impl` restates it too, and Voltaic has no name resolution to copy one
> from the class — AGENTS.md forbids growing any.

The reversal removes the cause, so the deviation goes with it.

### 3.3 The one-file rule is lifted

Generation 1 has a trap recorded in `AGENTS.md`: **an `instance` must be in the same
file as its `class`**, because the parser learns a member's signature shape from the
class declaration and each file gets a fresh `Parser`. An instance elsewhere fails
with `RDX0101` on every name in the line, which looks nothing like the cause.

Lattice resolves names across a crate, so an equation and the class it satisfies may
live in different files. The trap is deleted rather than documented. Clause 1 again:
this was never a design choice, it was the absence of resolution showing through.

### 3.4 Deriving templates are deleted

Generation 1's `deriving template` — an `instance` over the reserved type `Target`,
substituted into every declaration listing it — is deleted. Structural classes
subsume it: a template was a way to write one `instance` for many types without a
proc macro, and an equation whose receiver pattern is a *class constraint* rather
than a concrete type does the same thing directly:

```
fmt (s impl Display) f = Display::fmt (s.Field0) f
```

What the template could do and this cannot is reach `Fields` / `FieldNames` — every
field as an access and as a string. That is reflection over a declaration's shape,
and it is **declined** for this generation (§22).

---

## 4. Data

### 4.1 Juxtaposition

```
data Point { x f64, y f64 }
data Shape = Circle f64 | Square f64
data Tag String
data ConfigError
  = Missing (key String)            "missing key: {key}"
  | Closed                          "closed"
```

Doc 86 §4.2–§4.4 is normative. The surface points worth making here:

- `name Type` rather than `name : Type` frees `:` for ascription and removes one
  token per field. Measured over generation 1's corpus, field and payload `:` tokens
  are the largest single repeated-token class after signature lines.
- A single unnamed field is `newtype`'s shape; the keyword is deleted.
- A trailing string literal on a variant makes the type an error type.
- Derives are computed (doc 86 §4.3), so `deriving (Debug, Clone, Copy)` is never
  written. `data !Copy` opts out.

### 4.2 Field punning

`Cursor { src, pos 0 }` — a bare name is `src: src`. Generation 1 has this and it is
kept.

---

## 5. Expressions

### 5.1 Chains

```
pub peek self = self.src.get(self.pos .. self.src.len).unwrap_or("").chars.next
```

and across lines, by indentation:

```
pub peek self = self.src
  .get (self.pos .. self.src.len)
  .unwrap_or ""
  .chars
  .next
```

A leading `.` on a continuation line continues the chain. That is `|>`'s whole job,
spelled with the token the language already uses for method application.

### 5.2 Sections

```
(+ 1)     # |x| x + 1
(2 *)     # |x| 2 * x
(> 0)     # |x| x > 0
```

Kept from generation 1, with one trap removed. Generation 1 must document that
`(> 0)` lowers to the *method* form `|x| x.gt(&0)` because an iterator adaptor hands
the closure a reference, and that `(- 1)` is negation and `(* 2)` a dereference so
those two want the left section. The first is a mode question and Lattice answers it —
the section's parameter mode is inferred from the use, so `(> 0)` lowers to whichever
of `x > 0` and `x.gt(&0)` type-checks. The second is genuine lexical ambiguity and is
kept as generation 1 resolved it: unary wins, write `(1 -)` and `(2 *)`.

### 5.3 Four operators deleted

| Deleted | Replacement |
|---|---|
| `\|>` | a `.` chain (§5.1) |
| `>>>` | `(.f.g)` — a point-free chain is a chain with no receiver |
| `<<<` | `(.g.f)` written the other way round |
| `$` | a chain, or parentheses |

`$` is the one worth justifying, because it is one character and it lowers to nothing
at all. It existed to kill nesting: `.ok $ std::fs::read_to_string path`. With
receiver-first dots that expression is `std::fs::read_to_string(path).ok`, which has
no nesting to kill. `$` survived in generation 1 because receiver-last application
*created* the nesting it then removed.

### 5.4 Comprehensions

```
doubled = [n * 2 | n in xs]
odd     = [n | n in xs, n % 2 == 1]
pairs   = [x * y | x in 1..4, y in 1..4, x != y]
seen    = [c | c in xs] : Set
```

Generation 1's comprehension, with the per-line `: Vec<i32>` ascription gone: a
comprehension whose result is used as a sequence is a `Vec`, and the only ascription
still required is a *different container* (doc 86 §4.1's third required position).

The generator word is `in`, as generation 1 settled on after doc 30 §4.2 gave up
`<-` to free the token for `await`. `await` no longer exists, so `<-` is free again —
and it stays free. `in` won on its own merits and re-litigating a settled token to
recover a Haskell resemblance is not worth a migration.

### 5.5 Strings

Every string literal interpolates:

```
say "hello {name}, you are {age} years old"
```

`{{` is a literal brace. Generation 1's `\->` nullary lambda — which existed so a
macro argument could be passed unevaluated to `println!` — is deleted along with the
need for it: `say` is a function, not a macro, and takes an ordinary expression.

Macros that genuinely need unevaluated arguments take a block:

```
assert! (c.eat "...")
```

### 5.6 `loop` and `break`

```
loop
  ch = self.peek else break
  self.bump
```

The only imperative iteration construct. `for`, `while` and `until` are deleted;
bounded iteration is a comprehension or an iterator method, and unbounded iteration
with early exit is this. `break` carries a value where Rust's does.

`else break` is worth noticing: `break` is an expression of type `!` with the
`diverge` effect, so it is a legal `else` arm and the two constructs compose without
a rule saying they may.

### 5.7 Conditionals

`if`, `else if` and `case … of` are deleted. A conditional is a **guard ladder**,
and §2.2's ladder is admissible wherever a value is rather than only as a clause
body.

```
tier spend
  | spend > 10_000 = "gold"
  | spend > 1_000  = "silver"
  | _              = "basic"
```

and nested, as a value:

```
settled n =
  adjusted =
    | n > 5 = n - 5
    | _     = n
  adjusted * 2
```

The arms are §2.2's arms **unchanged** — an ordinary condition, a pattern row
(`held is Some n`), or `_`. So there is one notion of an arm, and pattern matching
needs no second construct: `case e of` is deleted because a ladder of pattern rows
is what it was.

Three rules, and the off-side rule supplies all three:

- A ladder opens on the line after `=`, or after a clause's parameters, indented
  past it. **Nothing delimits it; the dedent ends it.** This is why no new
  bracket or keyword appears — the construct that needed `if`/`else`/`end` in
  other languages is carried by indentation here, as every other block is.
- **An arm's body is a single expression**, which is §2.2's arm unchanged, so a
  ladder does not nest *directly* inside an arm. **Admitting the ladder as a value
  is what lets ladders nest at all**, through a binding:

  ```
  graded spend floor =
    inner =
      | floor > 100 = "high"
      | _           = "low"
    outer =
      | spend > 1_000 = inner
      | _             = "small"
    outer
  ```

  This is the subsection earning its place rather than a restriction it adds.
  §2.2's clause ladder could never nest — an arm takes one expression, and
  widening it would change the ladder §20 keeps. What nests is the *expression*
  ladder, and it nests the way every other value does.

  The binding evaluates `inner` whichever arm is taken, where a ladder written
  inside the arm would not. Where that matters — cost, or an effect — the arm
  calls a function, which is the same answer the language gives everywhere else
  and needs nothing added here.

  Two drafts of this rule were wrong before this one, both in the same direction:
  they described a nesting the parser does not accept, because nesting was read
  off the shape of the construct instead of tried. The first put a ladder directly
  in an arm; the second moved it to a `where` group. **A `where` group cannot
  follow a guard list at any indentation** — `clause` expects a `Dedent` where
  `where_locals` needs an `Indent`, so the guarded equation is the one equation
  shape with no `where` available to it. That is a real asymmetry in §2.2, it is
  recorded in doc 97 §10.24, and it is not this subsection's to fix, because the
  binding above needs no `where`.
- A ladder in expression position is a **value**, so its arms must agree in type
  and it must be total. Voltaic does not check either: rustc does, per §19, and a
  ladder with no `_` arm is an ordinary `E0317` against the generated
  `if`/`else if` chain. That is the charter working rather than a gap — a Voltaic
  exhaustiveness check would be the second checker doc 86 §2 forbids.

The lowering is §2.2's, unchanged: an `if`/`else if` chain, with `if let` where an
arm is a pattern row. Nothing new is emitted, which is the point — the construct
is a respelling of what generation 1 already lowered, not a new shape in the
output.

**`guard … else` is deleted too, and §5.6 already shows its replacement**:
`ch = self.peek else break` is a binding with an `else` tail. `else` is one of
§1.4's eleven surviving keywords for exactly this, and it is why `else` survives
while `if` does not.

---

## 6. Effects on the surface

Doc 86 §5 is normative. The surface summary:

```
parse_it s = s.parse : i32 ! fail String   # declare, where inference needs the type
fallback s = parse_it s else 0             # discharge with a value
reason s   = (parse_it s).show else e -> e # discharge, binding the error
attempt x y = parse_it x + parse_it y      # propagate — nothing written
first_name u = u.profile.name              # optional chaining — nothing written
held = ~(attempt "1" "2")                  # reify to a Result value
forced = (attempt "1" "2").must            # discharge by panicking; lint-flagged
```

Seven constructs from generation 1 are deleted by those six lines: `try`, `catch`,
`?`, `??`, `.await`, `async` as a block modifier, and `do .and_then` / `do .flat_map`.
Nine, counting `.map_err` and `?.` which generation 1 had already retired.

### 6.1 What `else` is not

`else` is not generation 1's `catch` with a new spelling. `catch` took the `Ok` value
or the fallback and had to lower to a `match` rather than `unwrap_or_else`, because
that method's closure arity differs between `Option` and `Result` and generation 1
had no types to tell them apart. `else` has types: it lowers to `unwrap_or`,
`unwrap_or_else`, or a `match`, whichever fits the row it is discharging. Clause 1,
in the direction the reversal opens.

---

## 7. Modes on the surface

Doc 86 §6 is normative. On the surface, modes are visible in exactly two places:

**An ascription, when an author wants to pin one:**

```
b = a@clone        # the only allocating mode, and the only one written in ordinary code
f (x @owned) = …   # pin a parameter to by-value
```

**A diagnostic, when no assignment satisfies every use.** `VLT0601` names the
conflicting uses and the modes they demand, and suggests the edit — usually a
restructure, sometimes `@clone`. It never inserts the clone itself (doc 86 §6.5,
and doc 01 §25.1's non-negotiable decline, which stands).

Everywhere else there is nothing to write. The `.vrs` carries every `&`, `&mut` and
lifetime in full (doc 86 §6.6), so the borrow structure is reviewable; it is simply
not *authored*.

---

## 8. Modules and visibility

### 8.1 No imports

`import`, `use`, `export`, `export(crate)`, `module`, `mod` and `island` are all
deleted. A name resolves against:

1. the current crate,
2. the dependency set in `Cargo.toml`,
3. `std`.

Ambiguity is the only error — `VLT0501`, naming every candidate and the dependency
each came from. A path may always be written in full (`std::fs::read_to_string`) and
that is how an ambiguity is resolved.

Generation 1's own measurement is the argument: plan 91 found **2,358 lines, 18% of
all code**, were `import` and `module` declarations.

### 8.2 `pub`

One visibility keyword. `pub` means "part of this crate's published surface" and is
what puts a definition in the sidecar (doc 86 §8.2). Rust's `pub(crate)`,
`pub(super)` and `pub(in path)` are reached by the module tree: a definition without
`pub` is visible in its own file and its directory's descendants, which is the
granularity 97% of generation 1's `export(crate)` uses actually wanted.

Where finer visibility is genuinely needed, the `rust` block has it.

---

## 9. Refinements on the surface

```
head (xs [a] where xs.len > 0) = xs.0
get (xs [a]) (i usize where i < xs.len) = xs.i
```

`where` inside a parameter's parentheses is a refinement; `where` after a body is a
binding group (§2.4). The two are unambiguous by position and the token is reused
deliberately — both mean "subject to".

Doc 86 §7.3 is the fragment. On the surface the rule to remember is that a predicate
may mention parameters, their fields, `.len`, indexing, comparisons and boolean
connectives, and nothing else. Anything further is `VLT0801` with the offending
subterm named.

---

## 10. The escape hatch

```
rust
  #[no_mangle]
  pub extern "C" fn voltaic_version() -> *const u8 {
      c"2.0.0".as_ptr()
  }
```

A `rust` block is emitted verbatim into the `.vrs`. It is how `unsafe impl`,
`unsafe trait`, `extern "C"` exports, macro definitions and `pub(in path)` are
reached, and it is the reason those stay `UNREACHABLE` in the grammar inventory
without being *unavailable*.

Generation 1's `HATCH_ALLOWED` list in `crates/voltaic-cli/tests/port.rs` names the
files permitted inline Rust; the list and its gate carry forward unchanged except for
the extension.

---

## 11. Attributes

Unchanged from generation 1. Rust attributes pass through; `#[voltaic::…]`
attributes are consumed by the emitter and never reach rustc. The set is:

| Attribute | Effect |
|---|---|
| `#[voltaic::allow(must)]` | silence the `.must` lint on one expression |
| `#[voltaic::allow(unreachable)]` | silence the reachability residue lint |
| `#[voltaic::opaque]` | keep a `pub` definition's inferred type out of the sidecar's published set |

`#[voltaic::opaque]` is new and is the sidecar's pressure valve: a `pub` definition
whose inferred type is an implementation detail can refuse to publish it, at the cost
of being uncallable across the crate boundary.

---

## 12. Operators and fixity

```
infixl 6 <+> = merge
```

Kept from generation 1 exactly as plan 90 shipped it: fixity plus alias, a closed set
of twelve spellings (`<` · one character · `>`), precedence 0–9 always looser than
arithmetic, emitting no item. The Haskell form where `(<+>)` *is* the name stays out
of reach for generation 1's reason, which the reversal does not touch: no Rust
function can be named `<+>`, so the emitter would have to invent one, and a
fabricated API is refused on independent grounds.

---

## 13. Pattern synonyms

```
pattern Origin = Point { x 0, y 0 }
pattern Both a b = (Some a, Some b)
```

Kept from generation 1. Substituted at every use site, so rustc's exhaustiveness
check sees the written-out pattern. A synonym's parameters are formal: every binder
is written at the use site.

---

## 14. Optics

```
over (.addr.zip.code) doubled target
```

Kept, and improved by the reversal. `over` says a path once where `:=` says it
twice, and generation 1's argument for it is that two spellings of one path that must
agree is the shape of a typo that type-checks. Generation 2 adds that **`over`'s path
is now checked**: `(.addr.zip.town)` against a `code`-typed update is a Lattice
diagnostic rather than a program that compiles and does the wrong thing.

`:=` is kept as the general assignment and reaches a name, a field, an index and a
deref target.

---

## 15. Tests

```
test "eat matches only an exact prefix"
  mut c = Cursor.new "...x"
  !(c.eat "..=")
  c.eat "..."
  c.peek == 'c'
```

Kept from generation 1, with one change: a `test` block's tail expression is the
assertion. A `test` whose body evaluates to `false`, or whose row carries an
undischarged `fail`, fails. `assert!` and `assert_eq!` remain available and are
ordinary macro calls.

The convention that **public-API tests are integration tests** under
`crates/<crate>/tests/` and inline tests are for private constructs only is a
repository rule, not a language one, and carries forward in `AGENTS.md`.

---

## 16. What the formatter does

`voltaic fmt` is canonical and idempotent, as in generation 1, and gains one job: it
maintains the `.vlt.sig` sidecar (doc 86 §8.2). A formatter run that changes a
sidecar is a formatter run that changed a published type, and the diff says so.

---

## 17. Diagnostics

Codes move from `RDX####` to `VLT####`. The ranges:

| Range | Area |
|---|---|
| `VLT01xx` | lexical and layout |
| `VLT02xx` | parse |
| `VLT03xx` | name resolution |
| `VLT04xx` | type inference |
| `VLT05xx` | resolution ambiguity, visibility |
| `VLT06xx` | modes |
| `VLT07xx` | effect rows |
| `VLT08xx` | refinements |
| `VLT09xx` | the sidecar |

Every diagnostic is reported against the `.vlt` the author wrote, with 1-based line
and column in characters. `docs/99 - 101 - Diagnostics Without Signatures.md` is the
design for the four ranges that are new, and it is the plan this generation's risk
rests on.

---

## 18. Residue

The lints generation 1 ships — unreachable residue, `.must` discouragement,
declaration hygiene — carry forward. One is added: **`unused inference`**, which
fires on an annotation that the solver would have inferred identically. It is
`allow`-by-default rather than `warn`, because an author pinning a type for
documentation is doing something legitimate; it exists so that `voltaic check
--pedantic` can report how much of a file's annotation is load-bearing.

---

## 19. Interoperability charter

Unchanged in intent from generation 1's: a Voltaic program must be able to express
anything a Rust program can, and `docs/GRAMMAR.md` is the risk detector that proves
it. The reversal does not weaken it — it closes three rows (doc 86 §10) and leaves
five, four of which the `rust` block answers.

---

## 20. What this surface does not change

Said plainly, because a generation change invites the assumption that everything
moved:

- the off-side rule, and how blocks open
- pattern rows, guard ladders, `where` groups
- sections, holes, pattern synonyms, optics, `:=`
- operator declaration, attributes, tests
- comments as trivia, preserved into the output
- the output being ordinary, reviewable, fully-annotated Rust
- rustc as the authority on everything it can check
- no standard library, no runtime, no GC

---

## 21. Open items

None. §22 holds the declines; doc 86 §10 holds the predicted unreachable set; the
sidecar's semver behaviour is the one piece of design whose details are deferred to
its own plan rather than settled here, and that is named in `docs/89 - Voltaic 2 Plan
of Plans.md` row 100 rather than left implicit.

**One item was open and is now closed, which this section claimed was impossible.**
§5 had no subsection on conditionals while §1.4's surviving eleven keywords deleted
`if` and `case` — so the surface deleted a construct and did not say what replaced
it. Found by row 99 wave 2 trying to implement the deletion and having nothing to
implement it *to*; `docs/97 - 99 - The Surface Contraction.md` §10.24 and §10.28
record the discovery, and §5.7 above is the answer. The lesson is the one doc 04
rule 2 already states: *"None"* is a claim, and a claim in an open-items section is
the one most likely to be made by not looking.

---

## 22. Declined

Each entry is permanent unless a venue is named. `crates/voltaic-cli/tests/declined.rs`
asserts every construct in this section appears in `docs/DECLINED.md`, so the two
cannot drift; plan 104 re-points that gate from doc 03 §22 to this section.

### 22.1 Declined because it needs more than a decidable fragment

| Construct | From | Reason |
|---|---|---|
| Dependent types | Idris, F* | the refinement fragment is QF_ALIA by doc 86 §7.3 and clause 5 makes the bound a condition of admission |
| Quantified refinements (`forall i. …`) | Liquid Haskell | outside the fragment; **venue: a later generation, if and only if a bounded instantiation strategy is measured** |
| User function calls in a predicate | Liquid Haskell | needs either inlining with a termination proof or an uninterpreted-function encoding; both leave the fragment |
| Const-generic arithmetic | Rust, Zig | rustc owns it; a Voltaic surface for it would be a second solver over the same facts |

### 22.2 Declined because it would need a higher-rank or higher-kinded system

| Construct | From | Reason |
|---|---|---|
| Higher-kinded abstraction (`Monad m`) | Haskell | rank-1 HM is what makes doc 86 §3's inference complete; rank-N breaks it and clause 1's printability with it |
| General monadic comprehension over user types | Haskell, generation 1's own `do` | the effect alphabet is closed (doc 86 §5.1). **Venue: a `with M` block modifier naming the type**, sketched in `docs/93 - 95 - Effect Rows.md` §6 |
| Effect handlers (resumable) | Koka, Eff | the row is for *tracking*, not for delimited control; resumption needs a runtime representation and clause 2 forbids one |

The second row is the only decline in this document that is a **regression** against
generation 1 rather than a refusal of something new, and doc 86 §5.6 states it in
those words. It should be read as a debt, not a decision.

### 22.3 Declined because it would imply a dependency

Generation 1's §22 reasons, unchanged and carried forward verbatim in substance:

| Construct | From | Reason |
|---|---|---|
| Regex literals, extensible sigils | Ruby, Elixir | would imply an undeclared crate |
| Property-test and benchmark blocks | Emerald | a property block is `proptest`, a benchmark block is `criterion` |
| Concurrent-await construct | — | every joining form is runtime-crate-provided; doc 86 §5.5 ships `async` without an executor for that reason |
| An executor | — | the same |
| A standard library beyond `say` / `ask` / `die` | — | doc 86 §7.4 closes the set at three |

### 22.4 Declined because the output would stop being honest

| Construct | From | Reason |
|---|---|---|
| Default arguments | Python, C++ | fabricates API surface; breaks trait impls, rustdoc and go-to-definition. Generation 1's reason, undisturbed by the reversal |
| Implicit cloning | — | non-negotiable; doc 01 §25.1 and doc 86 §6.5 |
| Inheritance / `extends` | Java | Rust has no inheritance; classes plus composition are the answer |
| Declaration reflection (`Fields`, `FieldNames` beyond a derive template) | — | a construct that enumerates a declaration's shape is a macro system with no macro keyword. **Venue: the `macro` construct, declined below** |
| `macro` definitions | Rust | still `UNREACHABLE` rather than declined — the `rust` block reaches it, and a Voltaic surface for macro definition is a generation of its own |

### 22.5 Lifted from generation 1's decline list

Two rows leave the table, and both are lifted by the reversal rather than by new
machinery. Rule 2 of the register applies: the row is kept with the evidence.

> **`UFCS` — lifted.** Generation 1 declined it as *"requires type knowledge to
> choose a shape"*, which was exactly true. The reversal supplies the type knowledge,
> and §1.2 is what it buys: three application orders become two that agree. This is
> the clearest single case of a decline whose *reason* was the charter rather than the
> construct.

> **`A Voltaic type checker` — lifted, and renamed.** Generation 1 declined it
> *permanently*, calling it *"the project's load-bearing constraint"*. It was, and the
> constraint earned its keep for 91 plans. `docs/90 - 92 - The Charter Reversal.md`
> is the argument; doc 86 §1.1 is the measurement; and the thing that ships is a
> **solver that infers and prints**, not a checker that second-guesses rustc — every
> check rustc can make, rustc still makes (doc 86 §2, first corollary).

---

## 23. Citations

- `docs/86 - Voltaic 2.md` — the specification this surface implements.
- `docs/03 - Voltaic Surface Design.md` — generation 1's surface, superseded here.
  §2's clauses 2–4 are carried forward verbatim; §22's reasons are carried forward in
  §22.3 and §22.4.
- `docs/DECLINED.md` — the user-facing decline list; plan 104 rewrites it against
  §22 above.
- `docs/GRAMMAR.md` — the graded inventory; plan 104 regrades it.
- `docs/MIGRATION.md` — what each deleted construct becomes.

# Voltaic 2 — Specification

Status: normative for the language. Supersedes `docs/01 - Voltaic.md` where the two
disagree; doc 01 stays as the record of generation 1 and is not edited.

**The name does not change.** The language is Voltaic. What changes is its
generation — `Voltaic 1` is the surface plans 01–91 built, `Voltaic 2` is the
surface this document specifies — and the thing inside it that is new.

**Lattice** is that thing: the solver. One crate, `voltaic-lattice`, holding a type
engine, an effect-row engine, a mode engine and a refinement engine. The language is
Voltaic; Lattice is how it knows what it knows. Every deletion in this document is
paid for by Lattice, and §2 is the measurement that bought it.

---

## 1. What generation 2 is

Voltaic 1 was a **re-spelling**: a Haskell-shaped surface whose lowering was
determined by source text alone, because doc 02 §12.4 permanently declined a type
checker and doc 03 §2 clause 1 made type-freedom an admission test. Every construct
it shipped was syntax over Rust's own semantics.

Voltaic 2 reverses exactly one of those four admission tests and keeps the other
three. It does so because the re-spelling has been measured to exhaustion, and the
measurement says there is nothing left to re-spell.

### 1.1 The number that forced this

Measured 2026-11-13 over the whole self-hosted compiler — 13 crates, every `.sdx`
against the `.rs` it generates, comments stripped from both sides so the comparison
is code against code:

| | Voltaic 1 source | Generated Rust | Ratio |
|---|---:|---:|---:|
| lines | 11,668 | 18,013 | **1.544×** |
| tokens | 129,939 | 153,167 | **1.179×** |
| characters | 434,662 | 435,454 | **1.002×** |

The line saving is real. The character saving is **two parts in a thousand**, which
is noise. Repeated over the 43-program example corpus it is 1.290× / 1.197× /
1.045× — same shape, same conclusion.

What that gap says is precise and not a matter of taste. Voltaic 1 deletes
**punctuation and layout** — braces, semicolons, `fn`, `let`, closing-brace lines —
and deletes them well. It deletes almost no **information**, because the information
is types: `&'a str`, `Option<char>`, `impl Fn(char) -> bool`,
`Result<i32, String>`. Those characters are 82% of what is left, and **no
re-spelling removes them. Only knowledge removes them.**

Two further measurements from the same sweep, both of them symptoms of the same
cause:

- **8.7% of all source lines are standalone type-signature lines** — 1,132 of 13,043
  across 122 files. Every definition writes its own name at least twice.
- **74 keywords and 128 graded grammar rows** buy the 1.179×. Doc 03 is 2,283 lines
  of surface design. The exchange rate is poor and it is poor *structurally*, not
  because any individual construct was a mistake.

### 1.2 The register predicted this, in writing

`docs/04 - Voltaic Plan of Plans.md`, under *What this milestone does not claim*,
says it outright:

> It does not reach 10/10 on the axis it is named for, and should not try. A score
> that high requires inference, higher-kinded abstraction or effect tracking, each of
> which needs the checker doc 02 §12.4 forbids — so the honest ceiling under the
> constraint is **8–9**, and the constraint is worth more than the tenth point. Doc
> 02 §12.4 stays.

Generation 2 agrees with every clause of that paragraph except the last sentence, and
agrees with it for the reason the paragraph gives: inference and effect tracking *are*
what the remaining distance costs. The question the paragraph leaves open is whether
the tenth point is worth the constraint, and §1.1 is the answer — the constraint was
worth more than the tenth point when the price was the tenth point. The price is
actually **1.002× in characters**, which is the whole of the terseness claim.

The same document, two paragraphs later, names the re-cut this generation takes and
leaves it to a later row:

> the constraint is stated broadly enough to read as forbidding all static analysis,
> where what it means to forbid is a *type* checker and a *borrow* checker.

Plan 92 is that row. It is the only reversal in this generation, and `docs/90 - 92 -
The Charter Reversal.md` carries the argument in full.

### 1.3 What does not change

Six commitments carry forward unaltered, and they are the reason this is a generation
of Voltaic rather than a different project:

1. **The output is ordinary Rust**, human-facing, with comments preserved as trivia.
2. **rustc is the final authority on codegen, borrows and monomorphisation.** Lattice
   infers; rustc confirms. A program Lattice accepts and rustc rejects is a *Lattice
   bug*, reported as one.
3. **No standard library.** crates.io is the answer to "where is the `Vec`". The one
   addition is the `io` effect's primitives, and §7.4 bounds them.
4. **No implied dependency.** A construct's lowering names nothing outside `core`,
   `alloc` and `std`.
5. **Semantics-preserving, never a silent clone.** Mode inference chooses *which*
   borrow; it never inserts a copy to make a program type-check. §6.5.
6. **Hygiene is reserved.** Generated identifiers carry a reserved prefix and user
   identifiers may not. The prefix changes from `__rdx_` to `__vlt_` — see §9.2.

### 1.4 What this document is not

It is not a tutorial, not a migration guide and not a plan. The migration is
`docs/MIGRATION.md`; the surface rationale, construct by construct, is `docs/87 -
Voltaic 2 Surface Design.md`; the work is `docs/89 - Voltaic 2 Plan of Plans.md`.

---

## 2. The admission test, generation 2

A construct is admitted if and only if it satisfies all four. **Clause 1 is the
reversal; clauses 2–4 are doc 03 §2's, unchanged in wording.**

1. **Solver-determined.** Its lowering is determined by the source text **and the
   judgement Lattice infers for it**. Where two Rust shapes are possible, either the
   user's syntax selects between them or the solver does — and when the solver does,
   it must be able to *print the choice it made* (§8.1). A construct whose lowering
   cannot be explained to the author is not admitted.
2. **No implied dependency.** Its lowering names nothing outside `core`, `alloc` and
   `std`.
3. **Semantics-preserving.** It does not alter Rust's ownership, borrowing,
   mutability or evaluation-order semantics, and never inserts a clone.
4. **Reachability-positive.** It either adds expressiveness or closes a gap in the
   set of Rust constructs a Voltaic program can express.

And one clause that is new, because the reversal creates the failure mode it guards:

5. **Decidable, or diagnosed.** Every inference this surface depends on terminates
   on every input. Where a judgement is not decidable in general, the admitted
   fragment is stated in this document and anything outside it is a **diagnostic, not
   a hang**. §7.3 states the refinement fragment; §5.4 states the row fragment.

The three corollaries of doc 03 §2 are replaced by these:

- **Every check rustc can make, rustc still makes.** Lattice does not re-implement
  borrow checking, exhaustiveness checking or monomorphisation. It infers *what to
  write down* and emits Rust that says it explicitly.
- **Ownership stays explicit — in the output.** The emitted `.vrs` carries every
  `&`, `&mut` and lifetime in full. What the author stops writing, the emitter starts
  writing. §6.6.
- **Inference is never invisible.** `voltaic sig` prints the inferred judgement of
  anything; the formatter maintains a sidecar; the LSP shows it inline. §8.

---

## 3. The judgement

Every expression in Voltaic 2 has a three-part judgement:

```
τ ! ε @ μ
```

- **τ** — the type. Rank-1 Hindley–Milner with row polymorphism over records and
  effects. Inference is complete and decidable; there are no higher-rank types and
  no impredicativity, which is what keeps it so.
- **ε** — the effect row. A finite set drawn from a closed effect alphabet (§5.1).
  The lattice is the subset lattice; join is union.
- **μ** — the mode. One of `shared ⊑ unique ⊑ owned`, a three-element chain (§6.1).

Lattice solves τ by unification, and ε and μ by **least fixed point**: each binding
gets the smallest point in each lattice that all of its uses permit. "Smallest" is
defined by the orderings in §5.2 and §6.1 and is a property of the lattice, not a
heuristic — which is what makes the result stable under edits that do not change
uses.

A judgement is written in source only where it cannot be inferred or where the author
chooses to pin it:

```
parse_it s = s.parse : i32 ! fail String
```

`:` ascribes τ. `!` ascribes ε. `@` ascribes μ. All three are optional and all three
are checked, never trusted.

---

## 4. Types

### 4.1 Inference, and where an annotation is required

No definition carries a signature line. The signature of every definition is
inferred and recorded in a sidecar (§8.2).

An annotation is **required** in exactly four positions, and the list is closed:

| Position | Why |
|---|---|
| A `class` requirement | the requirement *is* the contract; there is nothing to infer it from |
| A numeric literal whose width is unconstrained by any use | HM would default, and a silent default to `i32` is the kind of invisible choice clause 1 forbids |
| A container whose element type is used but whose container is not determined (`[c \| c in xs] : Set`) | `collect`'s target is genuinely ambiguous |
| An `extern` boundary — `rust` blocks, FFI, and anything `pub` whose inferred type the author wants frozen | §8.3 |

Everywhere else an annotation is legal and is checked. Nowhere else is it required.
`voltaic check --require-signatures` promotes every `pub` definition's inferred type
to a required annotation, for crates that want the stricter discipline; it is off by
default and `docs/98 - 100 - The Signature Sidecar.md` argues why.

### 4.2 Declarations

```
data Point { x f64, y f64 }
data Shape = Circle f64 | Square f64
data Tag String
```

Field and payload types are written by **juxtaposition**: `x f64`, not `x : f64`.
The `:` is freed for ascription, and the token it cost is the one measured in §1.1 as
the largest per-field cost in generation 1.

A `data` with one unnamed field is generation 1's `newtype`: single-field,
`#[repr(transparent)]` inferred, no keyword. A `data` with variants is an `enum`. A
`data` with named fields is a `struct`. One keyword, three Rust shapes, chosen by the
shape of the declaration rather than by a word.

### 4.3 Derives are computed

`deriving (…)` is not written. Lattice computes the derivable set from the fields:
`Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash` are derived
for every `data` whose fields all support them, and emitted into the `.vrs` as an
ordinary `#[derive(…)]` list.

Opt out by name, because move semantics are sometimes the contract:

```
data !Copy Token { kind TokenKind, span Span }
```

A name in no derivable set — a user's own trait, a proc-macro derive from a
dependency — is written where it always was, as an attribute:

```
#[derive(serde::Serialize)]
data Config { root String }
```

This is clause 2 holding: Lattice derives only what `core` can derive, and anything a
crate provides stays the crate's business.

### 4.4 Error types are `data`

Generation 1's `errors` declaration — its only construct emitting more than one Rust
item — is deleted as a construct and kept as a *shape*. A `data` whose variants carry
a trailing string literal is an error type:

```
data ConfigError
  = Missing (key String)            "missing key: {key}"
  | Invalid (v String) (line u32)   "invalid value {v} at line {line}"
  | Closed                          "closed"
```

`Display` and `Error` are derived from the strings by the same rule as §4.3, and
`From` conversions are derived from the effect row rather than declared: a `fail
ParseIntError` raised inside a `fail ConfigError` context converts if a `From` exists
and is a diagnostic naming both types if it does not. Generation 1's `from … as …`
row is therefore deleted too.

### 4.5 Optionals

`?T` is `Option<T>`. `?` alone is the absent value:

```
data User { profile ?Profile }
empty = ? : ?User
```

`?T` is not merely sugar — §5.3 makes absence an *effect*, which is what deletes
optional chaining as a construct. §5.6 records what that costs.

---

## 5. Effects

### 5.1 The alphabet is closed

| Effect | Means | Lowers to |
|---|---|---|
| `fail E` | may raise an `E` | `Result<_, E>` and `?` at every call site |
| `none` | may be absent | `Option<_>` and `?` |
| `async` | suspends | `async fn` and `.await` at every call site |
| `io` | performs I/O | nothing — a marker, §7.4 |
| `mut ρ` | mutates the region ρ | `&mut` on the relevant binding |
| `panic` | may panic | nothing — a marker, propagated for lint purposes |
| `diverge` | may not return | `!` where Rust can express it |

**Seven effects, and the set is closed by this document.** A closed alphabet is what
makes row unification decidable and what makes clause 5 satisfiable; it is also the
whole of what §5.6 gives up.

### 5.2 The lattice

The effect lattice is the subset lattice over the alphabet. `{}` is bottom, the full
set is top, join is union, and `ε₁ ⊑ ε₂` iff `ε₁ ⊆ ε₂`. A function's row is the join
of its body's rows minus what its body discharges. `fail` is parameterised by its
error type, and two `fail`s join by `From`-convertibility or diagnose.

Inference runs **upward**: a body that calls something fallible is fallible.
Discharge runs **downward**: `else` removes `fail` and `none` from the row of the
expression it guards.

### 5.3 Propagation is implicit, discharge is explicit

There is no `?`, no `try`, no `catch`, no `await`, no `.map_err` and no `do`.

```
parse_it s = s.parse : i32 ! fail String

attempt x y = parse_it x + parse_it y        # row: fail String — propagated
fallback s  = parse_it s else 0              # row: {} — discharged
reason s    = (parse_it s).show else e -> e  # discharged, error bound
first_name u = u.profile.name                # row: none — optional chaining
```

The fourth line is the clearest statement of what the row buys. `u.profile` is
`?Profile`, so the `none` effect enters the row and the field access after it is
ordinary; the result is `?String`. Generation 1 needs a `try` block, two `&`s, two
`.as_ref`s and two `?`s for that line, and `examples/fallible.sdx` spends 30 lines of
comment explaining why.

Discharge forms, and they are the complete set:

| Form | Discharges | Notes |
|---|---|---|
| `e else v` | `fail`, `none` | `v` is the fallback value |
| `e else x -> body` | `fail` | `x` binds the error |
| `e.must` | `fail`, `none` | panics; adds `panic` to the row; lint-flagged (§8.4) |
| `~e` | all of `fail`, `none` | **reifies** the row into a value — `Result`/`Option` as data |
| a `test` block | `fail` | a failing row fails the test |
| `main` | `fail` | lowers to `fn main() -> Result<_, _>` |

`~` is the operator that makes an effect first-class again, and it exists because a
program sometimes needs to *hold* a failure rather than handle it —
`examples/fallible.sdx`'s `print_outcome` takes a `Result` as an argument, and
without `~` that function cannot be written.

### 5.4 The decidable fragment

Row unification over a closed finite alphabet with `From`-convertibility on `fail`'s
parameter is decidable, and the termination argument is the finiteness of the
alphabet: a row is a subset of a seven-element set, so the ascending chain condition
holds and the fixed point is reached in at most seven iterations per binding.

Effect **polymorphism** is admitted in one form only: a higher-order parameter may
carry an effect variable, so `map` over a fallible function is expressible:

```
map f xs = [f x | x in xs]    # row: ε where ε is f's row
```

Effect variables are rank-1, like type variables, and for the same reason.

### 5.5 `async` has no executor

Unchanged from generation 1, and for generation 1's reason: every joining form is
runtime-crate-provided, so shipping one would violate clause 2. `async` is inferred
and lowered; `tokio::spawn` is the author's dependency and always was.

### 5.6 What the row costs, stated once

Two regressions against generation 1, both real and both documented rather than
argued away:

**General monadic comprehension is lost.** Generation 1's `do .and_then` works over
*any* type with an `and_then`-shaped method, including one defined in the author's
own crate with no relation to `std`. That construct is the single best piece of
design in generation 1 — it was declined for the whole of the project's life as
*"must choose `flat_map` versus `and_then`"*, carrying the only reopening venue in
`docs/DECLINED.md`, and plan 86 lifted it with the argument written down. The effect
row's alphabet is closed, so it cannot subsume it. In Voltaic 2 a user monad is
chained explicitly:

```
doubled = (Tally { value 5 }).and_then (n -> Tally { value (n*2) })
```

For a two-row chain this is terser than the block. **For a five-row chain over a user
type, generation 1 is terser, and this document does not pretend otherwise.** The
venue for reopening is a `with M` block modifier that names the type rather than the
method; `docs/93 - 95 - Effect Rows.md` §6 holds the sketch and the reason it is not
in this generation.

**Nested optionals flatten.** Doc 01 §13.1 deliberately does not flatten
`Option<Option<T>>`, and generation 1 honours it because `?` does not flatten. An
effect cannot nest — that is what being an effect means — so
`u.profile.nickname` where `nickname` is `?String` yields `?String`, not
`?(?String)`. The nested shape is recovered with `~`:

```
nickname_of u = ~u.profile.nickname          # ?(?String)
```

The terser spelling is now the lossy one, which is the wrong way round and is the
price of the row. It is recorded here, in `docs/DECLINED.md` once plan 104 rewrites
it, and in `docs/MIGRATION.md` as a **manual** migration class.

---

## 6. Modes

### 6.1 The chain

```
shared  ⊑  unique  ⊑  owned
  &T        &mut T       T
```

Every binding carries a mode variable. Uses impose constraints — a read requires at
least `shared`, a write at least `unique`, an escape past the frame requires
`owned` — and Lattice assigns the **least** mode satisfying all of them. Three
elements, so the solve terminates in at most two iterations per binding.

### 6.2 What is deleted

`&`, `&mut`, `'a` and every other lifetime, `.clone()`, `Box::new`, `Rc::new`,
`.as_ref`, `.to_owned`, `.borrow`. None of them is written in Voltaic 2 source. All
of them appear in the emitted `.vrs`, in full.

```
# Voltaic 1
peek : &self -> Option<char>
peek self = self.src |> .get (self.pos .. .len self.src) |> .unwrap_or "" |> .chars |> .next

# Voltaic 2
peek self = self.src.get(self.pos .. self.src.len).unwrap_or("").chars.next
```

### 6.3 Lifetimes

Lattice does not infer lifetimes. It emits **elided** lifetimes and lets Rust's own
elision rules and borrow checker resolve them, because those rules are already
complete for the shapes Lattice emits — a function whose parameters and return are
all borrows from `self` elides, and one that returns a borrow from an unrelated
parameter is exactly the case where elision fails and Lattice must name the
parameter. In that case the emitter introduces `'__vlt_a`, `'__vlt_b`, … in
parameter order. The author never writes one and never reads one unless they open
the `.vrs`.

### 6.4 Regions

`mut ρ` in a row names the *region* mutated, where a region is a root binding. Two
`mut` effects over distinct regions commute and may be reordered by neither Lattice
nor rustc — Lattice does not reorder anything — but they are known not to alias,
which is what lets a function mutate two of its parameters without the author
writing `&mut` twice.

### 6.5 No silent clone, and this is clause 3

If no mode assignment satisfies every use, **Lattice emits a diagnostic naming the
conflicting uses**. It does not insert `.clone()`. A clone is an author's decision
about cost and is spelled:

```
b = a@clone
```

`@` is mode ascription; `@clone` is the only mode that allocates and it is the only
mode an author writes by hand in ordinary code. `@owned`, `@shared` and `@unique`
exist for pinning and are checked like any other ascription.

This is the single most important paragraph in §6. Implicit cloning was
*non-negotiably* declined in doc 01 §25.1 and remains so. The reversal in §2 is about
clause 1 and clause 1 only.

### 6.6 The output carries everything

A `.vrs` is fully annotated: every signature written out, every `&` and `&mut`
present, every lifetime that elision does not supply named. This is deliberate and it
is what keeps rustc the authority — rustc is type-checking a program that states its
own types, not re-deriving Lattice's reasoning. It is also what makes the `.vrs`
reviewable, which doc 02 §10.1 required of generation 1 and this document keeps.

---

## 7. Refinements

### 7.1 The one addition beyond Rust

A parameter may carry a predicate:

```
head (xs [a] where xs.len > 0) = xs.0
get  (xs [a]) (i usize where i < xs.len) = xs.i
```

The predicate is checked at every call site at compile time and **erased** at
runtime. `head []` does not compile. There is no runtime check, no panic branch, and
no `Option` in the return type — which is why this is the only feature in this
document that makes a program smaller by deleting work rather than by deleting
syntax.

### 7.2 Why this and nothing else

Clause 4 asks what a construct adds. Everything else in generation 2 deletes tokens
for behaviour Rust already has; refinements are the only construct that lets a Voltaic
program state something Rust cannot state at all. That is the whole of the
expressiveness case for the generation, and it is deliberately one feature rather
than several: dependent types, higher-kinded abstraction and const-generic arithmetic
are all **declined** for this generation, and `docs/87 - Voltaic 2 Surface Design.md`
§22 records each with a venue.

### 7.3 The fragment, and clause 5

Refinement predicates are admitted over **quantifier-free linear integer arithmetic
with the theory of arrays** — QF_ALIA — restricted to:

- integer-valued expressions over parameters, their fields, and `.len`
- comparison and the boolean connectives
- `.len` and indexing of built-in sequences and maps

Nothing else. No multiplication of two variables, no quantifiers, no user function
calls in a predicate, no recursion. A predicate outside the fragment is **`VLT0801`:
refinement outside the decidable fragment**, naming the offending subterm and the
rule it broke.

This bound is the design's load-bearing honesty. Unrestricted refinement checking is
undecidable, and a compiler that hangs on a predicate is worse than one that refuses
it. The fragment is chosen so that the solver answers in bounded time on every input
this surface can express; `docs/95 - 97 - Refinements.md` §3 has the solver pin, the
timeout policy and what happens when the timeout fires (a diagnostic, never a pass).

### 7.4 The `io` effect's primitives

`io` is the one place a prelude exists, and it is three names: `say`, `ask`, `die`.
They exist because `io` must be *observable* in the row for the row to be worth
having, and because `println! \-> "#{x}"` was measured in §1.1's corpus as the
single most repeated token sequence in generation 1.

Everything else — files, sockets, time, randomness — is crates.io's, unchanged.
Three names is the whole of the prelude and this document closes the set.

---

## 8. Inference is never invisible

The reversal in §2 creates one genuinely new risk, and it is not a technical one: a
language with no written signatures has nowhere to anchor a type error, which is the
known failure mode of every language that inferred everything. Four mechanisms
answer it, and together they are the condition on which clause 1 was reversed.

### 8.1 Every solver choice is printable

```
voltaic sig crates/voltaic-lexer/vlt/cursor.vlt
voltaic sig --why peek          # why `shared` and not `unique`
```

`voltaic sig` prints the inferred judgement of every definition in a file. `--why`
prints the constraint set that produced a mode or a row, naming the use that forced
each bound. A solver choice that cannot be explained this way is a bug in Lattice, not
a fact about the program.

### 8.2 The sidecar

The formatter maintains `<file>.vlt.sig` beside every source file: one line per
`pub` definition, carrying its inferred judgement. It is **generated, committed, and
diffed** — which is what turns an inferred API into a reviewable one. A change to a
`pub` definition's inferred type shows up as a one-line diff in review.
`docs/98 - 100 - The Signature Sidecar.md` is the whole design.

### 8.3 The sidecar is the published API

Cross-crate inference does not happen. A crate's `pub` surface is its sidecar, and a
downstream crate type-checks against that file, not against the upstream source. This
is what keeps separate compilation working and it is the reason the sidecar is
committed rather than cached.

A sidecar that disagrees with the source is **`VLT0901`**, with the fix named:
re-run the formatter. A sidecar whose published types *narrow* is a breaking change
and `voltaic check --semver` says so — a thing Rust cannot do at all, and the one
place where inferring the API is better than writing it.

### 8.4 Lints survive

`.must` is lint-flagged exactly as generation 1's `unwrap` discouragement flagged
`.unwrap`, by name and without types. `#[voltaic::allow(must)]` is consumed by the
emitter and never reaches rustc.

---

## 9. Surface inventory

### 9.1 Keywords

```
data  class  where  pub  mut  self  test  loop  break  rust  else
```

**Eleven.** Generation 1 has 74.

> **Correction, recorded rather than quietly fixed.** The design dossier that argued
> for this generation claimed ten and listed ten, omitting `break`. `break` is not
> optional — `loop` without it is a non-terminating construct, and the lexer's
> `eat_while` needs both. The count is eleven. This correction is kept here because
> `docs/04 - Voltaic Plan of Plans.md` rule 2 applies to specifications as much as to
> registers: correct stale labels in place, with the evidence.

Effect names (`fail`, `none`, `async`, `io`, `mut`, `panic`, `diverge`) are
**contextual**: they are only effect names after `!` in a row position, and `fail` is
a legal identifier everywhere else. `mut` is the exception and is a keyword in both
positions.

What each deleted keyword became is `docs/MIGRATION.md`'s table. The summary: control
flow collapses into the guard ladder and pattern rows; iteration into comprehensions,
iterator methods and `loop`/`break`; fallibility and asynchrony into the row; modules
into the manifest; `instance`/`newtype`/`deriving`/`errors`/`const`/`static` into
`data` and `class`.

### 9.2 Reserved spellings

| Spelling | Reserved for |
|---|---|
| `__vlt_` prefix on identifiers | generated identifiers. User identifiers may not use it. |
| `'__vlt_` prefix on lifetimes | emitted lifetimes where elision fails (§6.3) |
| `VLT####` | diagnostic codes |

The prefix changes from generation 1's `__rdx_`, and the lifetime and diagnostic
prefixes change with it. `rdx` is a fossil: it is the project's pre-rename
identity, and generation 1 carries it in every generated binder, every diagnostic
code and the string `RDX0101` in 61 UI fixtures. A generation that renames the file
extensions and nothing else would leave the fossil in the one place users read it.
`docs/91 - 93 - Extensions and Nomenclature.md` is the rename.

### 9.3 File extensions

| | Generation 1 | Generation 2 |
|---|---|---|
| source | `.sdx` | **`.vlt`** |
| generated Rust | `.xrs` | **`.vrs`** |
| source map | `.xrs.map` | **`.vrs.map`** |
| signature sidecar | — | **`.vlt.sig`** |
| per-crate source directory | `crates/<c>/sdx/` | `crates/<c>/vlt/` |

`sdx` and `xrs` are fossils of the same rename as `__rdx_`: `.xrs` is `.rs` with a
letter in front and `.sdx` means nothing at all in a project called Voltaic. `.vlt`
and `.vrs` name the language and the language's Rust. Every touchpoint — 2,644
occurrences across the tree, 1,158 of them outside `docs/` — is enumerated in
`docs/91 - 93 - Extensions and Nomenclature.md` §3.

---

## 10. What is still unreachable

Generation 1 grades 8 Rust constructs `UNREACHABLE` in `docs/GRAMMAR.md`, each with a
fixture asserted to keep failing. Generation 2's effect on that list, predicted here
and gated by plan 104:

| Construct | Generation 2 |
|---|---|
| `\|` or-patterns | **reachable** — patterns gain `\|`, which generation 1 could not add because `\|` was the guard token in the same position |
| `as` casts | **reachable** — `x : u8` is an ascription and the solver knows whether it is a coercion or a cast |
| `ref` / `ref mut` patterns | **reachable** — a pattern binding's mode is inferred like any other |
| Macro definitions | still unreachable; venue: a `macro` construct is declined for this generation |
| Macro invocation with a non-expression argument | still unreachable |
| `extern "C" fn` export | still unreachable; the `rust` block is the answer |
| `unsafe impl` / `unsafe trait` | still unreachable; the `rust` block is the answer |

Three of eight close as a *consequence* of the solver rather than as features, which
is clause 4 being satisfied by the reversal itself. The remaining five are unchanged
and four of them have the same answer they always had: `rust`.

**This table is a prediction until plan 104 regrades `docs/GRAMMAR.md` against real
fixtures.** Generation 1's plan 46 measured that of sixteen apparent gaps in a
careless first pass, **eleven were the probe's own bug** — so a predicted grade here
carries exactly the weight of a prediction, and `docs/GRAMMAR.md` remains the
authority.

---

## 11. Citations

- `docs/01 - Voltaic.md` — generation 1's specification. Superseded, not edited.
- `docs/02 - Voltaic Update Blueprint.md` §12.4 — the declined type checker, reversed
  by plan 92.
- `docs/03 - Voltaic Surface Design.md` §2 — the four admission tests; clause 1
  reversed, 2–4 carried forward verbatim in §2 above.
- `docs/04 - Voltaic Plan of Plans.md`, *What this milestone does not claim* — the
  paragraph that predicted this generation.
- `docs/87 - Voltaic 2 Surface Design.md` — construct by construct, with §22 holding
  the declines.
- `docs/88 - Voltaic 2 Blueprint.md` — architecture, crates, and what the reversal
  costs the pipeline.
- `docs/89 - Voltaic 2 Plan of Plans.md` — plans 92–104.
- `docs/MIGRATION.md` — generation 1 to generation 2, construct by construct.

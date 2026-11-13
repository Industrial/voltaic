# Migrating from Voltaic 1 to Voltaic 2

The language is still Voltaic. What changed is its generation, and the change is large:
new file extensions, 74 keywords down to 11, no signature lines, no `&`, no `?`, no
imports.

**Most of it is mechanical.** `voltaic migrate` does the mechanical part, refuses what
it cannot do, and proves its own work by generating byte-identical Rust. Three
constructs have no mechanical answer and §6 is the whole list.

- Normative: `docs/86 - Voltaic 2.md` (the language), `docs/87 - Voltaic 2 Surface
  Design.md` (the surface).
- Why: `docs/90 - 92 - The Charter Reversal.md`.
- The tool: `docs/101 - 103 - The Generation Migrator.md`.

---

## 1. Run the migrator

```bash
voltaic migrate src/            # classify and rewrite
voltaic migrate --dry-run src/  # classify only, write nothing
```

Every file is classified, and the class determines what happens:

| Class | What the migrator does |
|---|---|
| **auto** | rewrites it, and proves the rewrite by generating Rust byte-identical to what Voltaic 1 generated |
| **review** | rewrites it and names it in the report — the rewrite is correct, but you want to read it |
| **manual** | **refuses**, naming the construct and the line. §6. |

A `manual` file is left untouched. A half-migrated file that compiles is worse than one
that was skipped, which is why the tool refuses rather than guesses.

### 1.1 It migrates in two passes, and so should you

Pass 1 translates and **keeps your explicit types and modes as ascriptions**. The
output is correct and verbose: `&self` becomes `self @shared`, a signature line becomes
an inline ascription.

```bash
voltaic migrate --simplify src/
```

Pass 2 deletes the ascriptions the solver infers identically — gated by the same
byte-identity check, so a deletion that would change the program is refused.

Run pass 1, commit, then run pass 2. The intermediate commit is ugly and it is the one
you can review.

---

## 2. Extensions

| | Voltaic 1 | Voltaic 2 |
|---|---|---|
| source | `.sdx` | `.vlt` |
| generated Rust | `.xrs` | `.vrs` |
| source map | `.xrs.map` | `.vrs.map` |
| signature sidecar | — | `.vlt.sig` |

`.sdx` is **accepted for one release**, with a deprecation note naming
`voltaic migrate`. It is removed in the release after the one that ships generation 2;
`CHANGELOG.md` names which.

Two renames reach your output rather than your source:

- generated identifiers go from `__rdx_a0` to `__vlt_a0`. Both prefixes stay reserved,
  so a program that used `__rdx_` before it was reserved keeps failing rather than
  starting to pass.
- diagnostic codes go from `RDX####` to `VLT####`, and are renumbered into ranges —
  `VLT04xx` types, `VLT06xx` modes, `VLT07xx` effects, `VLT08xx` refinements. If you
  match on codes in tooling, this is a breaking change.

---

## 3. The construct table

Everything that changed, with what it becomes. The **Class** column is the migrator's.

### 3.1 Definitions

| Voltaic 1 | Voltaic 2 | Class |
|---|---|---|
| `f : T -> U` on its own line | **deleted** — inferred. `voltaic sig` prints it; the LSP shows it inline | auto |
| `f x = …` | unchanged | — |
| multiple equations | unchanged | — |
| `\| guard = …` | unchanged | — |
| `otherwise` | `_` | auto |
| `where` group | unchanged | — |
| `name = expr` as a local | unchanged | — |
| `mut name = expr` | unchanged | — |

### 3.2 Control flow — eleven keywords become the ladder

| Voltaic 1 | Voltaic 2 | Class |
|---|---|---|
| `if` / `else` / `else if` | a guard ladder | auto |
| `case x of` | pattern rows, or an inline ladder | auto |
| `cond`, `switch`, `switched`, `when`, `iff`, `unless`, `then`, `guard` | the ladder | auto |
| `for x in xs` | a comprehension or `xs.each` | auto |
| `while c`, `until c` | `loop` with `break`, or an iterator method | auto |
| `loop`, `break`, `continue` | `loop`, `break` kept; `continue` deleted — restructure the guard | review |

### 3.3 Effects — nine constructs become one row

| Voltaic 1 | Voltaic 2 | Class |
|---|---|---|
| `Result<T, E>` in a signature | `! fail E` in the row, usually inferred | auto |
| `Option<T>` in a signature | `?T` | auto |
| `x?` | **deleted** — propagation is implicit | auto |
| `try` block | **deleted** — the row propagates | auto |
| `e catch v` | `e else v` | auto |
| `e catch (\err -> …)` | `e else err -> …` | auto |
| `a ?? b` | `a else b` | auto |
| `async` block modifier | **deleted** — the `async` effect is inferred | auto |
| `.await` | **deleted** — the call is the await | auto |
| `.map_err (\e -> …)` for conversion | **deleted** — `From` applies on row join | auto |
| `errors E` with a `from … as …` row | `data E` with display strings; the `from` row is deleted | auto |
| `do .and_then` over `Option`/`Result`/iterators | **deleted** — the row does it | review |
| `do .and_then` over **your own type** | **no mechanical answer.** §6.1 | **manual** |
| `u?.a?.b` → `try` + `&` + `.as_ref` + `?` | `u.a.b` | auto |
| nested `Option<Option<T>>` | **flattens.** §6.2 | **manual** |
| `.unwrap` with `#[voltaic::allow(unwrap)]` | `.must` with `#[voltaic::allow(must)]` | auto |

The third-from-last row is the generation's single biggest saving. Voltaic 1:

```
first_name : &Option<User> -> Option<&String>
first_name u = try
  profile = &(.as_ref u)? .profile
  Some (&(.as_ref profile)? .name)
```

Voltaic 2:

```
first_name u = u.profile.name
```

### 3.4 Ownership — four token classes deleted

| Voltaic 1 | Voltaic 2 | Class |
|---|---|---|
| `&self`, `&mut self`, `self` in a signature | **deleted** — inferred | auto |
| `&T`, `&mut T` | **deleted** — inferred | auto |
| `'a` and every other lifetime | **deleted** — elided, or emitted as `'__vlt_a` | auto |
| `.clone()` | `x@clone`, and only where you meant it | review |
| `.as_ref`, `.to_owned`, `.borrow` | **deleted** where they were there to satisfy a borrow | auto |
| `Box::new`, `Rc::new` | kept — they are `std` constructors, not modes | — |

If no mode assignment works, you get `VLT0601` naming the conflicting uses. **Voltaic
never inserts a clone for you.**

### 3.5 Application order — the one that changes how code reads

| Voltaic 1 | Voltaic 2 | Class |
|---|---|---|
| `f x` | unchanged | — |
| `.trim raw` (receiver last) | `raw.trim` | auto |
| `.insert k v m` | `m.insert k v` | auto |
| `raw \|> .trim \|> .len` | `raw.trim.len`, or a `.` chain across lines | auto |
| `f >>> g` | `(.f.g)` | auto |
| `f <<< g` | `(.g.f)` | auto |
| `.ok $ read path` | `read(path).ok` | auto |
| `.insert k _ m` (a hole) | `m.insert k _` — **the hole's position moves** | review |
| `(.name)` as a field selector | `(.name)` — unchanged, and the Voltaic 1 selector-vs-method trap is gone | auto |
| `(> 0)` lowering to `x.gt(&0)` | `(> 0)` — the solver picks the form | review |

The hole row is `review` because the hole marks a *position*, and reversing the
receiver order moves it. The migrator gets it right; you want to look.

### 3.6 Declarations

| Voltaic 1 | Voltaic 2 | Class |
|---|---|---|
| `data P { x : f64 }` | `data P { x f64 }` — juxtaposition, no `:` | auto |
| `newtype Tag = Tag String` | `data Tag String` | auto |
| `deriving (Debug, Clone, Copy)` | **deleted** — computed from the fields | auto |
| a type that must **not** be `Copy` | `data !Copy P { … }` | review |
| `deriving (serde::Serialize)` | `#[derive(serde::Serialize)]` — a dependency's derive stays an attribute | auto |
| `errors E` | `data E` with a display string per variant | auto |
| `errors` positional `#{0}` | named payloads: `Missing (key String)` with `"{key}"` | auto |

### 3.7 Classes

| Voltaic 1 | Voltaic 2 | Class |
|---|---|---|
| `class C` with signatures | unchanged — the one place a signature is still written | — |
| `instance C T` header | **deleted** | auto |
| a member signature restated inside an instance | **deleted** | auto |
| `instance T` (inherent) | an equation with a receiver pattern: `area (s T) = …` | auto |
| `class Debug => C` | a constraint list on the class | auto |
| an instance in a different file from its class | **now legal.** Voltaic 1 failed with `RDX0101` on every name in the line | auto |
| `deriving template` using `Field0` only | an equation with a class-constraint receiver | review |
| `deriving template` using `Fields` / `FieldNames` | **no mechanical answer.** §6.3 | **manual** |

### 3.8 Modules

| Voltaic 1 | Voltaic 2 | Class |
|---|---|---|
| `import std::iter::once` | **deleted** — names resolve against your dependencies | auto |
| `use`, `mod`, `module`, `island` | **deleted** | auto |
| `export` | `pub` | auto |
| `export(crate)` | nothing — a definition is visible in its file and its directory's descendants | auto |
| `pub(super)`, `pub(in path)` | the module tree, or a `rust` block | review |

On an ambiguous name you get `VLT0501`, naming every candidate and the dependency each
came from. Resolve it by writing the path in full.

### 3.9 Strings, macros, tests

| Voltaic 1 | Voltaic 2 | Class |
|---|---|---|
| `println! \-> "#{name}"` | `say "{name}"` | auto |
| `"#{x}"` | `"{x}"` | auto |
| `\->` nullary lambda | **deleted** — `say` is a function, and macros take expressions | auto |
| `assert! \-> cond` | `assert! cond` | auto |
| `test "name"` block | unchanged, and the tail expression is now the assertion | auto |

### 3.10 Kept exactly

Sections `(+ 1)`. Holes `_`. Pattern synonyms. `over` and `:=`. `infixl 6 <+> = merge`.
Attributes. `rust` blocks. Comments as trivia, preserved into the output. The off-side
rule. Comprehensions, with `in` as the generator word.

---

## 4. What you gain

Three things Voltaic 1 could not express, and one Rust cannot:

| | |
|---|---|
| **or-patterns** | `is_space ' ' \| '\t' \| '\n' = true`. Voltaic 1 graded this `UNREACHABLE`: `\|` was the guard token in the same position |
| **`as` casts** | `x : u8` — the solver knows whether it is a coercion or a cast |
| **`ref` / `ref mut` patterns** | a pattern binding's mode is inferred like any other |
| **refinements** | `head (xs [a] where xs.len > 0) = xs.0` — checked at compile time, erased at runtime, and `head []` does not compile. Rust cannot say this |

And one tool with no Rust equivalent: `voltaic check --semver` diffs your crate's
inferred API against a previous version's and tells you what narrowed. A written
signature cannot be diffed against the signature it replaced, because the old one is
gone.

---

## 5. Working without signatures

The source stops carrying them. Four things give them back:

```bash
voltaic sig src/cursor.vlt           # the inferred judgement of every definition
voltaic sig --why peek               # why `shared` and not `unique`
```

- **`<file>.vlt.sig`** is generated by `voltaic fmt`, committed, and diffed in review.
  A change to a `pub` definition's inferred type is a one-line diff in your pull
  request. It is also what downstream crates compile against — there is no cross-crate
  inference.
- **The LSP** shows every inferred judgement as an inlay hint, `--why` on hover, and a
  code action that writes an inferred signature into the source as an ascription.
- **`voltaic check --require-signatures`** demands a written ascription on every `pub`
  definition, checked against inference. Off by default. Voltaic's own crates turn it
  on, and that is the recommendation for a published library: infer internally, state
  your API.

---

## 6. The three constructs with no mechanical answer

These are the `manual` class. Each is a loss recorded in the specification rather than
discovered by the tool.

### 6.1 `do` over your own type

Voltaic 1's `do .and_then` works over **any** type with an `and_then`-shaped method.
Voltaic 2's effect row has a closed alphabet — `fail`, `none`, `async`, `io`, `mut`,
`panic`, `diverge` — so it cannot reach a user type.

Rewrite as an explicit chain:

```
# Voltaic 1
doubled = do .and_then
  n <- Tally { value = 5 }
  Tally { value = n * 2 }

# Voltaic 2
doubled = (Tally { value 5 }).and_then (n -> Tally { value (n*2) })
```

For a two-row block this is terser. **For a five-row chain, Voltaic 1 was terser**, and
the specification says so: doc 86 §5.6 records it as a debt rather than a decision, and
doc 93 §6 names the venue — a `with M` block modifier that names the type rather than
the method.

### 6.2 Nested optionals

Voltaic 1 deliberately does not flatten `Option<Option<T>>`. An effect cannot nest, so
Voltaic 2 does flatten, and `~` recovers the shape:

```
# Voltaic 1: Option<&Option<String>>
nickname_of u = try
  profile = &(.as_ref u)? .profile
  Some (&(.as_ref profile)? .nickname)

# Voltaic 2: ?(?String)
nickname_of u = ~u.profile.nickname
```

Without the `~` you get `?String` — flattened, and silently. The migrator cannot tell
whether you relied on the nesting, so it refuses. **Read every one of these**; the
terser spelling is the lossy one here, which is the wrong way round and is why this is
`manual` rather than `review`.

### 6.3 `deriving template` using `Fields` or `FieldNames`

A template using only `Field0` ports to a class-constraint receiver pattern. One using
`Fields` or `FieldNames` — every field as an access and as a string — needs reflection
over a declaration's shape, and Voltaic 2 declines it (doc 87 §22.4), naming a future
`macro` construct as the venue.

Until then: write the impl out per type, or use a proc-macro crate.

---

## 7. Compile times, and what to expect

Voltaic 2 solves where Voltaic 1 only transcribed. Three of the four stages are bounded
by construction — effect rows in at most 7 iterations per binding, modes in at most 2,
types linear in program size. The fourth, refinements, is the one with a policy:

- the solver runs **only** on definitions with a refinement in scope, so a crate with
  no refinements pays nothing;
- queries are cached;
- a timeout is a **diagnostic** (`VLT0802`), never a pass;
- `voltaic check --no-refine` skips the stage for a fast local loop.

If a build gets slower than you can explain, `voltaic check --no-refine` is the first
thing to try, and the difference tells you whether refinements are the cause.

---

## 8. If something goes wrong

1. **`voltaic migrate --dry-run`** first, always. The report tells you the `manual`
   count before you change anything.
2. **Commit between the two passes.** The verbose intermediate is the reviewable state.
3. **A rejected file is untouched.** The migrator writes to a temporary path, checks
   byte-identity of the generated Rust, and only then moves it into place.
4. **The byte-identity gate is the proof.** If `voltaic migrate` accepted a file, the
   Rust it now generates is byte-for-byte what Voltaic 1 generated. A behaviour change
   after an accepted migration is a migrator bug — file it, with the file.
5. **It cannot see a deleted test.** A comparison between generated output and its
   source cannot detect something absent from both. **Count your tests before and
   after**, per crate. Voltaic's own plan 74 lost seven inline tests this way with every
   gate green, and `docs/101 - 103 - The Generation Migrator.md` §5.9 makes it a
   release criterion for exactly that reason.

# Voltaic's grammar, as a graded inventory

Doc 02 §4.1 asks for this document and says what it is for:

> **any Rust construct marked `UNREACHABLE` is a construct Voltaic users cannot write**, which
> directly violates §26's interoperability charter.

So this is **risk detection**, not documentation. Plan 46
(`docs/40 - 46 - Grammar Inventories.md`) built it; `docs/03 - Voltaic Surface Design.md` §18 is
a summary of one axis rather than a substitute for this one.

**Every row carries evidence, and `crates/voltaic-cli/tests/grammar_inventory.rs` resolves it.**
That is not ceremony. Plan 46 §1.3 measured what an unchecked inventory costs: of sixteen
apparent gaps in a first careless pass, **eleven were the probe's own bug** — and a false
`UNREACHABLE` is worse than a missing row, because it sends a future plan to design a surface
for something that already works.

| Grade | Means | Evidence it carries |
|---|---|---|
| `PASSTHROUGH` | Voltaic writes the Rust construct essentially unchanged | a corpus file that uses it |
| `SUGARED` | reachable, through a different spelling | a corpus file, and the Rust it lowers to |
| `UNREACHABLE` | a Voltaic program cannot express it | a fixture under `tests/unreachable/` that **still fails**, and the nearest thing that does work |

An `UNREACHABLE` fixture failing is asserted, so when a later row closes a gap the fixture
starts compiling and the test breaks. It fails when the compiler gets *better*, deliberately
— the alternative is a document still claiming a gap years after it closed.

---

## 1. Rust's grammar

Organised by the Rust reference's areas rather than by Voltaic's constructs, so coverage can be
checked against the host language instead of against this project's history.

### 1.1 Lexical structure

| Area | Grade | Evidence and note |
|---|---|---|
| Integer, float, bool literals | `PASSTHROUGH` | `examples/expressions.vlt` — written through unchanged, suffixes included |
| String literals, escapes | `PASSTHROUGH` | `examples/strings.vlt` |
| String interpolation | `SUGARED` | `examples/strings.vlt` — `#{x}` becomes `format!`'s `{}` with the expression as an argument |
| Raw strings `r"…"`, `r#"…"#` | `PASSTHROUGH` | `examples/lexical.vlt` |
| Byte and byte-string literals | `PASSTHROUGH` | `examples/lexical.vlt` |
| C-string literals `c"…"` | `PASSTHROUGH` | `examples/lexical.vlt` |
| Char literals | `PASSTHROUGH` | `examples/lexical.vlt` |
| Multi-line strings `"""…"""` | `SUGARED` | `examples/strings.vlt` — no Rust equivalent; dedented per doc 01 §7.2 and emitted as one literal |
| Identifiers | `PASSTHROUGH` | `examples/hello.vlt` |
| Raw identifiers `r#type` | `PASSTHROUGH` | `examples/reachability.vlt` — plan 45 leaf 4; the `r#` spelling was already taken by raw strings |
| Line and block comments | `SUGARED` | `examples/comments.vlt` — `--` and `{- -}`, emitted as `//` and `/* */`; comments are trivia and reach the `.vrs` (doc 02 §10.1) |
| Doc comments | `SUGARED` | `examples/comments.vlt` — `---` becomes `///` |

### 1.2 Items

| Area | Grade | Evidence and note |
|---|---|---|
| Functions | `SUGARED` | `examples/hello.vlt` — a signature line and one or more equations, not `fn name(a: T)` |
| `const fn` | `SUGARED` | `examples/constants.vlt` — marker inside the `::` run |
| `async fn` | `SUGARED` | `examples/async.vlt` — likewise; plan 39 |
| `unsafe fn` | `SUGARED` | `examples/reachability.vlt` — likewise; plan 45 leaf 2 |
| `extern "C" fn` (export) | `UNREACHABLE` | `tests/unreachable/extern_fn_abi.vlt` — plan 45 shipped the *import* direction only. **No alternative** in the surface; inline Rust reaches it |
| Structs (braced) | `SUGARED` | `examples/data.vlt` — one constructor named for the type is a `struct` (doc 18 §2.4) |
| Tuple structs | `SUGARED` | `examples/data.vlt` — `newtype`, and a constructor with positional fields |
| Unit structs | `SUGARED` | `examples/data.vlt` — `data Marker = Marker` |
| Enums, variants, discriminants | `SUGARED` | `examples/data.vlt` — anything other than one self-named constructor is an `enum` |
| Unions | `SUGARED` | `examples/reachability.vlt` — `union`, plan 45 leaf 4 |
| Type aliases | `PASSTHROUGH` | `examples/generic.vlt` — `type` and the type verbatim |
| `const` and `static` items | `SUGARED` | `examples/constants.vlt` |
| Traits | `SUGARED` | `examples/class.vlt` — `class`, with default bodies, associated types and consts, supertraits |
| Generic traits | `SUGARED` | `examples/receiver.vlt` — `class Scale t` is `trait Scale<T>`, plan 67 leaf 2. Unreachable until then: the head's one slot named the *implementing* type, so a second lowercase name became a **method** generic and rustc answered `E0049` (plan 67 §1.4). `crates/voltaic-cli/tests/check/generic_class.vlt` is the program that used to |
| `unsafe trait` | `UNREACHABLE` | `tests/unreachable/unsafe_class.vlt` — plan 45 §6 asked for this check and plan 45 §10 did not record it. **No alternative**: an invariant rustc cannot check has no safe spelling |
| Impls, inherent and trait | `SUGARED` | `examples/class.vlt` — `instance` |
| Two impls of one trait for one type | `SUGARED` | `examples/receiver.vlt` — `instance Scale f64 Meters` and `instance Scale u32 Meters`, which is `impl From<T>`'s shape. The **last** name in the head is the implementing type (plan 67 §9 decision 3), so the head reads in the same order as the `impl Scale<f64> for Meters` it becomes |
| Blanket impls | `SUGARED` | `examples/binder.vlt` — `instance given <T: Display> Describe T` emits `impl<T: Display> Describe for T`, plan 84 leaf 4. Closed 2026-11-11. Through `=>` it is still unreachable and the cause is plan 67 leaf 3's: `instance Display t => Describe t` emits `impl Describe for t`, with a lowercase target rustc rejects, because `TypeApp::vars` reads the *arguments* and a bare variable is the head — so the generic list comes out empty and the bound goes with it. That is a **synthesis** failure rather than a type one, which is why writing the list verbatim is the whole fix and `TypeApp` needed no change |
| `unsafe impl` | `UNREACHABLE` | `tests/unreachable/unsafe_class.vlt` — same row, same fixture |
| Modules, inline and nested | `SUGARED` | `examples/module.vlt` — `module` |
| Items in a function body | `SUGARED` | `examples/local.vlt` — plan 55 leaf 5. A `data`, `errors`, `class`, `instance`, `import`, `const` or `type` is a block element and lowers to the nested Rust item. `test` is refused (`VLT0112`: rustc does not collect a nested `#[test]`) and `module` is out of scope, a bodiless one loading a file |
| `use` declarations, groups, globs, aliases | `SUGARED` | `examples/module.vlt` — `import` |
| `extern` blocks | `SUGARED` | `examples/reachability.vlt` — plan 45 leaf 3; `unsafe extern` is emitted because edition 2024 requires it |
| Nested `fn` items | `SUGARED` | `examples/wheres.vlt` — plan 61 leaf 0. A `where` entry may be a **function**, declared exactly as at module scope: a signature line and one or more equation rows, which emit as a nested `fn`. Mutual recursion among them needs no ordering, because Rust items are order-independent inside a block. Voltaic's `where` had no graded row at all until this one — which is why plan 61's criterion 8 was free, and why it should not have been |
| Macro definitions | `UNREACHABLE` | `tests/unreachable/macro_def.vlt` — **declined** under doc 03 §23 item 3. Alternative: inline Rust, which plan 45 leaf 1 shipped and plan 64 leaf 4 gave an expression position and a layout-delimited form. Invocation is `SUGARED` since plan 64; this row is the half that stays declined, which is plan 46's rule working in both directions at once |
| deriving templates | `SUGARED` | `examples/template.vlt` — plan 89. **Rust has no file-local derive at all**: `#[derive(Forward)]` needs a `proc-macro = true` crate of its own, `syn`, `quote` and a second compilation unit, so a trait one crate wants forwarded three times is written out three times. `deriving template Forward` declares an `instance` whose target is the reserved name `Target`, and every declaration that names `Forward` in its `deriving` list gets that `instance` with `Target` replaced by its own type name and `Field0`… by its field names. The lowering is **textual substitution on the emitted Rust**, so rustc checks the generated `impl` exactly as it would a hand-written one, and a template name is split out of `#[derive(…)]` by a table lookup rather than by name resolution — an undeclared name stays in the derive list, which is `E0433`'s job and doc 03 §9.5's existing behaviour. `template` is Voltaic's **third contextual keyword** after `from` and `pattern`. **Repetition shipped as leaf 3**: `Fields` is every field as a `self` access and `FieldNames` every field's name as a string literal, both comma-separated, so one template visits declarations of any width — `examples/template.stdout` asserts `x=1, y=2` and `a=3, b=4, c=5` from the same template. The loop runs over the declaration's own field list and produces the text that is substituted, which is what `emit_data` already does per field; doc 03 §13.3's line against `comptime` is untouched, because an interpreter runs the author's code and this joins a list the parser produced. What is still missing needs a **construct** rather than a table entry: a per-field *fragment* (a whole `fn` per field, a `match` arm per variant), and `$variants`, which also wants a constructor choice `field_names` refuses to make for a multi-constructor `data`. Plan 89 §10.3 records both |
| Visibility modifiers | `SUGARED` | `examples/visibility.vlt` — **added 2026-11-14** by row 104 leaf 1, which found this had no row at all. `export` is `pub`, `export(crate)` is `pub(crate)`, and `export(in path::to)` is `pub(in path::to)`; `Visibility` in `crates/voltaic-ast/vlt/item.vlt` carries all five forms including `super`. Private is the default and unspelled, as it is in Rust. The construct was reachable and exampled for a whole generation and simply ungraded, which is the gap a completeness pass exists to catch: neither `grammar_inventory` nor `construct_coverage` can notice a row that is **absent**, because both start from this document's rows |
| Attributes, inner and outer | `PASSTHROUGH` | `examples/attributes.vlt` — Rust's own `#[…]`, plan 36. **One exception, and only one:** plan 53's `#[voltaic::allow(unwrap)]` is *consumed* by the emitter and never reaches the `.vrs`, because an un-consumed `voltaic::` attribute is rustc's `E0433` — a hard error, not an unknown lint. **Placement** (leaf v5): on imports, items, statements, `case` arms (`examples/case.vlt`), equations that become `match` arms (`examples/guards.vlt`) and class `type`/`const` members; above the only equation of a function that binds every parameter it is `VLT0103` (`crates/voltaic-cli/tests/ui/equation_attribute.vlt`). An attributed parameterless binding is a function, never a `const`. Doc 87 §11 has the table |

### 1.3 Types

| Area | Grade | Evidence and note |
|---|---|---|
| Paths and generic arguments | `PASSTHROUGH` | `examples/generic.vlt` — a type is verbatim text (doc 03 §14.1) |
| References, `&mut` | `PASSTHROUGH` | `examples/class.vlt` |
| The receiver, marked in a type | `SUGARED` | `examples/receiver.vlt` — plan 67. `self` in a type is the receiver at **any** position, and the borrow prefix on the marker chooses between `&self`, `&mut self` and `self`. `minus : u32 -> &self -> u32` and `plus : &self -> u32 -> u32` emit byte-identical Rust |
| Raw borrows `&raw const`, `&raw mut` | `UNREACHABLE` | `tests/unreachable/raw_borrow.vlt` — **added 2026-11-14**, row 104 leaf 1; no row existed. `raw` is an ordinary identifier, so `&raw const n` lexes as `&raw` applied to `const` and the parser stops at the keyword with `VLT0101`. Nearest: `std::ptr::addr_of!`, which is what `&raw` replaced in Rust 2024 and which Voltaic can call — so the capability is reachable and only the operator spelling is not, the same cosmetic shape as the `\|` or-pattern row. Closing it is a surface **addition**, which doc 97 wave 9's note records as unavailable in generation 2 |
| Raw pointers | `PASSTHROUGH` | `examples/reachability.vlt` — verbatim, as any type is |
| Tuples | `PASSTHROUGH` | `examples/destructuring.vlt` |
| Arrays and slices | `PASSTHROUGH` | `examples/arrays.vlt` |
| Trait objects `dyn Trait` | `PASSTHROUGH` | `examples/generic.vlt` |
| `impl Trait`, argument and return | `PASSTHROUGH` | `examples/generic.vlt` — plan 25 |
| Function pointers and `Fn` bounds | `PASSTHROUGH` | `examples/generic.vlt` |
| Lifetimes | `PASSTHROUGH` | `examples/strings.vlt` — `&'static str`; carried verbatim, so there is no separate construct |
| Generic parameters and bounds | `SUGARED` | `examples/generic.vlt` — a **bound clause** read off `=>` (doc 18 §2.2), not `where`. **The clause accepts only `Trait var`**, where the trait is a bare type identifier and the variable a bare lowercase word; `Ord t` and `Ord t, Clone t` work and the six rows below do not. This row read as unrestricted until plan 84 leaf 0 measured it |
| `where` clauses | `SUGARED` | `examples/generic.vlt` — the same bound clause, with the same restriction. Voltaic's own `where` introduces **local bindings** and is a different construct, graded on the row below |
| The verbatim binder, `given <…>` | `SUGARED` | `examples/binder.vlt` — plan 84 leaf 1. Doc 03 §14.1's argument applied one level up: a **type** is carried verbatim, and so now is the **binder**. `given` turns doc 18 §2.2's synthesis off for its signature entirely — no generic list, no `=>` bounds, no type rewrite (plan 84 §1.6) — so lowercase spine atoms are ordinary verbatim types. The six rows below are what it bought, each `UNREACHABLE` until this row |
| A bound whose trait has type arguments | `SUGARED` | `examples/binder.vlt`'s `shout` — `given <T: Into<String>>`. **The largest of the six**: `T: Into<String>`, `U: From<T>`, `I: IntoIterator<Item = T>` and `F: Fn(i32) -> i32` are all this shape. Through `=>` it is still unreachable — `Into<String> t` is not `Trait var` — and that is now **`VLT0120`** rather than the silent swallowing plan 84 leaf 0 measured, which emitted `x: Into<String> T FatArrow T` and `E0782` |
| A projection predicate | `SUGARED` | `examples/binder.vlt`'s `total` — a signature-position `where`, which is doc 03 §14.2's declared role for that position and was unimplemented until plan 84 leaf 3. The `=>` clause pairs a trait with a *variable*, so a path has no position in it |
| A relaxed bound `?Sized` | `SUGARED` | `examples/binder.vlt`'s `width` — `given <T: ?Sized>`. Plan 84 leaf 0 found the `?` had **no respelling at all**, so it reached generated Rust as the text `QuestionSized` — a `TokenKind`'s own Debug name. `token_text` is complete as of leaf 1 and `respells_every_operator_the_lexer_produces` asks the lexer rather than a second table |
| A bound between two lifetimes | `SUGARED` | `examples/binder.vlt`'s binder clauses — a `given <'a, 'b: 'a>` or a signature `where 'a: 'b` carries it verbatim. Through `=>` it is unreachable, because the clause has one trait slot and a lifetime bound has a lifetime on both sides |
| Const generic parameters | `SUGARED` | `examples/binder.vlt`'s `first_of` — `given <const N: usize> [u8; N]`. Synthesis cannot reach it because `type_vars` scans lowercase words in verbatim type text without distinguishing a type position from an array's length position, so the `n` in `[u8; n]` became a **type** parameter and `fn first_of<N>(xs: [u8; N])` used a type as a value (`E0423`) |
| Higher-ranked bounds `for<'a>` | `SUGARED` | `examples/binder.vlt`'s `trimmed` — `where for<'a> F: Fn(&'a str) -> &'a str`. Synthesis was worse than absent here: it excludes only `dyn`, `impl`, `mut` and `as`, so `for` and `fn` both became type parameters — `fn calls<'a, FOR, FN>(f: FOR<'a> FN(&'a str) -> &'a str)`, `E0109` and `E0618` |
| Associated types and constants | `SUGARED` | `examples/class.vlt` |
| Associated functions | `SUGARED` | `examples/receiver.vlt` — a member with no `self` in any of its types has no receiver, which is how Rust spells one too: `start : u32 -> Self` is `fn start(__vlt_a0: u32) -> Self`, called `Counter::start 3`. Plan 67 leaf 4; the construct existed before the row and had no witness |

### 1.4 Expressions

| Area | Grade | Evidence and note |
|---|---|---|
| Operators and precedence | `PASSTHROUGH` | `examples/expressions.vlt` |
| Functional update through a path | `SUGARED` | `examples/optic.vlt`'s `over` — plan 88 leaf 3. `over (.addr.zip.code) doubled target` is `target.addr.zip.code = doubled(target.addr.zip.code)`: the path is said **once** in the source and twice in the output, because Rust has no other spelling for *apply through a place*. That is the construct's whole argument — two spellings of one path that must agree is the shape of a typo that type-checks, and `target.addr.zip.code := doubled target.addr.zip.town` compiles whenever both fields are the same type. The lowering is an **assignment through the place**, so nothing is moved, nothing is cloned and rustc judges the borrow; the emitted Rust is byte-identical to the `:=` form. The function is read as an *atom*, which is what lets it be a bare method reference — `over (.addr.city) .to_uppercase u` — because the place is a separate atom rather than the method's receiver. `over` is Voltaic's **fourth contextual keyword**, settled by three tokens: the word, `(`, then `.`. `VLT0127` refuses a selector that is not a path; an operator section cannot reach it, because the dispatch requires `(.` |
| Operator declaration | `SUGARED` | `examples/operator.vlt` — plan 90. **Rust has no custom operator at all**, and `docs/DECLINED.md` refused this for that reason — *"no Rust lowering target exists"* — which plan 90 §1.1 corrected: the surface already carries six operators Rust has no counterpart for, and every one lowers to an ordinary expression. `infixl 6 <+> = merge` gives an **existing** function a spelling and a fixity and emits no item, so `a <+> b` is `merge(a, b)`: nothing is declared, nothing renamed, and go-to-definition has somewhere real to go. A Haskell-style *definition* stays out of reach, because no Rust function can be named `<+>` and inventing one is the fabricated API doc 03 §8.7 refuses. The spelling set is **closed** — twelve `<` · *char* · `>` forms — which is what kept the lexer static where §1.3 priced a permanent pre-pass over raw source; an undeclared use is `VLT0123` from the parser, where declaration order is already known. Precedence is Haskell's 0-9, mapped to the even binding powers between `|>` and `+`, so every user operator binds looser than arithmetic. `infix` is non-associative by **diagnostic** (`VLT0125`), because a Pratt loop's powers can only group one way. `infix`/`infixl`/`infixr` are contextual, settled by a three-token lookahead |
| Method calls, field access | `SUGARED` | `examples/receiver_first.vlt`, `examples/comprehension.vlt`, `examples/application.vlt`. **Regraded by row 104 leaf 1**, and the row gained a citation rather than a grade: it cited only the last two, and **neither witnesses the receiver-first form** — the spelling row 103 wrote the whole compiler in, 8,479 call sites. `examples/receiver_first.vlt` does witness it and was simply not cited, which is a weaker defect than an unwitnessed row and a real one: the gate resolves a row's evidence and cannot tell that a *better* witness exists.<br><br>**Three forms, and position decides which.** `recv.name` with nothing after it is a **field read**. `recv.name args` is a **method call, receiver first**. `recv.name ()` is a method call taking **no** arguments — the parentheses are the whole difference from the field read, since `h.items.len` would select a field called `len`. A `.` in *head* position is still a call with the receiver **last**, so `.len xs`, `xs \|> .len` and `.insert k v m` all remain live; doc 97 §1.3 deletion 4 retires receiver-last from the compiler's own source, not from the language. `(.age)` is the selector because a `)` follows the name.<br><br>**Chaining needs no parentheses between zero-argument links** — `h.items.len ().count_ones ()` — on two type-free parser rules: no postfix is meaningful on a unit literal, so the `()` stays the empty argument list rather than becoming an argument; and an application continues its own postfix chain, without which the `?` in `x.ok ()?` would dangle. A link that **carries arguments** does need its receiver bracketed — `((h.items.iter ()).map (\n -> *n * 2)).collect ()` — because `(g x).y` is a legitimate expression and there is nothing type-free to decline.<br><br>A method whose name is a Voltaic keyword is **unspellable**, which is what plan 55 leaf 7 bought back for one: `Ordering::then` was `VLT0103` while `then` was reserved, and `examples/control.vlt`'s `ordered` now writes it |
| Function calls | `SUGARED` | `examples/hello.vlt` — parenless application, Haskell's rule (doc 18 §2.1) |
| Indexing | `PASSTHROUGH` | `examples/arrays.vlt` |
| Tuple indexing `.0` | `PASSTHROUGH` | `examples/destructuring.vlt` |
| Ranges | `PASSTHROUGH` | `examples/control.vlt` |
| Struct and record literals | `SUGARED` | `examples/record.vlt` — braces with `=` for values |
| Array literals and repeats | `SUGARED` | `examples/arrays.vlt` — `[v; n]`, and `:: Vec<T>` for the growable form (plan 29) |
| Closures | `SUGARED` | `examples/expressions.vlt` — `\x -> e` |
| The composed selector, `(.a.b.c)` | `SUGARED` | `examples/optic.vlt` — plan 88. `(.name)` selected **one** field; this selects a path, at any depth, and is what a sort key or a `map` actually wants. Graded here because leaf 0 found the old reading was not *absent* but **wrong**: `(.addr.city)` parsed as a selector of `addr` followed by a *method call* on the closure it produced, emitting `(\|__vlt_1\| __vlt_1.addr()).city(o)` — not Rust, no Voltaic diagnostic, and `E0282` from rustc. Milestone 9's worst class, found by writing the second dot. The **functional** setter is a different matter and is declined rather than missing: see `docs/DECLINED.md` |
| The application hole, `_` | `SUGARED` | `examples/hole.vlt` — plan 85. `_` in an **argument** position is a closure parameter, so *n* holes make an *n*-parameter closure and the callee is never consulted. It reaches what currying cannot: plan 56 §11.1 proved arity is knowable only for a bare name with an in-file signature, so a Rust path (`String::from _`), a method (`.insert "a" _ m`) and an imported name (`max _ 0`) were all uncurryable. It also abstracts over **any** position where currying takes a suffix. **`_` now has four jobs and position decides** — the other three are a pattern's wildcard, a type's inferred `_`, and the target of `:=`, all unchanged. It was never `VLT0101`: a bare `_` already emitted verbatim, so `add _ 2` gave `add(_, 2)` and rustc's *"`_` can only be used on the left-hand side of an assignment"*, which is still the answer for a hole with no enclosing application |
| A method awaiting its receiver | `SUGARED` | `examples/hole.vlt`'s `trimmed` — `.map (.trim _)`. **Unwritable before plan 85**, and the README's own table was wrong about it: `(.trim)` is the *selector*, because a `.name` followed by `)` selects a field, so it emitted `\|x\| x.trim` and rustc answered `E0615`, *"attempted to take value of method"*. A hole makes the receiver the closure's parameter, which is the `\|x\| x.trim()` the README claimed all along |
| Assignment to `_` | `PASSTHROUGH` | `examples/hole.vlt` — `_ := e` is Rust's discard assignment, `_ = e;`. Reachable since plan 54 and **in no corpus file until plan 85**, which is why `grammar.js` still parsed it as a wildcard pattern followed by an `ERROR`; the assignment row below lists five target shapes and did not list this one |
| `move` closures | `SUGARED` | `examples/async.vlt` |
| Blocks as expressions | `SUGARED` | `examples/control.vlt` — an **indented block**, with an optional `'label`. Spelled `do` and `do 'label` until plan 55 leaf 7 retired the keyword: a block is an expression, so nothing announces one. `do` stays reserved and is `VLT0114`. The three rows below are **modifiers on this one block**, not constructs of their own |
| `async` blocks | `SUGARED` | `examples/async.vlt` — plan 39; `work = async` and the block indented under it. Regraded at plan 55 leaf 7: `async`, `unsafe` and `try` were three `prefix` arms of one shape and are now one arm reading an adjective and then the block above. `async move` with the block indented beneath emits `async move { … }` (doc 03 §14.3, amended 2026-11-20): `tests/probes/id_effect/async_move_block.vlt` and `tests/probes/id_effect/from_async_move.vlt` (an example waits on a seed refresh: the g2 seed cannot read `async move`); on a non-block it is the error in `crates/voltaic-cli/tests/ui/async_move_not_block.vlt` |
| `unsafe` blocks | `SUGARED` | `examples/reachability.vlt` — plan 45 leaf 2; `n = unsafe` and the block indented under it. A modifier on the block above, since plan 55 leaf 7 |
| `try` blocks | `SUGARED` | `examples/fallible.vlt` — an immediately-invoked closure, because Rust has try blocks only unstably. `r = try` and the block indented under it; a modifier on the block above, since plan 55 leaf 7 |
| `do .method` — the named bind | `SUGARED` | `examples/bind.vlt` — plan 86, and **the only decline this project has ever lifted**. `docs/DECLINED.md` refused a *"general monadic comprehension"* for having to *"choose `flat_map` versus `and_then`"*, with the sole reopening venue *"revisit only if a type-free discrimination is found"*; plan 60's block modifier already was one. The fifth construct of the modifier shape and the only one whose modifier **names a method** — so unlike `try` and `async`, which read `<-` as a *postfix* and emit a statement per row, this folds the rows **rightwards into nested closures** with `move` on each, which is what doc 03 §10.1's comprehension already does. Nothing is fabricated, the final row included: choosing between `Ok`, `Some` and `iter::once` would be the inference §2 clause 1 forbids. `do` is still `VLT0114` in every other position, and the grammar reuses the `(reserved_word)` node so the word stays globally reserved |
| `?` operator | `PASSTHROUGH` | `examples/fallible.vlt` |
| `.await` | `SUGARED` | `examples/async.vlt` — the draw arrow `<-` (doc 30) |
| `as` casts | `UNREACHABLE` | `tests/unreachable/as_cast.vlt` — `as` is reserved for `import … as`. Nearest: `u8::try_from n`, which is **not** a cast — `as` truncates, `try_from` is fallible |
| Macro invocations | `SUGARED` | `examples/macros.vlt` — plan 64. `name! \-> e`: `!` marks the name and a nullary lambda opens each argument, so no bracket appears. Interpolation splices, so `bail! \-> "no route for #{id}"` reaches Rust as two arguments. Was `UNREACHABLE` for nineteen rows — doc 18 §7 spent `!` before any row implemented it, and plan 48 priced the gap at the figure `docs/CONFORMANCE.md` carries |
| Macro invocation with a non-expression argument | `PASSTHROUGH` | `examples/macro_bodies.vlt` — row v2, regraded 2026-11-20 from `UNREACHABLE` (plan 81). A **verbatim body** — `name! { … }`, `name! ( … )`, `name! [ … ]`, the bracket right after the `!` or after spaces, or an indented block under a bare `name!` at the end of a line — is one lexer token copied through as Rust, so `matches!`'s pattern, `thread_local!`'s item and an `rsx!` DSL are all reachable. The bracket kind is kept, the indented form becomes `name! { … }`, and each body line maps to its own `.vlt` line. Nothing inside is Voltaic: `--`, `~` and `#{…}` are Rust text, and `//` comments survive. Brackets are balanced Rust's way — strings, raw strings, chars and comments do not count, a lifetime is not a char — and an unclosed body is `VLT0012` at its opener. `name! \-> e` is unchanged beside it |
| Turbofish `x.parse::<T>()` | `SUGARED` | `examples/hole.vlt` — **added 2026-11-14**, row 104 leaf 1; no row existed. Voltaic emits no `::<>` and needs none: the **ascription** pins the type and rustc infers the rest. `(s.parse ()) : Result<u32, ParseIntError>` lowers to a typed `let` binding, and `words : Vec<&str> = (s.split …).collect ()` likewise. So the construct is reachable through a different spelling, which is what `SUGARED` means — and the turbofish's usual job, disambiguating an inferred type parameter, is the one thing Lattice already has to do anyway |
| Inline Rust | `SUGARED` | `examples/reachability.vlt` — doc 01 §23.1, plan 45 leaf 1. No Rust equivalent: it *is* Rust. Plan 64 leaf 4 added the **expression** position and a layout-delimited body beside the `"""` string form, because a token-stream DSL's input is not a Voltaic expression and no value-passing shape can carry it |

### 1.5 Patterns

| Area | Grade | Evidence and note |
|---|---|---|
| Literal and wildcard patterns | `PASSTHROUGH` | `examples/case.vlt` |
| Identifier and binding patterns | `SUGARED` | `examples/destructuring.vlt` |
| `@` bindings | `PASSTHROUGH` | `examples/case.vlt` |
| Pattern synonyms | `SUGARED` | `examples/synonym.vlt` — plan 87. **Rust has no pattern abstraction at all**: a pattern cannot be named, reused or parameterised, and `macro_rules!` does not reach the ergonomics, which is why the RFC has stood unaccepted for years. The lowering is **textual substitution at emit time**, so rustc performs every check it would have on the written-out form — exhaustiveness, field names, types, binding modes — and doc 03 §2's corollary holds both ways. Reaches an equation head, a `case` arm and a position **nested inside another pattern**. Parameters are *formal*, so every binder is written at the use site, which is the one property separating this from `docs/DECLINED.md`'s refusal of `RecordWildCards`; `VLT0121` refuses a body that binds anything else. `pattern` is Voltaic's **second contextual keyword** after `from` — reserving it was tried and reverted, because it is a field name in four of this compiler's own declarations |
| A bare constructor as a clause's whole pattern | `SUGARED` | `examples/synonym.vlt`'s `describe` — `describe Origin = …`. Graded 2026-11-11 by plan 87, which **fixed it**: an upper-case `Binding` is a constructor and not a binder, and `signature_from` was lending its name to the parameter — so `pick None = 0` emitted `fn pick(None: Option<i32>)`, a refutable pattern in argument position that rustc answers with `E0005`. Pre-existing, and no corpus file had one, because every existing clause pattern is an `Apply` or a `Record`. The parameter now takes `__vlt_a0` and the constructor moves into the `match` |
| Parameter names of a multi-equation function | `SUGARED` | `examples/guards.vlt` — leaf v5 (gap G4). Position *i* takes the first lower-case binder from **any** equation, so `fib 0 = 0; fib n = …` is `fn fib(n: u64)`, but only when every other equation binds that name in that position or never mentions it; otherwise, and for a name bound in two positions (`pick`), it is `__vlt_aN`. `crates/voltaic-cli/tests/check/clause_scope.vlt` is the program that proves an arm cannot read a name it did not bind. A required class member has no equation and keeps `__vlt_aN` by design. Doc 87 §2.2 |
| Struct and record patterns | `SUGARED` | `examples/destructuring.vlt` |
| Tuple patterns | `PASSTHROUGH` | `examples/destructuring.vlt` |
| Slice patterns, with `..` | `PASSTHROUGH` | `examples/destructuring.vlt` |
| Range patterns | `PASSTHROUGH` | `examples/case.vlt` |
| `ref` and `ref mut` | `UNREACHABLE` | `tests/unreachable/ref_pattern.vlt` — regraded 2026-11-10. `ref` is **not reserved**, so `ref a` lexes as two identifiers and the pattern parser answers `VLT0101`; the old note graded this `PASSTHROUGH` and cited a destructuring example that contains no `ref` at all. Nearest: **match ergonomics**, which is what that example actually uses — `case &pair of (a, b) ->` binds by reference with no keyword. What is lost is the case where ergonomics do not apply and a binder must be explicitly `ref mut` |
| `\|` or-patterns | `UNREACHABLE` | `tests/unreachable/or_pattern.vlt` — doc 18 made `\|` a **guard**, so `1 \| 2 ->` parses and means `1` guarded by `2`. Nearest: **separate arms**, which match the same values; what is lost is one arm body shared across patterns |

### 1.6 Statements and control flow

| Area | Grade | Evidence and note |
|---|---|---|
| `let` bindings, `let mut` | `SUGARED` | `examples/expressions.vlt` — `x = e` and `mut x = e` |
| Assignment | `SUGARED` | `examples/control.vlt` — `x := e`, because `=` is a binding (plan 54). Covers a name, a field, an index, a deref target and Rust's destructuring assignment, the target being a general expression |
| Compound assignment | `SUGARED` | `examples/control.vlt` — all five of `+=`, `-=`, `*=`, `/=`, `%=`. Shipped in plan 20 and graded here for the first time |
| Shadowing | `PASSTHROUGH` | `examples/control.vlt` — an inner `n = 1` inside a nested block over an outer `n`, which is the contrast that makes `:=` necessary. Cited `examples/expressions.vlt` until plan 54; that file holds exactly one binding, and nothing in the corpus shadowed anything |
| `if` / `else` | `SUGARED` | `examples/control.vlt` — an expression, with layout, and **one spelling** since plan 55 leaf 7 retired `then`. A conditional in argument position is written indented inside the brackets (plan 65); inside a `#{…}` interpolation it cannot be written at all, and `bracketed` calls one instead |
| `if let` | `SUGARED` | `examples/draw_condition.vlt` — `if e is P`, plan 60. Spelled `if P = e` until that row moved the pattern test onto `is` and put the scrutinee first |
| `let` chains | `SUGARED` | `examples/draw_condition.vlt` — the draw arrow in a condition (doc 30) |
| `while`, `while let` | `SUGARED` | `examples/control.vlt` |
| `loop` | `SUGARED` | `examples/control.vlt` |
| `for` | `SUGARED` | `examples/control.vlt` — `for x in xs` |
| `break`, `continue`, with value | `PASSTHROUGH` | `examples/control.vlt` |
| Labelled loops and blocks | `SUGARED` | `examples/control.vlt` — `'outer loop`, `'label` on a bare block, `break 'label v`. The label is the one capability `do` carried that nothing else provides (plan 55 §1.7), so it outlived the keyword |
| `match` | `SUGARED` | `examples/case.vlt` — `case … of`, with guards |
| Guards | `SUGARED` | `examples/guards.vlt` — `\|` in an arm, which is what shadows Rust's or-pattern |
| `let … else` | `SUGARED` | `examples/else_tail.vlt` — **added 2026-11-14**, row 104 leaf 1; no row existed, though doc 87 §5.6's `else` tail shipped with row 99 wave 2. `Some n = held else` followed by an indented diverging body is `let Some(n) = held else { … };`. The pattern must be **refutable** for the `else` to be reachable at all, which is why §5.6's own example was corrected from a plain binder: `x = held else` compiles with an unreachable tail, and that is a worse document than no example |
| `return` | `PASSTHROUGH` | `examples/control.vlt` — `first_divisor`, returning out of a `for`. Regraded at plan 55 leaf 8: `return` is an **early exit** and nothing else. It used to be the only way a branch could produce a value, which plan 55 §1.2 measured at fourteen non-tail uses against two conditional expressions in the same corpus; every body position is an expression now, so `examples/draw_condition.vlt`'s `describe` dropped its three |

### 1.7 Beyond the grammar

Two areas that are not productions but are what a user actually hits.

| Area | Grade | Evidence and note |
|---|---|---|
| `std` and crate paths | `PASSTHROUGH` | `examples/fallible.vlt` — any path works, which is what makes the macro gap survivable |
| Error types and conversions | `SUGARED` | `examples/errors.vlt` — `errors`, the one declaration that emits more than one Rust item |
| Tests | `SUGARED` | `examples/testing.vlt` — `test "name"`, gathered into one `#[cfg(test)]` module per file |

---

## 2. MoonScript's grammar

Doc 02 §4.1 asked for this axis graded `KEEP` / `MODIFY` / `REMOVE` against Emerald's
761-line Ruby inventory. **Doc 18 then replaced the surface**, and doc 18 §10 records this
axis as *"mostly moot"*.

Grading a discarded grammar production by production would be archaeology, so what follows is
the question a reader actually has — what survived — with doc 18 §§2.1–2.7 cited for the
rest. Plan 46 §6 records the decline.

| MoonScript trait | Fate | Replaced by |
|---|---|---|
| Significant indentation | **KEEP** | unchanged: the off-side rule is the one thing both source languages agree on |
| `->` and `=>` for functions | **REMOVE** | equations and a signature line (doc 18 §2.1) |
| `if`/`unless`/`while` as expressions | **MODIFY** | kept as expressions, Haskell's layout (doc 18 §2.3) |
| Implicit return of the last expression | **KEEP** | unchanged, and it is Rust's rule too |
| Comprehensions | **KEEP** | `examples/comprehension.vlt` |
| `class` with `@` fields and `self` | **REMOVE** | `data` plus `class`/`instance`, which are Haskell's trait and impl rather than an object (doc 18 §2.5) |
| String interpolation `#{…}` | **KEEP** | `examples/strings.vlt` |
| Table literals | **REMOVE** | records and `data`; Voltaic has no table type |
| `switch`/`when` | **REMOVE** | `case … of` (doc 18 §2.3) |
| `with` blocks | **REMOVE** | doc 18 §7 deleted them; the keyword was reused for the callback bind (doc 30 §3.3) |
| `do` blocks | **REMOVE** | an indented block **is** an expression, so no keyword announces one — plan 55 leaf 7. The labelled form survives as `'label` on a block, and the word stays reserved for plan 60's do-notation (`VLT0114`). Graded **MODIFY** — *"a labelled block expression (doc 03 §7.2)"* — until that leaf |

---

## 3. What this document is not

- **A list of declines.** `UNREACHABLE` means *cannot be written*; a decline means
  *deliberately not offered*. Doc 03 §22's declines are plan 47's `docs/DECLINED.md`, and
  conflating the two would make both useless. The one row that is both — macro definition —
  says so.
- **A claim that every citation exercises its construct.** The test asserts the cited file
  exists. Asserting it *contains* the construct needs a parse and a query per row, which is
  plan 41's grammar doing this document's job; the `UNREACHABLE` fixtures carry the stronger
  property instead. Plan 46 §6 records it.
- **A library inventory.** This is the grammar. `std` reachability is a different question
  with a different answer, and plan 47's status document is where that gap table belongs.

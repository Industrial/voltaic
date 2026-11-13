# Plan 96 — Mode Inference

Plan ID: `modes`
Depends on: 94, 95
Milestone: 15 — *the solver*
Status: `planned`

---

## 1. What this plan is

`voltaic-lattice/vlt/mode/`: a three-element chain, use-site constraints, a
least-fixed-point solve, lifetime emission where elision fails, `@` ascription, and
`VLT0601`.

**This is the row that earns the generation.** Doc 86 §1.1's finding is that
generation 1 saves 1.002× of the characters of its own output, and the characters it
cannot delete are types and ownership. Rows 94 and 95 delete the types. This row
deletes the ownership, and ownership is the larger half.

### 1.1 The chain

```
shared  ⊑  unique  ⊑  owned
  &T        &mut T       T
```

Three elements. Each binding gets a mode variable; uses impose lower bounds:

| Use | Lower bound |
|---|---|
| read a field, call a `shared` method, pass to a `shared` parameter | `shared` |
| write a field, call a `unique` method, `+=`, `:=` | `unique` |
| return past the frame, store in a longer-lived structure, move into a closure that escapes | `owned` |

The solve assigns the **least** mode satisfying every bound, which terminates in at
most two iterations per binding because the chain has three elements. That is this
row's clause-5 argument, and like row 95's it is a counting argument.

### 1.2 What is deleted

`&`, `&mut`, every lifetime, `.clone()`, `Box::new`, `Rc::new`, `.as_ref`,
`.to_owned`, `.borrow`. None appears in `.vlt` source. **All** appear in the emitted
`.vrs`, in full (doc 86 §6.6).

Generation 1, from `crates/voltaic-lexer/sdx/cursor.sdx`:

```
peek : &self -> Option<char>
peek self = self.src |> .get (self.pos .. .len self.src) |> .unwrap_or "" |> .chars |> .next
```

Generation 2:

```
pub peek self = self.src.get(self.pos .. self.src.len).unwrap_or("").chars.next
```

Two lines to one, and the `&` in `&self` is gone because the body only reads.

### 1.3 Lattice does not infer lifetimes

Doc 86 §6.3 is the rule and it is a deliberate narrowing of this row's scope.

The emitter writes **elided** lifetimes and lets Rust's own elision rules resolve
them, because those rules are already complete for the shapes the emitter produces: a
function whose parameters and return are all borrows from `self` elides. Where elision
fails — a function returning a borrow from an unrelated parameter — the emitter
introduces `'__vlt_a`, `'__vlt_b`, … in parameter order.

The reason not to infer lifetimes is that rustc's borrow checker is the authority
(doc 86 §2, first corollary) and a second lifetime analysis would be the thing the
charter reversal explicitly did *not* license. Row 92 §1.4: Lattice decides what to
write down; it does not decide whether a program is well-borrowed.

### 1.4 No silent clone, and this is the non-negotiable one

If no mode assignment satisfies every bound, the answer is **`VLT0601`**, naming the
conflicting uses and the modes each demands. It is never a `.clone()`.

Doc 01 §25.1 declines implicit cloning *non-negotiably*, and doc 86 §6.5 keeps it.
The reversal in row 92 is about clause 1 and clause 1 only; a solver that silently
allocated to make a program type-check would be a different and much worse language.

A clone is spelled, because it is a decision about cost:

```
b = a@clone
```

`@clone` is the only allocating mode and the only one an author writes in ordinary
code. `@owned`, `@shared`, `@unique` exist for pinning and are checked like any other
ascription.

### 1.5 The zero-cost gate is the real test of quality

`crates/voltaic-bench/tests/zero_cost.rs` asserts that generated and hand-written Rust
produce **identical optimised LLVM IR**. For every row before this one it is a
regression check. For this row it is the *only* automatic check that the inference is
not merely correct but **good**: a mode solve that picks `owned` where `shared` would
do compiles fine, runs fine, and fails this gate.

That makes `benchmarks/pairs/` — ten programs with hand-written Rust counterparts —
this row's primary corpus, ahead of `tests/solve/`.

---

## 2. Preconditions

- Row 94 `done`: store, driver, provenance.
- Row 95 `done`, because **a discharge creates mode constraints**. `e else x -> body`
  binds the error, and whether that binding is `shared` or `owned` depends on what
  `body` does with it. The row engine must have run before the mode engine can see
  that bound.
- `benchmarks/pairs/*.vlt` renamed by row 93 and still green under generation 1, so a
  failure in this row is attributable.

---

## 3. Concrete proof this plan targets

`crates/voltaic-lexer/vlt/cursor.vlt` with every `&self`, `&mut self` and `&'a str`
deleted from the source, generating a `.vrs` whose borrows are **byte-identical** to
generation 1's `cursor.xrs`.

Twelve members, each hand-annotated in generation 1, four of them `&mut self` and
eight `&self`. The file is the answer key, and it is a good one precisely because its
mode choices are not uniform: `bump`, `eat`, `eat_while` and `eat_spaces` mutate;
`peek`, `pos`, `is_eof`, `starts_with`, `rest`, `slice`, `mark` do not; `reset_to`
mutates through `:=`.

---

## 4. Leaves

### Leaf 1 — the chain and the constraint kinds

The three elements, the ⊑ relation, the table in §1.1 as constraint generators, and
the iteration bound asserted in row 94's driver.

### Leaf 2 — the solve

Least fixed point over the crate's call graph. A parameter's mode is a function of
every call site *and* every use in the body, which is what makes this a whole-crate
solve and what forces doc 88 §3's incrementality design.

### Leaf 3 — escape analysis

The `owned` lower bound. A value returned past its frame, stored in a longer-lived
structure, or captured by a closure that outlives the frame. This leaf is where the
row is most likely to be *conservative* — an over-approximation here costs an
unnecessary `owned`, which the zero-cost gate catches rather than the compiler.

### Leaf 4 — lifetime emission

Elision, and `'__vlt_a` where elision fails (§1.3). Includes the case generation 1
documented in `cursor.sdx`: `new : &'a str -> Self` and `rest : &self -> &'a str`,
where the returned borrow comes from a field rather than from `self`.

### Leaf 5 — `@` ascription and `VLT0601`

The four modes as ascriptions, and the conflict diagnostic. `VLT0601` must name both
sides with their provenance, which is row 94 §1.3's chain being used in anger for the
first time.

### Leaf 6 — `mut ρ` in the row

Regions. `mut ρ` names the root binding mutated, so two `mut` effects over distinct
regions are known not to alias (doc 86 §6.4). This is what lets a function mutate two
parameters without the author writing `&mut` twice, and it is the one place the mode
engine writes *into* the effect row rather than reading from it.

### Leaf 7 — corpora

`benchmarks/pairs/` as the primary gate (§1.5), `tests/solve/` mode expectations, and
`VLT06xx` UI fixtures.

---

## 5. Acceptance criteria

1. `cursor.vlt` with no mode annotations generates borrows byte-identical to
   generation 1's `cursor.xrs`. Any difference is recorded in §10 with the reason.
2. `benchmarks/pairs/` — all ten — pass the zero-cost gate with modes inferred.
3. The iteration bound is asserted and holds on the fuzz target.
4. **No `.clone()` is ever emitted that the source did not ask for.** Asserted by
   searching every generated `.vrs` in the corpus for `clone` and requiring a
   corresponding `@clone` in the source. This is the mechanical form of doc 01 §25.1
   and it is the criterion this row would be most tempting to quietly relax.
5. `VLT0601` names both conflicting uses with spans.
6. Elision covers every corpus program; each `'__vlt_` the emitter introduces is
   justified in §10 by the shape that forced it.

---

## 6. Out of scope

- **Lifetime inference.** §1.3.
- **Cross-crate mode inference.** A `pub` definition's modes are published in its
  sidecar (row 100) and a downstream crate reads them, exactly as Rust reads a
  signature. Inferring across a crate boundary would make a crate's API depend on its
  dependents, which is the wrong direction.
- **Interior mutability.** `Cell`, `RefCell`, `Mutex` are ordinary types from `std`
  and the mode engine treats them as it treats any other: their methods are `shared`.
  No special handling, and none needed.
- **Reordering anything.** Lattice does not reorder; `mut ρ` records non-aliasing, it
  does not license a permutation.

---

## 7. Gate

Standing gate, plus the zero-cost gate over `benchmarks/pairs/`, plus §5.4's no-clone
assertion, plus the bound.

---

## 8. Emerald reuse

None. Emerald had no ownership story to reuse — it ran on a garbage collector, which
is the whole reason this project targets Rust instead.

---

## 9. Decision log

| Decision | Why |
|---|---|
| Three-element chain | `&`, `&mut`, by-value are the three Rust forms; a fourth element would not map to anything |
| Least fixed point, not greatest | the least mode is the cheapest that works, which is what a competent Rust author writes |
| Do not infer lifetimes | rustc's borrow checker is the authority; a second analysis is not what row 92 licensed |
| `VLT0601` rather than an inserted clone | doc 01 §25.1 is non-negotiable and the reversal did not touch it |
| `@clone` is spelled | a clone is a cost decision and belongs to the author |
| Zero-cost gate as the quality test | the only automatic check that distinguishes *correct* inference from *good* inference |
| `cursor.vlt` as the proof target | twelve members, non-uniform modes, hand-annotated — a real answer key |
| Depend on row 95 | a discharge creates mode constraints; the dependency is real, not scheduling |

---

## 10. Result

*To be filled after execution. In particular: every mode that differs from generation
1's hand-written annotation, and every `'__vlt_` lifetime the emitter had to invent,
each with the shape that forced it.*

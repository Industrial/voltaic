# Plan 97 — Refinements

Plan ID: `refine`
Depends on: 94
Milestone: 15 — *the solver*
Status: `planned`

---

## 1. What this plan is

`voltaic-lattice/vlt/refine/`: predicate extraction, the QF_ALIA fragment, a solver
interface and its pin, the timeout-is-a-diagnostic policy, and `tests/refine/`.

```
head (xs [a] where xs.len > 0) = xs.0
get  (xs [a]) (i usize where i < xs.len) = xs.i
```

Checked at every call site at compile time, **erased** at runtime. `head []` does not
compile. No runtime check, no panic branch, no `Option` in the return type.

### 1.1 It is the only row in the generation that adds expressiveness

Every other row deletes tokens for behaviour Rust already has. This one lets a Voltaic
program state something Rust cannot state at all, which is what satisfies doc 86 §2
clause 4 for the generation as a whole rather than construct by construct.

It is also the only row that makes a program smaller by deleting **work** rather than
syntax: a `head` with a refinement has no `Option` to unwrap, so its callers have no
discharge to write.

### 1.2 One feature, not a family

Dependent types, quantified refinements, user function calls in predicates and
const-generic arithmetic are all **declined** (doc 87 §22.1), each with a venue.

The reason is clause 5. Unrestricted refinement checking is undecidable, and the
honest way to ship a decidable fragment is to state it in the specification and refuse
everything outside it — not to ship a solver and hope. Doc 86 §7.3 states the
fragment; this plan implements the refusal.

### 1.3 The fragment

**Quantifier-free linear integer arithmetic with the theory of arrays** — QF_ALIA —
restricted to:

- integer-valued expressions over parameters, their fields, and `.len`
- comparison (`<`, `<=`, `==`, `!=`, `>=`, `>`) and the boolean connectives
- `.len` and indexing of built-in sequences and maps

Nothing else. Specifically **not**: multiplication of two variables, division,
quantifiers, user function calls, recursion, floating point, or anything reached
through a trait method.

A predicate outside the fragment is **`VLT0801`**, naming the offending subterm and
the rule it broke. Not a warning, not a best-effort check — a refusal.

### 1.4 The timeout is an answer

Even inside a decidable fragment a query can be slow. The policy, from doc 88 §4.1:

1. The solver runs **only** on definitions with a refinement in scope. A crate with no
   refinements pays nothing, which is most crates and all of generation 1's corpus
   except the fixtures this row adds.
2. Queries are **cached by predicate hash plus call-site context hash**.
3. A timeout is **`VLT0802`**, naming the predicate and the call site. **It is never a
   pass.** This is the clause-5 commitment: an unanswered query is a diagnostic, not
   an assumption.
4. `--no-refine` skips the stage entirely and is what the fast CI gate uses; the full
   gate runs it.

Point 3 is the one that matters and the one most likely to be eroded under schedule
pressure. A refinement checker that passes on timeout is a refinement checker that
reports success for programs it did not check, which is worse than having none.

---

## 2. Preconditions

- Row 94 `done`. A predicate mentions `.len`, which is a type question, so the
  refinement engine asks the type engine — doc 88 §1.2's reason these are modules.
- A solver must be **vendorable and offline**. The project builds in a devenv shell
  with a pinned toolchain and CI has no network at build time; a solver fetched at
  build time is not admissible.

### 2.1 The solver is a dependency, and `AGENTS.md` has a rule about this

*"No plan implements a library function. Voltaic has crates.io. Emerald spent 112 of
its 203 plans building a standard library."*

So: **the solver is a dependency, not an implementation.** This plan does not write a
decision procedure. What it writes is the *interface* — predicate extraction, the
fragment check, the encoding, the cache, the timeout policy — and it binds that
interface to a pinned external solver.

The requirement on the choice, stated here so `docs/COMPILER.md` can record the pin
with a reason:

| Requirement | Why |
|---|---|
| vendorable, offline, reproducible under Nix | §2 — CI has no build-time network |
| a stable QF_ALIA front end | §1.3 is the fragment; a solver that only speaks a richer logic invites scope creep |
| a hard timeout the caller controls | §1.4 point 3 |
| a license compatible with MIT | the repository is MIT |
| incremental solving, or cheap process startup | the cache in §1.4 point 2 is the fallback if startup dominates |

`docs/COMPILER.md` records the pin. This document states the requirements; it does not
pick, because a pin without a measured startup cost is a guess and the measurement
belongs to leaf 2.

### 2.2 This is the one unbounded stage, and the only one with a thresholds table

Doc 88 §4.2's falsification thresholds exist mostly for this row:

- cold full-workspace solve above **10×** generation 1's transpile time is a design
  failure
- a refinement cache hit rate below **80%** on a warm incremental edit means the
  context hash is too fine

---

## 3. Concrete proof this plan targets

Four programs, and the fourth is the important one:

1. `head` on a known-non-empty literal — compiles.
2. `head []` — **`VLT0803`**, precondition unsatisfied, naming the predicate and the
   call site.
3. `get xs i` inside `if i < xs.len` — compiles, because the branch condition is a
   path constraint the solver has.
4. A predicate with a variable product, `where i * j < xs.len` — **`VLT0801`**,
   outside the fragment, naming `i * j`. This is the one that proves the refusal
   works, and it is more important than the three positive cases: a refinement
   engine that quietly accepts a predicate it cannot decide is the clause-5 failure.

---

## 4. Leaves

### Leaf 1 — extraction and the fragment check

Parse `where` inside a parameter's parentheses, build the predicate, and **check the
fragment before anything else runs**. `VLT0801` on the way in, not after a failed
solve — a fragment violation is a syntactic property and must not depend on solver
behaviour.

### Leaf 2 — the solver interface and the pin

The encoding, the process or library boundary, the timeout, and a **measurement of
startup cost** that decides whether the cache in leaf 3 is sufficient or whether
incremental solving is required. Amends `docs/COMPILER.md` with the pin and the
measurement.

### Leaf 3 — the cache

Predicate hash plus call-site context hash, in `voltaic-driver`'s existing cache. Hit
rate reported by `voltaic-bench`.

### Leaf 4 — path constraints

Proof target 3. A call inside a guarded branch gets the branch condition as a
hypothesis. Without this the feature is nearly useless, because the common shape is
`if xs.len > 0 then head xs`.

### Leaf 5 — erasure

The emitted `.vrs` contains no trace of the predicate: no assertion, no `debug_assert`,
no comment. Asserted by text search over the goldens. A refinement that emitted a
runtime check would be a worse `Option`.

### Leaf 6 — `tests/refine/` and `VLT08xx` fixtures

The corpus of accepted and rejected predicates, and a UI fixture per code.

---

## 5. Acceptance criteria

1. §3's four programs behave as stated, the fourth included.
2. Erasure asserted: no generated `.vrs` in the corpus contains a check derived from a
   predicate.
3. A timeout produces `VLT0802` and **fails the build**. Tested with a deliberately
   slow predicate inside the fragment.
4. A crate with no refinements shows **no measurable solve time** for this stage.
5. Doc 88 §4.2's two thresholds are measured and reported in `benchmarks/REPORT.md`.
   Exceeding either is a design failure to record in §10, not a tuning task to defer.
6. The fuzz target cannot make the stage exceed its timeout without producing
   `VLT0802`.
7. `docs/COMPILER.md` carries the pin, the license, and the startup measurement.

---

## 6. Out of scope

- **Everything in doc 87 §22.1.** Dependent types; quantified refinements (venue: a
  later generation, and only with a measured bounded-instantiation strategy); user
  function calls in predicates; const-generic arithmetic.
- **Refinements on return types.** A predicate on what a function *returns* is a
  postcondition, which needs the solver to reason about the body rather than the call
  site. It is a natural extension and it is not in this generation; venue is a later
  row, after §5.5's thresholds have been measured in real use.
- **Refinements on `data` fields.** Invariants on a type rather than on a parameter.
  Same reason: the obligation moves to every constructor and every mutation, which is
  a larger design than this row.

---

## 7. Gate

Standing gate, plus `tests/refine/`, plus the erasure assertion, plus the two
thresholds reported. Per plan 44's standing decision the thresholds are **reported,
not asserted** — a timing in the gate is a flaky gate — except the iteration and
timeout bounds, which are bounds rather than timings and therefore *are* asserted.

---

## 8. Emerald reuse

Negative, and it is the rule in §2.1: Emerald spent 112 of 203 plans on a standard
library. This plan's entire discipline is that the decision procedure is somebody
else's code.

---

## 9. Decision log

| Decision | Why |
|---|---|
| QF_ALIA, stated in the specification | clause 5 requires a stated fragment, not a best effort |
| Fragment check before the solve | a fragment violation is syntactic and must not depend on solver behaviour |
| Timeout is a diagnostic, never a pass | §1.4 point 3; the alternative reports success for unchecked programs |
| Solver is a pinned dependency | `AGENTS.md`: no plan implements a library function |
| Pin chosen in leaf 2, not here | a pin without a measured startup cost is a guess |
| Erasure asserted by text search | a refinement that emitted a check would be a worse `Option` |
| Path constraints in scope | without them the common shape does not work |
| Postconditions and field invariants out of scope | both move the obligation somewhere this row does not reach |

---

## 10. Result

*To be filled after execution. Including: the solver pinned, its startup cost, the
measured cache hit rate, and whether either of doc 88 §4.2's thresholds was exceeded.*

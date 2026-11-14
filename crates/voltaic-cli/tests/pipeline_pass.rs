//! Row 103 leaf 2, first transform: `|>` becomes a receiver-first `.`.
//!
//! Doc 97 §1.3 deletion 3 — `|>`, `>>>`, `<<<` and `$` are replaced by `.`
//! chains that break across lines — and the `|>` half is the one with a
//! one-to-one rewrite: `a |> .f` is `a.f` and `a |> .f b` is `a.f b`, because
//! both already lower to `a.f()` and `a.f(b)`.
//!
//! # Why this is run over the compiler's own source and not a fixture
//!
//! §1.1's identity gate is the whole correctness argument for pass 1:
//!
//! > The `.vrs` generated from the migrated source is byte-identical to the
//! > `.xrs` generation 1 generated from the original.
//!
//! A fixture proves the transform does something. The compiler's own 105 `.vlt`
//! files prove it does *only* that, over 8,479 `.`-calls and every shape of
//! pipeline the project actually writes — a corpus nobody wrote to be easy to
//! migrate. That is doc 97 §10.2's finding in a second place: the demanding
//! corpus is the one that was not designed for the test.
//!
//! # What a failure here means
//!
//! A file whose migrated source lowers differently. The gate reports the first
//! differing line of generated Rust, which is the only report that localises a
//! migration defect — "the tests still pass" is compatible with a changed
//! borrow, a changed clone, a different `match` shape.
//!
//! Nothing is written. This asserts the transform *would* be safe; `apply` is
//! what writes, and row 103 leaf 6 is what calls it crate by crate.
//!
//! It lives in `voltaic-cli` rather than `voltaic-driver` because it needs both
//! the driver (the pass and the gate) and the formatter (to print the rewritten
//! AST back to `.vlt`), and the driver does not depend on the formatter. Adding
//! that edge for a test would also mean amending `dependency_sources` and
//! `moon_project_graph.rs` — a real cost for no gain, since `voltaic-cli`
//! already depends on both.

use std::path::{Path, PathBuf};

use voltaic_driver::{Gate, Pass, gate, rewrite_program};

mod support;

fn repo_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .and_then(Path::parent)
    .expect("the crate sits two levels below the repository root")
    .to_path_buf()
}

/// Every `.vlt` the compiler is written in, read from `seed/GENERATION`.
///
/// The list rather than a glob, for `seed/GENERATION`'s own reason: it is the
/// only copy of which crates are generated, and a second walk would be a second
/// copy free to drift.
fn sources() -> Vec<PathBuf> {
  let root = repo_root();
  let listed =
    std::fs::read_to_string(root.join("seed/GENERATION")).expect("should read seed/GENERATION");
  let mut found = Vec::new();
  for line in listed.lines().map(str::trim) {
    if line.is_empty() || line.starts_with('#') {
      continue;
    }
    let Some(name) = line.split_whitespace().next() else {
      continue;
    };
    collect(&root.join("crates").join(name).join("vlt"), &mut found);
  }
  found.sort();
  found
}

fn collect(dir: &Path, into: &mut Vec<PathBuf>) {
  let Ok(entries) = std::fs::read_dir(dir) else {
    return;
  };
  for entry in entries.flatten() {
    let path = entry.path();
    if path.is_dir() {
      collect(&path, into);
    } else if path.extension().is_some_and(|e| e == "vlt") {
      into.push(path);
    }
  }
}

/// The source with every `|>` collapsed, as text.
///
/// Parse, rewrite, print — the printer is what turns the rewritten AST back into
/// `.vlt`, which is why doc 97 §10.43's rule matters here more than anywhere: a
/// printer that drops a field would silently delete it from every migrated file.
fn migrated(text: &str) -> Option<String> {
  let mut program = voltaic_parser::parse_source(text).ok()?;
  rewrite_program(&mut program, Pass::Pipelines);
  Some(voltaic_fmt::format(&program, text))
}

#[test]
fn the_corpus_is_the_size_generation_one_was() {
  let found = sources();
  assert!(
    found.len() > 100,
    "the compiler is written in 105 `.vlt` files and this found {}; a shrinking \
     corpus means the walk stopped finding them, not that the work got smaller",
    found.len()
  );
}

/// Leaf 6 migrates crate by crate, so the **apply** path can be scoped to one
/// crate with `VOLTAIC_MIGRATE_CRATE=voltaic-diag`.
///
/// Only the apply path. The gate path always covers the whole corpus, because a
/// scoped gate is a weaker gate and nothing should be able to narrow it by
/// setting an environment variable — the scoping exists to make a *commit*
/// reviewable, not to make the check smaller.
fn scoped_to() -> Option<String> {
  std::env::var("VOLTAIC_MIGRATE_CRATE").ok()
}

/// The crates `seed/GENERATION` marks `g2` — the ones leaf 6 has already taken
/// across. Read from the file rather than counted, for that file's own reason:
/// it is the only copy of which crate is which generation.
fn migrated_crates() -> Vec<String> {
  support::generations()
    .into_iter()
    .filter(|(_, _, generation)| generation == "g2")
    .map(|(name, _, _)| name)
    .collect()
}

#[test]
fn collapsing_every_pipeline_changes_no_program() {
  let updating = std::env::var("VOLTAIC_UPDATE").is_ok();
  let only = scoped_to();
  let mut rejected = Vec::new();
  let mut written = Vec::new();
  let mut touched = 0;
  let mut stages = 0;
  let mut survived = Vec::new();
  let mut unmigrated = Vec::new();
  for path in sources() {
    let text = std::fs::read_to_string(&path).expect("a listed source should be readable");
    if !text.contains("|>") {
      continue;
    }
    let Some(after) = migrated(&text) else {
      rejected.push(format!("{}: the original does not parse", path.display()));
      continue;
    };
    touched += 1;
    if after != text
      && let Some(crate_name) = path
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .find(|c| migrated_crates().iter().any(|m| m == c))
    {
      unmigrated.push(format!("{} (in {crate_name}, marked g2)", path.display()));
    }
    if after.contains("|>") {
      let line = after
        .lines()
        .find(|l| l.contains("|>"))
        .unwrap_or_default()
        .trim();
      survived.push(format!(
        "{}: {}",
        path.display(),
        &line[..line.len().min(90)]
      ));
    } else {
      stages += 1;
    }
    if after == text {
      continue;
    }
    if updating {
      if let Some(wanted) = &only
        && !path.components().any(|c| c.as_os_str() == wanted.as_str())
      {
        continue;
      }
      // `apply` gates before it writes, so a rejected file stays untouched even
      // here — leaf 3's "the gate runs before the write, not after".
      match voltaic_driver::apply(&path, &after) {
        Ok(verdict) if verdict.passed() => written.push(path.display().to_string()),
        Ok(verdict) => rejected.push(format!("{}: {}", path.display(), verdict.explain())),
        Err(e) => rejected.push(format!("{}: {e}", path.display())),
      }
      continue;
    }
    let verdict = gate(&path, &text, &after);
    if !verdict.passed() {
      rejected.push(format!("{}: {}", path.display(), verdict.explain()));
    }
  }
  // **A crate marked `g2` must already be migrated**, which is the assertion
  // that replaced a hard floor of `touched > 50`.
  //
  // That floor was right while the corpus was wholly generation 1 and said "the
  // compiler's own source is written in pipelines". Row 103 leaf 6 drains it by
  // design — it fell below 50 at the fifth crate — so a floor would have to be
  // lowered each commit, which is a gate that reports whatever it is told.
  //
  // This is two-sided instead, and strictly stronger: it fails if a `g2` crate
  // still has a pipeline the pass would rewrite, and it keeps meaning something
  // when the last crate crosses, where a floor would have to be deleted.
  assert!(
    unmigrated.is_empty(),
    "these files are in crates `seed/GENERATION` marks `g2`, but the pass still \
     rewrites them — so the crate was flipped without being migrated, or a \
     pipeline was written into it afterwards: {unmigrated:#?}"
  );
  // The reach, reported rather than assumed. Until the formatter printed a
  // zero-argument `Dot` as `recv.name ()` the pass had to *decline* every
  // no-argument stage — 450 of 473 of them — because `recv.name` reparses as a
  // field access, so the rewrite was not a respelling. The count is here
  // because "the gate passes" is equally true of a pass that rewrites nothing.
  println!("collapsed every `|>` in {stages} of {touched} files that hold one");
  let quoted: Vec<_> = survived.iter().filter(|s| !s.contains('"')).collect();
  assert!(
    quoted.is_empty(),
    "every `|>` left in a migrated file should be the operator written as data \
     — the AST's spelling, the lexer's table, a diagnostic's help text, a test \
     fixture — and these are on lines with no string literal at all, so they \
     are pipeline *expressions* the pass declined: {quoted:#?}\n\nSurvivors in \
     full:\n{survived:#?}"
  );
  assert!(
    rejected.is_empty(),
    "collapsing `|>` to `.` changed what these files lower to, so the rewrite is \
     not a respelling: {rejected:#?}"
  );
  assert!(
    written.is_empty(),
    "migrated {} files. The generated Rust is byte-identical by construction — \
     `apply` gated each one before writing it — so review the `.vlt` diff and \
     re-run without VOLTAIC_UPDATE:\n{written:#?}",
    written.len()
  );
}

/// The one line that unblocked the no-argument half, pinned where it can be
/// read: a collapsed stage with no arguments must print as a **call**.
///
/// `s.len` reparses as `Expr::Field`, and a field read is not a method call, so
/// the pass had to decline every such stage — 450 of the corpus's 473. Printing
/// `s.len ()` makes the rewritten source say what the AST means, and the
/// surface already admitted that spelling: `recv.name ()` parses as a
/// zero-argument `Dot` and lowers to `recv.name()`.
///
/// It is **half** of what the migration needs, and doc 101 §10.14 is careful
/// about which half. The parentheses are mandatory — `h.s.len ().count_ones ()`
/// parses as `h.s.len((), .count_ones, ())`, not as a chain — so a migrated
/// chain nests one parenthesis per zero-argument link and reads worse than the
/// `|>` it replaced. What removes them is the field-vs-method classification,
/// which works for declared methods and not for `std` receivers; that is why
/// the transform is gated and correct here and still **not applied**.
#[test]
fn a_collapsed_no_argument_stage_prints_as_a_call() {
  let after = migrated("f : &str -> usize\nf s = s |> .len\n").expect("should parse");
  assert!(
    after.contains("s.len ()"),
    "a no-argument stage must print its parentheses or it reparses as a field \
     access: {after}"
  );
  assert!(
    !after.contains("|>"),
    "and the pipeline must be gone, not merely rewritten alongside: {after}"
  );
}

/// A zero-argument chain composes with nothing between the links.
///
/// **This assertion was the reverse, one commit ago**, and the reversal is the
/// finding. `s.len ().count_ones ()` used to parse as `s.len((), .count_ones,
/// ())`, because `()` was collected as a juxtaposed Unit argument and then
/// `postfix` attached `.count_ones` to *it*. Two parser rules fix it, and both
/// are type-free facts rather than guesses:
///
/// - **no postfix is meaningful on a unit literal** — `().name`, `()?`, `()[i]`
///   and `(){…}` are all nonsense, so `postfix` declines them all and the `()`
///   stays the call's empty argument list;
/// - **an application continues its own postfix chain**, so the `?` in
///   `x.ok ()?` and the `.count_ones` here attach to the link just built rather
///   than dangling. Without the second rule the first one breaks `x.ok ()?`.
///
/// A link that *carries* arguments still needs its parenthesis, and that is not
/// fixable the same way — see
/// `a_dot_with_arguments_still_needs_its_parenthesis`.
#[test]
fn a_zero_argument_chain_composes_without_parentheses() {
  let chained = voltaic_parser::parse_source("f : &str -> u32\nf s = s.len ().count_ones ()\n")
    .expect("should parse");
  let shown = format!("{chained:?}");
  assert_eq!(
    shown.matches("Dot").count(),
    2,
    "`s.len ().count_ones ()` must be a two-link chain, not one call swallowing \
     the rest: {shown}"
  );
}

/// The limit of the fix above, pinned because it is what keeps the formatter's
/// parentheses honest.
///
/// `xs.map (\a -> …)` as a receiver must be parenthesised: a following
/// `.collect` would otherwise attach to the lambda's closing paren and become an
/// argument to `map`. That is not the unit case — `(g x).y` is a real
/// expression — so there is nothing type-free to decline, and the parenthesis
/// stays. Hence `takes_a_postfix` is `false` for a `Dot` **with** arguments and
/// `true` for one without.
#[test]
fn a_dot_with_arguments_still_needs_its_parenthesis() {
  let bare = support::transpile_source(
    "chain_bare",
    "f : &Vec<u64> -> Vec<u64>\nf xs = (xs.iter ()).map (\\a -> *a + 1).collect ()\n",
  );
  assert!(
    bare.contains(".collect, ())"),
    "unparenthesised, the `.collect` belongs to the lambda — which is why the \
     printer brackets a Dot that carries arguments: {bare}"
  );
  let bracketed = support::transpile_source(
    "chain_bracketed",
    "f : &Vec<u64> -> Vec<u64>\nf xs = ((xs.iter ()).map (\\a -> *a + 1)).collect ()\n",
  );
  assert!(
    bracketed.contains(".map(|a| *a + 1).collect()"),
    "and bracketed it is the chain it reads as: {bracketed}"
  );
}

/// The other half, pinned: a declared method classifies and a `std` one does
/// not, which is what makes the migration read worse than `|>` today.
///
/// Doc 101 §10.14's demonstration, as a gate. It fails if `std` receivers ever
/// start classifying — which would be good news, and would mean the parentheses
/// above can be dropped and leaf 6 can run.
#[test]
fn a_declared_method_classifies_and_a_std_method_does_not() {
  let source = "data H = H { n : u64 }\n\ninstance H\n  export twice : &self -> u64\n  \
                twice self = self.n * 2\n\nexport chained : &H -> String\nchained h = \
                h.twice.to_string\n";
  let rust = support::transpile_source("pipeline_classified_chain", source);
  assert!(
    rust.contains("h.twice().to_string"),
    "`twice` is declared on `H`, so it should classify and emit a call with no \
     parentheses in the source: {rust}"
  );
  // Asserted as an absence, because the presence above is satisfied by
  // `h.twice().to_string()` too — and that is the outcome this gate exists to
  // notice. It going red is good news: it means `std` receivers classify, the
  // migrated chain needs no parentheses, and leaf 6 can run.
  assert!(
    !rust.contains("to_string()"),
    "`to_string` is `std`'s and nothing declares it, so it should still be a \
     field read. If it now classifies, doc 101 §10.14's declination is lifted: \
     {rust}"
  );
}

/// The second half of why deletion 3 cannot be written as specified: a `.`
/// chain cannot break across lines at all.
///
/// Doc 97 §1.3 deletion 3 replaces `|>` with "`.` chains that break across
/// lines". A `|>` leads its continuation line with an explicit operator, so the
/// lexer knows the line continues; a bare `.` has no such marker and the
/// off-side rule ends the expression at the newline. The continuation is then a
/// receiver-last application on its own, and the item fails with `VLT0103`.
///
/// So a migrated chain is not merely parenthesis-heavy — it must be **one
/// line**, however long. That is the cost `lex.vlt`'s three-line pipeline paid
/// when the pass was applied and reverted.
#[test]
fn a_dot_chain_cannot_break_across_lines() {
  assert!(
    voltaic_parser::parse_source(
      "data H = H { s : String }\n\nexport two : &H -> usize\ntwo h = (h.s.to_string ())\n  .len ()\n"
    )
    .is_err(),
    "if a `.` chain can now break across lines, deletion 3's second obstacle is \
     gone — update doc 101 §10.14 and the formatter can grow the multi-line \
     chain printer leaf 6 wants"
  );
}

/// The gate can fail, demonstrated rather than assumed.
///
/// A pass asserted over 105 files that all pass proves nothing unless the
/// assertion can go red. This feeds the gate a migration that *is* wrong — the
/// method renamed — and requires it to say so.
#[test]
fn the_gate_rejects_a_rewrite_that_changes_the_call() {
  let before = "f : u64 -> u64\nf n = n |> .max 3\n";
  let wrong = "f : u64 -> u64\nf n = n.min 3\n";
  let verdict = gate(&PathBuf::from("probe.vlt"), before, wrong);
  assert!(
    matches!(verdict, Gate::Differs { .. }),
    "renaming the method must fail the gate: {}",
    verdict.explain()
  );
}

/// Formatting is a **fixpoint**: formatting twice equals formatting once.
///
/// This is the gate that was missing, and its absence cost a bad migration.
/// `collapse_pipeline` builds `Expr::Dot { args: [] }`, the printer writes that
/// as `recv.name ()`, and reparsing *that* yields `Expr::Dot { args: [Unit] }` —
/// a different AST for the same program. `takes_a_postfix` distinguished the
/// two, so pass one printed `self.cur.peek ().expect "…"` and pass two printed
/// `(self.cur.peek ()).expect "…"`, adding a parenthesis per link forever.
///
/// Three things hid it, and all three are reasons to assert the property
/// rather than any of its symptoms:
///
/// - the **identity gate still passed**, because both spellings lower to the
///   same Rust, which is exactly what it compares;
/// - `formatting_preserves_meaning` still passed, because the meaning *is*
///   preserved — only the spelling grows;
/// - the migration writes each file once, so the second format never ran until
///   a later crate's migration re-walked an already-migrated file.
///
/// `no_real_arguments` is the fix: one `Unit` argument is no arguments, which is
/// what the emitter has always believed — `h.items.len ()` emits
/// `h.items.len()`, not `h.items.len(())`.
#[test]
fn formatting_is_a_fixpoint_over_the_whole_corpus() {
  let mut unstable = Vec::new();
  for path in sources() {
    let text = std::fs::read_to_string(&path).expect("a listed source should be readable");
    let Ok(program) = voltaic_parser::parse_source(&text) else {
      continue;
    };
    let once = voltaic_fmt::format(&program, &text);
    let Ok(reparsed) = voltaic_parser::parse_source(&once) else {
      unstable.push(format!("{}: its own output does not parse", path.display()));
      continue;
    };
    let twice = voltaic_fmt::format(&reparsed, &once);
    if twice != once {
      let at = once
        .lines()
        .zip(twice.lines())
        .enumerate()
        .find(|(_, (a, b))| a != b);
      unstable.push(match at {
        Some((n, (a, b))) => format!(
          "{} line {}:\n  once: {}\n  twice: {}",
          path.display(),
          n + 1,
          a.trim(),
          b.trim()
        ),
        None => format!("{}: differs only in length", path.display()),
      });
    }
  }
  assert!(
    unstable.is_empty(),
    "formatting twice must equal formatting once, or every pass over a file \
     rewrites it again — and a migration that re-walks an already-migrated \
     crate would compound it: {unstable:#?}"
  );
}

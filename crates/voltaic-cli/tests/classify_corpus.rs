//! Row 103 leaf 1: the whole corpus classified **before any migration happens**.
//!
//! > Run over all 122 files before any migration happens, so the `manual` count
//! > is known at the start rather than discovered at file 97.
//!
//! That is the leaf's own sentence and it is the whole reason this file exists
//! rather than the classifier being invoked per crate during row 103's
//! migration. The `manual` count is a **release-blocking number** — doc 89's
//! milestone 17 is that it reaches zero — and a release-blocking number
//! discovered two thirds of the way through is a number nobody budgeted for.
//!
//! # What the classifier is, and what it is not
//!
//! It matches construct *spellings*, line by line, skipping comments. It is not
//! a parse. A needle inside a string literal counts, and that is deliberate for
//! this leaf: the question is *"does a human have to look at this file"*, and a
//! `do` inside a string is a thing a human has to look at to rule out. Row 103's
//! pass 1 translates from the AST; this only decides who gets to do the work.
//!
//! So the counts here are an **upper bound on the human effort**, and the
//! classifier erring towards `manual` is the safe direction. The opposite error
//! — classifying a file `auto` that needs a human — is the one that would ship
//! a silently wrong migration, and it is why §1.3 says the classes are
//! determined "by construct, not by confidence".
//!
//! # The recorded numbers, and the one the plan got wrong
//!
//! [`FLOOR`] pins what the corpus looked like when this landed: **105 files**.
//! The leaf's sentence quoted above says 122, and doc 101 §2.1's table says 122
//! files and 11,668 code lines. Measured here: **105 files and 15,264 code
//! lines** — wrong on both axes and in opposite directions, because the file
//! count predates plan 91's merge of 572 files into cohesive groups while the
//! line count predates `voltaic-lattice`. Doc 101 §2.1 and §10.3 carry the
//! account; `AGENTS.md` had both figures too.
//!
//! It is a floor rather than an equality for the reason every floor in this tree
//! is: the corpus grows, and an exact count would fail on an ordinary addition
//! while telling nobody anything. What it catches is the corpus *shrinking* or
//! the classifier quietly stopping matching.

mod support;

use std::path::{Path, PathBuf};

use voltaic_driver::{Class, classify};

/// The corpus size when this gate landed, as a floor.
const FLOOR: usize = 105;

fn repo_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .and_then(Path::parent)
    .expect("the crate sits two levels below the repository root")
    .to_path_buf()
}

/// There are no exclusions, and that is the result of a rule rather than luck.
///
/// `classify.vlt` is the classifier, and its marker table holds the needles as
/// string literals — so it classified *itself* as `manual`, because the line
/// `add (&mut found) "do " …` does contain the word `do`. The first fix was to
/// exclude the file by name. The anchor in `hits` removed the need: `do`
/// introduces a block, so it is the last token on its line, and a needle
/// mentioned inside a string is not.
///
/// Keeping the list, empty, is deliberate. An exemption is a permanent hole with
/// a comment beside it; a rule is a rule. Recording that the list *can* be empty
/// is the thing that makes a future exemption look like the compromise it is.
const EXCLUDED: &[(&str, &str)] = &[];

/// Every `.vlt` the compiler is written in, minus [`EXCLUDED`].
fn compiler_sources() -> Vec<PathBuf> {
  let mut found = Vec::new();
  for (crate_name, _) in support::ported() {
    collect(
      &repo_root().join("crates").join(&crate_name).join("vlt"),
      &mut found,
    );
  }
  found.sort();
  let root = repo_root();
  found.retain(|path| {
    let shown = path
      .strip_prefix(&root)
      .unwrap_or(path)
      .display()
      .to_string();
    !EXCLUDED.iter().any(|(excluded, _)| *excluded == shown)
  });
  found
}

fn collect(dir: &Path, into: &mut Vec<PathBuf>) {
  let Ok(entries) = std::fs::read_dir(dir) else {
    return;
  };
  for entry in entries.filter_map(Result::ok) {
    let path = entry.path();
    if path.is_dir() {
      collect(&path, into);
    } else if path.extension().is_some_and(|ext| ext == "vlt") {
      into.push(path);
    }
  }
}

/// `(auto, review, manual)` over the corpus, with the manual files named.
fn survey() -> (usize, usize, usize, Vec<String>) {
  let mut auto = 0;
  let mut review = 0;
  let mut manual = 0;
  let mut blocking = Vec::new();
  for path in compiler_sources() {
    let source = std::fs::read_to_string(&path).expect("a listed file should be readable");
    let verdict = classify(&source);
    match verdict.kind {
      Class::Auto => auto += 1,
      Class::Review => review += 1,
      Class::Manual => {
        manual += 1;
        let first = verdict
          .of_class(Class::Manual)
          .first()
          .map(|f| format!("{}:{} {}", path.display(), f.line, f.construct))
          .unwrap_or_else(|| path.display().to_string());
        blocking.push(first);
      }
    }
  }
  (auto, review, manual, blocking)
}

#[test]
fn the_whole_corpus_classifies() {
  let (auto, review, manual, blocking) = survey();
  let total = auto + review + manual;
  assert!(
    total >= FLOOR,
    "the corpus classified {total} files and the floor is {FLOOR}; a shrinking corpus \
     means the walk stopped finding files, not that the work got smaller"
  );
  // Not an assertion — the point of leaf 1 is that the number is *reported*
  // before migration starts, and a test is where this tree reports numbers.
  println!("auto {auto}, review {review}, manual {manual}, of {total}");
  for one in &blocking {
    println!("manual: {one}");
  }
}

/// Every file lands in exactly one class, because `survey` matches exhaustively
/// on `Class` — this asserts the arithmetic, which is the part a refactor breaks.
#[test]
fn every_file_is_counted_exactly_once() {
  let (auto, review, manual, _) = survey();
  assert_eq!(
    auto + review + manual,
    compiler_sources().len(),
    "a file was counted twice or not at all"
  );
}

/// The blocking number is the one doc 89's milestone 17 drives to zero, so it
/// has to be *reachable* — a classifier that can never say `manual` would make
/// the milestone vacuously true.
#[test]
fn the_manual_class_is_reachable() {
  let verdict = classify("held : Option<Option<u64>>\n");
  assert_eq!(
    verdict.kind,
    Class::Manual,
    "nothing can be manual, so the release-blocking count is vacuously zero"
  );
}

/// And it has to be *avoidable*, or every file would block and the count would
/// carry no information either.
#[test]
fn the_auto_class_is_reachable() {
  let verdict = classify("twice : u64 -> u64\ntwice n = n + n\n");
  assert_eq!(verdict.kind, Class::Auto);
}

/// An exclusion that names a file which no longer exists is a hole that looks
/// like a decision. The list is empty today, and this is what keeps it honest if
/// it stops being.
#[test]
fn every_exclusion_is_still_a_file() {
  for (shown, reason) in EXCLUDED {
    let path = repo_root().join(shown);
    assert!(
      path.exists(),
      "{shown} is excluded from the survey and does not exist, so the exclusion is \
       stale ({reason})"
    );
  }
}

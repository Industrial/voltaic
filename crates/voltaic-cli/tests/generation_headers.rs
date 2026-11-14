//! Row 104 leaf 7: every generation-1 document points at its successor.
//!
//! > A one-line header on each of generation 1's numbered documents pointing at
//! > its successor. Not a rewrite: `docs/01` through `docs/85` stay exactly as
//! > they are, because they are the record of what generation 1 did and editing
//! > them falsifies it.
//!
//! # Why a reader needs this and a `grep` does not
//!
//! There are 85 of them, and nothing in any one of them says it has been
//! superseded. A reader who opens `docs/03 - Voltaic Surface Design.md` because
//! it is the document whose title matches their question gets generation 1's
//! surface, stated with no hedging, and no reason to doubt it — `.f x` with the
//! receiver *last*, signatures required, `if` and `case` present. All three were
//! reversed by generation 2.
//!
//! Doc 04 rule 3 forbids deleting a superseded document, which is right: it is
//! the record of what was done and why. The cost of keeping it is that it is
//! indistinguishable from current documentation when read on its own. One line
//! is what closes that.
//!
//! # Why the plan documents all point at the register
//!
//! Seventy-eight of the eighty-five are generation-1 plans, and a generation-1
//! plan has no generation-2 counterpart: the work was reorganised into
//! nineteen rows, not renumbered. So the honest successor is doc 89, which is
//! where the planning went, and inventing a per-plan mapping would be asserting
//! a correspondence the project never made. The seven foundational documents do
//! have specific successors and name them — and for `docs/04`, the register *is*
//! that successor, which is why the count below is six rather than seven.
//!
//! # What this cannot check
//!
//! That the documents are otherwise **unedited** — that is acceptance criterion
//! 8 and it is git's to answer, not a test's. What this checks is the half a
//! test can: that the header is present, that it is one line, and that every
//! successor it names resolves to a file that exists. A header pointing at a
//! document that has been renamed is worse than no header, because it reads as
//! maintained.

use std::path::{Path, PathBuf};

/// The marker every generation-1 document opens with.
const MARK: &str = "> **Generation 1.**";

fn repo_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .and_then(Path::parent)
    .expect("the crate sits two levels below the repository root")
    .to_path_buf()
}

/// Generation 1's numbered documents: `docs/NN - …` with `NN <= 85`.
///
/// The boundary is the number itself rather than a list, which is the same rule
/// `declined.rs`'s `is_historical_record` uses — generation 2 starts at 86, so a
/// list would be a second copy of one fact.
fn generation_one() -> Vec<(String, String)> {
  let docs = repo_root().join("docs");
  let mut found = Vec::new();
  for entry in std::fs::read_dir(&docs).expect("docs/ should be readable") {
    let path = entry.expect("should read a docs/ entry").path();
    if path.extension().is_none_or(|e| e != "md") {
      continue;
    }
    let name = path
      .file_name()
      .expect("a file has a name")
      .to_string_lossy()
      .to_string();
    let number = name
      .split(' ')
      .next()
      .and_then(|n| n.parse::<u32>().ok())
      .unwrap_or(u32::MAX);
    if number > 85 {
      continue;
    }
    let text = std::fs::read_to_string(&path).expect("a listed document should be readable");
    found.push((name, text));
  }
  found.sort();
  found
}

#[test]
fn the_corpus_is_the_size_generation_one_was() {
  let found = generation_one();
  assert_eq!(
    found.len(),
    85,
    "generation 1 ran to 85 numbered documents; found {}. A document gained or \
     lost a number, or the walk stopped finding them — either way this gate is \
     measuring the wrong set.",
    found.len()
  );
}

#[test]
fn every_generation_one_document_names_its_successor() {
  let mut bare = Vec::new();
  for (name, text) in generation_one() {
    if !text.starts_with(MARK) {
      bare.push(name);
    }
  }
  assert!(
    bare.is_empty(),
    "these generation-1 documents do not open with `{MARK}`, so a reader who \
     opens one has no way to know it was superseded — and generation 1's surface \
     reads as current when read alone: {bare:#?}"
  );
}

#[test]
fn the_header_is_one_line() {
  let mut sprawling = Vec::new();
  for (name, text) in generation_one() {
    let header = text.lines().next().unwrap_or_default();
    let second = text.lines().nth(1).unwrap_or_default();
    if !header.starts_with(MARK) || !second.trim().is_empty() {
      sprawling.push(name);
    }
  }
  assert!(
    sprawling.is_empty(),
    "leaf 7 says *one line*, so the header is one line followed by a blank one. \
     These are not that, which means the edit grew past what the leaf \
     sanctions: {sprawling:#?}"
  );
}

#[test]
fn every_successor_a_header_names_exists() {
  let root = repo_root();
  let mut dangling = Vec::new();
  for (name, text) in generation_one() {
    let header = text.lines().next().unwrap_or_default();
    for cited in header.split('`').skip(1).step_by(2) {
      // A `§` suffix addresses a section of the document, not a path.
      let path = cited.split(" §").next().unwrap_or(cited);
      if !path.starts_with("docs/") {
        continue;
      }
      if !root.join(path).exists() {
        dangling.push(format!("{name} → {path}"));
      }
    }
  }
  assert!(
    dangling.is_empty(),
    "these headers name a successor that does not exist, which reads as \
     maintained and is not: {dangling:#?}"
  );
}

/// The foundational documents name a specific successor, not the register.
///
/// Spelled as the count rather than as the list, because the list is in the
/// documents and a second copy here would drift. What matters is that the
/// *distinction* survives: a plan points at doc 89 because its work was
/// reorganised, and a foundational document points at the document that replaced
/// it. If every header pointed at doc 89 the leaf would be satisfied in letter
/// and useless in fact.
///
/// **Six, not seven, and the seventh is the interesting one.** Generation 1 has
/// seven non-plan documents, and `docs/04 - Voltaic Plan of Plans.md`'s specific
/// successor *is* doc 89 — a register's successor is a register. So it is counted
/// with the plans here and that is correct rather than a gap. This gate asserted
/// seven on its first run and was wrong about its own corpus, which is the shape
/// doc 101 §2.1 records twice: a count written from the structure rather than
/// measured from the tree.
#[test]
fn the_foundational_documents_name_something_other_than_the_register() {
  let register = "89 - Voltaic 2 Plan of Plans.md";
  let specific: Vec<String> = generation_one()
    .into_iter()
    .filter(|(_, text)| {
      let header = text.lines().next().unwrap_or_default();
      !header.contains(register)
    })
    .map(|(name, _)| name)
    .collect();
  assert_eq!(
    specific.len(),
    6,
    "six of generation 1's documents are foundational *and* had a successor \
     other than the register; {} name something other than it: {specific:#?}",
    specific.len()
  );
}

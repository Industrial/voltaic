//! Row 99 wave 1's answer key: the inferred signature equals the written one.
//!
//! # Why this test is the precondition for deleting signatures
//!
//! Wave 1 deletes signature lines and has the emitter write the inferred ones.
//! Plan `97 - 99` §4 orders it first for one reason: it is the only deletion
//! whose correctness the tree can check against an answer key, because
//! generation 1's hand-written signatures are *still in the files*. Once they
//! are gone there is nothing left to compare against, so the comparison has to
//! be made — and gated — before the deletion, not after.
//!
//! So this file is not a test of `voltaic sig`. It is the evidence on which
//! wave 1 is allowed to proceed, and it has to stay green for as long as the
//! written signatures exist, which is until row 103 migrates the compiler.
//!
//! # The numbers, and why the set is bigger than it first looked
//!
//! 229 written signatures in the examples and 910 in the compiler, all of them
//! equal to the inferred judgement. That set was 215 and 594 until
//! `signature_type` stopped returning `None` for every method with a `self`
//! receiver: `self` has no written param type, so a fully annotated
//! `skip_layout : &mut self -> ()` read as unannotated and was skipped. The
//! gate was true about a set 330 definitions smaller than it appeared to
//! describe, which is the quieter failure mode of a gate keyed on "has a
//! written type" -- it narrows without going red.
//!
//! # Both corpora, because they fail differently
//!
//! `examples/` is the surface the language documents, and `crates/*/vlt/` is
//! 11,668 lines of real program that nobody wrote to be easy to infer. The
//! examples found both of the defects this gate caught; the compiler's source
//! found neither, because it happens not to return interpolated strings from
//! annotated functions. A gate on either corpus alone would have been green
//! while inference was wrong.
//!
//! # `examples/lexical.vlt` is excluded, and that is not a carve-out
//!
//! It is a *lexer* example — a catalogue of lexical forms, not a program — and
//! it does not parse by design. [`the_excluded_example_is_the_lexer_catalogue`]
//! asserts that the exclusion stays justified, so the list cannot quietly grow
//! to cover a file that stopped parsing for a real reason.

mod support;

use std::path::{Path, PathBuf};

use voltaic_lattice::walk;
use voltaic_parser::parse_source;

/// The one example that is a lexical catalogue rather than a program.
const NOT_A_PROGRAM: &str = "lexical.vlt";

fn repo_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .and_then(Path::parent)
    .expect("the crate sits two levels below the repository root")
    .to_path_buf()
}

/// Every `.vlt` under `examples/`, minus the lexical catalogue.
fn example_programs() -> Vec<PathBuf> {
  let mut found: Vec<PathBuf> = std::fs::read_dir(repo_root().join("examples"))
    .expect("examples/ should be readable")
    .filter_map(Result::ok)
    .map(|entry| entry.path())
    .filter(|path| path.extension().is_some_and(|ext| ext == "vlt"))
    .filter(|path| path.file_name().is_some_and(|name| name != NOT_A_PROGRAM))
    .collect();
  found.sort();
  found
}

/// Every `.vlt` the compiler itself is written in.
fn compiler_sources() -> Vec<PathBuf> {
  let mut found = Vec::new();
  for (crate_name, _) in support::ported() {
    let dir = repo_root().join("crates").join(&crate_name).join("vlt");
    collect_vlt(&dir, &mut found);
  }
  found.sort();
  found
}

fn collect_vlt(dir: &Path, into: &mut Vec<PathBuf>) {
  let Ok(entries) = std::fs::read_dir(dir) else {
    return;
  };
  for entry in entries.filter_map(Result::ok) {
    let path = entry.path();
    if path.is_dir() {
      collect_vlt(&path, into);
    } else if path.extension().is_some_and(|ext| ext == "vlt") {
      into.push(path);
    }
  }
}

/// One definition whose inferred type differs from the type its author wrote.
struct Disagreement {
  where_: String,
  written: String,
  inferred: String,
}

/// `(checked, disagreements)` over one file.
fn verify(path: &Path) -> (usize, Vec<Disagreement>) {
  let source = std::fs::read_to_string(path).expect("a listed file should be readable");
  let shown = path
    .strip_prefix(repo_root())
    .unwrap_or(path)
    .display()
    .to_string();
  let program = parse_source(&source)
    .unwrap_or_else(|e| panic!("{shown} should parse, since it is a program: {e:?}"));
  let found = walk(&program);
  let outcome = voltaic_lattice::solve_with(&found.store, &found.known);

  let mut checked = 0;
  let mut differed = Vec::new();
  for definition in &found.definitions {
    let Some(written) = definition.declared() else {
      continue;
    };
    checked += 1;
    let inferred = definition.judged(&outcome);
    let expanded = found.known.expand(&written);
    if inferred != expanded {
      differed.push(Disagreement {
        where_: format!("{shown}: {}", definition.name),
        written: expanded.to_string(),
        inferred: inferred.to_string(),
      });
    }
  }
  (checked, differed)
}

fn report(corpus: &str, files: &[PathBuf]) {
  assert!(!files.is_empty(), "{corpus} should not be empty");
  let mut checked = 0;
  let mut differed = Vec::new();
  for path in files {
    let (n, mut found) = verify(path);
    checked += n;
    differed.append(&mut found);
  }
  assert!(
    checked > 0,
    "{corpus} has no written signatures, so this gate is asserting nothing"
  );
  let n_differed = differed.len();
  if !differed.is_empty() {
    let listed = differed
      .iter()
      .map(|d| {
        format!(
          "  {}\n    written  {}\n    inferred {}",
          d.where_, d.written, d.inferred
        )
      })
      .collect::<Vec<_>>()
      .join("\n");
    panic!(
      "{n_differed} of {checked} written signatures in {corpus} disagree with the inferred \
       one.\n\nWave 1 deletes these signatures and emits the inferred ones, so a \
       disagreement here is a signature that would change meaning when \
       deleted:\n\n{listed}"
    );
  }
}

/// The 43 example programs.
#[test]
fn every_written_signature_in_the_examples_is_the_inferred_one() {
  report("examples/", &example_programs());
}

/// The compiler's own 11,668 lines.
#[test]
fn every_written_signature_in_the_compiler_is_the_inferred_one() {
  report("crates/*/vlt/", &compiler_sources());
}

/// Floors on the set, because a shrinking set is how this gate fails quietly.
///
/// `report` already refuses a corpus with *no* written signatures. That was not
/// enough: the receiver defect left it comparing 809 of 1,139 and the gate was
/// green throughout, because 809 is not zero. A floor is cruder than an exact
/// count and survives ordinary additions, which an exact count would not.
#[test]
fn the_compared_set_has_not_shrunk() {
  let (examples, _) = example_programs()
    .iter()
    .map(|p| verify(p))
    .fold((0, 0), |(n, z), (c, _)| (n + c, z));
  let (compiler, _) = compiler_sources()
    .iter()
    .map(|p| verify(p))
    .fold((0, 0), |(n, z), (c, _)| (n + c, z));
  assert!(
    examples >= 229,
    "the examples compared 229 signatures when this floor was set; now {examples}. A \
     *smaller* set passing is the failure mode, not a success"
  );
  assert!(
    compiler >= 910,
    "the compiler compared 910 signatures when this floor was set; now {compiler}"
  );
}

/// The exclusion is justified by the file, not by this list.
#[test]
fn the_excluded_example_is_the_lexer_catalogue() {
  let path = repo_root().join("examples").join(NOT_A_PROGRAM);
  let source = std::fs::read_to_string(&path).expect("the catalogue should exist");
  assert!(
    parse_source(&source).is_err(),
    "{NOT_A_PROGRAM} parses, so it is a program and belongs in the gate"
  );
}

/// A gate over a file list is narrow in two ways, and this is the other one.
#[test]
fn both_corpora_are_the_size_the_tree_says_they_are() {
  let examples = example_programs().len();
  let listed = std::fs::read_dir(repo_root().join("examples"))
    .expect("examples/ should be readable")
    .filter_map(Result::ok)
    .filter(|e| e.path().extension().is_some_and(|ext| ext == "vlt"))
    .count();
  assert_eq!(
    examples + 1,
    listed,
    "exactly one example is excluded from the gate"
  );
  assert!(
    compiler_sources().len() > 100,
    "the compiler is 122 files; found {}",
    compiler_sources().len()
  );
}

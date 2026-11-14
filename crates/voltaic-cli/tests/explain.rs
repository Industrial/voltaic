//! The **explain** corpus — row 101 leaf 4: `.vlt` beside the `.why` it earns.
//!
//! # What this corpus is for, and why a blessable text file is the right shape
//!
//! `--why` walks the constraint chain and renders it for a human. There is no
//! way to assert that an explanation is *good* — but there is a way to make a
//! change that makes one *worse* show up as a reviewable diff, which is what
//! leaf 4 asks for. A solver change that silently degrades an explanation is
//! exactly the regression a unit test cannot see: the types still unify, the
//! build is still green, and the sentence a reader gets is now unhelpful.
//!
//! So each case records the full rendered chain as text, and
//! `VOLTAIC_UPDATE=1` rewrites it. As everywhere in this tree, an update
//! rewrites and then **fails the run**, so a changed explanation is reviewed
//! rather than absorbed into a green build.
//!
//! # Why the cases are deliberately boring
//!
//! One construct each — an ascription, a call argument, a field read, and a
//! contradiction. An explanation of a complicated program is a wall of text in
//! which a degradation is invisible, so the corpus trades coverage for
//! legibility and gets its coverage from the reason set instead:
//! [`every_reachable_reason_is_pinned_by_a_case`] fails if a reachable `Reason`
//! appears in no expectation, and [`every_unreachable_reason_is_still_unreachable`]
//! fails when an unreachable one starts appearing — which is how a new reason
//! acquires a case instead of shipping unasserted.
//!
//! # `--why` needs a name, so each case names one
//!
//! The name is the file stem's definition, by convention, so there is nothing
//! to keep in sync. A case whose stem is not a definition fails loudly rather
//! than silently explaining nothing.

use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

/// Every `.vlt` under `tests/explain/`, sorted.
fn cases() -> Vec<PathBuf> {
  let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/explain");
  let mut found: Vec<PathBuf> = std::fs::read_dir(&dir)
    .unwrap_or_else(|e| panic!("should read {}: {e}", dir.display()))
    .map(|e| e.expect("should read an entry").path())
    .filter(|p| p.extension().is_some_and(|x| x == "vlt"))
    .collect();
  found.sort();
  found
}

/// The definition `--why` is asked about: the file stem.
fn subject(source: &Path) -> String {
  source
    .file_stem()
    .expect("a case has a stem")
    .to_string_lossy()
    .to_string()
}

/// Run `voltaic sig --why <stem>` on one case and return what it printed.
///
/// Both streams are captured. A contradiction goes to stderr and the chain to
/// stdout, and a case that exercises a contradiction needs both halves or its
/// expectation would record only the part that happened to be on the stream the
/// harness read — which is how `conflict.vlt` would have recorded nothing.
fn explanation_of(source: &Path) -> String {
  let out = Command::new(env!("CARGO_BIN_EXE_voltaic"))
    .arg("sig")
    .arg(source)
    .arg("--why")
    .arg(subject(source))
    .output()
    .expect("should run the voltaic binary");

  let stdout = String::from_utf8_lossy(&out.stdout).to_string();
  let stderr = String::from_utf8_lossy(&out.stderr).to_string();
  assert!(
    !stdout.is_empty(),
    "{} explained nothing on stdout, so `--why` found no definition called `{}`:\n{stderr}",
    source.display(),
    subject(source)
  );

  let name = source.file_name().expect("a name").to_string_lossy();
  let joined = if stderr.is_empty() {
    stdout
  } else {
    format!("{stdout}--- stderr\n{stderr}")
  };
  joined.replace(&source.display().to_string(), &name)
}

#[test]
fn every_explain_case_matches_its_expectation() {
  let updating = std::env::var_os("VOLTAIC_UPDATE").is_some();
  let cases = cases();
  assert!(!cases.is_empty(), "the explain corpus should not be empty");

  for source in &cases {
    let actual = explanation_of(source);
    let expected_path = source.with_extension("why");
    if updating {
      std::fs::write(&expected_path, &actual).expect("should write the expectation");
      continue;
    }
    let expected = std::fs::read_to_string(&expected_path).unwrap_or_else(|e| {
      panic!(
        "{} has no expectation ({e}); run with VOLTAIC_UPDATE=1 to record one",
        source.display()
      )
    });
    assert_eq!(
      actual,
      expected,
      "{} explained itself differently; re-run with VOLTAIC_UPDATE=1 if the change was \
       deliberate, and read the diff — a worse explanation is the regression this corpus \
       exists to catch",
      source.display()
    );
  }

  assert!(
    !updating,
    "the expectations were rewritten; re-run without VOLTAIC_UPDATE to check them"
  );
}

/// A `.why` with no `.vlt` beside it is a stale expectation nothing reads.
#[test]
fn every_expectation_has_a_program() {
  let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/explain");
  for entry in std::fs::read_dir(&dir).expect("should read the corpus") {
    let path = entry.expect("should read an entry").path();
    if path.extension().is_some_and(|x| x == "why") {
      assert!(
        path.with_extension("vlt").exists(),
        "{} has no program beside it",
        path.display()
      );
    }
  }
}

/// The twelve `Reason` variants split three ways, and the split is the finding.
///
/// Leaf 3 gives every structured reason a phrasing. Measured here, **half the
/// vocabulary has no way to appear**, so half those phrasings are written and
/// untestable — which is worth a gate rather than a note, because an
/// unreachable phrasing is indistinguishable from a working one until something
/// makes it reachable.
///
/// Three categories, each asserted below:
///
/// - [`REACHED`] — produced by the generator *and* visible through `--why`, so
///   the corpus pins them.
/// - [`PRODUCED_BUT_UNREACHABLE`] — the generator emits the reason, but `--why`
///   cannot show it, because [`touching`] selects constraints whose holes
///   intersect the *subject's* and these constraints' holes belong to a
///   subexpression. Reaching them needs a richer selection than hole
///   intersection, which is leaf 6's territory.
/// - [`NEVER_PRODUCED`] — no code anywhere constructs the reason. Six of
///   twelve. Each is waiting on the engine that would emit it: `Discharged` on
///   row 95, `ElementOf` and `ParameterUse` on fuller expression inference,
///   `CallArgument` on per-argument constraints rather than one bundled
///   `Callable`, and `LiteralKind` / `ReturnPosition` on the generator using
///   them where it currently uses `Ascribed` and `EquationBody`.
const REACHED: &[&str] = &[
  "pinned by an ascription",
  "an equation's body returns this",
  "a field is read here",
  "this is called",
];

/// Emitted by the generator; not visible through `--why`. See [`REACHED`].
const PRODUCED_BUT_UNREACHABLE: &[&str] = &["a guard must be a bool", "required by a class"];

/// Constructed by no code at all. See [`REACHED`].
const NEVER_PRODUCED: &[&str] = &[
  "passed as argument 1 here",
  "an effect is discharged here",
  "an element of this collection",
  "this literal fixes the type",
  "a parameter is used here",
  "returned from here",
];

fn every_expectation() -> String {
  let all: String = cases()
    .iter()
    .filter_map(|c| std::fs::read_to_string(c.with_extension("why")).ok())
    .collect();
  assert!(
    !all.is_empty(),
    "no expectations exist yet; record them with VOLTAIC_UPDATE=1"
  );
  all
}

#[test]
fn every_reachable_reason_is_pinned_by_a_case() {
  let all = every_expectation();
  let missing: Vec<&str> = REACHED
    .iter()
    .filter(|phrasing| !all.contains(**phrasing))
    .copied()
    .collect();
  assert!(
    missing.is_empty(),
    "these phrasings appear in no expectation, so nothing asserts them: {missing:?}"
  );
}

/// The other half of the gate, and the half that does the work.
///
/// A list of what a corpus covers is narrow in two ways — a phrasing missing
/// from the list, or a phrasing in the list that nothing produces. This is the
/// second: if a later row makes one of these reachable, this test fails and
/// demands a case for it, instead of the phrasing shipping unasserted.
#[test]
fn every_unreachable_reason_is_still_unreachable() {
  let all = every_expectation();
  let surfaced: Vec<&str> = PRODUCED_BUT_UNREACHABLE
    .iter()
    .chain(NEVER_PRODUCED)
    .filter(|phrasing| all.contains(**phrasing))
    .copied()
    .collect();
  assert!(
    surfaced.is_empty(),
    "these phrasings were recorded as unreachable and now appear in an expectation: \
     {surfaced:?}.\n\nThat is progress, not a failure — move each one into REACHED and \
     keep the case that reached it."
  );
}

/// The three lists together are the whole `Reason` vocabulary, so none can be
/// quietly dropped from all three.
#[test]
fn the_three_categories_cover_every_reason() {
  let total = REACHED.len() + PRODUCED_BUT_UNREACHABLE.len() + NEVER_PRODUCED.len();
  assert_eq!(
    total, 12,
    "`Reason` has twelve variants; the categories account for {total}"
  );
}

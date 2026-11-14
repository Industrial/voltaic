//! The **sidecar** corpus — row 100 leaves 1 and 4.
//!
//! # What a sidecar is for
//!
//! Row 99 deletes signature lines, which removes the thing a reader and a
//! downstream crate both relied on: a written statement of the published API.
//! The sidecar puts it back as a *generated* file — `<file>.vlt.sig`, the full
//! `τ ! ε @ μ` judgement, one definition per line, source order, no timestamps.
//!
//! # Why the format has no column alignment, against doc 98 §1.3's example
//!
//! §1.3 draws the format with padded columns and then states, as the second of
//! three load-bearing properties, that ordering is stable *"so a diff is local
//! to the definition that changed"*. Those two cannot both hold: padding is
//! computed from the longest name in the file, so adding one long name reflows
//! every line and the diff is the whole file.
//!
//! The property wins over the drawing, so the format is single-space separated.
//! [`adding_a_long_name_does_not_reflow_the_other_lines`] is the assertion that
//! keeps it that way, because the padded version is the one a future reader
//! will see in the plan and reasonably try to restore.
//!
//! # Why `--semver` is the leaf worth having
//!
//! Comparing two sidecars classifies every change as an addition, a removal, or
//! an alteration, and the last two break. Doc 98 §4 calls this a capability
//! Rust does not have at all, and the reason is exact: a *written* signature
//! cannot be diffed against the signature it replaced, because the old one was
//! overwritten by the edit. An inferred one can, because the old one is a file.
//!
//! [`a_mode_change_alone_is_breaking`] is the case that shows it earning its
//! keep — the type is byte-identical and the API changed.

use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn corpus() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/sidecar")
}

/// Every `.vlt` under `tests/sidecar/`, sorted.
fn cases() -> Vec<PathBuf> {
  let mut found: Vec<PathBuf> = std::fs::read_dir(corpus())
    .expect("should read the corpus")
    .map(|e| e.expect("should read an entry").path())
    .filter(|p| p.extension().is_some_and(|x| x == "vlt"))
    .collect();
  found.sort();
  found
}

/// `voltaic sig --emit <source>`, returning what landed in `<source>.vlt.sig`.
///
/// The source is copied into a scratch directory first, so emitting never
/// writes a `.sig` next to a corpus file that the harness would then have to
/// clean up — and a leftover one would be read by the next run as an
/// expectation.
fn emitted(source: &Path) -> String {
  let scratch = std::env::temp_dir().join(format!(
    "voltaic-sidecar-{}-{}",
    std::process::id(),
    source.file_stem().expect("a stem").to_string_lossy()
  ));
  std::fs::create_dir_all(&scratch).expect("should make a scratch directory");
  let copy = scratch.join(source.file_name().expect("a name"));
  std::fs::copy(source, &copy).expect("should copy the case");

  let out = Command::new(env!("CARGO_BIN_EXE_voltaic"))
    .arg("sig")
    .arg("--emit")
    .arg(&copy)
    .output()
    .expect("should run the voltaic binary");
  assert!(
    out.status.success(),
    "`sig --emit` failed on {}:\n{}",
    source.display(),
    String::from_utf8_lossy(&out.stderr)
  );

  let mut expected = copy.clone().into_os_string();
  expected.push(".sig");
  let text = std::fs::read_to_string(PathBuf::from(expected)).expect("a sidecar should exist");
  std::fs::remove_dir_all(&scratch).ok();
  text
}

#[test]
fn every_case_emits_its_expectation() {
  let updating = std::env::var_os("VOLTAIC_UPDATE").is_some();
  let cases = cases();
  assert!(!cases.is_empty(), "the sidecar corpus should not be empty");

  for source in &cases {
    let actual = emitted(source);
    let expected_path = source.with_extension("vlt.sig");
    if updating {
      std::fs::write(&expected_path, &actual).expect("should write the expectation");
      continue;
    }
    let expected = std::fs::read_to_string(&expected_path).unwrap_or_else(|e| {
      panic!(
        "{} has no sidecar ({e}); run with VOLTAIC_UPDATE=1 to record one",
        source.display()
      )
    });
    assert_eq!(
      actual,
      expected,
      "{}'s published API changed; re-run with VOLTAIC_UPDATE=1 if that was deliberate",
      source.display()
    );
  }

  assert!(
    !updating,
    "the sidecars were rewritten; re-run without VOLTAIC_UPDATE to check them"
  );
}

/// The property doc 98 §1.3 states, asserted against the format it draws.
#[test]
fn adding_a_long_name_does_not_reflow_the_other_lines() {
  let short = "a : u64\n";
  let long = "a : u64\nan_extremely_long_published_name : u64\n";
  let scratch = std::env::temp_dir().join(format!("voltaic-reflow-{}", std::process::id()));
  std::fs::create_dir_all(&scratch).expect("should make a scratch directory");

  let render = |body: &str, tag: &str| -> Vec<String> {
    let path = scratch.join(format!("{tag}.vlt"));
    let mut program = String::new();
    for line in body.lines() {
      let (name, _) = line.split_once(" : ").expect("a fixture line");
      program.push_str(&format!("{line}\n{name} = 1\n"));
    }
    std::fs::write(&path, program).expect("should write the probe");
    let out = Command::new(env!("CARGO_BIN_EXE_voltaic"))
      .arg("sig")
      .arg("--emit")
      .arg(&path)
      .output()
      .expect("should run the voltaic binary");
    assert!(
      out.status.success(),
      "{}",
      String::from_utf8_lossy(&out.stderr)
    );
    let mut sig = path.into_os_string();
    sig.push(".sig");
    std::fs::read_to_string(PathBuf::from(sig))
      .expect("a sidecar")
      .lines()
      .map(str::to_string)
      .collect()
  };

  let before = render(short, "short");
  let after = render(long, "long");
  std::fs::remove_dir_all(&scratch).ok();

  assert_eq!(
    before[1], after[1],
    "adding a longer name reflowed the line above it, so a one-definition change would \
     diff as the whole file — which is the property doc 98 §1.3 states and its drawn \
     format breaks"
  );
}

/// `voltaic sig <source> --semver <sidecar>`: the output and the exit status.
fn semver(tag: &str, source: &Path, recorded: &str) -> (bool, String) {
  let scratch = std::env::temp_dir().join(format!("voltaic-semver-{}-{tag}", std::process::id()));
  std::fs::create_dir_all(&scratch).expect("should make a scratch directory");
  let old = scratch.join("recorded.vlt.sig");
  std::fs::write(&old, recorded).expect("should write the recorded sidecar");

  let out = Command::new(env!("CARGO_BIN_EXE_voltaic"))
    .arg("sig")
    .arg(source)
    .arg("--semver")
    .arg(&old)
    .output()
    .expect("should run the voltaic binary");
  std::fs::remove_dir_all(&scratch).ok();
  (
    out.status.success(),
    format!(
      "{}{}",
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    ),
  )
}

#[test]
fn an_unchanged_api_compares_clean_and_exits_zero() {
  let source = corpus().join("surface.vlt");
  let recorded = std::fs::read_to_string(corpus().join("surface.vlt.sig"))
    .expect("the committed sidecar should exist");
  let (ok, said) = semver("unchanged", &source, &recorded);
  assert!(ok, "an unchanged API should exit zero: {said}");
  assert!(said.contains("no API change"), "{said}");
}

/// The case that justifies the leaf.
#[test]
fn a_mode_change_alone_is_breaking() {
  let source = corpus().join("surface.vlt");
  let recorded = std::fs::read_to_string(corpus().join("surface.vlt.sig"))
    .expect("the committed sidecar should exist");
  let narrowed = recorded.replace("@ p shared", "@ p owned");
  assert_ne!(narrowed, recorded, "the fixture should carry a mode");

  let (ok, said) = semver("narrowed", &source, &narrowed);
  assert!(
    !ok,
    "a mode change is breaking and must exit non-zero: {said}"
  );
  assert!(said.contains("altered"), "{said}");
  assert!(
    said.contains("breaking change"),
    "the count should name it breaking: {said}"
  );
}

#[test]
fn a_removal_breaks_and_an_addition_does_not() {
  let source = corpus().join("surface.vlt");
  let recorded = std::fs::read_to_string(corpus().join("surface.vlt.sig"))
    .expect("the committed sidecar should exist");

  let with_extra = format!("{recorded}gone : u64\n");
  let (ok, said) = semver("extra", &source, &with_extra);
  assert!(!ok, "a definition that disappeared is breaking: {said}");
  assert!(said.contains("removed  gone"), "{said}");

  let without_one = recorded
    .lines()
    .filter(|l| !l.starts_with("scaled "))
    .map(|l| format!("{l}\n"))
    .collect::<String>();
  let (ok, said) = semver("fewer", &source, &without_one);
  assert!(ok, "an addition alone is not breaking: {said}");
  assert!(said.contains("added    scaled"), "{said}");
  assert!(said.contains("none breaking"), "{said}");
}

/// `--semver` takes one source. Two would silently compare only one of them.
#[test]
fn semver_refuses_more_than_one_source() {
  let out = Command::new(env!("CARGO_BIN_EXE_voltaic"))
    .arg("sig")
    .arg(corpus().join("surface.vlt"))
    .arg(corpus().join("effects.vlt"))
    .arg("--semver")
    .arg(corpus().join("surface.vlt.sig"))
    .output()
    .expect("should run the voltaic binary");
  assert!(!out.status.success());
  assert!(
    String::from_utf8_lossy(&out.stderr).contains("one .vlt against one sidecar"),
    "{}",
    String::from_utf8_lossy(&out.stderr)
  );
}

/// A `.vlt.sig` with no `.vlt` beside it is a stale expectation nothing reads.
#[test]
fn every_sidecar_has_a_program() {
  for entry in std::fs::read_dir(corpus()).expect("should read the corpus") {
    let path = entry.expect("should read an entry").path();
    if path.to_string_lossy().ends_with(".vlt.sig") {
      let program = PathBuf::from(path.to_string_lossy().trim_end_matches(".sig").to_string());
      assert!(
        program.exists(),
        "{} has no program beside it",
        path.display()
      );
    }
  }
}

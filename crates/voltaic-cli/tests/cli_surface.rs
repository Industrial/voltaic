//! Voltaic's command-line surface is exactly what doc 01 §27 says it is.
//!
//! Plan 11 (`docs/16 - 11 - CLI Surface.md`) leaf 4.
//!
//! > `build`, `check`, `fmt`, `watch`, `--version`, `--help` … Cargo owns
//! > build, test, docs and dependencies.
//!
//! **The list below is a deliberate second copy.** A test that read `COMMANDS`
//! would pass for any table, which is no test at all; this one fails when the
//! table changes, so a seventh subcommand costs three edits — the specification,
//! the table, and a line here that says why it is here. Doc 02 §3 records
//! Emerald's CLI reaching seventeen subcommands, and nothing in its code said
//! when to stop. This is that something.
//!
//! # `sig` is the seventh, and it paid the three edits
//!
//! Generation 2 adds one subcommand. Its authority is **doc 86 §8.1**, not doc
//! 01 §27: doc 01 is generation 1's record and is not edited (doc 04 rule 3),
//! so the specification edit landed in the document that supersedes it.
//!
//! It is not a convenience. Doc 86 §2 clause 1 admits a solver-determined
//! lowering *only if the choice can be printed*, so `voltaic sig` is the
//! mechanism that discharges the clause the whole generation rests on — row 92
//! §2.3 made it a condition of the charter reversal. A seventh subcommand that
//! the specification requires is a different thing from a seventh that someone
//! wanted.
//!
//! # `gen` is the eighth, and it paid them too
//!
//! Its authority is **doc 86 §8.5**. `build` transpiles one file; a crate is a
//! tree, and a downstream project otherwise needs its own shell loop to walk one
//! — mirroring directories, writing sidecars before builds so cross-crate
//! resolution (§8.3) sees them, and sweeping outputs whose source was deleted.
//! `gen` is that loop, written once,
//! with `--check` for CI.

use std::process::Command;

/// The surface, written out: doc 01 §27's six, plus doc 86 §8.1's `sig` and §8.5's `gen`.
const SURFACE: &[&str] = &[
  "build",
  "check",
  "fmt",
  "gen",
  "sig",
  "watch",
  "--version",
  "--help",
];

fn voltaic(args: &[&str]) -> std::process::Output {
  Command::new(env!("CARGO_BIN_EXE_voltaic"))
    .args(args)
    .output()
    .expect("should run the voltaic binary")
}

fn help_text() -> String {
  let out = voltaic(&["--help"]);
  String::from_utf8_lossy(&out.stdout).to_string()
}

/// Criteria 1 and 2: the surface is doc 01 §27's, and `--help` shows all of it.
#[test]
fn the_surface_is_exactly_doc_01_section_27() {
  let help = help_text();
  for name in SURFACE {
    assert!(
      help.contains(name),
      "`{name}` is in doc 01 §27 but not in --help:\n{help}"
    );
  }

  // And nothing else. Every `voltaic <word>` line in the usage must name one of
  // the eight — a ninth would show up here.
  for line in help.lines().filter(|l| l.starts_with("  voltaic ")) {
    let named = line.trim_start().trim_start_matches("voltaic ");
    let name = named.split_whitespace().next().unwrap_or_default();
    assert!(
      SURFACE.contains(&name),
      "--help advertises `{name}`, which the specification does not list. \
       If this is deliberate, the specification and SURFACE both need \
       editing — and for generation 2 the specification is doc 86, because \
       doc 01 is generation 1's record and is not edited:\n{line}"
    );
  }
}

/// Criterion 3: **every declared command is implemented.**
///
/// This test used to pick the one planned command and assert it said so, and
/// its own comment recorded the pattern: *"`watch` was here too until plan 12
/// implemented it, and this test failing was how plan 12 learned it had
/// changed the surface."*
///
/// Plan 34 implemented `fmt`, which was the last one — so the test inverts.
/// There is nothing left to assert *about* a planned command, and what is
/// worth asserting is that the list is empty: doc 01 §27's surface is fully
/// built, and a future row that declares a command ahead of itself will make
/// this fail and have to say so.
///
/// The `Planned` rendering it used to check is unit-tested in
/// `run/command.rs`, which is where it belongs — it is a property of the type,
/// not of any command.
#[test]
fn every_declared_command_is_implemented() {
  let out = voltaic(&["--help"]);
  let stdout = String::from_utf8_lossy(&out.stdout);
  assert!(
    !stdout.contains("not implemented"),
    "a command in the usage text is declared but unbuilt:\n{stdout}"
  );
  assert!(
    !stdout.contains("plan "),
    "the usage text names a plan, so some command is still planned:\n{stdout}"
  );
}

/// Criterion 4: an unknown command is reported as unknown, with the usage.
#[test]
fn an_unknown_command_is_reported_as_unknown() {
  let out = voltaic(&["frobnicate"]);
  let stderr = String::from_utf8_lossy(&out.stderr);
  assert!(!out.status.success());
  assert!(stderr.contains("unknown command `frobnicate`"), "{stderr}");
  assert!(
    stderr.contains("usage:"),
    "the usage should follow:\n{stderr}"
  );
  assert!(
    !stderr.contains("not implemented yet"),
    "a typo is not a planned feature:\n{stderr}"
  );
}

/// Criterion 5: `--help` and `--version` succeed, on stdout.
#[test]
fn help_and_version_succeed_on_stdout() {
  for args in [vec!["--help"], vec!["-h"], vec!["help"], vec![]] {
    let out = voltaic(&args);
    assert!(out.status.success(), "{args:?} should succeed");
    assert!(!out.stdout.is_empty(), "{args:?} should print to stdout");
    assert!(
      String::from_utf8_lossy(&out.stdout).contains("usage:"),
      "{args:?} should print the usage"
    );
  }

  for args in [["--version"], ["-V"]] {
    let out = voltaic(&args);
    assert!(out.status.success(), "{args:?} should succeed");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.starts_with("voltaic "), "{stdout}");
    assert!(
      stdout
        .trim()
        .split(' ')
        .nth(1)
        .is_some_and(|v| v.contains('.')),
      "a version should look like one: {stdout}"
    );
  }
}

/// Every command the table calls `Ready` must actually dispatch.
///
/// `run`'s match panics on a `Ready` command with no arm, so this is what
/// turns that `unreachable!` into a test failure rather than a user-visible
/// crash. Invoked with no arguments, so a ready command fails on its *usage*
/// rather than succeeding — what matters is that it did not panic and did not
/// claim to be unimplemented.
#[test]
fn every_ready_command_dispatches() {
  for name in ["build", "check"] {
    let out = voltaic(&[name]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
      !stderr.contains("not implemented yet"),
      "`{name}` is Ready but reported as planned:\n{stderr}"
    );
    assert!(
      !stderr.contains("has no dispatch arm"),
      "`{name}` is Ready but has no dispatch arm:\n{stderr}"
    );
    assert!(
      stderr.contains("no input file"),
      "`{name}` with no arguments should ask for an input:\n{stderr}"
    );
  }
}

/// The surface names Cargo as the owner of everything it declines, so a reader
/// learns where `test`, `doc` and `add` went.
#[test]
fn the_usage_says_what_cargo_owns() {
  let help = help_text();
  assert!(help.contains("Cargo owns"), "{help}");
  for declined in ["test", "doc", "add", "publish", "init", "new"] {
    for line in help.lines().filter(|l| l.starts_with("  voltaic ")) {
      let name = line
        .trim_start()
        .trim_start_matches("voltaic ")
        .split_whitespace()
        .next()
        .unwrap_or_default();
      assert_ne!(
        name, declined,
        "`{declined}` is Cargo's, and doc 01 §27 declines it"
      );
    }
  }
}

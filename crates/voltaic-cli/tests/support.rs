//! Shared helpers for the end-to-end tests.
//!
//! A `mod support;` include rather than a construct file, because an
//! integration test's helpers are not part of the crate's vocabulary — doc 06
//! §7 exempts `tests/` from the one-construct rule for exactly this reason.

#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::LazyLock;

/// The toolchain's rustc, from `voltaic_driver::rustc_path`.
///
/// This was `env!("VOLTAIC_RUSTC")` against `voltaic-cli`'s own `build.rs` until
/// plan 50 leaf 1 moved that script to `voltaic-driver` — `cargo::rustc-env`
/// bakes into the crate carrying the script, so the value is no longer
/// readable from here and the driver's resolution is now the only one.
///
/// Resolving from `PATH` directly is still unreliable, which is why the
/// resolution is the driver's and not a `PATH` lookup: under `blaze test` the
/// test process sees NixOS's `stub-ld` shim instead. `rustc_path` falls
/// through it because it **verifies** each candidate.
static RUSTC: LazyLock<String> = LazyLock::new(|| {
  voltaic_driver::rustc_path()
    .map(|path| path.display().to_string())
    // Name it anyway when nothing resolves, so `skip_without_rustc`'s message
    // says something rather than nothing.
    .unwrap_or_else(|| "rustc".to_string())
});

/// Every crate whose `src/` is generated from Voltaic, with the plan that ported
/// it, read from `seed/GENERATION`.
///
/// **A file rather than a constant, because the list was in four places and
/// three had drifted.** `conformance.rs` said *"Two crates"* with three
/// ported; `bootstrap.rs` and `bootstrap.sh` both named two, so a clean clone
/// did not rebuild the third and the test that would have caught it read the
/// same stale list. `seed/GENERATION` is now the only copy, and `bootstrap.sh`
/// reads the same file.
///
/// The order is the order they were ported, which is also dependency order:
/// `voltaic-fmt` reads `voltaic-ast`'s types, so a staged check of the second
/// needs the first already in place.
pub fn ported() -> Vec<(String, u32)> {
  generations()
    .into_iter()
    .map(|(name, plan, _)| (name, plan))
    .collect()
}

/// The same list with row 102's third column: the generation each crate's
/// `.vlt` is written in, `g1` or `g2`.
///
/// `bootstrap.sh` reads the same file and chooses the blob per crate, which is
/// what makes row 103 a migration rather than one commit — crates cross the turn
/// one at a time, with a reviewable state after each.
pub fn generations() -> Vec<(String, u32, String)> {
  let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../seed/GENERATION");
  let text = std::fs::read_to_string(&path)
    .unwrap_or_else(|e| panic!("should read {}: {e}", path.display()));
  let out: Vec<(String, u32, String)> = text
    .lines()
    .map(str::trim)
    .filter(|l| !l.is_empty() && !l.starts_with('#'))
    .map(|l| {
      let mut parts = l.split_whitespace();
      let name = parts
        .next()
        .unwrap_or_else(|| panic!("seed/GENERATION: `{l}` is not `crate plan generation`"));
      let plan: u32 = parts
        .next()
        .unwrap_or_else(|| panic!("seed/GENERATION: `{l}` has no plan number"))
        .parse()
        .unwrap_or_else(|e| panic!("seed/GENERATION: `{l}` has no plan number: {e}"));
      let generation = parts
        .next()
        .unwrap_or_else(|| {
          panic!(
            "seed/GENERATION: `{l}` has no generation column. Row 102 added it; a line              without one would silently pick a blob."
          )
        })
        .to_string();
      assert!(
        generation == "g2",
        "seed/GENERATION: `{l}` marks the generation `{generation}`; expected g2 — \
         `g1` was retired with its blob by row 102 §3.1, so a line naming it \
         would send `bootstrap.sh` at a file that is not there"
      );
      assert!(
        parts.next().is_none(),
        "seed/GENERATION: `{l}` has a fourth column, which nothing reads"
      );
      (name.to_string(), plan, generation)
    })
    .collect();
  assert!(
    !out.is_empty(),
    "seed/GENERATION should name at least one crate"
  );
  out
}

pub struct Case {
  pub rust: String,
  vrs: PathBuf,
  dir: PathBuf,
}

impl Case {
  /// The source map `voltaic build` wrote beside the `.vrs` (plan 06).
  pub fn map_path(&self) -> PathBuf {
    let mut name = self.vrs.file_name().expect("a name").to_os_string();
    name.push(".map");
    self.vrs.with_file_name(name)
  }

  /// Compile the generated Rust with real rustc and run it, returning stdout.
  ///
  /// Asserts **zero stderr**, not merely a zero exit code: a warning is a
  /// defect in the emitter, because the user never wrote the generated line it
  /// would point at.
  pub fn run(&self) -> String {
    let bin = self.dir.join("program");
    let out = Command::new(rustc())
      .args(["--edition", "2024", "-o"])
      .arg(&bin)
      .arg(&self.vrs)
      .output()
      .expect("should run rustc");
    assert!(
      out.stderr.is_empty(),
      "rustc should emit nothing for generated code, got:\n{}",
      String::from_utf8_lossy(&out.stderr)
    );
    assert!(
      out.status.success(),
      "rustc should compile the generated .vrs"
    );

    let run = Command::new(&bin).output().expect("should run the binary");
    assert!(run.status.success(), "the compiled binary should exit 0");
    String::from_utf8_lossy(&run.stdout).into_owned()
  }

  /// Compile the generated Rust **as a test binary** and run its tests.
  ///
  /// Doc 03 §15's test module is `#[cfg(test)]`, so an ordinary build does
  /// not compile it at all — which means the corpus checked *nothing* about
  /// the generated tests until this existed: not that they compile, not that
  /// they pass, not that two slugs collided into one function name.
  ///
  /// A no-op for a file with no test module, so every case can call it.
  pub fn run_tests(&self) {
    let rust = std::fs::read_to_string(&self.vrs).expect("should read the .vrs");
    if !rust.contains("#[cfg(test)]") {
      return;
    }

    let bin = self.dir.join("tests");
    let out = Command::new(rustc())
      .args(["--edition", "2024", "--test", "-o"])
      .arg(&bin)
      .arg(&self.vrs)
      .output()
      .expect("should run rustc");
    assert!(
      out.stderr.is_empty(),
      "rustc should emit nothing for the generated test module, got:\n{}",
      String::from_utf8_lossy(&out.stderr)
    );
    assert!(
      out.status.success(),
      "rustc should compile the generated test module"
    );

    let run = Command::new(&bin).output().expect("should run the tests");
    assert!(
      run.status.success(),
      "the generated tests should pass:\n{}",
      String::from_utf8_lossy(&run.stdout)
    );
  }
}

impl Drop for Case {
  fn drop(&mut self) {
    std::fs::remove_dir_all(&self.dir).ok();
  }
}

fn workspace_root() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// A rustfmt that behaves like rustfmt.
///
/// Verified rather than trusted, the same rule `build.rs` applies to rustc —
/// plan 03's third blaze shim is why that rule exists.
fn rustfmt_path() -> Option<PathBuf> {
  let dir = Path::new(rustc()).parent()?;
  let candidate = dir.join("rustfmt");
  let works = Command::new(&candidate)
    .arg("--version")
    .output()
    .is_ok_and(|o| o.status.success() && String::from_utf8_lossy(&o.stdout).contains("rustfmt"));
  works.then_some(candidate)
}

pub fn rustfmt_available() -> bool {
  rustfmt_path().is_some()
}

/// Format `rust` with rustfmt's **defaults**, outside the repository.
///
/// This repo's own `rustfmt.toml` sets two-space indentation, which governs
/// Voltaic's *source*. Voltaic's *output* uses rustfmt's default four, so running
/// rustfmt inside the repo would grade the output against the wrong config —
/// a real finding from plan 04's probe, where every example "failed" purely on
/// indentation until the check moved out of the tree.
pub fn rustfmt(rust: &str) -> String {
  let fmt = rustfmt_path().expect("caller should check rustfmt_available");
  // `std::env::temp_dir()` is outside the repository, so no `rustfmt.toml`
  // applies and rustfmt uses its defaults.
  let dir = scratch("rustfmt");
  let file = dir.join("generated.rs");
  std::fs::write(&file, rust).expect("should write the generated Rust");

  let out = Command::new(&fmt)
    .args(["--edition", "2024"])
    .arg(&file)
    .output()
    .expect("should run rustfmt");
  assert!(
    out.status.success(),
    "rustfmt failed on generated code:\n{}",
    String::from_utf8_lossy(&out.stderr)
  );
  let formatted = std::fs::read_to_string(&file).expect("should read the formatted file");
  std::fs::remove_dir_all(&dir).ok();
  formatted
}

/// Format `rust` with **this repository's** config, for a file that will live
/// in the tree.
///
/// The opposite question from [`rustfmt`] above, and deliberately a second
/// function rather than a flag on the first. That one grades the **emitter**,
/// so it formats outside the tree where rustfmt's defaults apply; this one
/// produces a file that will be **committed into `src/`** and graded by
/// `cargo fmt --check` like every other file there, so the repository's
/// `rustfmt.toml` is the right config and not the wrong one.
///
/// Plan 68 leaf 0. Voltaic's output is four-space and this repo is two-space, so
/// without this step every generated file in `crates/voltaic-ast/src/` would
/// fail the lint gate on indentation alone.
pub fn rustfmt_in_place(rust: &str) -> String {
  let fmt = rustfmt_path().expect("caller should check rustfmt_available");
  let dir = workspace_root().join("target/voltaic-port-fmt");
  std::fs::create_dir_all(&dir).expect("should create the scratch directory");
  let file = dir.join("generated.rs");
  std::fs::write(&file, rust).expect("should write the generated Rust");

  let config = workspace_root().join("rustfmt.toml");
  let out = Command::new(&fmt)
    .arg("--config-path")
    .arg(&config)
    // **`skip_children`, or a `mod x;` sends rustfmt looking for the file.**
    // The port writes one module per file and nine of them declare
    // submodules, so without this rustfmt reports *"failed to resolve mod
    // `binding`"* against a scratch directory holding only the one file.
    // Formatting one file is the whole job here.
    //
    // A `--config` override rather than the `--skip-children` flag, which
    // this rustfmt does not have: it is a config key, and `--config` sets one
    // without disturbing the `rustfmt.toml` the repository commits.
    .args(["--config", "skip_children=true"])
    .arg(&file)
    .output()
    .expect("should run rustfmt");
  assert!(
    out.status.success(),
    "rustfmt failed on the port's output:\n{}",
    String::from_utf8_lossy(&out.stderr)
  );
  std::fs::read_to_string(&file).expect("should read the formatted file")
}

/// A scratch directory unique to this call.
///
/// Keyed by a counter as well as the pid: keying on the tag alone let two
/// concurrent tests share a directory, so one test's `Case::drop` deleted the
/// `.vrs` another was still compiling. Found by running the suite, not by
/// reading it.
fn scratch(tag: &str) -> PathBuf {
  use std::sync::atomic::{AtomicUsize, Ordering};
  static NEXT: AtomicUsize = AtomicUsize::new(0);
  let n = NEXT.fetch_add(1, Ordering::Relaxed);
  let dir = std::env::temp_dir().join(format!("voltaic-{tag}-{}-{n}", std::process::id()));
  std::fs::create_dir_all(&dir).expect("should create a scratch directory");
  dir
}

/// Transpile an `examples/` fixture.
pub fn transpile(example: &str) -> Case {
  transpile_path(&workspace_root().join("examples").join(example))
}

/// Transpile the `.vlt` at `source`, wherever it lives.
///
/// Plan 08 (`docs/13 - 08 - Run Expectations.md`) leaf 1: the run corpus is
/// under `tests/run/`, not `examples/`, so it needs a path rather than a name.
pub fn transpile_path(source: &Path) -> Case {
  let tag = source
    .file_stem()
    .expect("a stem")
    .to_str()
    .expect("a UTF-8 stem");
  build(source, tag)
}

/// The verified rustc path, for handing to a child process.
pub fn rustc() -> &'static str {
  RUSTC.as_str()
}

/// Whether a rustc that actually runs is available.
///
/// Verified by execution, not by the path existing — plan 01 found NixOS's
/// `stub-ld` shim answering to `rustc` under `blaze test`, which fails only
/// when invoked.
pub fn rustc_runs() -> bool {
  Command::new(rustc())
    .arg("--version")
    .output()
    .is_ok_and(|o| o.status.success())
}

/// Skip `test` — reporting that it was skipped — when no rustc will run.
///
/// Returns `true` if the caller should return early. A silent skip is worse
/// than no test, so the reason is printed.
pub fn skip_without_rustc(test: &str) -> bool {
  if rustc_runs() {
    return false;
  }
  eprintln!("skipping {test}: no working rustc at {}", rustc());
  true
}

/// A cargo that actually runs, for the tests that build a nested project.
///
/// **Not** `env!("CARGO")` alone. Under `blaze test` that is
/// `~/.blaze/toolchains/…/bin/cargo`, a generic-linux binary NixOS refuses to
/// execute:
///
/// ```text
/// Could not start dynamically linked executable: …/bin/cargo
/// ```
///
/// which is the fourth time a toolchain binary has answered to the right name
/// and not worked (plan 01's `stub-ld` rustc, plan 03's two blaze shims, now
/// this). So the rule AGENTS.md records is applied again: try the candidates
/// in order and **check the banner**, never trust the name.
pub fn cargo_path() -> Option<PathBuf> {
  let from_env = std::env::var_os("CARGO").map(PathBuf::from);
  let baked = PathBuf::from(env!("CARGO"));
  let from_path = std::env::var_os("PATH").into_iter().flat_map(|paths| {
    std::env::split_paths(&paths)
      .map(|d| d.join("cargo"))
      .collect::<Vec<_>>()
  });

  from_env
    .into_iter()
    .chain(std::iter::once(baked))
    .chain(from_path)
    .find(|c| {
      Command::new(c).arg("--version").output().is_ok_and(|o| {
        o.status.success() && String::from_utf8_lossy(&o.stdout).starts_with("cargo ")
      })
    })
}

/// Skip `test` when no cargo will run.
pub fn skip_without_cargo(test: &str) -> bool {
  if cargo_path().is_some() {
    return false;
  }
  eprintln!("skipping {test}: no working cargo (blaze's is not executable on NixOS)");
  true
}

/// Transpile inline source. Used only where a fixture would exist solely to
/// pin one grammar rule; anything a reader should be able to run lives in
/// `examples/` as a file (doc 06 §7).
pub fn transpile_source(tag: &str, src: &str) -> String {
  let dir = scratch(tag);
  let vlt = dir.join(format!("{tag}.vlt"));
  std::fs::write(&vlt, src).expect("should write the source");
  // Cloned rather than moved: `Case::drop` deletes the scratch directory, so
  // the field cannot be moved out of it.
  build(&vlt, tag).rust.clone()
}

fn build(source: &Path, tag: &str) -> Case {
  let dir = scratch(tag);
  let vrs = dir.join(format!("{tag}.vrs"));
  let out = Command::new(env!("CARGO_BIN_EXE_voltaic"))
    .arg("build")
    .arg(source)
    .arg("-o")
    .arg(&vrs)
    .output()
    .expect("should run the voltaic binary");
  assert!(
    out.status.success(),
    "voltaic build failed on {}:\n{}",
    source.display(),
    String::from_utf8_lossy(&out.stderr)
  );
  // **Voltaic's own diagnostics count too, as of plan 38.** The corpus has
  // always demanded zero *rustc* output; it said nothing about Voltaic's,
  // because until `VLT1001` every Voltaic diagnostic was fatal and
  // `status.success()` covered them all.
  //
  // A warning broke that: `examples/fallible.vlt` emitted three `VLT1001`s
  // that nothing noticed. An example is exemplary code, so a lint firing on
  // one is either a bug in the example or a deliberate exception that should
  // say so with the pragma. Plan 38 §10 records it.
  assert!(
    out.stderr.is_empty(),
    "voltaic should emit nothing for an example, got:\n{}\n\
     if a warning is intended, suppress it at the call site and say why",
    String::from_utf8_lossy(&out.stderr)
  );
  let rust = std::fs::read_to_string(&vrs).expect("should read the generated .vrs");
  Case { rust, vrs, dir }
}

/// The accepted surface and the tree's source are the same generation.
///
/// # What was here, and why it is gone
///
/// `mid_turn()` and a `the_surface_has_not_moved_yet` assertion that `ACCEPTS`
/// was `"g1"`. Doc 97 §2.2 predicted that a mixed-generation corpus would break
/// the five gates that parse the compiler's own source, so the mechanism let
/// them **defer** while the surface ran ahead, and row 103 was to re-arm them
/// crate by crate.
///
/// **The cost never came due.** Row 103 leaf 6 measured it at the first crate:
/// with one generation-2 crate and thirteen generation-1 ones, all five gates
/// passed, because the generation-2 spelling is **additive** — `x.f ()` and
/// `x |> .f` are both live, `name Type` and `name : Type` are both live. So the
/// whole migration ran with the full gate armed and `ACCEPTS` untouched.
///
/// It is removed rather than kept as an escape hatch, which was the earlier
/// plan, because after row 102 §3.1 it **cannot fire**: `mid_turn()` required a
/// `g1` line in `seed/GENERATION`, and [`generations`] now rejects one outright —
/// `g1`'s blob is gone, so a line naming it would send `bootstrap.sh` at a file
/// that is not there. A guard whose condition is unreachable is worse than no
/// guard: it reads as protection.
#[test]
fn the_accepted_surface_matches_the_trees_source() {
  assert_eq!(
    voltaic_lexer::ACCEPTS,
    "g2",
    "`ACCEPTS` names the generation the lexer's surface accepts, and every \
     `seed/GENERATION` line reads `g2` — so these must agree or one of them is \
     stale"
  );
  let generations: Vec<String> = generations().into_iter().map(|(_, _, g)| g).collect();
  assert!(
    generations.iter().all(|g| g == voltaic_lexer::ACCEPTS),
    "every crate should be written in the accepted generation; got {generations:?}"
  );
}

/// The marker has to name a real generation, or `bootstrap.sh` would dispatch to
/// a blob that cannot exist.
#[test]
fn the_accepted_generation_is_one_bootstrap_can_dispatch_to() {
  assert!(
    voltaic_lexer::is_generation(voltaic_lexer::ACCEPTS),
    "`ACCEPTS` is `{}`, which is not a generation",
    voltaic_lexer::ACCEPTS
  );
}

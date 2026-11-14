//! Row 103 leaf 3: the identity gate, and that it runs **before** the write.
//!
//! > Generate from the migrated source with the g2 compiler, diff against the
//! > generation-1 `.xrs` for the same file. Byte-identical or the file is
//! > rejected and left untouched.
//! >
//! > **The gate runs before the write, not after.** §2.3's first trap: a check
//! > that exists to prevent a destructive step must be checked before the step
//! > runs. The migrator writes to a temporary path, gates, then moves.
//!
//! # Why the gate needs no wasm
//!
//! The plan says *"with the g2 compiler"*, meaning the blob. It does not have to
//! be the blob: the compiler in the tree reads **both** generations for the
//! duration of the turn, so it can transpile the generation-1 original and the
//! generation-2 candidate itself and compare. That is the same comparison with
//! one fewer moving part, and it costs nothing in trust because
//! `voltaic-seed/tests/across_the_turn.rs` separately proves the blob and the
//! tree agree byte-for-byte over all fourteen ported crates and all forty-seven
//! compiling examples. The two gates compose: that one says blob == tree, this
//! one says tree(original) == tree(migrated).
//!
//! # What these tests assert that the inline ones cannot
//!
//! `migrate.vlt`'s own tests are pure — text in, [`Gate`] out — because that is
//! what an inline test may see. The claim this file exists for is about the
//! **filesystem**: a rejected migration must leave the file it was going to
//! overwrite byte-identical. A pure test cannot observe a write that did not
//! happen.

use std::path::PathBuf;

use voltaic_driver::{Gate, Side, apply, gate};

/// A scratch `.vlt` under `target/`, with the given contents.
///
/// `target/` rather than `std::env::temp_dir`, and that is not arbitrary:
/// `AGENTS.md` records `TMPDIR` arriving unset in this shell and scratch objects
/// landing in a 2.0 GB tmpfs that was 90% full. `target/` is on the 630 GB
/// `/home`.
fn scratch(label: &str, contents: &str) -> PathBuf {
  let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("../../target/identity-gate")
    .join(label);
  std::fs::create_dir_all(&dir).expect("should create the scratch directory");
  let path = dir.join("probe.vlt");
  std::fs::write(&path, contents).expect("should write the scratch file");
  path
}

const ORIGINAL: &str = "pick : u64 -> u64\npick n = n |> .max 3\n";

#[test]
fn a_migration_the_gate_accepts_is_written() {
  // Receiver-last and pipelined spell the same call, so the lowering is
  // identical and the migration is exactly what pass 1 produces.
  let migrated = "pick : u64 -> u64\npick n = .max 3 n\n";
  let path = scratch("accepted", ORIGINAL);

  let verdict = apply(&path, migrated).expect("the scratch file should be readable");
  assert!(
    matches!(verdict, Gate::Identical),
    "a spelling change with the same lowering should pass: {}",
    verdict.explain()
  );
  assert_eq!(
    std::fs::read_to_string(&path).expect("should read back"),
    migrated,
    "an accepted migration should have been written"
  );
}

#[test]
fn a_migration_the_gate_rejects_leaves_the_file_untouched() {
  // `.max 3` to `.min 3` is a changed program, not a changed spelling.
  let migrated = "pick : u64 -> u64\npick n = .min 3 n\n";
  let path = scratch("rejected", ORIGINAL);

  let verdict = apply(&path, migrated).expect("the scratch file should be readable");
  assert!(
    matches!(verdict, Gate::Differs { .. }),
    "a changed program should fail the gate: {}",
    verdict.explain()
  );
  assert_eq!(
    std::fs::read_to_string(&path).expect("should read back"),
    ORIGINAL,
    "THE GATE RAN AFTER THE WRITE. A rejected migration must leave the file \
     byte-identical — this is doc 101 §2.3's first trap, and the whole reason the \
     gate exists is that the step it guards is destructive."
  );
}

#[test]
fn a_migration_the_compiler_cannot_read_leaves_the_file_untouched() {
  let path = scratch("unreadable", ORIGINAL);

  let verdict = apply(&path, "pick n = = =\n").expect("the scratch file should be readable");
  assert!(
    matches!(
      verdict,
      Gate::Unreadable {
        side: Side::Migrated,
        ..
      }
    ),
    "unparseable candidate source should be rejected, not compared: {}",
    verdict.explain()
  );
  assert_eq!(
    std::fs::read_to_string(&path).expect("should read back"),
    ORIGINAL,
    "a candidate the compiler cannot read must leave the file byte-identical"
  );
}

/// The gate's verdict is about the **generated Rust**, not about the `.vlt` text.
///
/// Worth asserting on its own, because it is the property that makes pass 1
/// checkable at all: the migrated source is a different program by every textual
/// measure, and the only thing that may not change is what it lowers to.
#[test]
fn identical_lowering_from_different_text_is_what_passes() {
  let migrated = "pick : u64 -> u64\npick n = .max 3 n\n";
  assert_ne!(
    ORIGINAL, migrated,
    "the two spellings should differ as text"
  );

  let verdict = gate(&PathBuf::from("probe.vlt"), ORIGINAL, migrated);
  assert!(
    matches!(verdict, Gate::Identical),
    "different text, same lowering: {}",
    verdict.explain()
  );
}

/// A file that is already generation 2 gates against itself, which is what makes
/// the migrator **idempotent**: running it twice is running it once.
///
/// Row 103 migrates crate by crate and `seed/GENERATION`'s column is what tracks
/// progress, so a second run over an already-migrated crate has to be a no-op
/// rather than a rejection. Without this the gate would report every finished
/// crate as failing.
#[test]
fn an_already_migrated_file_gates_against_itself() {
  let already = "pick : u64 -> u64\npick n = .max 3 n\n";
  let path = scratch("idempotent", already);

  let verdict = apply(&path, already).expect("the scratch file should be readable");
  assert!(
    matches!(verdict, Gate::Identical),
    "a file already in the target generation should pass: {}",
    verdict.explain()
  );
  assert_eq!(
    std::fs::read_to_string(&path).expect("should read back"),
    already
  );
}

//! Every committed blob under `seed/` is the blob `seed/SEED.md` claims it is.
//!
//! Plan 72 leaf 3, generalised to two generations by row 102 leaf 3. A
//! checked-in binary is the one artifact in this repository a reader cannot
//! inspect, so the two things that *can* be asserted about it are asserted: its
//! size and its hash, against the table in `SEED.md` that also records the
//! commit and the command it came from.
//!
//! That makes a hand-edited blob and a stale table the same failure, which is
//! the point — a seed whose provenance row disagrees with its bytes is worse
//! than no row at all, because the row reads as verification.
//!
//! ```bash
//! VOLTAIC_UPDATE=1 cargo test -p voltaic-seed --test blob
//! ```
//!
//! # Why the tables are keyed by `compiles` and not by a filename
//!
//! A blob's `compiles` row says which generation it reads, and the file is named
//! `voltaic-<compiles>.wasm`. So one row determines the filename and there is no
//! second copy of the mapping to drift — the same reason `seed/GENERATION` is the
//! only copy of the crate list. A table with no `compiles` row names no blob and
//! is caught as such; a blob with no table is caught from the other side by
//! [`every_committed_blob_has_a_provenance_table`].
//!
//! # What this cannot assert
//!
//! That a blob is what the recipe *produces*. `wasm-opt` and rustc are
//! deterministic for a fixed input and a fixed version, but the versions are
//! pinned by `devenv.nix` rather than by this test, so a toolchain bump moves
//! the hash legitimately. The assertion is therefore about **agreement**
//! between a blob and its documentation, not about reproducibility — and
//! `tests/fixpoint.rs` is what checks a blob still *behaves*.

use std::path::PathBuf;

/// The workspace root, from this crate's manifest.
fn workspace_root() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// One `## Identity` section of `SEED.md`: the generation it describes and its text.
struct Table {
  compiles: String,
  built_by: String,
  hash: Option<String>,
  size: Option<usize>,
}

/// The value of a `| name | value |` row, or `None` if the section has no such row.
fn row(section: &str, name: &str) -> Option<String> {
  section
    .lines()
    .map(str::trim)
    .find(|l| l.starts_with(&format!("| {name} |")))
    .map(|l| {
      l.trim_start_matches('|')
        .trim()
        .trim_start_matches(name)
        .trim()
        .trim_start_matches('|')
        .trim()
        .trim_end_matches('|')
        .trim()
        .trim_matches('`')
        .to_string()
    })
}

/// Every identity table in `SEED.md`, keyed by the generation it compiles.
///
/// A section without a `compiles` row is not an identity table — `What they are
/// for` and the rebuild recipes are prose — so it is skipped rather than
/// rejected. A section *with* one and nothing else is a real failure and shows up
/// as a missing size or hash below.
fn tables(doc: &str) -> Vec<Table> {
  doc
    .split("\n## ")
    .filter_map(|section| {
      let compiles = row(section, "compiles")?;
      Some(Table {
        compiles,
        built_by: row(section, "built by").unwrap_or_default(),
        hash: row(section, "blake3"),
        size: row(section, "size")
          .and_then(|s| s.trim_end_matches(" bytes").replace(',', "").parse().ok()),
      })
    })
    .collect()
}

/// `SEED.md` with the size and blake3 rows of one generation's table rewritten.
///
/// Rewrites inside the right section only, which is the whole reason this is not
/// a line-wise pass over the document any more: with two tables, a pass that
/// rewrote every `| blake3 |` row would stamp one blob's hash onto both and the
/// test would then pass while the provenance was wrong for one of them.
fn blessed(doc: &str, compiles: &str, hash: &str, size: usize) -> String {
  let mut out = String::new();
  let mut in_target = false;
  for (i, section) in doc.split("\n## ").enumerate() {
    if i > 0 {
      out.push_str("\n## ");
    }
    in_target = row(section, "compiles").as_deref() == Some(compiles);
    for (j, line) in section.split('\n').enumerate() {
      if j > 0 {
        out.push('\n');
      }
      if in_target && line.trim_start().starts_with("| blake3 |") {
        out.push_str(&format!("| blake3 | `{hash}` |"));
      } else if in_target && line.trim_start().starts_with("| size |") {
        out.push_str(&format!("| size | {size} bytes |"));
      } else {
        out.push_str(line);
      }
    }
  }
  let _ = in_target;
  out
}

#[test]
fn the_committed_seeds_match_what_seed_md_records() {
  let root = workspace_root();
  let doc_path = root.join("seed/SEED.md");
  let doc = std::fs::read_to_string(&doc_path).expect("should read seed/SEED.md");
  let found = tables(&doc);
  assert!(
    !found.is_empty(),
    "seed/SEED.md holds no identity table, so no blob's provenance is recorded at all"
  );

  let mut updated = doc.clone();
  let mut blessed_any = false;

  for table in &found {
    let name = format!("seed/voltaic-{}.wasm", table.compiles);
    let bytes = std::fs::read(root.join(&name)).unwrap_or_else(|_| {
      panic!(
        "{name} should be committed — seed/SEED.md has an identity table for \
         `{}`, and `moon run :seed-refresh` / `:seed-refresh-g2` builds it",
        table.compiles
      )
    });
    let hash = blake3::hash(&bytes).to_hex().to_string();

    if std::env::var("VOLTAIC_UPDATE").is_ok() {
      updated = blessed(&updated, &table.compiles, &hash, bytes.len());
      blessed_any = true;
      continue;
    }

    assert_eq!(
      table.size,
      Some(bytes.len()),
      "seed/SEED.md records a different size than {name} has"
    );
    assert_eq!(
      table.hash.as_deref(),
      Some(hash.as_str()),
      "{name} does not hash to what seed/SEED.md records. Either the blob was \
       replaced without updating, or the table is stale — and a provenance row that \
       disagrees with its bytes reads as verification while providing none."
    );
  }

  if blessed_any {
    std::fs::write(&doc_path, updated).expect("should write seed/SEED.md");
    panic!(
      "updated seed/SEED.md — a changed hash is a changed compiler, so review the diff \
       rather than absorbing it. Also update the commit row by hand."
    );
  }
}

/// Row 102 leaf 8: `SEED.md`'s provenance is complete and the invariant holds.
///
/// Doc 100 §1.2:
///
/// > A generation's seed must be built by the previous generation's compiler,
/// > before the previous generation's source is replaced. Equivalently: at no
/// > point is the only compiler for a language written in that language.
///
/// A blob built by itself is what that forbids, so the check is that `built by`
/// never names the generation in `compiles`. For `voltaic-g1.wasm` it names no
/// generation at all — the base case, produced by the committed Rust compiler
/// plan 72 then deleted. For `voltaic-g2.wasm` it names `g1`, which is the
/// invariant's ordinary case: the compiler in the tree reads generation 2 and is
/// written in generation 1, so it can produce generation 2's seed before
/// generation 1's source is replaced.
///
/// This asserts a *recorded* fact rather than a derived one, and that is the
/// honest shape: nothing in the repository can recompute which compiler built a
/// committed binary. What it can do is refuse a provenance table that states the
/// forbidden thing, and refuse one that states nothing at all — which is the
/// failure that matters, because an absent row reads as "fine" and a present
/// wrong one reads as verified.
#[test]
fn seed_md_records_a_provenance_the_invariant_allows() {
  let doc = std::fs::read_to_string(workspace_root().join("seed/SEED.md"))
    .expect("should read seed/SEED.md");
  let found = tables(&doc);
  assert!(
    !found.is_empty(),
    "seed/SEED.md has no table with a `compiles` row. Row 102 leaf 8 added it; without \
     one the invariant in doc 100 §1.2 is stated and unchecked."
  );

  for table in &found {
    assert!(
      table.compiles == "g1" || table.compiles == "g2",
      "seed/SEED.md records `compiles: {}`; expected g1 or g2",
      table.compiles
    );
    assert!(
      !table.built_by.is_empty(),
      "the `{}` table's `built by` row is empty, so it records no provenance at all",
      table.compiles
    );
    assert!(
      !table
        .built_by
        .split_whitespace()
        .any(|word| word.trim_matches(|c: char| !c.is_alphanumeric()) == table.compiles),
      "seed/SEED.md says a blob compiles `{}` and was built by `{}`, which names the \
       same generation. That is the one thing doc 100 §1.2 forbids: a seed built by \
       the compiler it is supposed to underwrite.",
      table.compiles,
      table.built_by
    );
  }
}

/// The generation column is the thing `bootstrap.sh` dispatches on, so a blob
/// some crate is marked for has to exist.
#[test]
fn every_generation_named_in_the_list_has_its_blob() {
  let root = workspace_root();
  let text =
    std::fs::read_to_string(root.join("seed/GENERATION")).expect("should read seed/GENERATION");
  let mut wanted: Vec<String> = text
    .lines()
    .map(str::trim)
    .filter(|l| !l.is_empty() && !l.starts_with('#'))
    .filter_map(|l| l.split_whitespace().nth(2).map(str::to_string))
    .collect();
  wanted.sort();
  wanted.dedup();
  assert!(
    !wanted.is_empty(),
    "seed/GENERATION names no generations, so bootstrap.sh has nothing to dispatch on"
  );
  for generation in wanted {
    let blob = root.join(format!("seed/voltaic-{generation}.wasm"));
    assert!(
      blob.exists(),
      "seed/GENERATION marks a crate as `{generation}` and {} does not exist, so \
       ./bootstrap.sh would fail on a clean clone",
      blob.display()
    );
  }
}

/// The other side of [`the_committed_seeds_match_what_seed_md_records`]: a blob
/// that nothing documents.
///
/// The size-and-hash test iterates the tables, so it is blind to a `.wasm` in
/// `seed/` with no table — which is exactly how an orphaned blob from an
/// abandoned generation would survive, tracked and ~1 MB, with nothing asserting
/// anything about it. This iterates the directory instead, so the pair is narrow
/// in two directions rather than one.
#[test]
fn every_committed_blob_has_a_provenance_table() {
  let root = workspace_root();
  let doc = std::fs::read_to_string(root.join("seed/SEED.md")).expect("should read seed/SEED.md");
  let documented: Vec<String> = tables(&doc)
    .into_iter()
    .map(|t| format!("voltaic-{}.wasm", t.compiles))
    .collect();

  for entry in std::fs::read_dir(root.join("seed")).expect("should read seed/") {
    let name = entry.expect("should read a seed/ entry").file_name();
    let name = name.to_string_lossy().to_string();
    if !name.ends_with(".wasm") {
      continue;
    }
    assert!(
      documented.contains(&name),
      "seed/{name} is committed and seed/SEED.md has no identity table for it, so \
       nothing asserts its size, its hash, or which compiler built it. Documented: {documented:?}"
    );
  }
}

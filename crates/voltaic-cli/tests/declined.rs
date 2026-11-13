//! `docs/DECLINED.md` against doc 03 §22.
//!
//! Plan 47 (`docs/41 - 47 - Status and Declines.md`) leaf 2. §22 is the source
//! and will gain rows; `DECLINED.md` restates them for a reader who has not
//! read a 2,000-line surface document. A decline recorded in only one of the
//! two is a decline nobody can find, so this asserts every §22 construct
//! appears in the document.
//!
//! It checks **inclusion, not equality**. `DECLINED.md` deliberately says more
//! than §22 does — it groups the declines by the admission test each one fails,
//! and it carries a "not declined, merely absent" section distinguishing a
//! decline from a gap, which is the distinction that makes the document worth
//! having beside `docs/GRAMMAR.md`.

use std::path::PathBuf;

fn workspace_root() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The construct named by each row of doc 03 §22's table.
fn declines_in_the_specification() -> Vec<String> {
  let text = std::fs::read_to_string(workspace_root().join("docs/03 - Voltaic Surface Design.md"))
    .expect("should read doc 03");

  let section = text
    .split("## 22. Declines")
    .nth(1)
    .expect("doc 03 should have a §22")
    .split("\n## ")
    .next()
    .expect("§22 should end");

  section
    .lines()
    .filter(|l| l.starts_with("| "))
    .filter_map(|l| l.trim().trim_matches('|').split('|').next())
    .map(str::trim)
    .filter(|c| !c.is_empty() && *c != "Construct" && !c.starts_with("---"))
    .map(str::to_owned)
    .collect()
}

#[test]
fn the_specification_still_has_declines_to_check() {
  let declines = declines_in_the_specification();
  assert!(
    declines.len() >= 15,
    "doc 03 §22 should hold the declines; parsed only {}: {declines:?}",
    declines.len()
  );
}

#[test]
fn every_declined_construct_is_documented() {
  let declined = std::fs::read_to_string(workspace_root().join("docs/DECLINED.md"))
    .expect("should read docs/DECLINED.md");

  let missing: Vec<String> = declines_in_the_specification()
    .into_iter()
    .filter(|construct| !declined.contains(construct.as_str()))
    .collect();

  assert!(
    missing.is_empty(),
    "doc 03 §22 declines these and docs/DECLINED.md does not mention them. A decline \
     recorded in one place only is a decline nobody can find:\n  {}",
    missing.join("\n  ")
  );
}

/// Directories with nothing to say about the charter, skipped for cost.
const SKIP_DIRS: &[&str] = &[
  ".git",
  ".tmp",
  ".devenv",
  ".direnv",
  "target",
  "result",
  "node_modules",
  "tmp",
  "log",
  "book",
];

/// Phrasings of the constraint plan 92 reversed.
///
/// Doc 02 §12.4 declined a Voltaic type checker and doc 03 §2 clause 1 made
/// type-freedom an admission test. `docs/90 - 92 - The Charter Reversal.md`
/// reverses both: what ships is an engine that **infers and prints**, and rustc
/// stays the only authority on borrows and exhaustiveness.
const RETIRED: &[&str] = &["no type checker", "has no checker of its own"];

/// Where a retired phrasing is still correct to find.
///
/// Three classes, and the third is a knowing inconsistency rather than an
/// oversight — doc 90 §6 discloses it:
///
/// - **`docs/01`–`docs/85`** are generation 1's record. Doc 04 rule 3 keeps a
///   superseded row rather than deleting it, and the same applies to a
///   superseded document: rewriting them would falsify what generation 1 did.
/// - **`docs/90 - 92`** is the reversal, which has to quote what it reverses,
///   and this file, which holds the phrasings as data.
/// - **`docs/DECLINED.md`** still carries the decline. It is gated by this
///   file's other two tests against doc 03 §22, and a gated reference document
///   cannot be rewritten ahead of the code it grades — row 104 owns it. Until
///   then the repository knowingly ships a decline list naming a decline that
///   has been lifted, which is better than an evidence-carrying document whose
///   evidence does not resolve.
fn is_historical_record(relative: &str) -> bool {
  if relative == "docs/90 - 92 - The Charter Reversal.md"
    || relative == "docs/DECLINED.md"
    || relative == "crates/voltaic-cli/tests/declined.rs"
    || relative.starts_with("tools/port/batches/")
  {
    return true;
  }

  // `docs/NN - …` with NN <= 85 is generation 1's numbered record. Generation 2
  // starts at 86, so the boundary is the number itself rather than a list.
  relative
    .strip_prefix("docs/")
    .and_then(|name| name.split(' ').next())
    .and_then(|number| number.parse::<u32>().ok())
    .is_some_and(|number| number <= 85)
}

/// Generated output, which asserts nothing — it only restates its source.
///
/// Flagging a generated file would duplicate `port.rs`, the gate that compares a
/// `.sdx` against the Rust it produces, and would fail on a tree whose artifacts
/// are merely stale rather than whose documents disagree. It found exactly that
/// when plan 92 landed: `server.sdx` was amended and its `src/server.rs` and
/// transient `sdx/server.xrs` still carried the old string, which is a
/// regeneration to run and not a charter to fix.
///
/// Both rules are `AGENTS.md`'s: every crate under `crates/` is written in
/// Voltaic, so the `.rs` under its `src/` is generated and `.gitignore`d, and
/// `voltaic build` drops a transient `.xrs` beside each `.sdx` while the port
/// harness reads it.
fn is_generated(relative: &str) -> bool {
  let generated_tree = relative.starts_with("crates/")
    && relative
      .split('/')
      .nth(2)
      .is_some_and(|directory| directory == "src");

  generated_tree || relative.ends_with(".xrs") || relative.ends_with(".xrs.map")
}

fn files_under(root: &std::path::Path, prefix: &str, found: &mut Vec<(String, String)>) {
  let Ok(entries) = std::fs::read_dir(root) else {
    return;
  };

  for entry in entries.flatten() {
    let name = entry.file_name().to_string_lossy().into_owned();
    let relative = if prefix.is_empty() {
      name.clone()
    } else {
      format!("{prefix}/{name}")
    };

    if entry.file_type().is_ok_and(|kind| kind.is_dir()) {
      if !SKIP_DIRS.contains(&name.as_str()) {
        files_under(&entry.path(), &relative, found);
      }
      continue;
    }

    if is_historical_record(&relative) || is_generated(&relative) {
      continue;
    }

    let Ok(text) = std::fs::read_to_string(entry.path()) else {
      continue; // a binary, or not UTF-8; neither can assert a charter
    };

    for phrasing in RETIRED {
      if text.contains(phrasing) {
        found.push((relative.clone(), (*phrasing).to_owned()));
      }
    }
  }
}

/// Plan 92's gate: the tree does not assert both charters.
///
/// The failure mode of a documentation reversal is a **half-applied** one — a
/// repository that states the new rule in one file and the old rule in nine
/// others, where each of the nine reads as authoritative on its own. Plan 92
/// §2.1 listed seven locations and the sweep found twelve; that gap is exactly
/// why this is a test and not a checklist.
#[test]
fn no_living_document_asserts_the_retired_charter() {
  let mut found = Vec::new();
  files_under(&workspace_root(), "", &mut found);

  let report: Vec<String> = found
    .iter()
    .map(|(path, phrasing)| format!("{path}: {phrasing:?}"))
    .collect();

  assert!(
    report.is_empty(),
    "plan 92 reversed the no-type-checker charter, and these still assert it. A \
     reversal applied to some of its citations and not the rest leaves every one \
     of them reading as authoritative:\n  {}\n\nIf the phrasing is a deliberate \
     quotation of what was reversed, add the path to `is_historical_record`, with \
     the reason.",
    report.join("\n  ")
  );
}

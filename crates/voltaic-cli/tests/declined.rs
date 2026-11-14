//! `docs/DECLINED.md` against doc 03 §22 **and** doc 87 §22.
//!
//! Plan 47 leaf 2 built this against doc 03 §22; row 104 re-points it, and doc
//! 87 §22 names that as this row's job in its own opening paragraph. §22 is the
//! source and will gain rows; `DECLINED.md` restates them for a reader who has
//! not read a 2,000-line surface document. A decline recorded in only one of the
//! two is a decline nobody can find, so this asserts every §22 construct appears
//! in the document.
//!
//! # Both sections, not the newer one
//!
//! Doc 87 §22.3 and §22.4 carry generation 1's reasons forward *"in substance"*
//! rather than verbatim, and §22.5 **lifts two rows** — `UFCS` and
//! *a Voltaic type checker* — by the charter reversal. So doc 03 §22 is still the
//! only place several of generation 1's declines are stated in full, and
//! re-pointing *away* from it would silently stop asserting them. Checking both
//! is the only reading under which no decline loses its gate.
//!
//! [`LIFTED`] names the two rows §22.5 lifted. It is **not** a parser exception:
//! §22.5 states both as prose blockquotes rather than table rows, so nothing has
//! to be excused — and [`the_lifted_rows_are_stated_as_prose`] keeps it that way.
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

/// Rows doc 87 §22.5 **lifted**, which are therefore no longer declined.
///
/// Both were lifted by the charter reversal rather than by new machinery, which
/// is §22.5's own point: `UFCS` was declined because it *"requires type knowledge
/// to choose a shape"* and the reversal supplies the type knowledge; a Voltaic
/// type checker was declined as the project's load-bearing constraint, and what
/// ships is a solver that infers and prints rather than a checker that
/// second-guesses rustc.
///
/// This list is **not** an exception the parser applies. §22.5 states both as
/// prose blockquotes rather than table rows, so the table parser never sees them
/// and nothing has to be removed — the document's own structure draws the line.
/// [`the_lifted_rows_are_stated_as_prose`] is what keeps it drawn.
const LIFTED: &[&str] = &["UFCS", "A Voltaic type checker"];

/// Every declined construct from both generations' §22.
fn declines_in_the_specification() -> Vec<String> {
  let mut all = declines_in("docs/03 - Voltaic Surface Design.md", "## 22. Declines");
  all.extend(declines_in(
    "docs/87 - Voltaic 2 Surface Design.md",
    "## 22. Declined",
  ));
  all.sort();
  all.dedup();
  all
}

/// The construct named by each row of a `## 22.` section's tables.
fn declines_in(doc: &str, heading: &str) -> Vec<String> {
  let text =
    std::fs::read_to_string(workspace_root().join(doc)).unwrap_or_else(|e| panic!("{doc}: {e}"));

  let section = text
    .split(heading)
    .nth(1)
    .unwrap_or_else(|| panic!("{doc} should have a section headed `{heading}`"))
    .split("\n## ")
    .next()
    .expect("the section should end");

  section
    .lines()
    .filter(|l| l.starts_with("| "))
    .filter_map(|l| l.trim().trim_matches('|').split('|').next())
    .map(str::trim)
    .filter(|c| !c.is_empty() && *c != "Construct" && !c.starts_with("---"))
    .map(str::to_owned)
    .collect()
}

/// The two sections together, as a floor.
///
/// It was 15 when only doc 03 §22 was read. Row 104 added doc 87 §22, and a
/// floor that did not move with it would have let the newer section parse as
/// *empty* while the gate stayed green on generation 1's rows alone — which is
/// the same narrowing [`crate`]'s sibling gates keep catching.
#[test]
fn the_specification_still_has_declines_to_check() {
  let declines = declines_in_the_specification();
  assert!(
    declines.len() >= 24,
    "the two §22 sections should hold the declines; parsed only {}: {declines:?}",
    declines.len()
  );
}

/// Each section carries rows on its own, so neither can go dark behind the
/// other's count.
#[test]
fn each_specification_contributes_declines() {
  let first = declines_in("docs/03 - Voltaic Surface Design.md", "## 22. Declines");
  let second = declines_in("docs/87 - Voltaic 2 Surface Design.md", "## 22. Declined");
  assert!(
    first.len() >= 15,
    "doc 03 §22 parsed only {}: {first:?}",
    first.len()
  );
  assert!(
    second.len() >= 14,
    "doc 87 §22 parsed only {}: {second:?}",
    second.len()
  );
}

/// A lifted row must stay prose, because a table row reads as a decline.
///
/// If a later edit moves §22.5's two blockquotes into a table, this gate would
/// start demanding that `DECLINED.md` list `UFCS` and a Voltaic type checker —
/// two things that **ship**. The failure would read as a missing document entry
/// rather than as a mis-filed row, so it is worth catching at the structure.
#[test]
fn the_lifted_rows_are_stated_as_prose() {
  let text =
    std::fs::read_to_string(workspace_root().join("docs/87 - Voltaic 2 Surface Design.md"))
      .expect("should read doc 87");
  let tabled = declines_in("docs/87 - Voltaic 2 Surface Design.md", "## 22. Declined");
  for lifted in LIFTED {
    assert!(
      text.contains(lifted),
      "doc 87 §22.5 should still record `{lifted}` as lifted, with its evidence"
    );
    assert!(
      !tabled.iter().any(|c| c == lifted),
      "`{lifted}` is lifted and now appears as a §22 **table row**, which reads as a \
       decline. The gate would start requiring docs/DECLINED.md to list a construct \
       that ships. Keep §22.5's rows as prose."
    );
  }
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
/// Two classes, and there used to be three:
///
/// - **`docs/01`–`docs/85`** are generation 1's record. Doc 04 rule 3 keeps a
///   superseded row rather than deleting it, and the same applies to a
///   superseded document: rewriting them would falsify what generation 1 did.
/// - **`docs/90 - 92`** is the reversal, which has to quote what it reverses,
///   and this file, which holds the phrasings as data.
///
/// `docs/DECLINED.md` was the third, with this note beside it:
///
/// > Until then the repository knowingly ships a decline list naming a decline
/// > that has been lifted, which is better than an evidence-carrying document
/// > whose evidence does not resolve.
///
/// **Row 104 is the "then".** It rewrote the document against doc 87 §22, neither
/// retired phrasing survives, and the exemption now excuses nothing — so it is
/// gone. A disclosed inconsistency is still an inconsistency; removing the
/// exemption is what keeps it closed rather than merely admitted.
fn is_historical_record(relative: &str) -> bool {
  if relative == "docs/90 - 92 - The Charter Reversal.md"
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
/// `.vlt` against the Rust it produces, and would fail on a tree whose artifacts
/// are merely stale rather than whose documents disagree. It found exactly that
/// when plan 92 landed: `server.vlt` was amended and its `src/server.rs` and
/// transient `vlt/server.vrs` still carried the old string, which is a
/// regeneration to run and not a charter to fix.
///
/// Both rules are `AGENTS.md`'s: every crate under `crates/` is written in
/// Voltaic, so the `.rs` under its `src/` is generated and `.gitignore`d, and
/// `voltaic build` drops a transient `.vrs` beside each `.vlt` while the port
/// harness reads it.
fn is_generated(relative: &str) -> bool {
  let generated_tree = relative.starts_with("crates/")
    && relative
      .split('/')
      .nth(2)
      .is_some_and(|directory| directory == "src");

  generated_tree || relative.ends_with(".vrs") || relative.ends_with(".vrs.map")
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

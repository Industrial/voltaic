//! Row 99 wave 1's deletion half: a signature line that inference can supply
//! is removed from the compiler's own source.
//!
//! Doc 97 §1.3 deletion 1 — *standalone signature lines → inference* — and row
//! 94 made them optional. Wave 1 shipped the optionality and the answer key;
//! the deletion from `crates/*/vlt/**` is row 103's, because row 99's
//! criterion 8 reserves the compiler's source for the migration.
//!
//! # "Optional" is measured, not reasoned about
//!
//! Two plausible definitions are both wrong.
//!
//! **Not visibility.** `voltaic-lattice`'s `unwritten` demands a signature only
//! on a `published` definition, so private ones look optional — 181 of the
//! 1,002. But removing `gather : &Path -> &mut Vec<PathBuf> -> ()`, a private
//! signature in `voltaic-driver/vlt/resolve.vlt`, is `VLT0128` *"cannot infer a
//! signature for `gather`"*. Two unannotated parameters used only through `std`
//! calls give the solver nothing to work from, exactly as
//! `sidecar.vlt`'s *"a field access on an unannotated binder does not determine
//! the receiver"* predicts. Visibility says who may depend on the signature;
//! inferability says whether it can be dropped, and they are unrelated.
//!
//! **Not "it still compiles" either.** A signature can be dropped, the file
//! still compile, and the *generated Rust differ* — a different inferred mode
//! or effect row reaches the output. That is a changed program, not a deleted
//! annotation.
//!
//! So the test is leaf 3's identity gate, the same one row 103's `|>` migration
//! used: remove the line, transpile, and keep the removal **only if the
//! generated Rust is byte-identical**. A signature that fails stays, and is
//! reported with its reason.

use std::path::{Path, PathBuf};

use voltaic_driver::gate;

mod support;

fn repo_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .and_then(Path::parent)
    .expect("the crate sits two levels below the repository root")
    .to_path_buf()
}

/// Every `.vlt` the compiler is written in, from `seed/GENERATION`.
fn sources() -> Vec<PathBuf> {
  let root = repo_root();
  let mut found = Vec::new();
  for (name, _, _) in support::generations() {
    collect(&root.join("crates").join(name).join("vlt"), &mut found);
  }
  found.sort();
  found
}

fn collect(dir: &Path, into: &mut Vec<PathBuf>) {
  let Ok(entries) = std::fs::read_dir(dir) else {
    return;
  };
  for entry in entries.flatten() {
    let path = entry.path();
    if path.is_dir() {
      collect(&path, into);
    } else if path.extension().is_some_and(|e| e == "vlt") {
      into.push(path);
    }
  }
}

/// The 0-based line numbers that are a standalone signature line.
///
/// A signature is `name : type`, optionally `export`ed, whose next non-blank
/// line opens a definition of the same name. Matching the *pair* is what keeps
/// this off a record field (`export code Code`, no `:` since wave 5) and off an
/// ascription inside a body (indented, and its next line does not redefine it).
fn signature_lines(text: &str) -> Vec<usize> {
  let lines: Vec<&str> = text.lines().collect();
  let mut found = Vec::new();
  for (i, line) in lines.iter().enumerate() {
    let bare = line.trim_start();
    let body = bare
      .strip_prefix("export ")
      .or_else(|| {
        bare
          .strip_prefix("export(")
          .and_then(|rest| rest.split_once(") "))
          .map(|(_, rest)| rest)
      })
      .unwrap_or(bare);
    let Some((name, _)) = body.split_once(" : ") else {
      continue;
    };
    if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
      continue;
    }
    if !name.starts_with(|c: char| c.is_lowercase() || c == '_') {
      continue;
    }
    let defines_it = lines[i + 1..]
      .iter()
      .find(|l| !l.trim().is_empty())
      .is_some_and(|next| {
        let next = next.trim_start();
        next
          .strip_prefix(name)
          .is_some_and(|rest| rest.starts_with(' ') || rest.starts_with('\n') || rest.is_empty())
      });
    if defines_it {
      found.push(i);
    }
  }
  found
}

fn without_line(text: &str, drop: usize) -> String {
  let mut out = String::with_capacity(text.len());
  for (i, line) in text.lines().enumerate() {
    if i == drop {
      continue;
    }
    out.push_str(line);
    out.push('\n');
  }
  out
}

/// A signature line is removable when the file without it transpiles to
/// byte-identical Rust.
///
/// Greedy and re-measured after each removal, because signatures feed each
/// other's inference: dropping one can make the next un-inferable, and dropping
/// it can equally *free* the next. A single pass that decided every line
/// against the original text would be wrong in both directions.
fn strip(path: &Path, original: &str) -> (String, Vec<String>) {
  let mut current = original.to_string();
  let mut kept = Vec::new();
  loop {
    let candidates = signature_lines(&current);
    let mut removed_one = false;
    for line in candidates {
      let shorter = without_line(&current, line);
      let text = current
        .lines()
        .nth(line)
        .unwrap_or_default()
        .trim()
        .to_string();
      if kept.iter().any(|k: &String| k.starts_with(&text)) {
        continue;
      }
      let verdict = gate(path, &current, &shorter);
      if verdict.passed() {
        current = shorter;
        removed_one = true;
        break;
      }
      // Classified on the gate's own verdict, not by searching its text for a
      // diagnostic code: `Gate::Unreadable`'s `explain` reports *"does not
      // compile: N diagnostics"* and never names the code, so a first attempt
      // that grepped for `VLT0101` put 990 of 997 in an "other" bucket labelled
      // "the generated Rust differed" — which was true of almost none of them.
      let why = verdict.explain();
      let kind = if why.contains("does not compile") {
        "needs it to compile at all"
      } else if why.contains("differs at line") {
        "compiles without it, but the Rust changes"
      } else {
        "other"
      };
      kept.push(format!("{text}  [{kind}]"));
    }
    if !removed_one {
      return (current, kept);
    }
  }
}

/// How many of the compiler's 1,002 signature lines inference can supply.
///
/// Reported rather than asserted at a number, because the number is the
/// finding. What is asserted is that the pass is doing something and that
/// nothing it writes changes a program.
#[test]
fn every_removable_signature_is_measured() {
  let updating = std::env::var("VOLTAIC_UPDATE").is_ok();
  let mut total = 0usize;
  let mut removable = 0usize;
  let mut load_bearing: Vec<String> = Vec::new();
  let mut written = Vec::new();

  for path in sources() {
    let original = std::fs::read_to_string(&path).expect("a listed source should be readable");
    let here = signature_lines(&original).len();
    if here == 0 {
      continue;
    }
    total += here;
    let (stripped, kept) = strip(&path, &original);
    let gone = here - signature_lines(&stripped).len();
    removable += gone;
    for k in kept {
      load_bearing.push(format!("{}: {k}", path.display()));
    }
    if updating && stripped != original {
      std::fs::write(&path, &stripped).expect("should write the stripped source");
      written.push(format!("{} (-{gone})", path.display()));
    }
  }

  println!("signature lines:   {total}");
  println!("removable:         {removable}");
  println!("load-bearing:      {}", total - removable);
  let mut by_code: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
  for k in &load_bearing {
    let code = if k.contains("[needs it to compile at all]") {
      "needs it to compile at all"
    } else if k.contains("[compiles without it, but the Rust changes]") {
      "compiles without it, but the Rust changes"
    } else {
      "other"
    };
    *by_code.entry(code).or_default() += 1;
  }
  for (code, n) in &by_code {
    println!("  {n:>4}  {code}");
  }
  println!("the removable ones:");
  for r in &written {
    println!("  {r}");
  }

  assert!(
    total > 900,
    "the compiler carried 1,002 signature lines when this was written; found \
     {total}, so the pair-matching above has stopped recognising them"
  );
  assert!(
    written.is_empty(),
    "removed {} files' worth of signatures, each gated byte-identical before \
     writing. Review the `.vlt` diff and re-run without VOLTAIC_UPDATE:\n{written:#?}",
    written.len()
  );
}

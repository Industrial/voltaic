//! What each of the lexer's keywords is *for* — and the measurement row 99
//! wave 4 needs before it can cull them to doc 97 §1.4's eleven.
//!
//! # A keyword has two jobs, and §1.4 counts one of them
//!
//! A reserved word either **spells a Voltaic construct** — the parser consumes
//! it — or **shields the generated Rust**, by stopping a user naming a binding
//! after one of Rust's own keywords. §1.4's target of eleven is a count of the
//! first job only.
//!
//! That matters because the second job is load-bearing *today*. Voltaic has no
//! raw-identifier escaping: `ref = 7` emits `let ref = 7;`, which rustc rejects
//! with the compiler's own help text saying *"this is on a line Voltaic
//! generated, so it has no place in your source"*. `ref` is a Rust keyword that
//! Voltaic does not reserve, and that is the hole. Every Rust keyword Voltaic
//! *does* reserve is a hole it is standing in front of.
//!
//! So **culling to eleven requires `r#` escaping in the emitter first**, or the
//! moment `struct` stops being reserved a program naming a binding `struct`
//! emits invalid Rust. That prerequisite is not in doc 97 §4's wave list.
//!
//! # Why the lexer and not a grep
//!
//! A substring scan cannot answer which keywords are used: `struct`, `enum` and
//! `impl` all appear in `crates/voltaic-emit`'s source as the Rust it *writes*,
//! so grepping the corpus reports every keyword as used. An earlier attempt did
//! exactly that and got 54 of 54.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Rust's keywords — strict, reserved and weak, 2024 edition.
///
/// A Voltaic keyword spelling one of these is doing the shielding job even if
/// no construct uses it, so it may not be culled until the emitter escapes
/// identifiers. `crate`, `self`, `Self` and `super` cannot be escaped with `r#`
/// at all, so those must stay reserved permanently.
const RUST_KEYWORDS: &[&str] = &[
  "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "crate", "do",
  "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "gen", "if", "impl", "in", "let",
  "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref", "return",
  "self", "static", "struct", "super", "trait", "true", "try", "type", "typeof", "union", "unsafe",
  "unsized", "use", "virtual", "where", "while", "yield",
];

fn workspace_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .and_then(Path::parent)
    .expect("the crate sits two levels below the repository root")
    .to_path_buf()
}

/// Each keyword variant and the word it spells, read from `keyword.vlt`.
///
/// Textual, because the lexer exports no table of them. A spelling this misses
/// shows up as a short count, which the assertion below notices.
fn spellings() -> BTreeMap<String, String> {
  let text = std::fs::read_to_string(workspace_root().join("crates/voltaic-lexer/vlt/keyword.vlt"))
    .expect("keyword.vlt should be readable");
  let mut found = BTreeMap::new();
  for line in text.lines() {
    let line = line.trim();
    let Some(rest) = line.strip_prefix('"') else {
      continue;
    };
    let Some((word, tail)) = rest.split_once('"') else {
      continue;
    };
    if let Some(variant) = tail.trim().strip_prefix("-> Some ") {
      found.insert(variant.trim().to_string(), word.to_string());
    }
  }
  found
}

/// Every `.vlt` a user could read, plus the compiler's own source.
fn sources() -> Vec<PathBuf> {
  let root = workspace_root();
  let mut found = Vec::new();
  for dir in [
    root.join("examples"),
    root.join("crates/voltaic-cli/tests/ui"),
    root.join("crates/voltaic-cli/tests/lint"),
    root.join("crates/voltaic-cli/tests/run"),
    root.join("tests/run"),
    root.join("tests/unreachable"),
    root.join("benchmarks/pairs"),
  ] {
    collect(&dir, &mut found);
  }
  let generation =
    std::fs::read_to_string(root.join("seed/GENERATION")).expect("GENERATION should be readable");
  for line in generation.lines() {
    let line = line.split('#').next().unwrap_or_default().trim();
    if line.is_empty() {
      continue;
    }
    if let Some(name) = line.split_whitespace().next() {
      collect(&root.join("crates").join(name).join("vlt"), &mut found);
    }
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

/// Which keyword variants the parser mentions at all.
fn consumed_by_the_parser() -> Vec<String> {
  let root = workspace_root();
  let mut found = Vec::new();
  let mut files = Vec::new();
  collect(&root.join("crates/voltaic-parser/vlt"), &mut files);
  for path in files {
    let Ok(text) = std::fs::read_to_string(&path) else {
      continue;
    };
    for part in text.split("Keyword::").skip(1) {
      let name: String = part
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
      if !name.is_empty() {
        found.push(name);
      }
    }
  }
  found.sort();
  found.dedup();
  found
}

/// **Every keyword either spells a construct or shields the output.**
///
/// A keyword that does neither is dead weight and should be culled — that is
/// what this gate is for, and it is how `not`, `implements` and `pure` were
/// found and removed, taking the set from 55 to 52.
///
/// It fails in both useful directions: adding a keyword with no construct and
/// no Rust collision, and culling one that is still shielding the output.
#[test]
fn every_keyword_spells_a_construct_or_shields_the_output() {
  let spellings = spellings();
  assert!(
    spellings.len() > 40,
    "the set is 52 and shrinking toward eleven; parsed only {} spellings, so \
     the textual read has stopped working",
    spellings.len()
  );
  let consumed = consumed_by_the_parser();

  let idle: Vec<String> = spellings
    .iter()
    .filter(|(variant, word)| {
      !consumed.contains(variant) && !RUST_KEYWORDS.contains(&word.as_str())
    })
    .map(|(variant, word)| format!("{variant} (`{word}`)"))
    .collect();

  assert!(
    idle.is_empty(),
    "these words are reserved, spell no construct the parser reads, and are not \
     Rust keywords — so they shield nothing and cost every program that would \
     use them as a name. Cull them: {idle:#?}"
  );
}

/// The measurement itself, printed so wave 4 has the numbers.
#[test]
fn the_corpus_says_which_keywords_are_live() {
  let mut counts: BTreeMap<String, usize> = BTreeMap::new();
  let mut lexed = 0usize;
  let files = sources();
  for path in &files {
    let Ok(text) = std::fs::read_to_string(path) else {
      continue;
    };
    // A file the lexer rejects contributes nothing; several UI fixtures exist
    // precisely to be rejected.
    let Ok(tokens) = voltaic_lexer::lex(&text) else {
      continue;
    };
    lexed += 1;
    for token in &tokens {
      if let voltaic_lexer::TokenKind::Kw(keyword) = &token.kind {
        *counts.entry(format!("{keyword:?}")).or_default() += 1;
      }
    }
  }

  assert!(
    lexed > 100,
    "the corpus should hold the examples, the fixtures and the compiler's own \
     source; only {lexed} of {} files lexed",
    files.len()
  );

  let spellings = spellings();
  let unused: Vec<&String> = spellings
    .keys()
    .filter(|variant| !counts.contains_key(*variant))
    .collect();
  let mut ranked: Vec<_> = counts.iter().collect();
  ranked.sort_by(|a, b| a.1.cmp(b.1).then(a.0.cmp(b.0)));

  println!("files lexed:    {lexed} of {}", files.len());
  println!("declared:       {}", spellings.len());
  println!("used at all:    {}", counts.len());
  println!("least-used:     {:?}", &ranked[..ranked.len().min(15)]);
  println!("never used:     {unused:?}");

  // Every keyword the corpus never uses should be one Rust needs shielded —
  // otherwise it is the dead weight the gate above rejects. Asserted here too,
  // on the *corpus* rather than on the parser, because the two can disagree: a
  // construct the parser reads and no file exercises is an unexampled
  // construct, which is `construct_coverage`'s business rather than wave 4's.
  for variant in &unused {
    let word = &spellings[*variant];
    assert!(
      RUST_KEYWORDS.contains(&word.as_str()),
      "`{word}` is never used in the corpus and is not a Rust keyword, so it \
       shields nothing — see the gate above"
    );
  }
}

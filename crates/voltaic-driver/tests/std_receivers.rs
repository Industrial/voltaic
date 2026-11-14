//! What the solver actually resolves for a `std` receiver.
//!
//! Wave 3's no-argument half needs `xs.len` on a `Vec` to be classified as a
//! method, and `voltaic-lattice` has no declaration for `Vec`. A sidecar
//! wildcard (`Vec.* method`) can supply one, but only for receivers the solver
//! resolves to a named type — doc 97 §10.36's restriction means an unresolved
//! receiver claims nothing, catalogue or no catalogue.
//!
//! So this file measures the premise before anything is built on it. It is kept
//! rather than deleted because the numbers are the argument: the ceiling on any
//! `std` catalogue is *how many member obligations resolve at all*, and that is
//! a property of the corpus, not of the catalogue.

use std::collections::BTreeMap;

use voltaic_lattice::{Obligation, owner_of, solve_with, walk};

/// `receiver_owner` for every member obligation in a program, paired with the
/// member name, so a test can assert on what resolved rather than on a count.
fn owners(source: &str) -> Vec<(String, Option<String>)> {
  let program = voltaic_parser::parse_source(source).expect("should parse");
  let found = walk(&program);
  let outcome = solve_with(&found.store, &found.known);
  let mut seen = Vec::new();
  for held in found.store.all() {
    if let Obligation::Member { recv, name, .. } = &held.obligation {
      seen.push((name.clone(), owner_of(&outcome.resolved(recv))));
    }
  }
  seen
}

#[test]
fn a_written_signature_resolves_its_std_receiver() {
  let seen = owners("f : &Vec<u64> -> usize\nf xs = xs.len\n");
  assert_eq!(
    seen,
    vec![("len".to_string(), Some("Vec".to_string()))],
    "a written `&Vec<u64>` is what a wildcard needs: the `&` is stripped and \
     the head is the catalogue's key"
  );
}

#[test]
fn a_written_string_receiver_resolves_too() {
  let seen = owners("f : &String -> usize\nf s = s.len\n");
  assert_eq!(seen, vec![("len".to_string(), Some("String".to_string()))]);
}

/// One `let` between the signature and the use loses the type. This is the
/// first half of why a catalogue's reach is narrow: the written case is not
/// what the corpus is made of.
#[test]
fn one_binding_between_the_signature_and_the_use_loses_the_type() {
  let seen = owners("f : &Vec<u64> -> usize\nf xs =\n  held = xs\n  held.len\n");
  assert_eq!(
    seen,
    vec![("len".to_string(), None)],
    "a binder gets a fresh hole and the annotation does not flow into it"
  );
}

/// The second half, and the sharper one: a receiver-first stage raises **no
/// member obligation at all**, so there is no question for a catalogue to
/// answer. `.iter xs` is already an application, not a field access.
#[test]
fn a_receiver_first_stage_asks_no_member_question() {
  assert!(
    owners("f : &Vec<u64> -> usize\nf xs = .count (.iter xs)\n").is_empty(),
    "the pipeline pass's blocker is not that Lattice lacks `Vec` — it is that \
     this shape never reaches the member solver"
  );
}

/// The ceiling on any `std` catalogue, measured over the compiler's own source:
/// of every member obligation in the corpus, how many resolve to a named owner
/// at all, and how many of those name something `std` rather than a local type?
///
/// A catalogue cannot beat the first number, and can only collect the second.
#[test]
fn the_corpus_ceiling_on_a_std_catalogue() {
  let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .expect("crates/")
    .parent()
    .expect("workspace root")
    .to_path_buf();

  let mut sources = Vec::new();
  let generation = std::fs::read_to_string(root.join("seed/GENERATION"))
    .expect("seed/GENERATION should be readable");
  for line in generation.lines() {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
      continue;
    }
    let Some(crate_name) = line.split_whitespace().next() else {
      continue;
    };
    collect_vlt(
      &root.join("crates").join(crate_name).join("vlt"),
      &mut sources,
    );
  }
  sources.sort();
  assert!(!sources.is_empty(), "the corpus should not be empty");

  let mut total = 0usize;
  let mut resolved = 0usize;
  let mut by_owner: BTreeMap<String, usize> = BTreeMap::new();
  let mut parsed = 0usize;
  for path in &sources {
    let Ok(text) = std::fs::read_to_string(path) else {
      continue;
    };
    let Ok(program) = voltaic_parser::parse_source(&text) else {
      continue;
    };
    parsed += 1;
    let found = walk(&program);
    let outcome = solve_with(&found.store, &found.known);
    for held in found.store.all() {
      if let Obligation::Member { recv, .. } = &held.obligation {
        total += 1;
        if let Some(owner) = owner_of(&outcome.resolved(recv)) {
          resolved += 1;
          *by_owner.entry(owner).or_default() += 1;
        }
      }
    }
  }

  let mut ranked: Vec<_> = by_owner.iter().collect();
  ranked.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
  println!("files parsed:          {parsed} of {}", sources.len());
  println!("member obligations:    {total}");
  println!("with a resolved owner: {resolved}");
  println!(
    "owners, most first:    {:?}",
    &ranked[..ranked.len().min(25)]
  );
}

fn collect_vlt(dir: &std::path::Path, into: &mut Vec<std::path::PathBuf>) {
  let Ok(entries) = std::fs::read_dir(dir) else {
    return;
  };
  for found in entries.flatten() {
    let path = found.path();
    if path.is_dir() {
      collect_vlt(&path, into);
    } else if path.extension().is_some_and(|e| e == "vlt") {
      into.push(path);
    }
  }
}

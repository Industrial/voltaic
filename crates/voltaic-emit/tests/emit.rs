//! Behaviour of [`voltaic_emit::emit`], through the public API only.
//!
//! An integration test rather than a `#[cfg(test)]` module, per
//! `docs/06 - Voltaic Code Architecture.md` §7.

use voltaic_emit::emit;
use voltaic_parser::parse_source;

fn vrs(src: &str) -> String {
  emit(&parse_source(src).expect("should parse"))
}

/// Plan 01 §3's concrete proof, at the unit level.
#[test]
fn emits_the_hello_slice() {
  assert_eq!(
    vrs("-- The smallest Voltaic program.\nmain : ()\nmain =\n  println! \\-> \"hello\"\n  ()\n"),
    "// The smallest Voltaic program.\nfn main() {\n    println!(\"hello\");\n}\n"
  );
}

/// Plan 01 acceptance criterion 5.
#[test]
fn a_comment_survives_into_the_generated_rust() {
  let out = vrs("-- kept\nmain : ()\nmain =\n  println! \\-> \"x\"\n  ()\n");
  assert!(out.starts_with("// kept\n"), "{out}");
}

#[test]
fn an_inline_comment_survives_into_the_generated_rust() {
  let out = vrs("main : ()\nmain =\n  println! \\-> \"x\" -- why\n  ()\n");
  assert!(out.contains("println!(\"x\"); // why\n"), "{out}");
}

#[test]
fn a_comment_at_end_of_file_survives() {
  let out = vrs("main : ()\nmain =\n  println! \\-> \"x\"\n  ()\n-- the end\n");
  assert!(out.ends_with("// the end\n"), "{out}");
}

/// Plan 01 acceptance criterion 7: emission is a pure function of the tree.
#[test]
fn emission_is_deterministic() {
  let src = "-- a\nmain : ()\nmain =\n  println! \\-> \"x\"\n  ()\n\nother : ()\nother =\n  println! \\-> \"y\"\n  ()\n";
  assert_eq!(vrs(src), vrs(src));
}

#[test]
fn separates_items_with_one_blank_line() {
  assert_eq!(
    vrs("a : ()\na =\n  println! \\-> \"1\"\n  ()\nb : ()\nb =\n  println! \\-> \"2\"\n  ()\n"),
    "fn a() {\n    println!(\"1\");\n}\n\nfn b() {\n    println!(\"2\");\n}\n"
  );
}

#[test]
fn emits_four_space_indentation() {
  let out = vrs("main : ()\nmain =\n  println! \\-> \"x\"\n  ()\n");
  assert!(out.contains("\n    println!"), "{out}");
}

#[test]
fn a_non_print_call_keeps_its_name() {
  assert!(vrs("main : ()\nmain =\n  log \"x\"\n  ()\n").contains("log(\"x\");"));
}

/// Regression: the fixture's own final newline used to append a stray blank
/// line to every generated file. Found by the end-to-end test comparing
/// against plan 01 §3's stated output, not by inspection.
#[test]
fn ends_with_exactly_one_newline() {
  for src in [
    "main : ()\nmain =\n  println! \\-> \"x\"\n  ()\n",
    "main : ()\nmain =\n  println! \\-> \"x\"\n  ()",
    "main : ()\nmain =\n  println! \\-> \"x\"\n  ()\n\n\n",
    "main : ()\nmain =\n  println! \\-> \"x\"\n  ()\n-- tail\n",
  ] {
    let out = vrs(src);
    assert!(out.ends_with('\n'), "{out:?}");
    assert!(!out.ends_with("\n\n"), "{out:?}");
  }
}

/// Line-preserving discipline: source lines 1-3 map to output lines 1-3.
#[test]
fn preserves_line_positions_for_the_slice() {
  let out = vrs("-- one\nmain : ()\nmain =\n  println! \\-> \"three\"\n  ()\n");
  let lines: Vec<&str> = out.lines().collect();
  assert_eq!(lines[0], "// one");
  assert_eq!(lines[1], "fn main() {");
  assert_eq!(lines[2], "    println!(\"three\");");
}

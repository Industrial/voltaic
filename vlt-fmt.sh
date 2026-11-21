#!/usr/bin/env bash
# Format — or `--check` — every `.vlt` that is meant to be canonical.
#
# Plan 73. `voltaic fmt` existed from plan 34 and was in **no** gate: `moon run
# :format` is treefmt, which has no `.vlt` formatter at all, and the only
# `fmt --check` among the moon tasks is `cargo fmt --check` for Rust. So the
# project's own formatter had never been run over the project's own `.vlt`
# except by hand. Two things followed: `crates/voltaic-ast/vlt` had 50 of 53
# files non-canonical, and a formatter bug that **deleted a member's
# visibility** survived unnoticed (plan 73 §10).
#
# **One definition of the corpus, used twice.** `moon run :vlt-format` and
# `:vlt-format-check` both call this script, because two task bodies would be
# two exclusion lists free to drift — the one-predicate-two-positions defect
# plan 34 §10.31 found five times and plans 61, 69 and 70 each found again.
#
#     ./vlt-fmt.sh            # format in place
#     ./vlt-fmt.sh --check    # fail if anything is not canonical
set -euo pipefail

cd "$(dirname "$0")"

# **What is excluded, and why.** Measured rather than assumed: every tracked
# `.vlt` was run through `fmt --check` and classified, which gave 181
# canonical, 59 that do not parse, and 5 that parse but are deliberately
# unformatted. Every one of the 64 falls in this list, and the list contains
# nothing else.
#
# - `crates/voltaic-cli/tests/ui/` (48) — diagnostics fixtures. Most are
#   *supposed* not to parse; that is what their `.stderr` records.
# - `tests/unreachable/` (5) — constructs the grammar does not reach, kept as
#   witnesses that it does not reach them.
# - `tests/probes/` — the heimdall probe corpus (`scripts/probes.sh`). A red
#   probe is a program the compiler cannot yet handle, so it may not parse.
# - `crates/voltaic-lexer/tests/macro_body/` (4) — **lexer** fixtures for
#   verbatim macro bodies, read as token streams: an unclosed body and a
#   bare top-level binding are the point, and neither is a program.
# - `crates/voltaic-fmt/tests/fmt/` (7) — the formatter's **own** fixture pairs:
#   an input beside its `.canonical.vlt` expectation. Formatting the inputs
#   would delete the test.
# - `crates/voltaic-mcp/tests/fixtures/` (2) — includes `unformatted.vlt`, whose
#   whole purpose is to be unformatted so the MCP format tool has something to
#   change.
# - `examples/lexical.vlt` — the one example about **lexing**, containing
#   constructs the parser rejects such as `[0, ...rest, 9]`. Plan 64 §2
#   already names it as the exception.
# - `crates/voltaic-cli/tests/check/syntax.vlt` — a deliberate parse error. The
#   other nine files in that directory are canonical and are **not** excluded,
#   which is why this is a file and not a directory.
EXCLUDE='^(crates/voltaic-cli/tests/ui|tests/unreachable|tests/probes|crates/voltaic-lexer/tests/macro_body|crates/voltaic-fmt/tests/fmt|crates/voltaic-mcp/tests/fixtures)/|^examples/lexical\.vlt$|^crates/voltaic-cli/tests/check/syntax\.vlt$'

mapfile -t FILES < <(git ls-files '*.vlt' | grep -vE "$EXCLUDE")

if [ "${#FILES[@]}" -eq 0 ]; then
    echo "error: no .vlt files matched; the exclusion pattern or the corpus moved." >&2
    exit 1
fi

# A floor, so a broken glob cannot pass by formatting nothing. The corpus was
# 181 files at plan 73; this guards the order of magnitude, not the count.
if [ "${#FILES[@]}" -lt 100 ]; then
    echo "error: only ${#FILES[@]} .vlt files matched, expected well over 100." >&2
    exit 1
fi

exec cargo run -q -p voltaic-cli -- fmt "$@" "${FILES[@]}"

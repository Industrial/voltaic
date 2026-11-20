#!/usr/bin/env bash
# Run the probe corpus under tests/probes/ and print a red/green matrix.
#
# A probe is `<name>.vlt` plus one or more expectation files beside it:
#
#   <name>.expect   every non-blank line must appear in the generated Rust
#                   (whitespace-insensitive substring match)
#   <name>.reject   no non-blank line may appear in the generated Rust
#                   (for silent miscompiles)
#   <name>.error    `voltaic build` must FAIL, and every non-blank line must
#                   appear in its stderr (a negative probe)
#   <name>.rustc    the generated Rust must compile with
#                   `rustc --edition 2024 --crate-type lib`
#
# The first lines of the .vlt carry `-- leaf: <id>` and optionally
# `-- gap: <id>`, which the matrix prints so a red probe names its owner.
#
#   scripts/probes.sh            print the matrix; exit 0 (red is allowed)
#   scripts/probes.sh --strict   exit 1 if any probe is red
#   scripts/probes.sh <filter>   only probes whose path contains <filter>
set -uo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

strict=0
filter=""
for arg in "$@"; do
    case "$arg" in
        --strict) strict=1 ;;
        *) filter="$arg" ;;
    esac
done

cargo build -q -p voltaic-cli || { echo "probes: cannot build voltaic-cli" >&2; exit 2; }
voltaic="$root/target/debug/voltaic"
rustc_bin="${RUSTC:-rustc}"

work="${TMPDIR:-/tmp}/voltaic-probes"
rm -rf "$work"
mkdir -p "$work"

norm() { tr -s '[:space:]' ' ' <<<"$1"; }

# Every non-blank line of file $1 must (want=1) or must not (want=0) occur in
# haystack $2. Prints the first offending line and returns 1 on a miss.
lines_match() {
    local file="$1" hay want="$3" line n
    hay="$(norm "$2")"
    while IFS= read -r line || [ -n "$line" ]; do
        [ -z "${line//[[:space:]]/}" ] && continue
        n="$(norm "$line")"
        n="${n# }"
        n="${n% }"
        if [[ "$hay" == *"$n"* ]]; then
            [ "$want" = 0 ] && { echo "emitted forbidden: $n"; return 1; }
        else
            [ "$want" = 1 ] && { echo "missing: $n"; return 1; }
        fi
    done <"$file"
    return 0
}

green=0
red=0
rows=()
mapfile -t probes < <(find tests/probes -name '*.vlt' | sort)
for vlt in "${probes[@]}"; do
    [ -n "$filter" ] && [[ "$vlt" != *"$filter"* ]] && continue
    base="${vlt%.vlt}"
    name="${base#tests/probes/}"
    leaf="$(sed -n 's/^-- leaf: *//p' "$vlt" | head -1)"
    gap="$(sed -n 's/^-- gap: *//p' "$vlt" | head -1)"
    out="$work/${name//\//__}.rs"
    stderr="$(VOLTAIC_PLAIN=1 "$voltaic" build "$vlt" -o "$out" --plain 2>&1 >/dev/null)"
    built=$?
    reason=""
    if [ -f "$base.error" ]; then
        if [ "$built" = 0 ]; then
            reason="expected a build error, but it built"
        else
            reason="$(lines_match "$base.error" "$stderr" 1)" || true
        fi
    elif [ "$built" != 0 ]; then
        reason="build failed: $(grep -m1 -E 'error|VLT' <<<"$stderr" | cut -c1-90)"
    else
        rust="$(cat "$out")"
        if [ -z "$reason" ] && [ -f "$base.expect" ]; then
            reason="$(lines_match "$base.expect" "$rust" 1)" || true
        fi
        if [ -z "$reason" ] && [ -f "$base.reject" ]; then
            reason="$(lines_match "$base.reject" "$rust" 0)" || true
        fi
        if [ -z "$reason" ] && [ -f "$base.rustc" ]; then
            if ! msg="$("$rustc_bin" --edition 2024 --crate-type lib --emit=metadata \
        -o "$work/${name//\//__}.rmeta" "$out" 2>&1)"; then
                reason="rustc: $(grep -m1 -E '^error' <<<"$msg" | cut -c1-90)"
            fi
        fi
    fi
    if [ -z "$reason" ]; then
        status=GREEN
        green=$((green + 1))
    else
        status=RED
        red=$((red + 1))
    fi
    rows+=("$(printf '%-6s %-38s %-5s %-4s %s' "$status" "$name" "${leaf:--}" "${gap:--}" "$reason")")
done

printf '%-6s %-38s %-5s %-4s %s\n' STATUS PROBE LEAF GAP REASON
printf '%s\n' "${rows[@]}"
echo "probes: $green green, $red red"

[ "$strict" = 1 ] && [ "$red" -gt 0 ] && exit 1
exit 0

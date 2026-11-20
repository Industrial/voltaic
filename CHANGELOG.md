# Changelog

Notable changes per release. `docs/RELEASING.md` is how a release is cut.

Entries for shipped versions are left as they were written, so an older one may
cite a file or a channel that no longer exists — `docs/NIXPKGS.md` and the
nixpkgs route in 0.1.1 below, for instance, both retired in 0.2.0.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and
versions follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **`voltaic gen`**, the eighth subcommand (doc 86 §8.5): transpile a whole
  `.vlt` tree to mirrored `.rs` + `.rs.map`, or every crate with `--workspace`.
  Sidecars are written before builds, unchanged outputs are not rewritten,
  outputs of deleted sources are swept, `--check` exits 1 listing what would
  change, and `--rustfmt` formats each output and re-aligns its map. The driver
  gains `compile_tree`, and `compile_dir` no longer rewrites a cache hit.

## [0.2.0] - 2026-11-14

Generation 2: an inference engine, and the surface contraction it pays for.

Voltaic 1 could not know a type, so every type had to be written. Measured over
the whole compiler, that charter was not a tax on the terseness claim but its
**ceiling** — the line saving was 1.544× and the character saving 1.002×,
because what the surface deleted was punctuation and what it could not delete
was types. `docs/90 - 92 - The Charter Reversal.md` is the argument.

### Added

- **`voltaic-lattice`, the inference engine.** Rank-1 Hindley–Milner over a
  constraint store that keeps provenance, so a failure can name the chain that
  produced it. It infers types, **parameter modes** (`&`, `&mut`, owned) and
  **effect rows** (`io`, `panics`, `suspends`), and writes all of them into the
  generated Rust explicitly. What it computes is what the emitter writes down;
  deciding whether a program is *well-typed* stays rustc's, and there is still
  no borrow checker and no exhaustiveness checker. A program Lattice accepts and
  rustc rejects is a bug in Voltaic, filed as one.
- **Signatures are optional.** `voltaic sig` prints the inferred judgement —
  `doubled : u64 -> u64 @ n owned`, `main : () ! io` — with modes after `@` and
  the effect row after `!`. What decides whether one may be omitted is
  **inferability, not visibility**: an `export`ed definition needs none either,
  and an unsigned `main` compiles. A definition reaching no written type
  anywhere is `VLT0128`.
- **Inherent `instance` members infer too**, receiver mode included:
  `current self = self.value` emits `fn current(&self) -> u32` and
  `bump self = self.value += 1` emits `fn bump(&mut self)`. Doc 63 had held this
  impossible because an inherent instance names no class — true of the
  *resolver*, which looks a signature up, and silent about the *solver*, which
  computes one.
- **Signature sidecars.** 110 committed `.vlt.sig` files, written by
  `voltaic fmt`, read by cross-crate resolution, and gated for currency — a
  stale sidecar answers confidently about a program that no longer exists.
- **The guard ladder reaches value position**, pattern rows may be ladder arms,
  and a binding carries an `else` tail (`Some v = .pop (&mut rest) else break`,
  Rust's let-else stated explicitly).
- **Operator declarations**: `infixl 6 <+> = merge` gives an existing function a
  spelling and a fixity over a closed set of twelve forms, emitting no item.
- **`given <…>`**, the verbatim generic parameter list, for the six binder
  shapes synthesis cannot reach.

### Changed

- **`.` is inverted.** `x.f y` is `f(x, y)` — the receiver **first** — so a
  chain reads left to right and needs none of `|>`, `>>>`, `<<<` or `$`.
  Generation 1 chose receiver-last because it was the only way to make partial
  application and pipelines share a spelling without types; inference removed
  the constraint. A `.` in head position is still receiver-last, so `.len xs`
  remains live. The whole compiler — 8,479 call sites — is written in the new
  form.
- **Derives are computed.** The eight standard derives come from the
  intersection over a declaration's fields, and `deriving (…)` is no longer
  written; `data !Copy Token { kind u64 }` opts out by name. Generation 1 wrote
  the clause on 109 of 128 declarations, every one restating what the fields
  already implied.
- **Fields are `name Type`**, juxtaposed, freeing `:` for ascription. The `:`
  form still parses.
- **Self-hosting crossed to generation 2.** All fourteen crates, one crate per
  reviewable commit, each gated byte-identical before it landed. There is one
  seed blob, `seed/voltaic-g2.wasm`; `voltaic-g1.wasm` is retired, and
  `seed/SEED.md` now *checks* rather than states doc 100's invariant — a blob
  whose `built by` names the generation in its `compiles` is refused.
- An identifier that is a Rust keyword now emits `r#name` rather than invalid
  Rust.
- The book covers the generation-2 surface, and every example in the corpus is
  now cited by a chapter — a gate, because the existing provenance check
  structurally could not see an *absent* citation.

### Removed

- **Six keywords, 58 down to 52**: `otherwise` (now `_`), `until`, `when`, and
  wave 4's first cull. `cond` and `switch` were never keywords despite being
  named as constructs to delete — the wave list had been written from the
  languages being drawn from rather than from Voltaic's lexer.
- **The `errors` declaration**, deleted as a construct and kept as a shape: a
  `data` whose variants carry a trailing string literal is an error type, and it
  goes through the same emitter. It was the only construct in the language that
  emitted more than one Rust item.
- **nixpkgs is no longer a distribution channel.** The submission was declined
  by nixpkgs' maintainers, so `nix/nixpkgs-package.nix`, `docs/NIXPKGS.md` and
  the `nixpkgs` version-bump job in `publish.yml` are all gone, along with the
  `NIXPKGS_PR_TOKEN` secret the job needed — delete that secret if it is still
  set on the repository. **`flake.nix` is unaffected and is the Nix route**:
  `nix profile install github:Industrial/voltaic`, or the flake input shown in
  the book's Install chapter. It builds from the seed, so a clean checkout needs
  no previously installed Voltaic.

## [0.1.1] - 2026-11-13

### Added

- **A visual identity.** `design/` holds two sources — `geometry/pile.json` for
  the mark and `tokens/tokens.json` for the palette — and `design/build.sh`
  emits everything else: seven SVG masters in two optical cuts, a
  `prefers-color-scheme` favicon, `tokens.css` and `tokens.toml`.
  `design/rasterize.sh` renders 31 platform files from those masters — favicon
  and ICO, apple-touch, PWA icons including maskable and monochrome, an iOS
  appiconset with the dark and tinted appearances, a macOS `.icns`, a Windows
  `.ico` with MSIX tiles, and the Linux `hicolor` tree. Output is
  byte-reproducible across runs.
- **A documentation site.** `www/` is an mdBook of 22 chapters deployed to
  GitHub Pages by `.github/workflows/pages.yml`. Every code sample is
  `{{#include}}`d from `examples/`, which CI already compiles and runs against
  a recorded expectation, so a sample cannot drift from the compiler. Ctrl-K
  quick search ranks over mdBook's own index rather than building a second one.
  `llms.txt`, `llms-full.txt`, `sitemap.xml`, JSON-LD and per-page canonical
  links make it legible to crawlers and language models.
- **Editor themes for all five integrations** — VS Code, Zed, Helix, Neovim and
  Vim — generated from the same palette by `design/themes.sh`, plus a `.vlt`
  file icon for VS Code.
- Seventeen gates covering the above, each falsified before being trusted:
  generated assets match their sources, every ICO frame is the size it claims,
  no page scrolls sideways at either of two widths in either theme, and every
  palette pairing clears WCAG AA.

### Changed

- **Two optical cuts replace one scaled drawing.** Five stacked plates above
  32px, three below, because five collapse into a smear at favicon size. Each
  raster frame renders from the cut appropriate to its size rather than being
  downsampled from 1024.
- Three palette values moved for legibility. `ink-faint` was 2.69:1 on the code
  surface and is now 4.56:1; the syntax `string` class moved off `zinc`
  entirely, since zinc is 2.93:1 on white and solving for a readable variant
  landed within one hex of `ink-faint`. Zinc is the mark's second metal and is
  no longer used for text anywhere.

### Fixed

- The MIT switch was incomplete. `README.md` still carried a CC-BY-SA badge and
  licence section, `CONTRIBUTING.md` still told contributors their work was
  CC-BY-SA, and `deny.toml` still allowed `CC-BY-SA-4.0` in the dependency
  list. All three now say MIT. The references under `docs/` are left alone:
  they are plan documents recording what was true when written.

### Known

- `deny.toml` is read by nothing — no gate runs `cargo-deny`, and `:audit` runs
  `cargo audit`, which is the advisory database rather than the licence policy.
  That is why its licence entry survived the switch unnoticed.
- Every page of the book carries two `h1` elements: mdBook's
  `<h1 class="menu-title">` plus the page's own. Not a WCAG failure, and fixing
  it means forking `theme/index.hbs` and carrying it across mdBook versions.
- The book loads its typefaces from Google Fonts rather than self-hosting them.

## [0.1.0] - 2026-11-13

### Added

- A release pipeline. `.github/workflows/publish.yml` runs the gate, packages
  every published crate, writes a CycloneDX SBOM per crate, attests the lot with
  SLSA Build L3 provenance, publishes to crates.io in a derived dependency
  order, re-downloads each crate and checks it against the signed provenance,
  creates the GitHub Release, and then opens the nixpkgs version-bump PR.
  `docs/RELEASING.md` and `docs/NIXPKGS.md` are the prose.
- `scripts/validate-crate-packaging.sh` — derives the publish order by
  topological sort of the workspace manifests, checks the dependency rules, and
  proves each packaged crate contains Rust and compiles from its own archive.
- `crates/voltaic-cli/tests/publish_metadata.rs` — the manifest half of those
  rules, in the ordinary gate, so a broken manifest is reported on the commit
  that caused it rather than at the next release.
- `nix/nixpkgs-package.nix` — the nixpkgs expression, for the one-time
  submission described in `docs/NIXPKGS.md`.
- `.github/actions/setup-voltaic` — the pinned toolchain, wasmtime, tree-sitter,
  moon and the seed bootstrap, in one place instead of once per job.
- `include` keys on the eight published crates, which is what makes them
  publishable at all: `src/` is generated from `vlt/` and is `.gitignore`d, and
  `cargo package` enumerates files from git, so without `include` a published
  crate contains no Rust and uploads cleanly anyway.
- `description`, `repository` and `homepage` metadata; crates.io rejects a
  publish with no description.
- A root `LICENSE` file, which did not exist.

### Changed

- **The licence is now MIT**, replacing `CC-BY-SA-4.0`. CC-BY-SA is a content
  licence: it carries no patent grant and its copyleft scope over linked code is
  untested, which is a poor fit for libraries meant to be depended on.
- Intra-workspace dependencies are declared once in `[workspace.dependencies]`
  and inherited with `<crate>.workspace = true`, so each runtime dependency
  carries a version. `cargo publish` refuses a path dependency without one.
  Dev-dependencies deliberately stay path-only, which is the opposite rule and
  matters for the same reason.
- **The project was renamed from Redox to Voltaic**, including all thirteen
  crates and the `voltaic` binary. Not everything carries the new name yet: the
  `VLT####` diagnostic codes and the `__vlt_` generated-identifier prefix still
  use the old one, because each needs its own build → regenerate → reseed cycle.
  A diagnostic still reads `VLT0101`. The `.vlt` and `.vrs` extensions never
  encoded the old name and are unchanged by design.

### Fixed

- `AGENTS.md` placed `build.rs` under the CLI crate. It is at
  `crates/voltaic-driver/build.rs`, and it is the only tracked hand-written
  Rust left under `crates/`.

### Not published

`voltaic-bench`, `voltaic-fuzz` and `voltaic-seed` are internal; `voltaic-lsp`
and `voltaic-mcp` are binary-only and delivered through nixpkgs and
`flake.nix`. All five carry `publish = false`.

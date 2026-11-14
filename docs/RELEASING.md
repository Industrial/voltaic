# Releasing Voltaic

A release is a pushed `v*` tag. [`.github/workflows/publish.yml`](../.github/workflows/publish.yml)
does the rest: it runs the gate, packages, attests, publishes to crates.io,
verifies what crates.io serves against the attestation, and creates the GitHub
Release.

**Voltaic is not distributed through nixpkgs.** The submission was declined by
their maintainers, so there is no downstream bump to automate and no
`nix/nixpkgs-package.nix` to keep in step. `flake.nix` in this repository is the
Nix delivery path, it builds from the seed, and it needs nothing from a
release.

## Cutting a release

1. Bump `[workspace.package] version` in the root `Cargo.toml`, **and the
   matching versions in `[workspace.dependencies]` in the same file.** cargo
   does not allow `version.workspace = true` inside
   `[workspace.dependencies]`, so that is the one place the version is written
   twice; `scripts/validate-crate-packaging.sh` fails if the two drift, which is
   the only reason a second copy is tolerable. Update `CHANGELOG.md`.

   **It was written in four places until 0.2.0, and the other two were found by
   the bump failing.** `voltaic-lattice` pinned `voltaic-ast` and `voltaic-diag`
   with literal versions instead of `.workspace = true` — every other crate uses
   the inherited form — and `voltaic-seed`, `mdbook-voltaic` both set their own
   `version` and `edition` literally. `cargo update` is what caught it:
   *"failed to select a version for the requirement `voltaic-ast = "^0.1.1"`…
   candidate versions found which didn't match: 0.2.0"*. All four now inherit,
   so the two copies this step describes are the only two that exist. Run
   `grep -rn --include=Cargo.toml '<old version>' .` after a bump; a literal
   that survives is this.
2. Run the gate locally:

   ```bash
   ./scripts/validate-crate-packaging.sh    # order, dependency rules, packaging
   moon run :test                            # the same gate CI runs
   ```

3. Merge to `main`, then tag the merge commit and push the tag:

   ```bash
   git tag -a v0.1.0 -m "v0.1.0"
   git push origin v0.1.0
   ```

To rehearse, run the workflow by hand (Actions → Publish to crates.io → Run
workflow) with **Dry run** checked. A dry run packages and attests everything,
runs `cargo publish --dry-run` over the whole order, checks the packages against
the attested digests and runs `slsa-verifier` on them. It uploads nothing,
and creates no release. A run *without* **Dry run** must be
dispatched on a tag ref.

## What the workflow does

| Job | What it does |
|-----|--------------|
| `verify-versions` | The tag equals `[workspace.package] version`. Whether every crate *inherits* that version is checked by `validate-packaging`, not here — stating it twice would be two things free to disagree. |
| `test` | `moon run :test`, the same gate `ci.yml` runs, on the tagged tree. |
| `validate-packaging` | `scripts/validate-crate-packaging.sh`: the derived publish order, the dependency rules, that every packaged crate contains Rust, and that each one **compiles from its own archive**. |
| `build` | Checks that the bootstrap is byte-reproducible, then `cargo package --locked` of every published crate in one invocation, one CycloneDX SBOM each. The sha256 of each file is a provenance subject. |
| `provenance` | SLSA Build L3 provenance over those files, via [slsa-github-generator](https://github.com/slsa-framework/slsa-github-generator), signed keylessly with Sigstore. |
| `publish` | `cargo publish --locked` crate by crate in the same order. `.github/scripts/publish-crate.sh` skips a version the crates.io index already has, so re-running after a partial publish resumes at the first missing crate; after each upload it checks the `.crate` cargo uploaded has the attested sha256 and stops the release otherwise. |
| `verify` | Downloads every crate from `static.crates.io`, checks its sha256 equals the subject digest in the signed provenance, and runs `slsa-verifier verify-artifact`. Any mismatch fails the run and no release is created. |
| `release` | GitHub Release with the `.crate` files, the `.cdx.json` SBOMs and `voltaic-<version>.intoto.jsonl`. |

All actions are pinned by commit SHA. The one exception is the SLSA generator, a
reusable workflow that must be referenced by its `@vX.Y.Z` tag because it
verifies its own ref.

## What gets published, and what does not

Nine crates: `voltaic-diag`, `voltaic-lexer`, `voltaic-ast`, `voltaic-emit`,
`voltaic-fmt`, `voltaic-lattice`, `voltaic-parser`, `voltaic-driver` and
`voltaic-cli`, in that order. **`voltaic-lattice` is new in 0.2.0** — it did not
exist at 0.1.1 — so that release is the one that creates it on crates.io.
Don't take the count from here: `./scripts/validate-crate-packaging.sh
--print-order` derives it, and this sentence is a second copy free to drift,
which it did once already.

The order is not written down anywhere — it is derived by topological sort of
the intra-workspace dependencies in `scripts/validate-crate-packaging.sh`, so
adding a crate or changing a dependency reorders it automatically.

`voltaic-bench`, `voltaic-fuzz`, `voltaic-lsp`, `voltaic-mcp` and `voltaic-seed`
carry `publish = false`. The first two and the seed are internal. The language
server and the MCP server are binary-only and nothing links them: `flake.nix`
builds them, and the editor clients run them as processes. If
`cargo install voltaic-lsp` ever becomes the asked-for install path, the name is
free on crates.io — unlike the pre-rename `redox-lsp`, which belongs to an
unrelated project.

Only `voltaic-cli` carries a `readme`. The seven libraries deliberately do not:
there is one README in this repository, a per-crate copy of it would be seven
copies of one paragraph, and `description` plus `repository` plus docs.rs is
what a library's crates.io page actually needs.

## `include` is load-bearing, and this is the thing to know

Every crate under `crates/` is written in Voltaic. The `.vlt` under a crate's
`vlt/` is the source; the `.rs` under its `src/` is generated by the seed and is
`.gitignore`d. `cargo package` enumerates files from git.

So **without an `include` key, `cargo package` ships no Rust at all.** Measured
before the keys were added:

```
$ cargo package -p voltaic-ast --list
Cargo.toml   Cargo.toml.orig   moon.yml   vlt/*.vlt
```

That uploads cleanly. It produces an unbuildable empty shell on crates.io, with
no warning anywhere. An `include` switches cargo from the git listing to a
filesystem walk, which does see the generated tree — and its `src/**/*.rs` glob
also leaves the `.rs.map` source maps behind, which is what you want.

Two things guard it, because a manifest key that must not be removed and has no
gate on it will eventually be removed:

- `scripts/validate-crate-packaging.sh` reads the built `.crate` and fails if it
  contains no `src/**/*.rs`, or if it contains a `.rs.map`. It reads the archive
  rather than the manifest, because the manifest is not the thing that can be
  wrong.
- `cargo package` keeps verification **on** there, so every archive is extracted
  and compiled. That is the only check proving the bytes build rather than
  merely package. It costs about 17s for the whole set.

### And `--allow-dirty` is required, everywhere

The other half of the same fact, and it is counter-intuitive enough to be worth
stating on its own: **cargo refuses to package this repository without
`--allow-dirty`, even on a spotless checkout at a fresh commit.** The files the
manifests tell it to include are untracked by design, and cargo counts an
untracked file it is about to package as an uncommitted change:

```
error: 5 files in the working directory contain changes that were not yet
committed into git:
  crates/voltaic-diag/src/code.rs
  …
```

So `--allow-dirty` appears in the validator, in the `build` job and in
`publish-crate.sh`. It is not covering for a messy tree.

What it gives up is cargo's refusal to package *genuine* uncommitted edits, and
that is worth keeping. Both the `build` and `publish` jobs therefore assert
`git status --porcelain` is empty first — that command omits ignored files, so
it sees real dirt and not the generated trees. In `publish` it is the last thing
standing between a stray edit and crates.io.

## Reproducibility and the seed

`cargo package` is byte-reproducible for the same tree, `Cargo.lock` and cargo
version, which is what lets `publish` upload the bytes `build` attested: the
workflow pins one toolchain for every job and packages with `--locked`.

Here there is a second dependency, and it is specific to this project. The
packaged tree contains Rust that `./bootstrap.sh` **generated**, and `build` and
`publish` bootstrap independently. If the seed's output were not
byte-reproducible, the two jobs would package different bytes and the digest
check in `publish-crate.sh` would fail with what reads like evidence of
tampering.

`build` therefore bootstraps twice and compares hashes before anything is signed
or uploaded, so that failure arrives early and named. If it ever fires, the seed
is the suspect, not the pipeline.

A packaged crate also embeds a `Cargo.lock` with the checksums of its siblings.
If a re-run meets a sibling that an earlier run published from different bytes,
the per-crate check in `publish` fails rather than releasing an unverifiable
crate. Bump the patch version in that case rather than re-running.

### Changing the surface changes the seed

Anything that changes what the compiler emits or accepts needs
`moon run :seed-refresh` and `VOLTAIC_UPDATE=1 cargo test -p voltaic-seed --test
blob` **in the same commit**, or a clean clone stops bootstrapping. `AGENTS.md`
has the full rule. Note that `:seed-refresh` prints a *cached* size, so check
`git status seed/` and `stat` the file rather than believing its output.

## crates.io authentication

The `publish` job uses the `CARGO_REGISTRY_TOKEN` secret of the `crates-io`
environment; `publish-crate.sh` hands it to cargo via `cargo login` on stdin,
never on the command line. The token can also publish a crate's first version,
so a crate's first version needs no owner step beforehand.

### Secrets and environments to set up

**One secret**, and it is an *environment* secret rather than a repository one,
because the `publish` job declares `environment: crates-io`.

| Name | Where exactly | Needed by | Needed from |
|---|---|---|---|
| `CARGO_REGISTRY_TOKEN` | Settings → Environments → `crates-io` → *Add environment secret* | `publish` | the first release |

`CARGO_REGISTRY_TOKEN` comes from crates.io → Account Settings → API Tokens, with
the `publish-new` and `publish-update` scopes. Scope it to the `voltaic-*` crates
if you like; it needs `publish-new` because the first release creates them.

There was a second secret, `NIXPKGS_PR_TOKEN`, for the job that opened the
nixpkgs version-bump PR. Both are gone: nixpkgs declined the submission, so
there is nothing downstream to bump. **If that secret is still set on the
repository, delete it** — it now grants push access to a fork for no reason, and
an unused credential is the kind that outlives the memory of why it existed.

Protecting the `crates-io` environment is worth doing: Settings →
Environments → `crates-io`, deployment refs limited to `v*` tags plus whatever
branch dry runs are dispatched from.

## If the first release stops partway

Expected, recoverable, and worth knowing before it happens: the first release
creates several new crates in one run, and crates.io rate-limits new-crate
publishes. If it trips, the `publish` job fails with a 429. 0.1.0 created eight;
0.2.0 adds `voltaic-lattice`, so a release that introduces only one or two new
names is unlikely to trip it at all.

Nothing is lost and nothing needs fixing. `publish-crate.sh` checks the
crates.io index before each crate and skips the ones already there, so
**re-running the `publish` job resumes at the first crate that did not make
it.** The same property covers a dropped connection or an expired token.

The one case that is *not* resumable is a digest mismatch — see
*Reproducibility and the seed* above. That one means the bytes changed between
runs, and the fix is a patch version bump rather than a re-run.

## Known rough edges

Stated rather than left to be discovered:

- **The tool pins exist twice.** `.github/actions/setup-voltaic` pins wasmtime,
  tree-sitter and moon; `ci.yml` still carries its own copy in `env:`.
  Converging `ci.yml` onto the action is the obvious follow-up and was
  deliberately not done here: `ci.yml` gates every pull request, and a refactor
  of it cannot be verified from a release change.
- **The rename is not complete, and release notes should not say it is.** The
  project is Voltaic, but the `VLT####` diagnostic codes and the `__vlt_`
  generated-identifier prefix still carry the old name, because each needs its
  own build → regenerate → reseed cycle. A user-visible diagnostic still reads
  `VLT0101`. The `.vlt` and `.vrs` extensions never encoded the old name and are
  unchanged by design.
- **The licence changed to MIT** at the same time as this pipeline landed,
  replacing `CC-BY-SA-4.0`. A root `LICENSE` file now exists; it did not before.
- **`nextest.toml` has never been read, and should be moved — carefully.**
  nextest reads `.config/nextest.toml`; this repository's copy is at the root and
  there is no `.config/` directory, so every test run ever made here has used
  nextest's built-in defaults rather than the `retries = 3`, `fail-fast = 5` and
  `test-timeout = "60s"` the file declares.

  That is why the `watch.rs` timing flake is fatal rather than retried, and the
  `test` job works around it with `NEXTEST_RETRIES: 2` for itself alone.

  Relocating the file is the real fix and is deliberately **not** done here,
  because it is not just a move: `fail-fast = 5` is a boolean in nextest's schema
  (the numeric one is `max-fail`), and `cache`, `[profile.default.run].threads`
  and `capture-output` do not obviously correspond to profile keys either. A
  config nextest rejects stops every test run outright, which is worse than one
  it ignores. Validate it against the schema with `cargo nextest show-config
  test-groups` (or `--help` for the current key set) before moving it, and expect
  to rewrite it rather than relocate it.

## Verifying a release yourself

```bash
curl -fsSLO https://static.crates.io/crates/voltaic-cli/voltaic-cli-0.1.0.crate
gh release download v0.1.0 -R Industrial/voltaic -p 'voltaic-0.1.0.intoto.jsonl'
slsa-verifier verify-artifact voltaic-cli-0.1.0.crate \
  --provenance-path voltaic-0.1.0.intoto.jsonl \
  --source-uri github.com/Industrial/voltaic --source-tag v0.1.0
```

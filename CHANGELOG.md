# Changelog: LifePlanner

All notable changes to this project are documented here.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## [1.0.7] - 2026-07-29

### Security

- The release workflow no longer grants `contents: write` for its whole run. The permission moves to the one job that publishes the release, and everything else runs with `contents: read`. OpenSSF Scorecard scores the Token-Permissions check 0 out of 10 whenever any workflow holds a top-level write permission, regardless of how little of the run needs it, so this single line was what held the check at zero.
### Added

- `frontend/src/vite-env.d.ts`, referencing `vite/client`. Vite has always declared modules for `*.css` and the other asset types it handles, but nothing in this project pulled that declaration in. TypeScript 5 accepts the untyped side-effect import of `index.css` regardless, so the gap stayed invisible; TypeScript 7 rejects it with `TS2882`. The file belongs to Vite's own project scaffold and was simply missing, so this closes an existing hole rather than preparing for a specific upgrade.

---

## [1.0.6] - 2026-07-29

### Changed

Dependency and workflow updates merged since 1.0.5:

- chore(ci): bump the actions group across 1 directory with 3 updates
- chore(deps): bump the npm group across 1 directory with 3 updates

---

## [1.0.5] - 2026-07-28

### Fixed

- The CodeQL job requested `packages: read`, `actions: read` and `contents: read` at job level, repeating grants the workflow level already provides. OpenSSF Scorecard counts that as excessive token permissions and scores `Token-Permissions` at 0 out of 10 for it. The job now requests only `security-events: write`, which is the one grant that genuinely exceeds the workflow default.

## [1.0.4] - 2026-07-28

### Changed

- CodeQL moved from GitHub's default setup to an advanced setup with a committed `.github/workflows/codeql.yml`. The default setup skips pull requests that touch no code of a given language, so a dependency pull request changing only a lock file reported `skipping` on the required `Analyze (...)` checks forever and could never be merged. The workflow runs on every pull request regardless of what changed. It also uses the `security-extended` query suite, which the default setup does not allow choosing. Required checks are unchanged: verified on `BugRadar` that all eight, the generic `CodeQL` check included, turn green under this setup.
- Dependabot now groups only minor and patch updates per ecosystem; majors arrive as individual pull requests. The previous grouping put React 18 to 19, Tailwind 3 to 4 and similar breaking changes into one pull request together with urgently needed security patches, which made the whole batch unreviewable and unmergeable. Actions stay grouped wholesale. Follows `engineering-standards` v0.11.0.

## [1.0.3] - 2026-07-28

### Security

- `postcss` updated to 8.5.24, closing a high-severity path traversal in the source map auto-loading via `sourceMappingURL` that affects all versions up to and including 8.5.17.

Applied as a normal pull request rather than by merging Dependabot's, because Dependabot pull requests cannot currently pass this repository's required checks: CodeQL runs through GitHub's default setup, which does not trigger on a pull request that only touches a lock file, so its checks report `skipping` and never turn green. Bypassing a required check is not an option per `standards/ci-cd.md` section 7, so the fix takes the route that runs the full pipeline.

## [1.0.2] - 2026-07-28

### Added

- `.github/dependabot.yml`, with grouped weekly updates. The file was missing, and without it there are no version updates at all: repository security alerts only fire for disclosed vulnerabilities. Follows `engineering-standards` v0.10.0.

### Fixed

- 6 action references used a mutable tag or branch instead of a commit SHA, `dtolnay/rust-toolchain@stable` among them where applicable. A branch HEAD can be moved to point at different code at any time. All are now pinned, at the version that was actually running rather than upgraded, so any major bump arrives as its own reviewable Dependabot PR.
- The crates carried version 0.2.7 and `frontend/package.json` 0.2.7, while `tauri.conf.json` and the tag said 1.0.1. A `[workspace.package]` section now holds one version that the crates inherit, and every manifest agrees. `cargo metadata` confirms 1.0.2 across all three crates.

## [1.0.1] - 2026-07-20

### Changed

- OpenSSF Scorecard workflow and badge.
- `copilot-instructions.md` for consistent AI-assisted contributions.
- Coverage reporting in CI (cargo-tarpaulin).
- Split the README's security/CI badges onto their own line, separate from the platform/tech/AI badges (they were rendering as a single merged line).

## [1.0.0] - 2026-07-17

First stable release: a real, packaged, installable distribution now exists
for macOS, Windows, and Linux (DMG, EXE installer, AppImage), the
prerequisite for a 1.0 release per this portfolio's own SemVer discipline.

### Added
- Linux (Ubuntu/AppImage) is now a fully supported build target, alongside macOS and Windows.

## [0.2.9] - 2026-07-17

### Changed
- CI: added an explicit `permissions: contents: read` block to the workflow(s) that were missing one (CodeQL `actions/missing-workflow-permissions`), narrowing the default GITHUB_TOKEN scope.

## [0.2.8] (2026-07-12)

### Removed

- Stale scaffold-tool bookkeeping files SKELETON.md and TEMPLATE_NOTES.md (internal generator artifacts, not real project docs).

## [0.2.7] (2026-07-12)

### Security

- Bumped `vite` and `@vitejs/plugin-react` (frontend dev dependencies) to resolve 4 Dependabot-reported advisories: a high-severity `server.fs.deny` bypass on Windows, an NTLMv2 hash disclosure via UNC path handling in `launch-editor`, a path traversal in Vite's optimized-deps `.map` handling, and an esbuild dev-server request/response exposure. All four affect the Vite dev server only, not the built/shipped application.

## [0.2.6] (2026-07-12)

### Added

- TERMS_OF_SALE.md: terms covering the purchase of a pre-built, packaged distribution through a marketplace (as-is, no warranty, liability strictly capped at the amount paid). Does not modify the existing MIT LICENSE, which continues to cover the source code at no cost.

### Fixed

- Version drift: `src-tauri/Cargo.toml` was at 0.2.4 while the rest of the workspace was at 0.2.5. All crates now report 0.2.6 consistently.

## [0.2.5] (2026-07-12)

### Fixed

- Removed an eszett and em-dashes across the repo (TEMPLATE_NOTES.md, GETTING_STARTED.md, CONTRIBUTING.md, ARCHITECTURE.md, SKELETON.md, and three Rust source files). Swiss German orthography.

## [0.2.4] (2026-07-11)

### Added

- Documented Dual-Licensing assessment (Community-only) in ROADMAP.md.

### Fixed

- Removed em-dashes from ROADMAP.md and SECURITY.md.

## [0.2.3] (2026-07-11)

### Fixed

- Updated actions/setup-node and tauri-apps/tauri-action to their latest major versions in CI and the release workflow, since GitHub is deprecating the Node.js 20 runtime and older action versions were being forced onto Node 24 and crashing during post-run cleanup.

## [0.2.2] (2026-07-11)

### Fixed

- Fixed the release workflow's stable-named DMG/installer upload: it looked for the built bundle under `src-tauri/target/...`, but this is a Cargo workspace, so Cargo places build output under the workspace root `target/...`. The stable `LifePlanner.dmg`/`LifePlanner-Setup.exe` download links in README.md never actually got uploaded before this fix.

## [0.2.1] (2026-07-10)

### Fixed

- Removed em-dash from the download callout in README.md/README.de.md, replaced with a colon

## [0.2.0] (2026-07-10)

### Added

- Release workflow: pushing a `v*` tag now builds macOS (DMG) and Windows (NSIS installer) bundles via `tauri-action` and attaches them to a GitHub Release. Not code-signed/notarized

## [0.1.5] (2026-07-10)

### Changed

- Moved the "New here? -> beginners guide" callout in README.md above Features (previously only appeared near Requirements)

### Added

- Added the "New here?" beginner guide callout to README.de.md (was missing)

## [0.1.1] (2026-07-07)

### Fixed
- Build failed to compile: two commands used the removed `chrono::Utc.from_local_datetime()` call pattern without the `TimeZone` trait in scope
- `dirs` crate was used in `src-tauri/src/main.rs` for the database path but was never declared as a dependency, so a clean checkout could not build at all
- `cargo tauri build` panicked at compile time because `src-tauri/icons/` did not exist while the bundle config expected icon files

## [0.1.0] (2026-06-12)

### Added
- Text, email, and PDF parser for automatic extraction of events, tasks, and deadlines
- Intelligent linking between related events and tasks
- CalDAV sync with local network calendar servers (Nextcloud, Radicale)
- SQLite FTS5 storage for fast full-text search across all items
- AI-powered suggestions via local Ollama model
- Tauri v2 desktop shell with React/TypeScript frontend

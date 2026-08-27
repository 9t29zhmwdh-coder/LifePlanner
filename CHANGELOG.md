# Changelog: LifePlanner

All notable changes to this project are documented here.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## [1.2.3] - 2026-08-27

### Changed

- **Eigenes App-Icon.** Bis hierher trug jedes Werkzeug des Portfolios dasselbe RayStudio-Logo, byteweise identisch, was im Dock und in der Titelleiste nicht auseinanderzuhalten war. LifePlanner bekommt jetzt sein eigenes Zeichen im bestehenden Hausstil: runder Rahmen mit Goldkante, tiefdunkler Grund, die Initialen in einer Didone-Serife, der goldene Strahl mit Funkeln darueber.

  Zwei Fassungen, wie es Apple und Microsoft ebenfalls halten: ab 128 Punkten die feine mit dem Schriftzug, darunter eine ohne. Gesperrte Versalien werden bei 32 Punkten zu einem grauen Streifen und nehmen den Initialen nur den Platz weg, den sie dort brauchen.

  Die Farbwerte stammen aus `RegistrarCheck.png`, nicht aus einer Schaetzung: Grund `#010d22`, Gold von `#a7782f` ueber `#e2c47e` nach `#ca9f4d`. Die SVG-Quellen liegen unter `src-tauri/icons/source/`, damit sich das Zeichen spaeter aendern laesst, ohne es nachbauen zu muessen.

---

## [1.2.2] - 2026-08-05

### Added

- A smoke test in CI: the application is built, started, and checked to still be running five seconds later. Until now the pipeline only ever established that the code compiles. A program that builds cleanly and dies on launch would have passed every check and been discovered by whoever downloaded it.
- It runs on Linux and macOS. The Linux job needs `xvfb`, since a GTK window closes immediately without an X server, and that would produce a failure the runner invents rather than one the code has.
- The test also fails on a panic in the output even when the process survives, because a background task that dies quietly leaves the window open and useless.

---

## [1.2.1] - 2026-08-04

### Changed

- TypeScript 5.9.3 to 7. No source change was needed. The production build runs `tsc` ahead of vite, so the typecheck has to pass for anything to be produced at all, and the generated files come out with the same content hashes as before.

---

## [1.2.0] - 2026-08-03

### Changed

- `sqlx` 0.8 to 0.9. Two query sites assemble their SQL with `format!`, because SQL cannot bind a list as one parameter, and 0.9 rejects that unless the string is asserted safe. The only interpolated part is a chain of question marks derived from the length of the id list; the ids go through `bind`.
- `github/codeql-action` 4.37.3 to 4.37.4 and `actions/attest` 4.2.0 to 4.2.1, merged separately and carried by this version.

### Added

- A test that feeds two ids shaped like an attack, one of them `x'); DROP TABLE events; --`, into the search path. It returns nothing and the row count is unchanged, which proves the assertion the new sqlx demands rather than merely claiming it.

### Removed

- `recharts`. Dependabot proposed a 2 to 3 major, but the package is declared in `package.json` and imported in no source file. The built bundle keeps the same content hashes, which shows it was never in the output; `node_modules` drops by 18 MB, since recharts brings d3 with it.

---

## [1.1.0] - 2026-08-03

### Changed

- Tailwind CSS 3 to 4. The config file is gone, the stylesheet imports tailwindcss directly, and PostCSS uses `@tailwindcss/postcss`. autoprefixer is no longer a dependency because version 4 prefixes on its own.
- Three utility names were rewritten across eight components: `rounded` to `rounded-sm` 19 times, `outline-none` to `outline-hidden` 6 times, `flex-shrink-0` to `shrink-0` 8 times. Only the last two change behaviour. Measured under 4.3.3, `rounded` is still 0.25rem and kept as an alias; the scale did shift, but under the name `rounded-sm`, which this code never used. The rename is normalisation, not a fix.

---

## [1.0.17] - 2026-08-02

### Changed

- React 18 to 19, together with `react-dom` and both type packages. Dependabot had split these across separate pull requests and neither could be merged alone: `@types/react-dom` 18 requires `@types/react` 18, so raising either one left npm unable to resolve the peer dependency. All four move together here.
- No code changes were needed, checked against the list of things React 19 removes rather than assumed: `createRoot` is already in use, and there are no string refs, no `propTypes`, no argument-less `useRef`, no `forwardRef`, no `defaultProps` and no callback refs. Typecheck and production build both clean.

---

## [1.0.16] - 2026-08-02

### Security

- `keyring` switches from `crypto-openssl` to `crypto-rust`, which takes OpenSSL out of the Linux build. The Secret Service protocol encrypts the session between the application and the keyring daemon, and that encryption came from the OpenSSL C library, reaching this tree through `keyring` and `secret-service`. `crypto-rust` implements the same algorithms the specification prescribes, AES-CBC with SHA-2 and HKDF, from the RustCrypto crates. The wire format belongs to the specification rather than to either implementation, so an existing keyring stays readable.
- Afterwards `Cargo.lock` holds no `openssl` package at all, where it held one before. With it goes a C library with a long CVE history and the requirement to have its development headers present when building for Linux. macOS and Windows never compiled this path; both use their native keychain.

---

## [1.0.15] - 2026-08-01

### Changed

- Dependency updates merged since 1.0.14, all carried by this one version rather than one release each: `thiserror` 1.0.69 to 2.0.19, `dirs` 5.0.1 to 6.0.0, `base64` 0.22.1 to 0.23.0 and `ical` 0.10.0 to 0.11.0. The `dirs` bump was checked rather than assumed, because `data_dir()` decides where the user's database lives: both versions were built and their paths compared, and the source diff between `dirs-sys` 0.4.1 and 0.5.0 turns out to be a single Windows FFI line, `HANDLE::default()` replaced by `null_mut()`. No path logic changed on any platform.

### Removed

- five declared dependencies that no code references: quick-xml, anyhow, tracing, rayon, base64. They were compiled on every build, shipped their own transitive tree, counted toward the supply-chain surface, and produced Dependabot pull requests proposing upgrades to code nobody calls. Verified by removing them and running `cargo check`, `cargo clippy` with `-D warnings` and the full test suite, all clean.

---

## [1.0.14] - 2026-08-01

### Changed

- Dependabot no longer retries the `glib` update it cannot perform. GHSA-wrw7-89jp-8q8g is fixed in 0.20, and this project cannot reach it: `tauri` 2.x pins `gtk ^0.18`, `gtk` 0.18 requires `glib ^0.18`, and no patched 0.18.x exists, so cargo rejects the upgrade rather than resolving it. Three attempts had already failed identically, each one a red run on `main` that carried no information. Only the unreachable versions are ignored, so a backported 0.18.x fix would still arrive, and the advisory itself stays visible in the Security tab. The block goes away when Tauri moves to gtk-rs 0.20, the condition already recorded in `SECURITY.md`.

---

## [1.0.13] - 2026-07-31

### Added

- `SECURITY.md` records GHSA-wrw7-89jp-8q8g against `glib` 0.18.5, which cannot be fixed from this repository because Tauri 2.11.5 pins `gtk ^0.18` and no patched 0.18.x exists.

### Changed

- The scope section said the project runs fully locally and stopped there. It now names the one connection that exists, your own Ollama instance during the optional AI briefing, and states that extraction, calendar sync and conflict detection reach no network at all. "Fully locally" was true but unspecific, and a reader checking this file wants to know which connection to expect.

### Fixed

- The supported-versions table still listed `0.1.x`, a line that no longer exists.

---

## [1.0.12] - 2026-07-31

### Changed

- Both READMEs now open with where the dates actually sit, in a confirmation email or on page three of a PDF, rather than describing the recognition the app performs. It also states earlier that extraction and conflict detection need no model at all, so the AI reads as optional rather than as the point. A short paragraph says the tool has nothing to do when appointments already arrive as calendar invitations.

---

## [1.0.11] - 2026-07-30

### Added

- `Cargo.lock` is committed. It was listed in `.gitignore`, so every build resolved dependencies afresh and no two builds were guaranteed to use the same versions. For an application rather than a library the lock file belongs in the repository: it is what makes a release reproducible and what lets a security advisory be checked against what actually shipped.

---

## [1.0.10] - 2026-07-30

### Changed

- The `Check` job runs on Linux, macOS and Windows instead of macOS alone. The release builds artifacts for all three, so a fault that only shows on one of the other two reached a release before anything noticed.
- The Linux leg installs the GTK and WebKit packages Tauri builds against. The runner ships neither, and without them `cargo check` fails at `gobject-2.0` before reaching any code. The release workflow already installed the same packages, which is why releases worked while no Linux check existed.
- The ruleset now requires `Check (ubuntu-latest)`, `Check (macos-latest)` and `Check (windows-latest)` in place of the single `Check`. A matrix renames the job, so leaving the old context required would have left a check that can never report again.

---

## [1.0.9] - 2026-07-30

### Changed

- The keychain persistence test runs on every target platform rather than macOS alone. LifePlanner ships `.dmg`, `.msi` and `.deb` artifacts, and each platform has its own credential backend that can be missing independently of the others, so covering one of the three left the other two untested against the defect fixed in 1.0.8.
- The test separates a missing service from a missing backend. The in-memory fallback never fails to write, so a write error proves a real backend is compiled in and only its service is absent, which is the normal state of a Linux CI runner without a D-Bus secret service. A write that succeeds while a second process finds nothing is the defect.

---

## [1.0.8] - 2026-07-30

### Security

- `keyring` now names a platform backend, so calendar credentials actually reach the operating system keychain. It was declared without a platform feature, which compiles and raises no error but falls back to a store held in process memory. Credentials were gone after every restart, and because the fallback answers a read with `NoEntry`, the application read that as "no password stored" rather than as a loss.

### Added

- A test that stores a secret and reads it back from a second process, since a store-then-read inside one process is satisfied by the in-memory fallback and would have passed against the defect. It re-runs its own binary rather than reading through `/usr/bin/security`: the keychain grants read access per application, so a different binary asking for an item it did not create raises an authorisation dialog that blocks CI. The test's own `keyring` entry deliberately names no features, because Cargo unifies features across the graph and a feature there would keep the test green while the application loses its backend.

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

# Changelog: LifePlanner

All notable changes to this project are documented here.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

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

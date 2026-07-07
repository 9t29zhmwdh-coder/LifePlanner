# Changelog: LifePlanner

All notable changes to this project are documented here.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

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

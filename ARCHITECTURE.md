# Architecture — LifePlanner

## Overview

LifePlanner is a Rust workspace with a Tauri v2 desktop shell and a React/TypeScript frontend.

```
LifePlanner/
├── crates/
│   ├── lp-core/            # Core library
│   │   ├── src/
│   │   │   ├── parser/     # Text/email/PDF parser (events, tasks, deadlines)
│   │   │   ├── linker/     # Intelligent event-task linking
│   │   │   ├── caldav/     # CalDAV sync (reqwest + rustls)
│   │   │   ├── ai/         # Ollama integration for suggestions
│   │   │   ├── models/     # Shared data types
│   │   │   └── db/         # SQLite FTS5 storage + migrations
│   │   └── Cargo.toml
│   └── lp-cli/             # Headless CLI interface
│       ├── src/main.rs
│       └── Cargo.toml
├── src-tauri/              # Tauri v2 application shell
│   ├── src/
│   │   ├── main.rs
│   │   ├── state.rs
│   │   └── commands/       # IPC commands exposed to frontend
│   └── Cargo.toml
├── frontend/               # React + TypeScript UI
│   ├── src/
│   │   ├── App.tsx
│   │   ├── stores/
│   │   └── components/
│   └── package.json
└── Cargo.toml              # Workspace root
```

## Data Flow

1. **Parser** reads input (text, email, PDF, notes) and extracts structured events/tasks/deadlines using regex + AI
2. **Linker** connects related items (e.g., "dentist appointment" → "health" task cluster)
3. **AI** (Ollama) suggests priorities, due dates, follow-ups
4. **SQLite FTS5** stores all items with full-text search
5. **CalDAV** syncs confirmed events to local network calendar (Nextcloud, Radicale, etc.)
6. **UI** presents timeline, task list, and smart suggestions

## Key Dependencies

- `ical` — iCalendar parsing/generation
- `quick-xml` — CalDAV XML (WebDAV)
- `reqwest + rustls` — CalDAV HTTP client (TLS, no OpenSSL)
- `rusqlite` — SQLite FTS5 storage
- `ollama-rs` or raw HTTP — local AI backend

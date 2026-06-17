<div align="center">
  <img src="RayStudio.png" alt="RayStudio Logo" width="120"/>
  <h1>LifePlanner</h1>
</div>

[🇩🇪 Deutsche Version](README.de.md)

**Fully offline AI life planner built with Rust, Tauri and local AI via Ollama.**

LifePlanner automatically recognizes appointments, tasks, projects and deadlines from emails, PDFs and notes, links them intelligently and helps you plan your day; without a single byte leaving your device.

[![CI](https://github.com/9t29zhmwdh-coder/LifePlanner/actions/workflows/ci.yml/badge.svg)](https://github.com/9t29zhmwdh-coder/LifePlanner/actions/workflows/ci.yml)
![Rust](https://img.shields.io/badge/Rust-1.77+-orange?logo=rust)
![Tauri](https://img.shields.io/badge/Tauri-v2-blue?logo=tauri)
![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey)
![License](https://img.shields.io/badge/License-MIT-green)

---

## Features

- **Smart Extraction**: Paste any text (email, chat, document) and LifePlanner detects dates, deadlines and tasks automatically
- **Calendar Sync**: ICS files, CalDAV, Exchange, Google and Apple Calendar
- **Conflict Detection**: Overlapping appointments are flagged instantly
- **Free Slot Finder**: See where your day has breathing room
- **Energy Sorting**: Tasks grouped by focus / creative / routine energy level
- **Project Tracker**: Group tasks into projects with progress visualization
- **Daily AI Summary**: Local AI generates a plain-language briefing for your day
- **Full-text Search**: SQLite FTS5-powered instant search across all events and tasks
- **100% Offline**: No cloud, no account, no telemetry

---

## Requirements

| Component | Version |
|-----------|---------|
| Rust | 1.77+ |
| Node.js | 18+ |
| Tauri CLI | v2 |
| [Ollama](https://ollama.com) | latest (optional, for AI features) |

**Ollama model (recommended):** `llama3` or any instruction-tuned model

---

## Quick Start

```bash
# 1. Clone
git clone https://github.com/9t29zhmwdh-coder/LifePlanner.git
cd LifePlanner

# 2. Install frontend dependencies
cd frontend && npm install && cd ..

# 3. Development
cargo tauri dev

# 4. Build
cargo tauri build
```

To enable AI features, install [Ollama](https://ollama.com) and pull a model:
```bash
ollama pull llama3
```

Then set the Ollama URL in **Settings → Local AI**.

---

## Privacy

LifePlanner is designed for complete data sovereignty:

- All data stored locally in SQLite (`~/.local/share/LifePlanner/`)
- Calendar credentials stored in the OS keychain (macOS Keychain, Windows DPAPI, Linux SecretService)
- AI processing runs entirely on-device via Ollama. No data is sent to any server.
- No analytics, no crash reporting, no external connections

---

## Architecture

```
LifePlanner/
├── crates/
│   ├── lp-core/          # Core library: models, DB, calendar sync, AI, extractors
│   └── lp-cli/           # Optional CLI interface
├── src-tauri/            # Tauri backend + IPC commands
└── frontend/             # React + TypeScript + Tailwind UI
```

**Key technologies:**
- `sysinfo`, `ical`, `quick-xml` for data ingestion
- `sqlx + SQLite` with FTS5 for storage and search
- `keyring` for secure credential storage
- `reqwest + rustls` for CalDAV (fully local network support)
- `recharts`, `zustand`, `date-fns` on the frontend

---

**Author:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** Framework Preview · **Last Updated:** Juni 2026

<div align="center">
  <img src="RayStudio.png" alt="RayStudio Logo" width="120"/>
  <h1>LifePlanner</h1>
</div>

[🇬🇧 English Version](README.md)

**Bewusst offline: KI-Lebensplaner, entwickelt mit Rust, Tauri und lokaler KI via Ollama.**

LifePlanner erkennt Termine, Aufgaben, Projekte und Fristen automatisch aus E-Mails, PDFs und Notizen, verknüpft sie intelligent und hilft dir beim Planen des Tages; ohne dass ein einziges Byte dein Gerät verlässt.

[![CI](https://github.com/9t29zhmwdh-coder/LifePlanner/actions/workflows/ci.yml/badge.svg)](https://github.com/9t29zhmwdh-coder/LifePlanner/actions) [![CodeQL](https://github.com/9t29zhmwdh-coder/LifePlanner/actions/workflows/github-code-scanning/codeql/badge.svg)](https://github.com/9t29zhmwdh-coder/LifePlanner/security/code-scanning) [![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/9t29zhmwdh-coder/LifePlanner/badge)](https://securityscorecards.dev/viewer/?uri=github.com/9t29zhmwdh-coder/LifePlanner) [![OpenSSF Best Practices](https://www.bestpractices.dev/projects/13704/badge)](https://www.bestpractices.dev/projects/13704)

![Platform](https://img.shields.io/badge/Platform-macOS_%7C_Windows_%7C_Ubuntu-lightgrey) ![Rust](https://img.shields.io/badge/Rust-CE422B?logo=rust&logoColor=white) ![Tauri](https://img.shields.io/badge/Tauri-24C8D8?logo=tauri&logoColor=white) ![AI | Claude Code](https://img.shields.io/badge/AI-Claude_Code-black?logo=anthropic&logoColor=white) ![AI | Copilot](https://img.shields.io/badge/AI-Copilot-black?logo=github&logoColor=white) ![AI | Ollama](https://img.shields.io/badge/AI-Ollama-black?logo=ollama&logoColor=white)

> **So läuft das:** LifePlanner ist eine native Desktop-App (Tauri), kein Server und kein Browser-Tab. Sie öffnet ihr eigenes Fenster wie jedes installierte Programm und läuft vollständig offline.

![LifePlanner](docs/screenshot.de.png)

---

> 💾 **Download:** [macOS (DMG)](https://github.com/9t29zhmwdh-coder/LifePlanner/releases/latest/download/LifePlanner.dmg) · [Windows (Installer)](https://github.com/9t29zhmwdh-coder/LifePlanner/releases/latest/download/LifePlanner-Setup.exe) · [Linux (AppImage)](https://github.com/9t29zhmwdh-coder/LifePlanner/releases/latest/download/LifePlanner.AppImage): immer das neueste Release, nicht signiert/notarisiert (Gatekeeper/SmartScreen warnen beim ersten Start). Oder selbst aus dem Quellcode bauen, siehe Erste Schritte unten.

---

> 🌱 Neu hier? → [Schritt-für-Schritt-Anleitung für Einsteiger](GETTING_STARTED.md)

---

**In der Praxis:** Du bekommst eine native Desktop-App, die unübersichtliche E-Mails, PDFs und Notizen in einen strukturierten, konfliktgeprüften Tagesplan verwandelt. Erkennung, Kalender-Sync und Konflikterkennung funktionieren komplett ohne KI; die lokale KI-Zusammenfassung (via Ollama) ist eine optionale Ergänzung für eine Zusammenfassung in Klartext, keine Voraussetzung für die Nutzung der App.

## Funktionen

- **Intelligente Erkennung**: Text einfügen (E-Mail, Chat, Dokument) und LifePlanner erkennt Daten, Fristen und Aufgaben automatisch
- **Kalender-Sync**: ICS-Dateien und CalDAV
- **Konflikterkennung**: Überschneidungen werden sofort angezeigt
- **Freie-Zeitfenster-Finder**: Zeigt, wo im Tag noch Luft ist
- **Energie-Sortierung**: Aufgaben nach Fokus- / Kreativ- / Routine-Energie gruppiert
- **Projekt-Tracker**: Aufgaben in Projekte mit Fortschrittsanzeige bündeln
- **Tages-KI-Zusammenfassung**: Lokale KI erstellt einen verständlichen Tagesüberblick
- **Volltextsuche**: SQLite-FTS5-gestützte Sofortsuche über alle Termine und Aufgaben
- **100 % Offline**: Kein Cloud-Zwang, kein Account, kein Tracking

---

## Voraussetzungen

| Komponente | Version |
|-----------|---------|
| Rust | 1.77+ |
| Node.js | 18+ |
| Tauri CLI | v2 |
| [Ollama](https://ollama.com) | aktuell (optional, für KI-Funktionen) |

**Empfohlenes Ollama-Modell:** `llama3` oder ein anderes Instruction-Modell

---

## Schnellstart

```bash
# 1. Repository klonen
git clone https://github.com/9t29zhmwdh-coder/LifePlanner.git
cd LifePlanner

# 2. Frontend-Abhängigkeiten installieren
cd frontend && npm install && cd ..

# 3. Entwicklungsmodus
cargo tauri dev

# 4. Build
cargo tauri build
```

Für KI-Funktionen [Ollama](https://ollama.com) installieren und ein Modell laden:
```bash
ollama pull llama3
```

Dann unter **Einstellungen → Lokale KI** die Ollama-URL eintragen.

---

## Deinstallation / Datenbereinigung

Entferne die App wie auf deinem Betriebssystem üblich (auf macOS in den Papierkorb ziehen, unter Windows "Apps & Features").

Lokale Daten werden dabei nicht automatisch entfernt: siehe [Datenschutz](#datenschutz) unten für die genauen Pfade und den Schlüsselbund-Eintrag.

---

## Datenschutz

LifePlanner wurde für vollständige Datensouveränität entwickelt:

- Alle Daten werden lokal in SQLite gespeichert: `~/Library/Application Support/LifePlanner/` (macOS), `%APPDATA%\LifePlanner\` (Windows)
- Kalender-Zugangsdaten im OS-Schlüsselbund (macOS Keychain, Windows DPAPI), bei Bedarf manuell über Keychain Access / Credential Manager entfernbar
- KI-Verarbeitung läuft vollständig lokal via Ollama. Keine Daten verlassen das Gerät.
- Keine Analyse, kein Absturzbericht, keine externen Verbindungen

---

## Architektur

```
LifePlanner/
├── crates/
│   ├── lp-core/          # Kernbibliothek: Modelle, DB, Kalender-Sync, KI, Extraktoren
│   └── lp-cli/           # Optionales CLI
├── src-tauri/            # Tauri-Backend + IPC-Commands
└── frontend/             # React + TypeScript + Tailwind UI
```

**Wichtige Technologien:**
- `ical`, `quick-xml` für Dateneingabe
- `sqlx + SQLite` mit FTS5 für Speicherung und Suche
- `keyring` für sichere Zugangsdaten
- `reqwest + rustls` für CalDAV (vollständige Unterstützung lokaler Netzwerke)
- `recharts`, `zustand`, `date-fns` im Frontend

---

**Autor:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** Active · ![version](https://img.shields.io/github/v/release/9t29zhmwdh-coder/LifePlanner?color=6b7280&style=flat-square) · **Lizenz:** MIT

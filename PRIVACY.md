# Privacy Policy : LifePlanner

LifePlanner runs **fully offline** on your local machine.

## What data is processed

- Text, email content, PDF documents, and notes : parsed locally to extract events, tasks, and deadlines
- Calendar data : read/written via CalDAV to your local network calendar server

## What data leaves your machine

**Nothing external.** All AI inference runs via [Ollama](https://ollama.ai) locally.
CalDAV sync targets only your local network : no cloud calendar services are contacted by default.

## Storage

- A local SQLite (FTS5) database stores all parsed events, tasks, and links
- All data stays in your user home directory

## Third-party services

None. LifePlanner does not use cloud AI, analytics, or telemetry.

## Changes

This policy may be updated with new releases. Check the CHANGELOG for details.

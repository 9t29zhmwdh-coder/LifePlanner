CREATE TABLE IF NOT EXISTS projects (
    id            TEXT PRIMARY KEY,
    title         TEXT NOT NULL,
    description   TEXT,
    status        TEXT NOT NULL DEFAULT 'active',
    deadline      TEXT,
    task_ids      TEXT NOT NULL DEFAULT '[]',
    event_ids     TEXT NOT NULL DEFAULT '[]',
    tags          TEXT NOT NULL DEFAULT '[]',
    auto_detected INTEGER NOT NULL DEFAULT 0,
    color         TEXT NOT NULL DEFAULT '#58a6ff',
    created_at    TEXT NOT NULL,
    updated_at    TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_projects_status ON projects(status);

CREATE TABLE IF NOT EXISTS documents (
    id               TEXT PRIMARY KEY,
    title            TEXT NOT NULL,
    path             TEXT,
    content_preview  TEXT,
    kind             TEXT NOT NULL DEFAULT 'other',
    linked_event_ids TEXT NOT NULL DEFAULT '[]',
    linked_task_ids  TEXT NOT NULL DEFAULT '[]',
    tags             TEXT NOT NULL DEFAULT '[]',
    created_at       TEXT NOT NULL
);

CREATE VIRTUAL TABLE IF NOT EXISTS docs_fts USING fts5(
    id UNINDEXED,
    title,
    content_preview,
    content='documents',
    content_rowid='rowid'
);

CREATE TRIGGER IF NOT EXISTS docs_fts_insert AFTER INSERT ON documents BEGIN
    INSERT INTO docs_fts(rowid, id, title, content_preview)
    VALUES (new.rowid, new.id, new.title, new.content_preview);
END;

CREATE TABLE IF NOT EXISTS settings (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

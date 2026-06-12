CREATE TABLE IF NOT EXISTS events (
    id             TEXT PRIMARY KEY,
    title          TEXT NOT NULL,
    description    TEXT,
    start          TEXT NOT NULL,
    end            TEXT,
    all_day        INTEGER NOT NULL DEFAULT 0,
    location       TEXT,
    source         TEXT NOT NULL DEFAULT 'manual',
    calendar_id    TEXT,
    external_uid   TEXT,
    status         TEXT NOT NULL DEFAULT 'confirmed',
    recurrence     TEXT,
    linked_task_ids TEXT NOT NULL DEFAULT '[]',
    linked_doc_ids  TEXT NOT NULL DEFAULT '[]',
    tags           TEXT NOT NULL DEFAULT '[]',
    created_at     TEXT NOT NULL,
    updated_at     TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_events_start ON events(start);
CREATE INDEX IF NOT EXISTS idx_events_calendar ON events(calendar_id);
CREATE INDEX IF NOT EXISTS idx_events_status ON events(status);

CREATE VIRTUAL TABLE IF NOT EXISTS events_fts USING fts5(
    id UNINDEXED,
    title,
    description,
    location,
    content='events',
    content_rowid='rowid'
);

CREATE TRIGGER IF NOT EXISTS events_fts_insert AFTER INSERT ON events BEGIN
    INSERT INTO events_fts(rowid, id, title, description, location)
    VALUES (new.rowid, new.id, new.title, new.description, new.location);
END;

CREATE TRIGGER IF NOT EXISTS events_fts_delete AFTER DELETE ON events BEGIN
    INSERT INTO events_fts(events_fts, rowid, id, title, description, location)
    VALUES ('delete', old.rowid, old.id, old.title, old.description, old.location);
END;

CREATE TRIGGER IF NOT EXISTS events_fts_update AFTER UPDATE ON events BEGIN
    INSERT INTO events_fts(events_fts, rowid, id, title, description, location)
    VALUES ('delete', old.rowid, old.id, old.title, old.description, old.location);
    INSERT INTO events_fts(rowid, id, title, description, location)
    VALUES (new.rowid, new.id, new.title, new.description, new.location);
END;

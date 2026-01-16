use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use rusqlite::{Connection, params};

use rusqlite::OptionalExtension;

/// Current schema version for the SQLite database.
pub const SCHEMA_VERSION: i64 = 2;

#[derive(Debug)]
pub struct Store {
    conn: Connection,
    db_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct DocumentRow {
    pub id: i64,
    pub source: String,
    pub title: Option<String>,
    pub license: Option<String>,
    pub attribution: Option<String>,
    pub created_at: i64,
}

impl Store {
    /// Open (or create) the SQLite DB at `path`.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let db_path = path.as_ref().to_path_buf();

        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("failed to create db directory: {}", parent.display()))?;
        }

        let conn = Connection::open(&db_path)
            .with_context(|| format!("failed to open sqlite db: {}", db_path.display()))?;

        // Sensible defaults for our workload.
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;

        Ok(Self { conn, db_path })
    }

    pub fn path(&self) -> &Path {
        &self.db_path
    }

    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    /// Create / migrate schema to `SCHEMA_VERSION`.
    pub fn migrate(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS schema_migrations (
              version INTEGER NOT NULL
            );
            "#,
        )?;

        let current: i64 = self
            .conn
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
                [],
                |row| row.get(0),
            )
            .context("failed reading schema version")?;

        if current >= SCHEMA_VERSION {
            return Ok(());
        }

        for next in (current + 1)..=SCHEMA_VERSION {
            match next {
                1 => self.migration_v1()?,
                2 => self.migration_v2()?,
                _ => anyhow::bail!("unknown migration version: {next}"),
            }

            self.conn.execute(
                "INSERT INTO schema_migrations(version) VALUES (?1)",
                params![next],
            )?;
        }

        Ok(())
    }

    /// Quick smoke test: can we read/write?
    pub fn health_check(&self) -> Result<()> {
        self.conn
            .execute("CREATE TEMP TABLE IF NOT EXISTS __ddai_tmp (x INTEGER)", [])?;
        self.conn
            .execute("INSERT INTO __ddai_tmp(x) VALUES (1)", [])?;
        let x: i64 = self
            .conn
            .query_row("SELECT x FROM __ddai_tmp LIMIT 1", [], |r| r.get(0))?;
        if x != 1 {
            anyhow::bail!("unexpected db result")
        }
        Ok(())
    }

    fn column_exists(&self, table: &str, column: &str) -> Result<bool> {
        let mut stmt = self
            .conn
            .prepare(&format!("PRAGMA table_info({})", table))?;
        let rows = stmt.query_map([], |r| r.get::<_, String>(1))?;
        for r in rows {
            if r? == column {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn migration_v1(&self) -> Result<()> {
        // Phase 1 core tables: documents -> chunks -> embeddings
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS documents (
              id           INTEGER PRIMARY KEY AUTOINCREMENT,
              source       TEXT NOT NULL,
              title        TEXT,
              license      TEXT,
              attribution  TEXT,
              created_at   INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS chunks (
              id           INTEGER PRIMARY KEY AUTOINCREMENT,
              document_id  INTEGER NOT NULL,
              chunk_index  INTEGER NOT NULL,
              content      TEXT NOT NULL,
              token_count  INTEGER,
              sha256       TEXT NOT NULL,
              created_at   INTEGER NOT NULL,
              FOREIGN KEY(document_id) REFERENCES documents(id) ON DELETE CASCADE,
              UNIQUE(document_id, chunk_index)
            );

            CREATE TABLE IF NOT EXISTS embeddings (
              chunk_id     INTEGER NOT NULL,
              model        TEXT NOT NULL,
              dims         INTEGER NOT NULL,
              vector       BLOB NOT NULL,
              created_at   INTEGER NOT NULL,
              PRIMARY KEY(chunk_id, model),
              FOREIGN KEY(chunk_id) REFERENCES chunks(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_chunks_document_id ON chunks(document_id);
            CREATE INDEX IF NOT EXISTS idx_embeddings_model ON embeddings(model);
            "#,
        )?;

        // Touch time helper for sanity.
        let _ = unix_ms();
        Ok(())
    }

    fn migration_v2(&self) -> Result<()> {
        if !self.column_exists("chunks", "entity_id")? {
            self.conn
                .execute("ALTER TABLE chunks ADD COLUMN entity_id INTEGER", [])?;
        }

        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS entities (
              id           INTEGER PRIMARY KEY AUTOINCREMENT,
              kind         TEXT NOT NULL,
              api_index    TEXT NOT NULL,
              name         TEXT NOT NULL,
              source       TEXT NOT NULL,
              original_ref TEXT,
              json         TEXT NOT NULL,
              created_at   INTEGER NOT NULL,
              UNIQUE(kind, api_index)
            );

            CREATE INDEX IF NOT EXISTS idx_entities_kind ON entities(kind);
            CREATE INDEX IF NOT EXISTS idx_chunks_entity_id ON chunks(entity_id);
            "#,
        )?;

        Ok(())
    }

    pub fn insert_document(
        &self,
        source: &str,
        title: Option<&str>,
        license: Option<&str>,
        attribution: Option<&str>,
    ) -> Result<i64> {
        let now = unix_ms();
        self.conn.execute(
            r#"
            INSERT INTO documents(source, title, license, attribution, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            rusqlite::params![source, title, license, attribution, now],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn insert_chunk(
        &self,
        document_id: i64,
        chunk_index: i64,
        content: &str,
        token_count: Option<i64>,
        sha256: &str,
        entity_id: Option<i64>,
    ) -> Result<i64> {
        let now = unix_ms();
        self.conn.execute(
            r#"
            INSERT INTO chunks(document_id, chunk_index, content, token_count, sha256, created_at, entity_id)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
            rusqlite::params![document_id, chunk_index, content, token_count, sha256, now, entity_id],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn upsert_entity(
        &self,
        kind: &str,
        api_index: &str,
        name: &str,
        source: &str,
        original_ref: Option<&str>,
        json: &str,
    ) -> Result<i64> {
        let now = unix_ms();
        self.conn.execute(
            r#"
            INSERT INTO entities(kind, api_index, name, source, original_ref, json, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            ON CONFLICT(kind, api_index) DO UPDATE SET
              name=excluded.name,
              source=excluded.source,
              original_ref=excluded.original_ref,
              json=excluded.json
            "#,
            rusqlite::params![kind, api_index, name, source, original_ref, json, now],
        )?;

        let id: i64 = self.conn.query_row(
            "SELECT id FROM entities WHERE kind=?1 AND api_index=?2",
            rusqlite::params![kind, api_index],
            |r| r.get(0),
        )?;
        Ok(id)
    }

    pub fn get_document(&self, id: i64) -> Result<Option<DocumentRow>> {
        let row = self
            .conn
            .query_row(
                r#"
            SELECT id, source, title, license, attribution, created_at
            FROM documents
            WHERE id = ?1
            "#,
                rusqlite::params![id],
                |r| {
                    Ok(DocumentRow {
                        id: r.get(0)?,
                        source: r.get(1)?,
                        title: r.get(2)?,
                        license: r.get(3)?,
                        attribution: r.get(4)?,
                        created_at: r.get(5)?,
                    })
                },
            )
            .optional()?;

        Ok(row)
    }

    pub fn list_documents(&self, limit: i64) -> Result<Vec<DocumentRow>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT id, source, title, license, attribution, created_at
            FROM documents
            ORDER BY id DESC
            LIMIT ?1
            "#,
        )?;

        let rows = stmt
            .query_map(rusqlite::params![limit], |r| {
                Ok(DocumentRow {
                    id: r.get(0)?,
                    source: r.get(1)?,
                    title: r.get(2)?,
                    license: r.get(3)?,
                    attribution: r.get(4)?,
                    created_at: r.get(5)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(rows)
    }
}

pub fn unix_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrate_and_health_check() {
        let conn = Connection::open_in_memory().unwrap();
        let store = Store {
            conn,
            db_path: PathBuf::from(":memory:"),
        };
        store.migrate().unwrap();
        store.health_check().unwrap();
    }
}

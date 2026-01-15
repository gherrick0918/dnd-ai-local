use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use rusqlite::{Connection, params};

/// Current schema version for the SQLite database.
pub const SCHEMA_VERSION: i64 = 1;

#[derive(Debug)]
pub struct Store {
    conn: Connection,
    db_path: PathBuf,
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

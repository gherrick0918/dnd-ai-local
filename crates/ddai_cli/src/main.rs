use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "ddai", version, about = "Local D&D AI utilities (Phase 1 CLI)")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Create/migrate the SQLite database schema.
    InitDb,
}

fn main() -> Result<()> {
    let _ = dotenvy::dotenv(); // load .env if present
    let cli = Cli::parse();

    match cli.command {
        Command::InitDb => init_db(),
    }
}

fn init_db() -> Result<()> {
    let db_path =
        std::env::var("DDAI_DB_PATH").unwrap_or_else(|_| "./data/db/ddai.sqlite".to_string());

    let store =
        ddai_store::Store::open(&db_path).with_context(|| format!("opening db at {db_path}"))?;
    store.migrate().context("running migrations")?;
    store.health_check().context("db health check")?;

    println!(
        "DB ready: {} (schema v{})",
        store.path().display(),
        ddai_store::SCHEMA_VERSION
    );
    Ok(())
}

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
    InitDb,

    /// Ingest a text/markdown file into the DB as a document + chunks.
    Ingest {
        path: String,

        #[arg(long)]
        source: String,

        #[arg(long)]
        title: Option<String>,

        #[arg(long)]
        license: Option<String>,

        #[arg(long)]
        attribution: Option<String>,

        #[arg(long, default_value_t = 3500)]
        chunk_chars: usize,
    },

    /// Ingest D&D 5e API data (spells and monsters).
    IngestDnd5eapi {
        #[arg(long, default_value = "https://www.dnd5eapi.co")]
        base_url: String,

        #[arg(long)]
        limit: Option<usize>,

        #[arg(long, default_value = "dnd5eapi.co (SRD mirror)")]
        source: String,
    },

    /// List recent documents.
    ListDocs {
        #[arg(long, default_value_t = 10)]
        limit: i64,
    },
}

fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    let cli = Cli::parse();

    match cli.command {
        Command::InitDb => init_db(),
        Command::Ingest {
            path,
            source,
            title,
            license,
            attribution,
            chunk_chars,
        } => ingest(path, source, title, license, attribution, chunk_chars),
        Command::IngestDnd5eapi {
            base_url,
            limit,
            source,
        } => ingest_dnd5eapi(base_url, limit, source),
        Command::ListDocs { limit } => list_docs(limit),
    }
}

fn db_path() -> String {
    std::env::var("DDAI_DB_PATH").unwrap_or_else(|_| "./data/db/ddai.sqlite".to_string())
}

fn open_store() -> Result<ddai_store::Store> {
    let p = db_path();
    let store = ddai_store::Store::open(&p).with_context(|| format!("opening db at {p}"))?;
    store.migrate().context("running migrations")?;
    Ok(store)
}

fn init_db() -> Result<()> {
    let store = open_store()?;
    store.health_check().context("db health check")?;
    println!(
        "DB ready: {} (schema v{})",
        store.path().display(),
        ddai_store::SCHEMA_VERSION
    );
    Ok(())
}

fn ingest(
    path: String,
    source: String,
    title: Option<String>,
    license: Option<String>,
    attribution: Option<String>,
    chunk_chars: usize,
) -> Result<()> {
    let store = open_store()?;
    let doc_id = ddai_ingest::ingest_file(
        &store,
        &path,
        ddai_ingest::IngestOptions {
            source: &source,
            title: title.as_deref(),
            license: license.as_deref(),
            attribution: attribution.as_deref(),
            target_chunk_chars: chunk_chars,
        },
    )?;

    println!("Ingested document id={doc_id} from {path}");
    Ok(())
}

fn ingest_dnd5eapi(base_url: String, limit: Option<usize>, source: String) -> Result<()> {
    let store = open_store()?;
    ddai_ingest::dnd5eapi::ingest_spells_and_monsters(
        &store,
        ddai_ingest::dnd5eapi::Dnd5eApiOptions {
            base_url: &base_url,
            source: &source,
            limit,
        },
    )?;
    println!("DnD 5e API ingest completed successfully!");
    Ok(())
}

fn list_docs(limit: i64) -> Result<()> {
    let store = open_store()?;
    let docs = store.list_documents(limit)?;
    for d in docs {
        println!(
            "id={} source={} title={}",
            d.id,
            d.source,
            d.title.unwrap_or_else(|| "-".into())
        );
    }
    Ok(())
}

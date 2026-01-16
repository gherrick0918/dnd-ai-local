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
    ListDocs,

    Search {
        query: String,
        #[arg(long, default_value_t = 8)]
        k: i64,
    },

    ShowChunk {
        id: i64,
    },

    /// Ask a question using local retrieval + Ollama.
    /// Uses FTS to fetch top-k chunks as SOURCES, then prompts Ollama to answer with citations.
    Ask {
        question: String,

        #[arg(long, default_value_t = 8)]
        k: i64,

        /// Prompts directory (defaults to ./prompts)
        #[arg(long)]
        prompts_dir: Option<String>,
    },

    /// List entities (spells, monsters) in the database
    ListEntities {
        #[arg(long)]
        kind: Option<String>, // Filter by kind: "spells" or "monsters"

        #[arg(long, default_value_t = 20)]
        limit: i64,
    },
}

fn main() -> Result<()> {
    // Load .env file if it exists
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
        Command::ListDocs => list_docs(),
        Command::Search { query, k } => search(query, k),
        Command::ShowChunk { id } => show_chunk(id),
        Command::Ask {
            question,
            k,
            prompts_dir,
        } => ask(question, k, prompts_dir),
        Command::ListEntities { kind, limit } => list_entities(kind, limit),
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

fn list_docs() -> Result<()> {
    let store = open_store()?;
    let docs = store.list_documents(10)?;
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

fn search(query: String, k: i64) -> Result<()> {
    let store = open_store()?;
    let hits = store.search_chunks_fts(&query, k)?;
    for h in hits {
        println!(
            "chunk:{} doc:{} entity:{:?} score:{:.3}\n  {}\n",
            h.chunk_id, h.document_id, h.entity_id, h.score, h.snippet
        );
    }
    Ok(())
}

fn show_chunk(id: i64) -> Result<()> {
    let store = open_store()?;
    let chunk = store.get_chunk(id)?;
    match chunk {
        Some(c) => {
            println!(
                "chunk:{} doc:{} entity:{:?}\n\n{}",
                c.id, c.document_id, c.entity_id, c.content
            );
        }
        None => println!("chunk not found: {id}"),
    }
    Ok(())
}

fn ask(question: String, k: i64, prompts_dir: Option<String>) -> Result<()> {
    println!("Starting ask with question: '{}'", question);
    let store = open_store()?;

    // 1) Retrieve top-k chunks via FTS
    let hits = store.search_chunks_fts(&question, k)?;
    println!("Found {} search hits", hits.len());

    if hits.is_empty() {
        println!("No search hits. Try a different query or ingest more sources.");
        return Ok(());
    }

    // Print search results
    println!("Search results:");
    for h in &hits {
        println!(
            "  chunk:{} score:{:.3} snippet: {}",
            h.chunk_id, h.score, h.snippet
        );
    }

    // 2) Load prompts
    let dir = prompts_dir
        .or_else(|| std::env::var("DDAI_PROMPTS_DIR").ok())
        .unwrap_or_else(|| "./prompts".to_string());

    println!("Loading prompts from directory: {}", dir);

    let system = std::fs::read_to_string(format!("{dir}/system.md"))
        .with_context(|| format!("reading {dir}/system.md"))?;
    let task = std::fs::read_to_string(format!("{dir}/rules_qa.md"))
        .with_context(|| format!("reading {dir}/rules_qa.md"))?;

    println!(
        "Loaded system prompt ({} chars) and task prompt ({} chars)",
        system.len(),
        task.len()
    );

    // 3) Build SOURCES block (full chunk text)
    let mut allowed_ids: Vec<i64> = Vec::new();
    let mut sources = String::new();

    for h in hits {
        if let Some(c) = store.get_chunk(h.chunk_id)? {
            allowed_ids.push(c.id);
            sources.push_str(&format!(
                "[chunk:{} doc:{} entity:{:?}]\n{}\n\n",
                c.id, c.document_id, c.entity_id, c.content
            ));
        }
    }

    println!(
        "Built sources block with {} chunks ({} chars)",
        allowed_ids.len(),
        sources.len()
    );

    let prompt = format!("{system}\n\n{task}\n\nSOURCES:\n{sources}\nUSER QUESTION:\n{question}\n");

    println!("Full prompt length: {} chars", prompt.len());

    // 4) Call Ollama
    let host = std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://127.0.0.1:11434".into());
    let model = std::env::var("OLLAMA_MODEL").unwrap_or_else(|_| "llama3.1:8b".into());

    println!("Connecting to Ollama at {} using model {}", host, model);

    let client = ddai_llm::OllamaClient::new(host, model);

    match client.generate(&prompt) {
        Ok(answer) => {
            println!("Generated answer ({} chars):", answer.len());

            // 5) Basic citation sanity-check (warn if it cites unknown chunks)
            let unknown = find_cited_chunk_ids(&answer)
                .into_iter()
                .filter(|id| !allowed_ids.contains(id))
                .collect::<Vec<_>>();

            println!("{answer}");

            if !unknown.is_empty() {
                eprintln!(
                    "\n[warn] Model cited chunk IDs not in provided sources: {:?}\n\
                     (Try increasing --k or improving chunking/search.)",
                    unknown
                );
            }
        }
        Err(e) => {
            eprintln!("Error calling Ollama: {}", e);
            eprintln!("Make sure Ollama is running and the model is available.");
            eprintln!("Try: ollama serve");
            eprintln!("And: ollama pull llama3.1:8b");
            return Err(e);
        }
    }

    Ok(())
}

// Finds chunk ids referenced like "chunk:123" or "[chunk:123]".
fn find_cited_chunk_ids(text: &str) -> Vec<i64> {
    let mut out = Vec::new();
    let bytes = text.as_bytes();
    let mut i = 0;

    while i + 6 < bytes.len() {
        if bytes[i..].starts_with(b"chunk:") {
            i += 6;
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            if i > start {
                if let Some(n) = std::str::from_utf8(&bytes[start..i])
                    .ok()
                    .and_then(|s| s.parse::<i64>().ok())
                {
                    out.push(n);
                }
            }
        } else {
            i += 1;
        }
    }

    out.sort();
    out.dedup();
    out
}

fn list_entities(kind_filter: Option<String>, limit: i64) -> Result<()> {
    let store = open_store()?;

    let mut query = "SELECT id, kind, api_index, name, source FROM entities".to_string();
    let mut params: Vec<&dyn ddai_store::ToSql> = Vec::new();

    if let Some(kind) = &kind_filter {
        query.push_str(" WHERE kind = ?");
        params.push(kind as &dyn ddai_store::ToSql);
    }

    query.push_str(" ORDER BY kind, name LIMIT ?");
    params.push(&limit as &dyn ddai_store::ToSql);

    let conn = store.conn();
    let mut stmt = conn.prepare(&query)?;

    let entities = stmt.query_map(&params[..], |row| {
        Ok((
            row.get::<_, i64>(0)?,    // id
            row.get::<_, String>(1)?, // kind
            row.get::<_, String>(2)?, // api_index
            row.get::<_, String>(3)?, // name
            row.get::<_, String>(4)?, // source
        ))
    })?;

    println!("Entities in database:");
    for entity in entities {
        let (id, kind, api_index, name, source) = entity?;
        println!(
            "  {} | {} | {} | {} | {}",
            id, kind, api_index, name, source
        );
    }

    Ok(())
}

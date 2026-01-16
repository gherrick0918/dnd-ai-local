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
        #[arg(long, default_value_t = 20)]
        limit: i64,
    },

    /// List entities (spells, monsters, etc.).
    ListEntities {
        #[arg(long)]
        kind: String,

        #[arg(long, default_value_t = 20)]
        limit: i64,
    },

    /// Search chunks using full-text search.
    Search {
        query: String,

        #[arg(long, default_value_t = 10)]
        k: i64,
    },

    /// Show a specific chunk by ID.
    ShowChunk {
        id: i64,
    },

    /// Ask a question using local retrieval + Ollama.
    Ask {
        question: String,

        #[arg(long, default_value_t = 8)]
        k: i64,

        /// Prompts directory (defaults to ./prompts)
        #[arg(long)]
        prompts_dir: Option<String>,

        /// Output structured JSON with citations
        #[arg(long)]
        json: bool,
    },

    /// List available Ollama models.
    Models,
}

#[tokio::main]
async fn main() -> Result<()> {
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
        } => ingest_dnd5eapi(base_url, limit, source).await,
        Command::ListDocs { limit } => list_docs(limit),
        Command::ListEntities { kind, limit } => list_entities(kind, limit),
        Command::Search { query, k } => search(query, k),
        Command::ShowChunk { id } => show_chunk(id),
        Command::Ask {
            question,
            k,
            prompts_dir,
            json,
        } => ask(question, k, prompts_dir, json).await,
        Command::Models => models().await,
    }
}

fn open_store() -> Result<ddai_store::Store> {
    let store = ddai_store::Store::open("./data/db/ddai.sqlite")?;
    store.migrate()?; // Ensure schema is up to date
    Ok(store)
}

fn init_db() -> Result<()> {
    let store = open_store()?;
    store.health_check()?; // Verify database is working
    println!(
        "DB ready: ./data/db/ddai.sqlite (schema v{})",
        store.schema_version()
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
    let content =
        std::fs::read_to_string(&path).with_context(|| format!("Failed to read file: {}", path))?;

    let title = title.unwrap_or_else(|| {
        std::path::Path::new(&path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Untitled")
            .to_string()
    });

    let doc_id =
        store.ingest_document(&source, &title, license.as_deref(), attribution.as_deref())?;

    let chunks = ddai_ingest::chunk_text(&content, chunk_chars);
    for chunk in chunks {
        store.ingest_chunk(doc_id, None, &chunk)?;
    }

    println!("Ingested document id={} from {}", doc_id, path);
    Ok(())
}

async fn ingest_dnd5eapi(base_url: String, limit: Option<usize>, source: String) -> Result<()> {
    let store = open_store()?;
    ddai_ingest::ingest_dnd5eapi(&store, &base_url, limit, &source).await
}

fn list_docs(limit: i64) -> Result<()> {
    let store = open_store()?;
    let docs = store.list_recent_documents(limit)?;
    for doc in docs {
        println!(
            "id={} source={} title={}",
            doc.id,
            doc.source,
            doc.title.unwrap_or_else(|| "Untitled".to_string())
        );
    }
    Ok(())
}

fn list_entities(kind: String, limit: i64) -> Result<()> {
    let store = open_store()?;
    let entities = store.list_entities(&kind, limit)?;
    for entity in entities {
        println!("id={} name={}", entity.id, entity.name);
    }
    Ok(())
}

fn search(query: String, k: i64) -> Result<()> {
    let store = open_store()?;
    let hits = store.search_chunks_fts(&query, k)?;
    for hit in hits {
        println!(
            "chunk:{} doc:{} entity:{:?} score:{:.3}",
            hit.chunk_id, hit.document_id, hit.entity_id, hit.score
        );
        println!("  {}", hit.snippet);
        println!();
    }
    Ok(())
}

fn show_chunk(id: i64) -> Result<()> {
    let store = open_store()?;
    match store.get_chunk(id)? {
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

async fn ask(question: String, k: i64, prompts_dir: Option<String>, json: bool) -> Result<()> {
    let store = open_store()?;

    // 1) Retrieve top-k chunks via FTS
    println!("Starting ask with question: '{}'", question);
    let hits = store.search_chunks_fts(&question, k)?;
    if hits.is_empty() {
        println!("No search hits. Try a different query or ingest more sources.");
        return Ok(());
    }
    println!("Found {} search hits", hits.len());

    if !json {
        println!("Search results:");
        for hit in &hits {
            println!(
                "  chunk:{} score:{:.3} snippet: {}",
                hit.chunk_id, hit.score, hit.snippet
            );
        }
    }

    // 2) Load prompts
    let dir = prompts_dir
        .or_else(|| std::env::var("DDAI_PROMPTS_DIR").ok())
        .unwrap_or_else(|| "./prompts".to_string());

    println!("Loading prompts from directory: {}", dir);
    let system = std::fs::read_to_string(format!("{}/system.md", dir))
        .with_context(|| format!("reading {}/system.md", dir))?;
    let task = std::fs::read_to_string(format!("{}/rules_qa.md", dir))
        .with_context(|| format!("reading {}/rules_qa.md", dir))?;

    println!(
        "Loaded system prompt ({} chars) and task prompt ({} chars)",
        system.len(),
        task.len()
    );

    // 3) Build SOURCES block (full chunk text)
    let mut allowed_ids: Vec<i64> = Vec::new();
    let mut sources = String::new();

    for h in &hits {
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
        hits.len(),
        sources.len()
    );

    let prompt = format!(
        "{}\n\nSOURCES:\n{}\nUSER QUESTION:\n{}\n",
        task, sources, question
    );

    println!("Full prompt length: {} chars", prompt.len());

    // 4) Call Ollama
    let host = std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://127.0.0.1:11434".into());
    let model = std::env::var("OLLAMA_MODEL").unwrap_or_else(|_| "gemma2:2b".into());
    println!("Connecting to Ollama at {} using model {}", host, model);

    let client = ddai_llm::OllamaClient::new(host, model);

    if json {
        // Use structured JSON response with citations
        match client
            .generate_with_citations(&prompt, Some(&system), &allowed_ids)
            .await
        {
            Ok(citation_answer) => {
                println!("{}", serde_json::to_string_pretty(&citation_answer)?);
            }
            Err(e) => {
                println!("Error generating response: {}", e);
                return Err(e);
            }
        }
    } else {
        // Use regular text response
        match client
            .generate_with_options(&prompt, Some(&system), None, None)
            .await
        {
            Ok(response) => {
                println!("Generated answer ({} chars):", response.len());
                println!("{}", response);

                // Extract cited chunk IDs from the response
                let cited_ids = extract_chunk_ids(&response);
                if !cited_ids.is_empty() {
                    let valid_citations: Vec<i64> = cited_ids
                        .into_iter()
                        .filter(|id| allowed_ids.contains(id))
                        .collect();

                    if !valid_citations.is_empty() {
                        println!("\nCited chunks: {:?}", valid_citations);
                    }
                }
            }
            Err(e) => {
                println!("Error generating response: {}", e);
                return Err(e);
            }
        }
    }

    Ok(())
}

async fn models() -> Result<()> {
    let host = std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://127.0.0.1:11434".into());
    let client = ddai_llm::OllamaClient::new(host.clone(), "dummy".to_string());

    match client.list_models().await {
        Ok(models) => {
            if models.is_empty() {
                println!("No models found. Run 'ollama pull <model>' to install a model.");
            } else {
                println!("Available Ollama models:");
                for (name, size) in models {
                    let size_gb = size as f64 / (1024.0 * 1024.0 * 1024.0);
                    println!("  {} ({:.1} GB)", name, size_gb);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect to Ollama: {}", e);
            println!("Make sure Ollama is running and accessible at {}", host);
        }
    }

    Ok(())
}

// Helper function to extract chunk IDs from text response (for backwards compatibility)
fn extract_chunk_ids(text: &str) -> Vec<i64> {
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        if i + 5 < bytes.len() && &bytes[i..i + 6] == b"chunk:" {
            i += 6; // skip "chunk:"
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

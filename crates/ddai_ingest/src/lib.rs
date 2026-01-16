use anyhow::Result;
use sha2::{Digest, Sha256};
use std::path::Path;

use ddai_store::Store;

pub struct IngestOptions<'a> {
    pub source: &'a str,
    pub title: Option<&'a str>,
    pub license: Option<&'a str>,
    pub attribution: Option<&'a str>,
    pub target_chunk_chars: usize, // simple heuristic for now
}

pub fn ingest_file(store: &Store, path: impl AsRef<Path>, opts: IngestOptions<'_>) -> Result<i64> {
    let text = std::fs::read_to_string(&path)?;
    let doc_id = store.insert_document(opts.source, opts.title, opts.license, opts.attribution)?;

    let chunks = chunk_text(&text, opts.target_chunk_chars);

    for (i, chunk) in chunks.into_iter().enumerate() {
        let sha = sha256_hex(&chunk);
        let token_est = estimate_tokens(&chunk);
        store.insert_chunk(doc_id, i as i64, &chunk, Some(token_est), &sha)?;
    }

    Ok(doc_id)
}

fn chunk_text(text: &str, target_chars: usize) -> Vec<String> {
    // Paragraph-based: split on blank lines, then pack paragraphs up to ~target_chars.
    let paras: Vec<&str> = text
        .split("\n\n")
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .collect();

    let mut out = Vec::new();
    let mut buf = String::new();

    for p in paras {
        if !buf.is_empty() && (buf.len() + 2 + p.len()) > target_chars {
            out.push(buf.trim().to_string());
            buf.clear();
        }
        if !buf.is_empty() {
            buf.push_str("\n\n");
        }
        buf.push_str(p);
    }
    if !buf.trim().is_empty() {
        out.push(buf.trim().to_string());
    }
    out
}

fn sha256_hex(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    hex::encode(hasher.finalize())
}

fn estimate_tokens(s: &str) -> i64 {
    // crude but stable: ~4 chars per token average
    let chars = s.chars().count() as i64;
    (chars / 4).max(1)
}

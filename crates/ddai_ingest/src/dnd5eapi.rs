use anyhow::{Context, Result};
use ddai_store::Store;
use serde::Deserialize;
use serde_json::Value;

use sha2::{Digest, Sha256};

#[derive(Debug, Deserialize)]
struct ApiList {
    results: Vec<ApiRef>,
}

#[derive(Debug, Deserialize)]
struct ApiRef {
    index: String,
    name: String,
    url: String,
}

pub struct Dnd5eApiOptions<'a> {
    pub base_url: &'a str, // e.g. https://www.dnd5eapi.co
    pub source: &'a str,   // "dnd5eapi.co (SRD mirror)" or similar
    pub limit: Option<usize>,
}

pub fn ingest_spells_and_monsters(store: &Store, opts: Dnd5eApiOptions<'_>) -> Result<()> {
    ingest_kind(store, &opts, "spells")?;
    ingest_kind(store, &opts, "monsters")?;
    Ok(())
}

fn ingest_kind(store: &Store, opts: &Dnd5eApiOptions<'_>, kind: &str) -> Result<()> {
    let client = reqwest::blocking::Client::new();

    let list_url = format!("{}/api/{}", opts.base_url.trim_end_matches('/'), kind);
    let list: ApiList = client
        .get(&list_url)
        .send()
        .with_context(|| format!("GET {list_url}"))?
        .error_for_status()
        .with_context(|| format!("HTTP error {list_url}"))?
        .json()
        .with_context(|| format!("parse json {list_url}"))?;

    let title = format!("DND5EAPI: {}", kind);
    let doc_id = store.insert_document(opts.source, Some(&title), None, None)?;

    let mut count = 0usize;
    for (i, r) in list.results.iter().enumerate() {
        if let Some(limit) = opts.limit
            && i >= limit
        {
            break;
        }

        let item_url = format!(
            "{}/{}",
            opts.base_url.trim_end_matches('/'),
            r.url.trim_start_matches('/')
        );
        let v: Value = client
            .get(&item_url)
            .send()
            .with_context(|| format!("GET {item_url}"))?
            .error_for_status()
            .with_context(|| format!("HTTP error {item_url}"))?
            .json()
            .with_context(|| format!("parse json {item_url}"))?;

        let api_index = v.get("index").and_then(|x| x.as_str()).unwrap_or(&r.index);
        let name = v.get("name").and_then(|x| x.as_str()).unwrap_or(&r.name);

        let json = serde_json::to_string(&v)?;
        let entity_id =
            store.upsert_entity(kind, api_index, name, opts.source, Some(&item_url), &json)?;

        let rendered = render_entity(kind, &v);
        let sha = sha256_hex(&rendered);
        let token_est = estimate_tokens(&rendered);

        store.insert_chunk(
            doc_id,
            i as i64,
            &rendered,
            Some(token_est),
            &sha,
            Some(entity_id),
        )?;

        count += 1;
        if count.is_multiple_of(25) {
            println!("Ingested {count} {kind}...");
        }
    }

    println!("Ingest complete: {count} {kind} into document id={doc_id}");
    Ok(())
}

fn render_entity(kind: &str, v: &Value) -> String {
    match kind {
        "spells" => render_spell(v),
        "monsters" => render_monster(v),
        _ => format!("# {}\n\n{}", kind, v),
    }
}

fn render_spell(v: &Value) -> String {
    let name = s(v, "name");
    let level = v
        .get("level")
        .and_then(|x| x.as_i64())
        .map(|n| n.to_string())
        .unwrap_or("-".into());
    let school = v
        .get("school")
        .and_then(|x| x.get("name"))
        .and_then(|x| x.as_str())
        .unwrap_or("-");
    let casting_time = s(v, "casting_time");
    let range = s(v, "range");
    let duration = s(v, "duration");
    let components = v
        .get("components")
        .and_then(|x| x.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|i| i.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or("-".into());

    let desc = v
        .get("desc")
        .and_then(|x| x.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|i| i.as_str())
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default();

    let higher = v
        .get("higher_level")
        .and_then(|x| x.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|i| i.as_str())
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default();

    format!(
        "# Spell: {name}\n\n- Level: {level}\n- School: {school}\n- Casting Time: {casting_time}\n- Range: {range}\n- Components: {components}\n- Duration: {duration}\n\n## Description\n{desc}\n\n## Higher Level\n{higher}\n"
    )
}

fn render_monster(v: &Value) -> String {
    let name = s(v, "name");
    let size = s(v, "size");
    let mtype = v.get("type").and_then(|x| x.as_str()).unwrap_or("-");
    let alignment = s(v, "alignment");
    let ac = v
        .get("armor_class")
        .and_then(|x| x.as_array())
        .and_then(|a| a.first())
        .and_then(|x| x.get("value").or(Some(x)))
        .and_then(|x| x.as_i64())
        .map(|n| n.to_string())
        .unwrap_or("-".into());
    let hp = v
        .get("hit_points")
        .and_then(|x| x.as_i64())
        .map(|n| n.to_string())
        .unwrap_or("-".into());
    let cr = v
        .get("challenge_rating")
        .and_then(|x| x.as_f64())
        .map(|n| n.to_string())
        .unwrap_or("-".into());

    let str_ = n(v, "strength");
    let dex = n(v, "dexterity");
    let con = n(v, "constitution");
    let int_ = n(v, "intelligence");
    let wis = n(v, "wisdom");
    let cha = n(v, "charisma");

    let actions = v
        .get("actions")
        .and_then(|x| x.as_array())
        .map(|a| {
            a.iter()
                .map(|act| {
                    let an = act.get("name").and_then(|x| x.as_str()).unwrap_or("-");
                    let ad = act.get("desc").and_then(|x| x.as_str()).unwrap_or("");
                    format!("- **{an}:** {ad}")
                })
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default();

    format!(
        "# Monster: {name}\n\n- Size: {size}\n- Type: {mtype}\n- Alignment: {alignment}\n- AC: {ac}\n- HP: {hp}\n- CR: {cr}\n\n## Ability Scores\nSTR {str_} | DEX {dex} | CON {con} | INT {int_} | WIS {wis} | CHA {cha}\n\n## Actions\n{actions}\n"
    )
}

fn s(v: &Value, key: &str) -> String {
    v.get(key)
        .and_then(|x| x.as_str())
        .unwrap_or("-")
        .to_string()
}

fn n(v: &Value, key: &str) -> String {
    v.get(key)
        .and_then(|x| x.as_i64())
        .map(|n| n.to_string())
        .unwrap_or("-".into())
}

fn sha256_hex(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    hex::encode(hasher.finalize())
}

fn estimate_tokens(s: &str) -> i64 {
    let chars = s.chars().count() as i64;
    (chars / 4).max(1)
}

cargo fmt --all
cargo clippy --all-targets --all-features
cargo test
cargo run --bin ddai_cli
cargo run -p ddai_cli -- init-db
cargo run -p ddai_cli -- ingest .\data\raw\sample_rules.md --source "Sample Rules Pack" --title "Sample Rules Pack (Test Content)" --license "Internal test content" --attribution "Created by the developer for ingestion tests"
cargo run -p ddai_cli -- list-docs
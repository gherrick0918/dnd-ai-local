cargo fmt --all
cargo clippy --all-targets --all-features
cargo test
cargo run -p ddai_cli -- init-db
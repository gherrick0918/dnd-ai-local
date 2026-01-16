PS C:\dev\dnd-ai-local> ./buildandtest.ps1
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running unittests src\main.rs (target\debug\deps\ddai_cli-db64629aec67732e.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\lib.rs (target\debug\deps\ddai_core-d5bfabee4ffea726.exe)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\lib.rs (target\debug\deps\ddai_embed-e39a86e5b8292b90.exe)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\lib.rs (target\debug\deps\ddai_ingest-36f6dbfe7f2fc9aa.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\main.rs (target\debug\deps\ddai_ingest-268c2876f893a6be.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\lib.rs (target\debug\deps\ddai_llm-cbd469f8053f095b.exe)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\lib.rs (target\debug\deps\ddai_retrieve-de82e4354c7fdd54.exe)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\lib.rs (target\debug\deps\ddai_store-298d3f79f6397280.exe)

running 1 test
test tests::migrate_and_health_check ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ddai_core

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ddai_embed

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ddai_ingest

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ddai_llm

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ddai_retrieve

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ddai_store

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target\debug\ddai_cli.exe`
Local D&D AI utilities (Phase 1 CLI)

Usage: ddai_cli.exe <COMMAND>

Commands:
  init-db
  ingest     Ingest a text/markdown file into the DB as a document + chunks
  list-docs  List recent documents
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
error: process didn't exit successfully: `target\debug\ddai_cli.exe` (exit code: 2)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target\debug\ddai_cli.exe init-db`
DB ready: ./data/db/ddai.sqlite (schema v1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target\debug\ddai_cli.exe ingest .\data\raw\sample_rules.md --source "Sample Rules Pack" --title "Sample Rules Pack (Test Content)" --license "Internal test content" --attribution "Created by the developer for ingestion tests"`
Ingested document id=2 from .\data\raw\sample_rules.md
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target\debug\ddai_cli.exe list-docs`
id=2 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=1 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
PS C:\dev\dnd-ai-local> 
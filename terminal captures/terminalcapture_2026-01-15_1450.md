PS C:\dev\dnd-ai-local> ./buildandtest.ps1
    Checking ddai_store v0.1.0 (C:\dev\dnd-ai-local\crates\ddai_store)
    Checking ddai_cli v0.1.0 (C:\dev\dnd-ai-local\crates\ddai_cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.56s
   Compiling once_cell v1.21.3
   Compiling cfg-if v1.0.4
   Compiling zerocopy v0.8.33
   Compiling windows-link v0.2.1
   Compiling libsqlite3-sys v0.30.1
   Compiling fallible-iterator v0.3.0
   Compiling once_cell_polyfill v1.70.2
   Compiling anstyle v1.0.13
   Compiling windows-sys v0.61.2                                                                              
   Compiling bitflags v2.10.0
   Compiling smallvec v1.15.1                                                                                 
   Compiling utf8parse v0.2.2                                                                                 
   Compiling fallible-streaming-iterator v0.1.9                                                               
   Compiling anstyle-parse v0.2.7                                                                             
   Compiling anyhow v1.0.100                                                                                  
   Compiling colorchoice v1.0.4                                                                               
   Compiling is_terminal_polyfill v1.70.2                                                                     
   Compiling strsim v0.11.1                                                                                   
   Compiling clap_lex v0.7.7
   Compiling dotenvy v0.15.7                                                                                  
   Compiling anstyle-wincon v3.0.11                                                                           
   Compiling anstyle-query v1.1.5
   Compiling anstream v0.6.21                                                                                 
   Compiling clap_builder v4.5.54                                                                             
   Compiling ahash v0.8.12
   Compiling hashbrown v0.14.5
   Compiling hashlink v0.9.1
   Compiling rusqlite v0.32.1
   Compiling clap v4.5.54
   Compiling ddai_store v0.1.0 (C:\dev\dnd-ai-local\crates\ddai_store)
   Compiling ddai_cli v0.1.0 (C:\dev\dnd-ai-local\crates\ddai_cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 6.64s
     Running unittests src\main.rs (target\debug\deps\ddai_cli-bdae67d180b12fb2.exe)

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

     Running unittests src\main.rs (target\debug\deps\ddai_ingest-9feef775ce1a18ed.exe)

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

   Doc-tests ddai_llm

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ddai_retrieve

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ddai_store

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Compiling ddai_cli v0.1.0 (C:\dev\dnd-ai-local\crates\ddai_cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.93s
     Running `target\debug\ddai_cli.exe init-db`
DB ready: ./data/db/ddai.sqlite (schema v1)
PS C:\dev\dnd-ai-local> 
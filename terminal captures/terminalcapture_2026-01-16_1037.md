**********************
Windows PowerShell transcript start
Start time: 20260116103740
Username: GREG-ASUS-PC\gherr
RunAs User: GREG-ASUS-PC\gherr
Configuration Name: 
Machine: GREG-ASUS-PC (Microsoft Windows NT 10.0.26200.0)
Host Application: PowerShell Build and Test Script
Process ID: 23264
PSVersion: 5.1.26100.7462
PSEdition: Desktop
PSCompatibleVersions: 1.0, 2.0, 3.0, 4.0, 5.0, 5.1.26100.7462
BuildVersion: 10.0.26100.7462
CLRVersion: 4.0.30319.42000
WSManStackVersion: 3.0
PSRemotingProtocolVersion: 2.3
SerializationVersion: 1.1.0.1
**********************

PS C:\dev\dnd-ai-local> ./buildandtest.ps1
cmd.exe : warning: this `if` statement can be collapsed
At C:\dev\dnd-ai-local\buildandtest.ps1:43 char:15
+     $output = & cmd /c $Command 2>&1
+               ~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: this `...an be collapsed:String) [], R 
   emoteException
    + FullyQualifiedErrorId : NativeCommandError
 
  --> crates\ddai_ingest\src\dnd5eapi.rs:50:9
   |
50 | /         if let Some(limit) = opts.limit {
51 | |             if i >= limit {
52 | |                 break;
53 | |             }
54 | |         }
   | |_________^
   |
   = help: for further information visit 
https://rust-lang.github.io/rust-clippy/master/index.html#collapsible_if
   = note: `#[warn(clippy::collapsible_if)]` on by default
help: collapse nested if block
   |
50 ~         if let Some(limit) = opts.limit
51 ~             && i >= limit {
52 |                 break;
53 ~             }
   |

warning: manual implementation of `.is_multiple_of()`
  --> crates\ddai_ingest\src\dnd5eapi.rs:91:12
   |
91 |         if count % 25 == 0 {
   |            ^^^^^^^^^^^^^^^ help: replace with: `count.is_multiple_of(25)`
   |
   = help: for further information visit 
https://rust-lang.github.io/rust-clippy/master/index.html#manual_is_multiple_of
   = note: `#[warn(clippy::manual_is_multiple_of)]` on by default

warning: accessing first element with `a.get(0)`
   --> crates\ddai_ingest\src\dnd5eapi.rs:169:23
    |
169 |         .and_then(|a| a.get(0))
    |                       ^^^^^^^^ help: try: `a.first()`
    |
    = help: for further information visit 
https://rust-lang.github.io/rust-clippy/master/index.html#get_first
    = note: `#[warn(clippy::get_first)]` on by default

warning: `ddai_ingest` (lib) generated 3 warnings (run `cargo clippy --fix --lib -p 
ddai_ingest` to apply 3 suggestions)
warning: `ddai_ingest` (lib test) generated 3 warnings (3 duplicates)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
cmd.exe :     Finished `test` profile [unoptimized + debuginfo] target(s) in 0.26s
At C:\dev\dnd-ai-local\buildandtest.ps1:43 char:15
+     $output = & cmd /c $Command 2>&1
+               ~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `t...get(s) in 0.26s:String) [], R 
   emoteException
    + FullyQualifiedErrorId : NativeCommandError
 
     Running unittests src\main.rs (target\debug\deps\ddai_cli-b47ae3698efcb64d.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\lib.rs (target\debug\deps\ddai_core-d5bfabee4ffea726.exe)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running unittests src\lib.rs (target\debug\deps\ddai_embed-e39a86e5b8292b90.exe)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\lib.rs (target\debug\deps\ddai_ingest-24768ba619cf02aa.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\main.rs (target\debug\deps\ddai_ingest-0c32d51043c7646e.exe)

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

     Running unittests src\lib.rs (target\debug\deps\ddai_store-ecfebf4939d0b0d3.exe)

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

cmd.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
At C:\dev\dnd-ai-local\buildandtest.ps1:43 char:15
+     $output = & cmd /c $Command 2>&1
+               ~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.19s:String) [], R 
   emoteException
    + FullyQualifiedErrorId : NativeCommandError
 
     Running `target\debug\ddai_cli.exe`
Local D&D AI utilities (Phase 1 CLI)

Usage: ddai_cli.exe <COMMAND>

Commands:
  init-db          
  ingest           Ingest a text/markdown file into the DB as a document + chunks
  ingest-dnd5eapi  Ingest D&D 5e API data (spells and monsters)
  list-docs        List recent documents
  help             Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
error: process didn't exit successfully: `target\debug\ddai_cli.exe` (exit code: 2)
cmd.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
At C:\dev\dnd-ai-local\buildandtest.ps1:43 char:15
+     $output = & cmd /c $Command 2>&1
+               ~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.18s:String) [], R 
   emoteException
    + FullyQualifiedErrorId : NativeCommandError
 
     Running `target\debug\ddai_cli.exe init-db`
DB ready: ./data/db/ddai.sqlite (schema v2)
cmd.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
At C:\dev\dnd-ai-local\buildandtest.ps1:43 char:15
+     $output = & cmd /c $Command 2>&1
+               ~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.19s:String) [], R 
   emoteException
    + FullyQualifiedErrorId : NativeCommandError
 
     Running `target\debug\ddai_cli.exe ingest .\data\raw\sample_rules.md --source "Sample 
Rules Pack" --title "Sample Rules Pack (Test Content)" --license "Internal test content" 
--attribution "Created by the developer for ingestion tests"`
Ingested document id=6 from .\data\raw\sample_rules.md
cmd.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
At C:\dev\dnd-ai-local\buildandtest.ps1:43 char:15
+     $output = & cmd /c $Command 2>&1
+               ~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.18s:String) [], R 
   emoteException
    + FullyQualifiedErrorId : NativeCommandError
 
     Running `target\debug\ddai_cli.exe list-docs`
id=6 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=5 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=4 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells
id=3 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=2 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=1 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
cmd.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
At C:\dev\dnd-ai-local\buildandtest.ps1:43 char:15
+     $output = & cmd /c $Command 2>&1
+               ~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.18s:String) [], R 
   emoteException
    + FullyQualifiedErrorId : NativeCommandError
 
     Running `target\debug\ddai_cli.exe ingest-dnd5eapi --base-url https://www.dnd5eapi.co 
--limit 25 --source "dnd5eapi.co (SRD mirror)"`
Ingested 25 spells...
Ingest complete: 25 spells into document id=7
Ingested 25 monsters...
Ingest complete: 25 monsters into document id=8
DnD 5e API ingest completed successfully!
cmd.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
At C:\dev\dnd-ai-local\buildandtest.ps1:43 char:15
+     $output = & cmd /c $Command 2>&1
+               ~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.18s:String) [], R 
   emoteException
    + FullyQualifiedErrorId : NativeCommandError
 
     Running `target\debug\ddai_cli.exe list-docs`
id=8 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=7 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells
id=6 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=5 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=4 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells
id=3 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=2 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=1 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
PS C:\dev\dnd-ai-local>

**********************
Windows PowerShell transcript end
End time: 20260116103746
**********************

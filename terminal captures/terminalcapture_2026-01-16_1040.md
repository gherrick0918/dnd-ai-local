**********************
Windows PowerShell transcript start
Start time: 20260116104019
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
cmd.exe :     Checking ddai_ingest v0.1.0 (C:\dev\dnd-ai-local\crates\ddai_ingest)
At C:\dev\dnd-ai-local\buildandtest.ps1:43 char:15
+     $output = & cmd /c $Command 2>&1
+               ~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Checking dd...es\ddai_ingest):String) [], R 
   emoteException
    + FullyQualifiedErrorId : NativeCommandError
 
    Checking ddai_cli v0.1.0 (C:\dev\dnd-ai-local\crates\ddai_cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.73s
cmd.exe :    Compiling ddai_ingest v0.1.0 (C:\dev\dnd-ai-local\crates\ddai_ingest)
At C:\dev\dnd-ai-local\buildandtest.ps1:43 char:15
+     $output = & cmd /c $Command 2>&1
+               ~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling dd...es\ddai_ingest):String) [], R 
   emoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   Compiling ddai_cli v0.1.0 (C:\dev\dnd-ai-local\crates\ddai_cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.22s
     Running unittests src\main.rs (target\debug\deps\ddai_cli-b47ae3698efcb64d.exe)

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

cmd.exe :    Compiling ddai_cli v0.1.0 (C:\dev\dnd-ai-local\crates\ddai_cli)
At C:\dev\dnd-ai-local\buildandtest.ps1:43 char:15
+     $output = & cmd /c $Command 2>&1
+               ~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling dd...rates\ddai_cli):String) [], R 
   emoteException
    + FullyQualifiedErrorId : NativeCommandError
 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.87s
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
cmd.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
At C:\dev\dnd-ai-local\buildandtest.ps1:43 char:15
+     $output = & cmd /c $Command 2>&1
+               ~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.20s:String) [], R 
   emoteException
    + FullyQualifiedErrorId : NativeCommandError
 
     Running `target\debug\ddai_cli.exe init-db`
DB ready: ./data/db/ddai.sqlite (schema v2)
cmd.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
At C:\dev\dnd-ai-local\buildandtest.ps1:43 char:15
+     $output = & cmd /c $Command 2>&1
+               ~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.27s:String) [], R 
   emoteException
    + FullyQualifiedErrorId : NativeCommandError
 
     Running `target\debug\ddai_cli.exe ingest .\data\raw\sample_rules.md --source "Sample 
Rules Pack" --title "Sample Rules Pack (Test Content)" --license "Internal test content" 
--attribution "Created by the developer for ingestion tests"`
Ingested document id=9 from .\data\raw\sample_rules.md
cmd.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
At C:\dev\dnd-ai-local\buildandtest.ps1:43 char:15
+     $output = & cmd /c $Command 2>&1
+               ~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.20s:String) [], R 
   emoteException
    + FullyQualifiedErrorId : NativeCommandError
 
     Running `target\debug\ddai_cli.exe list-docs`
id=9 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=8 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=7 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells
id=6 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=5 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=4 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells
id=3 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=2 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=1 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
cmd.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
At C:\dev\dnd-ai-local\buildandtest.ps1:43 char:15
+     $output = & cmd /c $Command 2>&1
+               ~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.21s:String) [], R 
   emoteException
    + FullyQualifiedErrorId : NativeCommandError
 
     Running `target\debug\ddai_cli.exe ingest-dnd5eapi --base-url https://www.dnd5eapi.co 
--limit 25 --source "dnd5eapi.co (SRD mirror)"`
Ingested 25 spells...
Ingest complete: 25 spells into document id=10
Ingested 25 monsters...
Ingest complete: 25 monsters into document id=11
DnD 5e API ingest completed successfully!
cmd.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
At C:\dev\dnd-ai-local\buildandtest.ps1:43 char:15
+     $output = & cmd /c $Command 2>&1
+               ~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.19s:String) [], R 
   emoteException
    + FullyQualifiedErrorId : NativeCommandError
 
     Running `target\debug\ddai_cli.exe list-docs`
id=11 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=10 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells
id=9 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=8 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=7 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells
id=6 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=5 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=4 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells
id=3 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=2 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
PS C:\dev\dnd-ai-local>

**********************
Windows PowerShell transcript end
End time: 20260116104029
**********************

**********************
Windows PowerShell transcript start
Start time: 20260116144411
Username: GREG-ASUS-PC\gherr
RunAs User: GREG-ASUS-PC\gherr
Configuration Name: 
Machine: GREG-ASUS-PC (Microsoft Windows NT 10.0.26200.0)
Host Application: PowerShell Build and Test Script
Process ID: 31636
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
PS C:\dev\dnd-ai-local> cargo fmt --all

PS C:\dev\dnd-ai-local> cargo clippy --all-targets --all-features

PS C:\dev\dnd-ai-local> cargo check -p ddai_llm

PS C:\dev\dnd-ai-local> cargo check -p ddai_cli

PS C:\dev\dnd-ai-local> cargo build

PS C:\dev\dnd-ai-local> cargo test

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


PS C:\dev\dnd-ai-local> cargo run --bin ddai_cli

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- init-db
DB ready: ./data/db/ddai.sqlite (schema v3)

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- ingest .\data\raw\sample_rules.md --source 'Sample Rules Pack' --title 'Sample Rules Pack (Test Content)' --license 'Internal test content' --attribution 'Created by the developer for ingestion tests'
Ingested document id=41 from .\data\raw\sample_rules.md

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- list-docs
id=41 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=40 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=39 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells
id=38 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=37 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=36 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells
id=35 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=34 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=33 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells
id=32 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=31 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells
id=30 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=29 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=28 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells
id=27 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=26 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=25 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells
id=24 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=23 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=22 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- ingest-dnd5eapi --base-url https://www.dnd5eapi.co --source 'dnd5eapi.co (SRD mirror)'

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- list-docs
id=41 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=40 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=39 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells
id=38 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=37 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=36 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells
id=35 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=34 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=33 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells
id=32 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=31 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells
id=30 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=29 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=28 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells
id=27 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=26 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=25 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells
id=24 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=23 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=22 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- list-entities --kind monsters --limit 10
id=26 name=Aboleth
id=27 name=Acolyte
id=28 name=Adult Black Dragon
id=29 name=Adult Blue Dragon
id=30 name=Adult Brass Dragon
id=31 name=Adult Bronze Dragon
id=32 name=Adult Copper Dragon
id=33 name=Adult Gold Dragon
id=34 name=Adult Green Dragon
id=35 name=Adult Red Dragon

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- list-entities --kind spells --limit 10
id=1 name=Acid Arrow
id=2 name=Acid Splash
id=3 name=Aid
id=4 name=Alarm
id=5 name=Alter Self
id=6 name=Animal Friendship
id=7 name=Animal Messenger
id=8 name=Animal Shapes
id=9 name=Animate Dead
id=10 name=Animate Objects

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- models
Available Ollama models:
  llama3.2:3b (1.9 GB)
  mistral:7b (4.1 GB)
  gemma2:2b (1.5 GB)
  phi3.5:3.8b (2.0 GB)
  qwen2.5:0.5b (0.4 GB)
  llama3.2:1b (1.2 GB)
  llama3.2:latest (1.9 GB)
  llama3.1:8b (4.6 GB)
  phi3:mini (2.0 GB)
  llama3:8b (4.3 GB)
  nomic-embed-text:latest (0.3 GB)
  gemma:2b-instruct (1.4 GB)
  mistral:latest (4.1 GB)

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- search 'advantage' --k 5

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- search 'goblin' --k 5

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- search 'bronze dragon' --k 5

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- search 'dragon armor class' --k 5

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- show-chunk 1
chunk:1 doc:1 entity:None

# Sample Rules Pack (Test Content)

Purpose: This file exists to test ingestion, chunking, retrieval, and citation output.
It is NOT official D&D text. It is intentionally original, short, and structured.

## 1. The Core Loop

A round represents a short burst of action. Each round is broken into turns.  
During a turn, a creature usually does three things:
1) moves (optional),
2) performs one main action, and
3) may do a quick action if the scenario allows it.

Design note: This is a simplified test loop for software behavior, not a faithful copy of any ruleset.

## 2. Actions (Test Set)

Main actions (choose one on your turn):
- **Strike:** Make one attack roll using your best weapon or spell focus.
- **Cast:** Use a prepared spell or effect that targets a creature, area, or object.
- **Dash:** Double your movement for the turn.
- **Guard:** Gain +2 to defense checks until the start of your next turn.
- **Help:** Give an ally advantage on one check or attack they make before your next turn.
- **Use Item:** Consume or activate one item you carry.
- **Interact:** Pull a lever, open a door, draw a weapon, stow a weapon, etc.

Quick actions (0 or 1 per turn, if allowed by the current scene):
- **Shove aside:** Move a nearby creature 5 feet if you win an opposed check.
- **Snap shot:** Make a weak ranged attack that deals half damage.
- **Recall detail:** Ask the GM one focused question about what your character would likely know.

## 3. Movement and Positioning

Movement is measured in feet. If you do not have exact measurements, estimate with zones:
- **Near:** within a few steps
- **Far:** across the room
- **Distant:** requires significant travel or multiple turns

Difficult terrain halves movement.

## 4. Checks (Test Resolution)

When you attempt something risky, roll:
- d20 + relevant bonus

Compare the total to a Difficulty Class (DC):
- DC 10: easy
- DC 15: moderate
- DC 20: hard
- DC 25: extreme

If two creatures oppose each other, both roll and the higher total wins.

## 5. Advantage / Disadvantage (Test Mechanic)

Some situations grant an edge or a penalty.

- **Advantage:** roll two d20s and take the higher.
- **Disadvantage:** roll two d20s and take the lower.

If you have both advantage and disadvantage at the same time, they cancel out and you roll one d20.

## 6. Conditions (Test List)

Conditions apply ongoing effects until removed.

- **Stunned:** you lose your main action on your turn.
- **Slowed:** your movement is halved.
- **Weakened:** your damage is halved.
- **Marked:** the creature that marked you has advantage on attacks against you.
- **Inspired:** you may reroll one failed check, then the condition ends.

## 7. Healing and Resting

Healing restores hit points (HP).

- **Quick rest:** spend 10 minutes; regain a small amount of HP (GM decides a number).
- **Long rest:** spend 8 hours; restore HP to full and clear most conditions.

If a condition is ΓÇ£cursedΓÇ¥ or ΓÇ£persistent,ΓÇ¥ it may require a special cure rather than rest.

## 8. Example Q&A Prompts (For Testing Retrieval)

Use these to test your CLI:
- ΓÇ£What does advantage do?ΓÇ¥
- ΓÇ£How do opposed checks work?ΓÇ¥
- ΓÇ£What does the Marked condition do?ΓÇ¥
- ΓÇ£What happens on a long rest?ΓÇ¥
- ΓÇ£What are the main actions?ΓÇ¥

Expected behavior: your system should answer using only the ingested text and cite the chunk IDs used.

## 9. Optional: Fake Source Metadata

Source label suggestion for testing:
- source: "Sample Rules Pack"
- title: "Sample Rules Pack (Test Content)"
- license: "Internal test content"
- attribution: "Created by the developer for ingestion tests"

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- ask 'What does advantage do?' --k 6
Starting ask with question: 'What does advantage do?'

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- ask 'What is Armor Class for a goblin?' --k 8
Starting ask with question: 'What is Armor Class for a goblin?'

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- ask 'What is the AC of a Bronze Dragon?' --k 8
Starting ask with question: 'What is the AC of a Bronze Dragon?'

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- ask 'What is the AC of an Adult Bronze Dragon?' --k 8
Starting ask with question: 'What is the AC of an Adult Bronze Dragon?'

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- ask 'What does advantage do?' --k 6 --json
Starting ask with question: 'What does advantage do?'

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- ask 'What is Armor Class (AC)?' --k 8 --json
Starting ask with question: 'What is Armor Class (AC)?'

PS C:\dev\dnd-ai-local>

**********************
Windows PowerShell transcript end
End time: 20260116144518
**********************

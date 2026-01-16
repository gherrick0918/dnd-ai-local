**********************
Windows PowerShell transcript start
Start time: 20260116150107
Username: GREG-ASUS-PC\gherr
RunAs User: GREG-ASUS-PC\gherr
Configuration Name: 
Machine: GREG-ASUS-PC (Microsoft Windows NT 10.0.26200.0)
Host Application: PowerShell Build and Test Script
Process ID: 29776
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
Ingested document id=4 from .\data\raw\sample_rules.md

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- list-docs
id=4 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=3 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=2 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells
id=1 source=Sample Rules Pack title=Sample Rules Pack (Test Content)

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- ingest-dnd5eapi --base-url https://www.dnd5eapi.co --source 'dnd5eapi.co (SRD mirror)'
Ingested 25 spells...
Ingested 50 spells...
Ingested 75 spells...
Ingested 100 spells...
Ingested 125 spells...
Ingested 150 spells...
Ingested 175 spells...
Ingested 200 spells...
Ingested 225 spells...
Ingested 250 spells...
Ingested 275 spells...
Ingested 300 spells...
Ingest complete: 319 spells into document id=5
Ingested 25 monsters...
Ingested 50 monsters...
Ingested 75 monsters...
Ingested 100 monsters...
Ingested 125 monsters...
Ingested 150 monsters...
Ingested 175 monsters...
Ingested 200 monsters...
Ingested 225 monsters...
Ingested 250 monsters...
Ingested 275 monsters...
Ingested 300 monsters...
Ingested 325 monsters...
Ingest complete: 334 monsters into document id=6

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- list-docs
id=6 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=5 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells
id=4 source=Sample Rules Pack title=Sample Rules Pack (Test Content)
id=3 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: monsters
id=2 source=dnd5eapi.co (SRD mirror) title=DND5EAPI: spells
id=1 source=Sample Rules Pack title=Sample Rules Pack (Test Content)

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- list-entities --kind monsters --limit 10
id=320 name=Aboleth
id=321 name=Acolyte
id=322 name=Adult Black Dragon
id=323 name=Adult Blue Dragon
id=324 name=Adult Brass Dragon
id=325 name=Adult Bronze Dragon
id=326 name=Adult Copper Dragon
id=327 name=Adult Gold Dragon
id=328 name=Adult Green Dragon
id=329 name=Adult Red Dragon

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
chunk:1 doc:1 entity:None score:1.000
  # Sample Rules Pack (Test Content)
 
 Purpose: This file exists to test ingestion, chunking, retrieval, and citation output.
 It is NOT official D&D text. It is intentionally original, short, and stru

chunk:16 doc:2 entity:Some(15) score:1.000
  # Spell: Arcane Hand  - Level: 5 - School: Evocation - Casting Time: 1 action - Range: 120 feet - Components: V, S, M - Duration: Up to 1 minute  ## Description You create a Large hand of shimmering, 

chunk:26 doc:2 entity:Some(25) score:1.000
  # Spell: Beacon of Hope  - Level: 3 - School: Abjuration - Casting Time: 1 action - Range: 30 feet - Components: V, S - Duration: Up to 1 minute  ## Description This spell bestows hope and vitality. C

chunk:40 doc:2 entity:Some(39) score:1.000
  # Spell: Charm Person  - Level: 1 - School: Enchantment - Casting Time: 1 action - Range: 30 feet - Components: V, S - Duration: 1 hour  ## Description You attempt to charm a humanoid you can see with

chunk:91 doc:2 entity:Some(90) score:1.000
  # Spell: Dominate Beast  - Level: 4 - School: Enchantment - Casting Time: 1 action - Range: 60 feet - Components: V, S - Duration: Up to 1 minute  ## Description You attempt to beguile a creature that


PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- search 'goblin' --k 5
chunk:467 doc:3 entity:Some(466) score:1.000
  # Monster: Goblin  - Size: Small - Type: humanoid - Alignment: neutral evil - Armor Class (AC): 15 - Hit Points (HP): 7 - Challenge Rating (CR): 0.25  ## Ability Scores STR 8 | DEX 14 | CON 10 | INT 1

chunk:1121 doc:6 entity:Some(466) score:1.000
  # Monster: Goblin  - Size: Small - Type: humanoid - Alignment: neutral evil - Armor Class (AC): 15 - Hit Points (HP): 7 - Challenge Rating (CR): 0.25  ## Ability Scores STR 8 | DEX 14 | CON 10 | INT 1


PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- search 'bronze dragon' --k 5
chunk:326 doc:3 entity:Some(325) score:1.000
  # Monster: Adult Bronze Dragon  - Size: Huge - Type: dragon - Alignment: lawful good - Armor Class (AC): 19 - Hit Points (HP): 212 - Challenge Rating (CR): 15  ## Ability Scores STR 25 | DEX 10 | CON 

chunk:337 doc:3 entity:Some(336) score:1.000
  # Monster: Ancient Bronze Dragon  - Size: Gargantuan - Type: dragon - Alignment: lawful good - Armor Class (AC): 22 - Hit Points (HP): 444 - Challenge Rating (CR): 22  ## Ability Scores STR 29 | DEX 1

chunk:374 doc:3 entity:Some(373) score:1.000
  # Monster: Bronze Dragon Wyrmling  - Size: Medium - Type: dragon - Alignment: lawful good - Armor Class (AC): 17 - Hit Points (HP): 32 - Challenge Rating (CR): 2  ## Ability Scores STR 17 | DEX 10 | C

chunk:647 doc:3 entity:Some(646) score:1.000
  # Monster: Young Bronze Dragon  - Size: Large - Type: dragon - Alignment: lawful good - Armor Class (AC): 18 - Hit Points (HP): 142 - Challenge Rating (CR): 8  ## Ability Scores STR 21 | DEX 10 | CON 

chunk:980 doc:6 entity:Some(325) score:1.000
  # Monster: Adult Bronze Dragon  - Size: Huge - Type: dragon - Alignment: lawful good - Armor Class (AC): 19 - Hit Points (HP): 212 - Challenge Rating (CR): 15  ## Ability Scores STR 25 | DEX 10 | CON 


PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- search 'dragon armor class' --k 5
chunk:323 doc:3 entity:Some(322) score:1.000
  # Monster: Adult Black Dragon  - Size: Huge - Type: dragon - Alignment: chaotic evil - Armor Class (AC): 19 - Hit Points (HP): 195 - Challenge Rating (CR): 14  ## Ability Scores STR 23 | DEX 14 | CON 

chunk:324 doc:3 entity:Some(323) score:1.000
  # Monster: Adult Blue Dragon  - Size: Huge - Type: dragon - Alignment: lawful evil - Armor Class (AC): 19 - Hit Points (HP): 225 - Challenge Rating (CR): 16  ## Ability Scores STR 25 | DEX 10 | CON 23

chunk:325 doc:3 entity:Some(324) score:1.000
  # Monster: Adult Brass Dragon  - Size: Huge - Type: dragon - Alignment: chaotic good - Armor Class (AC): 18 - Hit Points (HP): 172 - Challenge Rating (CR): 13  ## Ability Scores STR 23 | DEX 10 | CON 

chunk:326 doc:3 entity:Some(325) score:1.000
  # Monster: Adult Bronze Dragon  - Size: Huge - Type: dragon - Alignment: lawful good - Armor Class (AC): 19 - Hit Points (HP): 212 - Challenge Rating (CR): 15  ## Ability Scores STR 25 | DEX 10 | CON 

chunk:327 doc:3 entity:Some(326) score:1.000
  # Monster: Adult Copper Dragon  - Size: Huge - Type: dragon - Alignment: chaotic good - Armor Class (AC): 18 - Hit Points (HP): 184 - Challenge Rating (CR): 14  ## Ability Scores STR 23 | DEX 12 | CON


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
End time: 20260116150148
**********************

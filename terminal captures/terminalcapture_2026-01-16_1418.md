**********************
Windows PowerShell transcript start
Start time: 20260116141848
Username: GREG-ASUS-PC\gherr
RunAs User: GREG-ASUS-PC\gherr
Configuration Name: 
Machine: GREG-ASUS-PC (Microsoft Windows NT 10.0.26200.0)
Host Application: PowerShell Build and Test Script
Process ID: 21320
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
Ingested document id=38 from .\data\raw\sample_rules.md

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- list-docs
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
Ingest complete: 319 spells into document id=39
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
Ingest complete: 334 monsters into document id=40
DnD 5e API ingest completed successfully!

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- list-docs
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

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- list-entities --kind monsters --limit 10
Entities in database:
  26 | monsters | aboleth | Aboleth | dnd5eapi.co (SRD mirror)
  27 | monsters | acolyte | Acolyte | dnd5eapi.co (SRD mirror)
  28 | monsters | adult-black-dragon | Adult Black Dragon | dnd5eapi.co (SRD mirror)
  29 | monsters | adult-blue-dragon | Adult Blue Dragon | dnd5eapi.co (SRD mirror)
  30 | monsters | adult-brass-dragon | Adult Brass Dragon | dnd5eapi.co (SRD mirror)
  31 | monsters | adult-bronze-dragon | Adult Bronze Dragon | dnd5eapi.co (SRD mirror)
  32 | monsters | adult-copper-dragon | Adult Copper Dragon | dnd5eapi.co (SRD mirror)
  33 | monsters | adult-gold-dragon | Adult Gold Dragon | dnd5eapi.co (SRD mirror)
  34 | monsters | adult-green-dragon | Adult Green Dragon | dnd5eapi.co (SRD mirror)
  35 | monsters | adult-red-dragon | Adult Red Dragon | dnd5eapi.co (SRD mirror)

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- list-entities --kind spells --limit 10
Entities in database:
  1 | spells | acid-arrow | Acid Arrow | dnd5eapi.co (SRD mirror)
  2 | spells | acid-splash | Acid Splash | dnd5eapi.co (SRD mirror)
  3 | spells | aid | Aid | dnd5eapi.co (SRD mirror)
  4 | spells | alarm | Alarm | dnd5eapi.co (SRD mirror)
  5 | spells | alter-self | Alter Self | dnd5eapi.co (SRD mirror)
  6 | spells | animal-friendship | Animal Friendship | dnd5eapi.co (SRD mirror)
  7 | spells | animal-messenger | Animal Messenger | dnd5eapi.co (SRD mirror)
  8 | spells | animal-shapes | Animal Shapes | dnd5eapi.co (SRD mirror)
  9 | spells | animate-dead | Animate Dead | dnd5eapi.co (SRD mirror)
  10 | spells | animate-objects | Animate Objects | dnd5eapi.co (SRD mirror)

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- search 'advantage' --k 5
chunk:609 doc:33 entity:Some(597) score:-5.330
  ...has [advantage] on strength checks, and his or her carrying capacity doubles.
***Cat's Grace.*** The target has [advantage] on...

chunk:1314 doc:39 entity:Some(597) score:-5.330
  ...has [advantage] on strength checks, and his or her carrying capacity doubles.
***Cat's Grace.*** The target has [advantage] on...

chunk:1 doc:1 entity:None score:-4.191
  ...[Advantage] / Disadvantage (Test Mechanic)

Some situations grant an edge or a penalty.

- **[Advantage]:** roll two d20s and take the higher...

chunk:2 doc:2 entity:None score:-4.191
  ...[Advantage] / Disadvantage (Test Mechanic)

Some situations grant an edge or a penalty.

- **[Advantage]:** roll two d20s and take the higher...

chunk:3 doc:3 entity:None score:-4.191
  ...[Advantage] / Disadvantage (Test Mechanic)

Some situations grant an edge or a penalty.

- **[Advantage]:** roll two d20s and take the higher...


PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- search 'goblin' --k 5
chunk:978 doc:34 entity:Some(966) score:-8.755
  # Monster: [Goblin]

- Size: Small
- Type: humanoid
- Alignment: neutral evil
- Armor Class (AC): 15
- Hit Points (HP): 7
- Challenge Rating (CR...

chunk:1683 doc:40 entity:Some(966) score:-8.755
  # Monster: [Goblin]

- Size: Small
- Type: humanoid
- Alignment: neutral evil
- Armor Class (AC): 15
- Hit Points (HP): 7
- Challenge Rating (CR...


PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- search 'bronze dragon' --k 5
chunk:885 doc:34 entity:Some(873) score:-7.305
  ...[Bronze] [Dragon] Wyrmling

- Size: Medium
- Type: [dragon]
- Alignment: lawful good
- Armor Class (AC): 17
- Hit Points (HP): 32
- Challenge Rating...

chunk:1590 doc:40 entity:Some(873) score:-7.305
  ...[Bronze] [Dragon] Wyrmling

- Size: Medium
- Type: [dragon]
- Alignment: lawful good
- Armor Class (AC): 17
- Hit Points (HP): 32
- Challenge Rating...

chunk:1158 doc:34 entity:Some(1146) score:-6.990
  ...Young [Bronze] [Dragon]

- Size: Large
- Type: [dragon]
- Alignment: lawful good
- Armor Class (AC): 18
- Hit Points (HP): 142
- Challenge Rating...

chunk:1863 doc:40 entity:Some(1146) score:-6.990
  ...Young [Bronze] [Dragon]

- Size: Large
- Type: [dragon]
- Alignment: lawful good
- Armor Class (AC): 18
- Hit Points (HP): 142
- Challenge Rating...

chunk:34 doc:5 entity:Some(31) score:-6.210
  ...Adult [Bronze] [Dragon]

- Size: Huge
- Type: [dragon]
- Alignment: lawful good
- AC: 19
- HP: 212
- CR: 15

## Ability Scores
STR 25...


PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- search 'dragon armor class' --k 5
chunk:885 doc:34 entity:Some(873) score:-3.381
  ...Bronze [Dragon] Wyrmling

- Size: Medium
- Type: [dragon]
- Alignment: lawful good
- [Armor] [Class] (AC): 17
- Hit Points (HP): 32
- Challenge Rating...

chunk:1590 doc:40 entity:Some(873) score:-3.381
  ...Bronze [Dragon] Wyrmling

- Size: Medium
- Type: [dragon]
- Alignment: lawful good
- [Armor] [Class] (AC): 17
- Hit Points (HP): 32
- Challenge Rating...

chunk:1158 doc:34 entity:Some(1146) score:-3.352
  ...Young Bronze [Dragon]

- Size: Large
- Type: [dragon]
- Alignment: lawful good
- [Armor] [Class] (AC): 18
- Hit Points (HP): 142
- Challenge Rating...

chunk:1863 doc:40 entity:Some(1146) score:-3.352
  ...Young Bronze [Dragon]

- Size: Large
- Type: [dragon]
- Alignment: lawful good
- [Armor] [Class] (AC): 18
- Hit Points (HP): 142
- Challenge Rating...

chunk:351 doc:23 entity:Some(42) score:-3.270
  ...Ancient Bronze [Dragon]

- Size: Gargantuan
- Type: [dragon]
- Alignment: lawful good
- [Armor] [Class] (AC): 22
- Hit Points (HP): 444
- Challenge Rating...


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
Found 6 search hits
Search results:
  chunk:1 score:-15.585 snippet: ...Example Q&A Prompts (For Testing Retrieval)

Use these to test your CLI:
- ΓÇ£[What] [does] [advantage] [do]?ΓÇ¥
- ΓÇ£How [do] opposed...
  chunk:2 score:-15.585 snippet: ...Example Q&A Prompts (For Testing Retrieval)

Use these to test your CLI:
- ΓÇ£[What] [does] [advantage] [do]?ΓÇ¥
- ΓÇ£How [do] opposed...
  chunk:3 score:-15.585 snippet: ...Example Q&A Prompts (For Testing Retrieval)

Use these to test your CLI:
- ΓÇ£[What] [does] [advantage] [do]?ΓÇ¥
- ΓÇ£How [do] opposed...
  chunk:54 score:-15.585 snippet: ...Example Q&A Prompts (For Testing Retrieval)

Use these to test your CLI:
- ΓÇ£[What] [does] [advantage] [do]?ΓÇ¥
- ΓÇ£How [do] opposed...
  chunk:105 score:-15.585 snippet: ...Example Q&A Prompts (For Testing Retrieval)

Use these to test your CLI:
- ΓÇ£[What] [does] [advantage] [do]?ΓÇ¥
- ΓÇ£How [do] opposed...
  chunk:156 score:-15.585 snippet: ...Example Q&A Prompts (For Testing Retrieval)

Use these to test your CLI:
- ΓÇ£[What] [does] [advantage] [do]?ΓÇ¥
- ΓÇ£How [do] opposed...
Loading prompts from directory: ./prompts
Loaded system prompt (399 chars) and task prompt (224 chars)
Built sources block with 6 chunks (22494 chars)
Full prompt length: 23170 chars
Connecting to Ollama at http://127.0.0.1:11434 using model llama3.2:3b
Generated answer (355 chars):
[chunk:165 doc:14 entity:] 

Advantage is a mechanic in D&D that allows you to roll two d20s and take the higher result. This can be especially useful when rolling against a difficult DC or when trying to hit a target. For example, if an enemy has a defense bonus of +3, you might gain advantage on your attack roll if they are injured or have a weakness.

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- ask 'What is Armor Class for a goblin?' --k 8
Starting ask with question: 'What is Armor Class for a goblin?'
Found 8 search hits
Search results:
  chunk:978 score:-9.332 snippet: # Monster: [Goblin]

- Size: Small
- Type: humanoid
- Alignment: neutral evil
- [Armor] [Class] (AC): 15
- Hit Points (HP): 7
- Challenge Rating (CR...
  chunk:1683 score:-9.332 snippet: # Monster: [Goblin]

- Size: Small
- Type: humanoid
- Alignment: neutral evil
- [Armor] [Class] (AC): 15
- Hit Points (HP): 7
- Challenge Rating (CR...
  chunk:678 score:-4.051 snippet: ...If it [is] [a] magic item or some other magic-imbued object, you learn its properties and how to use...
  chunk:1383 score:-4.051 snippet: ...If it [is] [a] magic item or some other magic-imbued object, you learn its properties and how to use...
  chunk:27 score:-3.938 snippet: ...target's AC can't be less than 16, regardless of [what] kind of [armor] it [is] wearing.

## Higher Level


  chunk:78 score:-3.938 snippet: ...target's AC can't be less than 16, regardless of [what] kind of [armor] it [is] wearing.

## Higher Level


  chunk:129 score:-3.938 snippet: ...target's AC can't be less than 16, regardless of [what] kind of [armor] it [is] wearing.

## Higher Level


  chunk:180 score:-3.938 snippet: ...target's AC can't be less than 16, regardless of [what] kind of [armor] it [is] wearing.

## Higher Level


Loading prompts from directory: ./prompts
Loaded system prompt (399 chars) and task prompt (224 chars)
Built sources block with 8 chunks (4056 chars)
Full prompt length: 4742 chars
Connecting to Ollama at http://127.0.0.1:11434 using model llama3.2:3b
Generated answer (127 chars):
The Goblin's Armor Class (AC) is 15.

Citations: 
[chunk:1683 doc:40 entity:Some(966)]
# Monster: Goblin
- Armor Class (AC): 15

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- ask 'What is the AC of a Bronze Dragon?' --k 8
Starting ask with question: 'What is the AC of a Bronze Dragon?'
Found 8 search hits
Search results:
  chunk:885 score:-7.305 snippet: ...one [of] [the] following breath weapons.
Lightning Breath. [The] [dragon] exhales lightning in [a] 40-foot line that [is] 5...
  chunk:1590 score:-7.305 snippet: ...one [of] [the] following breath weapons.
Lightning Breath. [The] [dragon] exhales lightning in [a] 40-foot line that [is] 5...
  chunk:1158 score:-6.990 snippet: ...one [of] [the] following breath weapons.
Lightning Breath. [The] [dragon] exhales lightning in [a] 60-foot line that [is] 5...
  chunk:1863 score:-6.990 snippet: ...one [of] [the] following breath weapons.
Lightning Breath. [The] [dragon] exhales lightning in [a] 60-foot line that [is] 5...
  chunk:34 score:-6.210 snippet: ...[the] [dragon]'s choice that [is] within 120 feet [of] [the] [dragon] and aware [of] it must succeed on [a]...
  chunk:85 score:-6.210 snippet: ...[the] [dragon]'s choice that [is] within 120 feet [of] [the] [dragon] and aware [of] it must succeed on [a]...
  chunk:136 score:-6.210 snippet: ...[the] [dragon]'s choice that [is] within 120 feet [of] [the] [dragon] and aware [of] it must succeed on [a]...
  chunk:187 score:-6.210 snippet: ...[the] [dragon]'s choice that [is] within 120 feet [of] [the] [dragon] and aware [of] it must succeed on [a]...
Loading prompts from directory: ./prompts
Loaded system prompt (399 chars) and task prompt (224 chars)
Built sources block with 8 chunks (10891 chars)
Full prompt length: 11578 chars
Connecting to Ollama at http://127.0.0.1:11434 using model llama3.2:3b
Generated answer (402 chars):
The Armor Class (AC) for a Bronze Dragon can be one of three values depending on its age: 

1. Young and Adult dragons have an AC of 18.
2. Adult, Large dragons have an AC of 19.
3. Adult, Huge dragons have an AC of 19.

These values are cited from [chunk:1158 doc:34 entity:Some(1146)], [chunk:1863 doc:40 entity:Some(1146)], [chunk:136 doc:11 entity:Some(31)], and [chunk:187 doc:14 entity:Some(31)].

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- ask 'What is the AC of an Adult Bronze Dragon?' --k 8
Starting ask with question: 'What is the AC of an Adult Bronze Dragon?'
Found 8 search hits
Search results:
  chunk:34 score:-8.277 snippet: ...[Adult] [Bronze] [Dragon]

- Size: Huge
- Type: [dragon]
- Alignment: lawful good
- [AC]: 19
- HP: 212
- CR: 15

## Ability Scores
STR 25...
  chunk:85 score:-8.277 snippet: ...[Adult] [Bronze] [Dragon]

- Size: Huge
- Type: [dragon]
- Alignment: lawful good
- [AC]: 19
- HP: 212
- CR: 15

## Ability Scores
STR 25...
  chunk:136 score:-8.277 snippet: ...[Adult] [Bronze] [Dragon]

- Size: Huge
- Type: [dragon]
- Alignment: lawful good
- [AC]: 19
- HP: 212
- CR: 15

## Ability Scores
STR 25...
  chunk:187 score:-8.277 snippet: ...[Adult] [Bronze] [Dragon]

- Size: Huge
- Type: [dragon]
- Alignment: lawful good
- [AC]: 19
- HP: 212
- CR: 15

## Ability Scores
STR 25...
  chunk:238 score:-8.277 snippet: ...[Adult] [Bronze] [Dragon]

- Size: Huge
- Type: [dragon]
- Alignment: lawful good
- [AC]: 19
- HP: 212
- CR: 15

## Ability Scores
STR 25...
  chunk:289 score:-8.277 snippet: ...[Adult] [Bronze] [Dragon]

- Size: Huge
- Type: [dragon]
- Alignment: lawful good
- [AC]: 19
- HP: 212
- CR: 15

## Ability Scores
STR 25...
  chunk:340 score:-8.213 snippet: ...[Adult] [Bronze] [Dragon]

- Size: Huge
- Type: [dragon]
- Alignment: lawful good
- Armor Class ([AC]): 19
- Hit Points (HP): 212
- Challenge Rating...
  chunk:391 score:-8.213 snippet: ...[Adult] [Bronze] [Dragon]

- Size: Huge
- Type: [dragon]
- Alignment: lawful good
- Armor Class ([AC]): 19
- Hit Points (HP): 212
- Challenge Rating...
Loading prompts from directory: ./prompts
Loaded system prompt (399 chars) and task prompt (224 chars)
Built sources block with 8 chunks (13632 chars)
Full prompt length: 14326 chars
Connecting to Ollama at http://127.0.0.1:11434 using model llama3.2:3b
Generated answer (181 chars):
The Armor Class (AC) of an Adult Bronze Dragon is 19.

Citations: 
chunk:34 doc:5 chunk:85 doc:8 chunk:136 doc:11 chunk:187 doc:14 chunk:238 doc:17 chunk:289 doc:20 chunk:340 doc:23

PS C:\dev\dnd-ai-local>

**********************
Windows PowerShell transcript end
End time: 20260116142024
**********************

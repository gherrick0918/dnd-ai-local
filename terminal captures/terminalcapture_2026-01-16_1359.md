**********************
Windows PowerShell transcript start
Start time: 20260116135917
Username: GREG-ASUS-PC\gherr
RunAs User: GREG-ASUS-PC\gherr
Configuration Name: 
Machine: GREG-ASUS-PC (Microsoft Windows NT 10.0.26200.0)
Host Application: PowerShell Build and Test Script
Process ID: 31648
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


PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- search 'advantage' --k 1
chunk:609 doc:33 entity:Some(597) score:-5.310
  ...has [advantage] on strength checks, and his or her carrying capacity doubles.
***Cat's Grace.*** The target has [advantage] on...


PS C:\dev\dnd-ai-local> cargo run --bin ddai_cli

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- init-db
DB ready: ./data/db/ddai.sqlite (schema v3)

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- ingest .\data\raw\sample_rules.md --source 'Sample Rules Pack' --title 'Sample Rules Pack (Test Content)' --license 'Internal test content' --attribution 'Created by the developer for ingestion tests'
Ingested document id=35 from .\data\raw\sample_rules.md

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- list-docs
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

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- ingest-dnd5eapi --base-url https://www.dnd5eapi.co --limit 25 --source 'dnd5eapi.co (SRD mirror)'
Ingested 25 spells...
Ingest complete: 25 spells into document id=36
Ingested 25 monsters...
Ingest complete: 25 monsters into document id=37
DnD 5e API ingest completed successfully!

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- list-docs
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

PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- search 'advantage' --k 5
chunk:609 doc:33 entity:Some(597) score:-5.310
  ...has [advantage] on strength checks, and his or her carrying capacity doubles.
***Cat's Grace.*** The target has [advantage] on...

chunk:1 doc:1 entity:None score:-4.243
  ...[Advantage] / Disadvantage (Test Mechanic)

Some situations grant an edge or a penalty.

- **[Advantage]:** roll two d20s and take the higher...

chunk:2 doc:2 entity:None score:-4.243
  ...[Advantage] / Disadvantage (Test Mechanic)

Some situations grant an edge or a penalty.

- **[Advantage]:** roll two d20s and take the higher...

chunk:3 doc:3 entity:None score:-4.243
  ...[Advantage] / Disadvantage (Test Mechanic)

Some situations grant an edge or a penalty.

- **[Advantage]:** roll two d20s and take the higher...

chunk:54 doc:6 entity:None score:-4.243
  ...[Advantage] / Disadvantage (Test Mechanic)

Some situations grant an edge or a penalty.

- **[Advantage]:** roll two d20s and take the higher...


PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- search 'Armor Class' --k 5
chunk:937 doc:34 entity:Some(925) score:-0.755
  ...unaligned
- [Armor Class] (AC): 11
- Hit Points (HP): 1
- Challenge Rating (CR): 0

## Ability Scores
STR 1 | DEX 13 | CON...

chunk:1085 doc:34 entity:Some(1073) score:-0.755
  ...unaligned
- [Armor Class] (AC): 5
- Hit Points (HP): 13
- Challenge Rating (CR): 0

## Ability Scores
STR 1 | DEX 1 | CON...

chunk:1081 doc:34 entity:Some(1069) score:-0.752
  ...unaligned
- [Armor Class] (AC): 11
- Hit Points (HP): 1
- Challenge Rating (CR): 0

## Ability Scores
STR 1 | DEX 12 | CON...

chunk:1120 doc:34 entity:Some(1108) score:-0.748
  ...lawful evil
- [Armor Class] (AC): 16
- Hit Points (HP): 144
- Challenge Rating (CR): 13

## Ability Scores
STR 18 | DEX 18...

chunk:866 doc:34 entity:Some(854) score:-0.719
  ...unaligned
- [Armor Class] (AC): 10
- Hit Points (HP): 3
- Challenge Rating (CR): 0

## Ability Scores
STR 4 | DEX 11 | CON...


PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- search 'Casting Time' --k 5
chunk:687 doc:33 entity:Some(675) score:-0.069
  ...Transmutation
- [Casting Time]: 1 action
- Range: Touch
- Components: V, S, M
- Duration: 1 minute

## Description
You touch a creature. The...

chunk:721 doc:33 entity:Some(709) score:-0.068
  ...Conjuration
- [Casting Time]: 1 bonus action
- Range: Self
- Components: V
- Duration: Instantaneous

## Description
Briefly surrounded by silvery mist, you teleport...

chunk:690 doc:33 entity:Some(678) score:-0.067
  ...Abjuration
- [Casting Time]: 1 action
- Range: Touch
- Components: V, S
- Duration: Instantaneous

## Description
You touch a creature and can end...

chunk:783 doc:33 entity:Some(771) score:-0.067
  ...Necromancy
- [Casting Time]: 1 action
- Range: Touch
- Components: V, S
- Duration: Instantaneous

## Description
You touch a living creature that has...

chunk:600 doc:33 entity:Some(588) score:-0.067
  ...Evocation
- [Casting Time]: 1 bonus action
- Range: Self
- Components: V, S
- Duration: Up to 1 minute

## Description
Your prayer empowers...


PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- search 'monster' --k 10
chunk:1073 doc:34 entity:Some(1061) score:-0.000
  ...Rust [Monster]

- Size: Medium
- Type: monstrosity
- Alignment: unaligned
- Armor Class (AC): 14
- Hit Points (HP): 27
- Challenge Rating (CR): 0...

chunk:937 doc:34 entity:Some(925) score:-0.000
  # [Monster]: Frog

- Size: Tiny
- Type: beast
- Alignment: unaligned
- Armor Class (AC): 11
- Hit Points (HP): 1
- Challenge Rating (CR): 0...

chunk:1085 doc:34 entity:Some(1073) score:-0.000
  # [Monster]: Shrieker

- Size: Medium
- Type: plant
- Alignment: unaligned
- Armor Class (AC): 5
- Hit Points (HP): 13
- Challenge Rating (CR): 0...

chunk:1081 doc:34 entity:Some(1069) score:-0.000
  # [Monster]: Sea Horse

- Size: Tiny
- Type: beast
- Alignment: unaligned
- Armor Class (AC): 11
- Hit Points (HP): 1
- Challenge Rating (CR...

chunk:1120 doc:34 entity:Some(1108) score:-0.000
  # [Monster]: Vampire, Mist Form

- Size: Medium
- Type: undead
- Alignment: lawful evil
- Armor Class (AC): 16
- Hit Points (HP): 144
- Challenge...

chunk:30 doc:5 entity:Some(27) score:-0.000
  # [Monster]: Acolyte

- Size: Medium
- Type: humanoid
- Alignment: any alignment
- AC: 10
- HP: 9
- CR: 0.25

## Ability Scores
STR 10...

chunk:81 doc:8 entity:Some(27) score:-0.000
  # [Monster]: Acolyte

- Size: Medium
- Type: humanoid
- Alignment: any alignment
- AC: 10
- HP: 9
- CR: 0.25

## Ability Scores
STR 10...

chunk:132 doc:11 entity:Some(27) score:-0.000
  # [Monster]: Acolyte

- Size: Medium
- Type: humanoid
- Alignment: any alignment
- AC: 10
- HP: 9
- CR: 0.25

## Ability Scores
STR 10...

chunk:183 doc:14 entity:Some(27) score:-0.000
  # [Monster]: Acolyte

- Size: Medium
- Type: humanoid
- Alignment: any alignment
- AC: 10
- HP: 9
- CR: 0.25

## Ability Scores
STR 10...

chunk:234 doc:17 entity:Some(27) score:-0.000
  # [Monster]: Acolyte

- Size: Medium
- Type: humanoid
- Alignment: any alignment
- AC: 10
- HP: 9
- CR: 0.25

## Ability Scores
STR 10...


PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- search 'goblin' --k 10
chunk:978 doc:34 entity:Some(966) score:-9.026
  # Monster: [Goblin]

- Size: Small
- Type: humanoid
- Alignment: neutral evil
- Armor Class (AC): 15
- Hit Points (HP): 7
- Challenge Rating (CR...


PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- search 'armor class' --k 10
chunk:937 doc:34 entity:Some(925) score:-0.755
  ...unaligned
- [Armor Class] (AC): 11
- Hit Points (HP): 1
- Challenge Rating (CR): 0

## Ability Scores
STR 1 | DEX 13 | CON...

chunk:1085 doc:34 entity:Some(1073) score:-0.755
  ...unaligned
- [Armor Class] (AC): 5
- Hit Points (HP): 13
- Challenge Rating (CR): 0

## Ability Scores
STR 1 | DEX 1 | CON...

chunk:1081 doc:34 entity:Some(1069) score:-0.752
  ...unaligned
- [Armor Class] (AC): 11
- Hit Points (HP): 1
- Challenge Rating (CR): 0

## Ability Scores
STR 1 | DEX 12 | CON...

chunk:1120 doc:34 entity:Some(1108) score:-0.748
  ...lawful evil
- [Armor Class] (AC): 16
- Hit Points (HP): 144
- Challenge Rating (CR): 13

## Ability Scores
STR 18 | DEX 18...

chunk:866 doc:34 entity:Some(854) score:-0.719
  ...unaligned
- [Armor Class] (AC): 10
- Hit Points (HP): 3
- Challenge Rating (CR): 0

## Ability Scores
STR 4 | DEX 11 | CON...

chunk:872 doc:34 entity:Some(860) score:-0.719
  ...unaligned
- [Armor Class] (AC): 12
- Hit Points (HP): 1
- Challenge Rating (CR): 0

## Ability Scores
STR 2 | DEX 15 | CON...

chunk:890 doc:34 entity:Some(878) score:-0.719
  ...unaligned
- [Armor Class] (AC): 12
- Hit Points (HP): 2
- Challenge Rating (CR): 0

## Ability Scores
STR 3 | DEX 15 | CON...

chunk:903 doc:34 entity:Some(891) score:-0.719
  ...unaligned
- [Armor Class] (AC): 11
- Hit Points (HP): 2
- Challenge Rating (CR): 0

## Ability Scores
STR 2 | DEX 11 | CON...

chunk:992 doc:34 entity:Some(980) score:-0.719
  ...unaligned
- [Armor Class] (AC): 13
- Hit Points (HP): 1
- Challenge Rating (CR): 0

## Ability Scores
STR 5 | DEX 16 | CON...

chunk:1017 doc:34 entity:Some(1005) score:-0.719
  ...unaligned
- [Armor Class] (AC): 10
- Hit Points (HP): 2
- Challenge Rating (CR): 0

## Ability Scores
STR 2 | DEX 11 | CON...


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
Loading prompts from directory: ./prompts
Loaded system prompt (399 chars) and task prompt (224 chars)
Built sources block with 6 chunks (22494 chars)
Full prompt length: 23170 chars
Connecting to Ollama at http://127.0.0.1:11434 using model gemma2:2b
Generated answer (152 chars):
According to the provided text, **advantage** means you get to roll two dice and take the higher value.  This gives you a bonus in certain situations. 


PS C:\dev\dnd-ai-local> cargo run -p ddai_cli -- ask 'What is Armor Class for a goblin?' --k 8
Starting ask with question: 'What is Armor Class for a goblin?'

PS C:\dev\dnd-ai-local>

**********************
Windows PowerShell transcript end
End time: 20260116135945
**********************

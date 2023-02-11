# Rogue 8
This is a project about making a roguelike.

## 8 Things
1. [Levels](#1-levels) (Dungeon Levels, Biomes, etc.)
2. [Monsters](#2-monsters)
3. [Items](#3-items)
4. [Spells](#4-spells)
5. [Traps and Interactables](#5-traps-and-interactables)
6. [Stats](#6-stats)
7. [Actions](#7-actions)
8. [NPCs](#8-npcs)

# Game Elements

## 1. Levels

## 2. Monsters
### 1. Undead*
- Can come back to life up to X times once killed (Undying X)
- Vulnerable to Holy Sunder and Holy Water (causes effect of Fear, cancels Undying)

### 3. Koblin (Goblin + Kobold mix)
- Very squishy
- Fast
- Deal more damage when close to another Koblin
- Chance to steal item on physical attack
### 6. Trolorc (Troll + Orc mix)
- Very tanky
- Slow
- Heals over time unless affected by fire
### 11. Vampire*
- Life steal on attacks
- Extra damage on physical attacks
- Can cast Flying once per day
- Takes damage from running water
- Cannot open doors
- Can give vampiric disease when hitting in melee
  - Vampiric disease: lots of damage per turn, if survive, become vampire, can be cured by Cure Disease effect

### 10. Hydrake (Hydra + Dragon mix)
- When taking damage, gains an extra attack
### 12. Mind Flayer
- Mind Control
- Fear

### 13. Beholder
- Flying
- 8 Tentacle eyes can each cast a spell

### 8. Demon
- Flying
- Teleport
- Fire Ball
- Vulnerable to Holy Sunder and Holy Water (causes effect of Fear)
### 14. Lich*
- Cannot be killed without destroying its phylactery (item hidden somewhere in the level)
- Won't attack character holding phylactery
- +2 Spells
### 15. Mimic**
- Takes form of some interactable
- Will tire out and return to its interactable form

> *: means it's a modifier, so it combines with another monster type (i.e. Undead Ogre, Vampire Trolorc, Hydrake Lich)

> **: means it's a modifier to an iteractable (i.e. Mimic Chest, Mimic Door, etc.)
## 3. Items

## 4. Spells
### 1. Cure Disease
### 2. Heal
### 3. Flying
### 4. Fire Ball
### 5. Teleport
### 6. Mind Control
### 7. Fear
### 8. Holy Sunder
- Can break down doors, chest, traps
- Can temporarily remove Undying
## 5. Traps and Interactables
### 1. Door
- Can be open or closed
- Could be locked
### 2. Chest
- Can contain items
- Could be locked
### 3. Secret Doors / Walls (*)
### 4. Trap (Pressure plate activated, Bear Trap, Pit, Chest trap) (*)
- Fire trap: shoots (1 or more) fireball in a specific direction
- Arrow trap: shoots (1 or more) arrow in a specific direction
- Spike trap: (8-ways) damage around location (1 tile radius)
- Poison trap: spreads poisonous gas slowly (1 tile radius per round)
- Pit trap: fall one level and take damage (1 tile radius)
- Trapped Chest (Fire, Arrow, Spike, Activated by opening)
- Pressure Plate (Activates a trap remotely, or in place)
- Sconce Activated (Activates a trap remotely by using sconce)

### 5. Corpse
- Can contain items
- Can be used as an item (drops its items)
### 6. Sconce
- Can place or take torches from them
### 7. Fountain
- Some form of buff or healing can be obtained
### 8. Stairs
- Go down or up a level

> *: means hidden without inspection

## 6. Stats

## 7. Actions
### 1. Move (Target Position)
### 2. Look (Target Position)
### 3. Hit (Target Position)
### 4. Use/Eat/Drink (Item in inventory)
### 5. Throw (Item in hand, Target Position)
### 6. Shoot (Equipped Ammo, Target Position)
### 7. Activate (Target Position)
### 8. Pickup/Drop Items

## 8. NPCs
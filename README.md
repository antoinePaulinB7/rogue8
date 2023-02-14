# Rogue 8
This is a project about making a roguelike.

## 8 Design Elements
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
### 1. Undead (*)
- Can come back to life up to X times once killed (Undying X)
- Vulnerable to Holy Sunder and Holy Water (causes effect of Fear, cancels Undying)

### 2. Koblin (Goblin + Kobold mix)
- Very squishy
- Fast
- Deal more damage when close to another Koblin
- Chance to steal item on physical attack

### 3. Trolorc (Troll + Orc mix)
- Very tanky
- Slow
- Heals over time unless affected by fire
- Loners

### 4. Vampire (*)
- Life steal on attacks
- Extra damage on physical attacks
- Can cast Flying once per day
- Takes damage from running water
- Cannot open doors
- Can give vampiric disease when hitting in melee
  - Vampiric disease: lots of damage per turn, if survive, become vampire, can be cured by Cure Disease effect
- Vampires are rarely found alone

### 5. Hydrake (Hydra + Dragon mix)
- When taking damage, gains an extra attack
- Loner

### 6. Mind Flayer
- Mind Control
- Fear

### 7. Beholder
- Flying
- 8 Tentacle eyes can each cast a spell
- Extremely lonely and will avoid loud noises (will try to move away from player while undetected, but will chase enemy down aggressively if found)

### 8. Demon
- Fire Ball
- Teleport
- Invisibility
- Vulnerable to Holy Sunder and Holy Water (causes effect of Fear)
- Will always cast Invisibility after Teleporting
- Looks for player all the time

### 9. Lich (*)
- Cannot be killed without destroying its phylactery (item hidden somewhere in the level)
- Won't attack character holding phylactery
- +2 Spells
- Has a bunch of Undead followers.

### 10. Mimic (**)
- Takes form of some interactable
- Will tire out and return to its interactable form
- Won't show its true form unless interacted with

> *: means it's a modifier, so it combines with another monster type (i.e. Undead Ogre, Vampire Trolorc, Hydrake Lich)

> **: means it's a modifier to an interactable (i.e. Mimic Chest, Mimic Door, etc.)
## 3. Items
### 1. Potions, Wands and Scrolls
- Potions can contain Healing Cleanse, Invisibility or Flying. Single use. Stored in stacks.
- Wands can contain any spell. Must be equipped to be used. X uses.
- Scrolls can contain any spell, but must be read to learn the effect of the spell. Single use.

### 2. Melee Weapons
Equipment. Sets base damage for melee attacks.

Example melee weapons:
- Sword, Cleaver, Scimitar
- Spear, Javelin (Can stack like ammo), Halberd
- Axe, Greataxe
- Hammer, Club, Morningstar
- Dagger, Knife, Dirk

### 3. Ranged Weapons
Equipment, allows to do ranged attacks through the [Shoot](#6-shoot) action. Requires matching ammunition equipped.

Example:
- Slings
- Bows
- Crossbows

### 4. Ammunition
Equipment. Required to be equipped alongside a matching ranged weapon. Stored in stacks.

Example:
- Rocks
- Arrows
- Bolts

### 5. Armor
Equipment. The better the armor, the more damage it negates.

Example:
- Half helm
- Chest plate
- Scale gloves
- Leather boots

### 6. Keys
Can unlock locked doors and chests. Stored in stacks.

### 7. Magic Items (*)
Equipment. Give stat bonuses and other passive effects. Can be standalone item, or applied to other equipment.

Example:
- Ring of Intelligence (increases Intelligence stat)
- Amulet of Perception (increases Perception stat)
- Belt of Strength (increases Strength stat)
- Boots of Speed (Armor piece [Boot], increases Speed stat)
- +1 Sword (Melee Weapon, +1 to damage)
- +2 Bow of Speed (Ranged Weapon, +2 to damage, increases Speed stat)

### 8. Loot?
Finding loot earns XP points.

Example:
- Gems
- Pieces of art
- Gold pieces



> *: Magic items can either be standalone equipment or applied as modifiers to other equipment.



## 4. Spells
### 1. Healing Cleanse
- Cure diseases and all negative effects.
- Heals some amount of health points.

### 2. Invisibility
- Become invisible to the naked eye (on screen as well, you have to track your own position!).
- Become untargetable by melee attacks.
- Any action but Move and Look break invisibility.

### 3. Flying
- Become much harder to hit in melee
- Doesn't trigger pressure plate and grounded traps
- Become unaffected by anything on the ground
- Can go up pit traps
- Move slightly faster

### 4. Baleful Fire
- Creates a firey projectile that deals Fire damage on impact and creates a radius of Fire damage from the impact. The farther it travels before impact, the bigger the radius of the explosion (minimum radius of 1).
- The Farther it travels, the more damage it deals.

### 5. Teleport
- Move to anywhere on the screen.
- Landing into occupied space creates an immense explosion that deals lots of damage.

### 6. Mind Control
- Take control of another visible creature

### 7. Fear
- X creatures are afraid of caster and have to move as far away from you as they can.
- While afraid, they can only move.

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
### 1. Move
- Target Position

### 2. Look/Inspect
- Target Position

### 3. Hit
- Target Position

### 4. Use/Eat/Drink
- Item in inventory

### 5. Throw
- Item in hand
- Target Position

### 6. Shoot
- Equipped Ammo
- Target Position

### 7. Activate
- Target Position

### 8. Pickup/Drop
- Items on same tile as character

## 8. NPCs

use crate::world::spell::Spell;
use crate::world::stats::Stats;

pub enum Equipment {
    MeleeWeapon(ItemInfo, WeaponInfo),
    RangedWeapon(ItemInfo, WeaponInfo, AmmunitionType),
    Ammunition(ItemInfo, AmmunitionType, usize), // info, damage
    Armor(ItemInfo, ArmorInfo, ArmorType),
}

enum Consumable {
    Potion(ItemInfo, Spell),
    Wand(ItemInfo, Spell, usize), // info, spell, charges
    Scroll(ItemInfo, Spell),
}

/// Consumables, Keys and Loot can stack, the rest cannot
pub enum Item {
    Equipment,
    MagicItem(Equipment, Effect),
    Consumable,
    Key,
    Loot(ItemInfo, usize), // info, xp value
}

struct ItemInfo {
    name: String,
    info: String,
}

struct Effect {
    name: String,
    info: String,
    modifier: Stats,
}

struct WeaponInfo {
    damage: usize,
    range: usize,
}

enum AmmunitionType {
    Ball,
    Arrow,
    Bolts,
}

struct ArmorInfo {
    physical_defense: usize,
    magical_defense: usize,
}

enum ArmorType {
    Head,
    Chest,
    Hand,
    Feet,
}

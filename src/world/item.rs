use crate::world::spell::Spell;
use crate::world::stats::Stats;

#[derive(Clone, PartialEq, Eq)]
pub enum Equipment {
    MeleeWeapon(ItemInfo, WeaponInfo),
    RangedWeapon(ItemInfo, WeaponInfo, AmmunitionType),
    Ammunition(ItemInfo, AmmunitionType, usize), // info, damage
    Armor(ItemInfo, ArmorInfo, ArmorType),
}

#[derive(Clone, PartialEq, Eq)]
enum Consumable {
    Potion(ItemInfo, Spell),
    Wand(ItemInfo, Spell, usize), // info, spell, charges
    Scroll(ItemInfo, Spell),
}

/// Consumables, Keys and Loot can stack, the rest cannot
#[derive(Clone, PartialEq, Eq)]
pub enum Item {
    Equipment,
    MagicItem(Equipment, Effect),
    Consumable,
    Key,
    Loot(ItemInfo, usize), // info, xp value
}

#[derive(Clone, PartialEq, Eq)]
struct ItemInfo {
    name: String,
    info: String,
}

#[derive(Clone, PartialEq, Eq)]
struct Effect {
    name: String,
    info: String,
    modifier: Stats,
}

#[derive(Clone, PartialEq, Eq)]
struct WeaponInfo {
    damage: usize,
    range: usize,
}

#[derive(Clone, PartialEq, Eq)]
enum AmmunitionType {
    Ball,
    Arrow,
    Bolts,
}

#[derive(Clone, PartialEq, Eq)]
struct ArmorInfo {
    physical_defense: usize,
    magical_defense: usize,
}

#[derive(Clone, PartialEq, Eq)]
enum ArmorType {
    Head,
    Chest,
    Hand,
    Feet,
}

//! Tombstone — a clean-room Horizon port of the core of Corail Tombstone.
//!
//! On death a player's inventory is not thrown on the ground: it goes into a
//! grave block at the place they died. Only the grave's owner (or an operator)
//! may open it or break it; opening it shows a 54-slot chest menu holding the
//! 36 inventory slots, the four armour slots and the off hand.
//!
//! Nothing here comes from the original mod, which is All Rights Reserved:
//! the behaviour was rebuilt from its public description ("keeps safe your
//! inventory items in graves on death") and the block, its texture and all the
//! code are original.
//!
//! How it fits into Horizon: the dying player's inventory reaches
//! `loot_drops` as one call (API 405), which lets a mod take every stack out
//! before they are thrown. The grave is then written with `edit_blocks`, its
//! block entity created by `block_entity` and filled in place, and its slots
//! come from `block_entity_container` with the vanilla 9x6 menu (API 16/17).

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use horizon_client::mod_api::serde_json;
use horizon_client::mod_api::{
    BlockEntityContainerEvent, BlockEntityEvent, Compound, DestroyBlockEvent,
    ExplosionDetonateEvent, Flow, Hooks, Host, ItemStack, LivingDeathEvent, LootDropsEvent,
    ModBlockEntity, ModContainer, NativeBlockEdit, NativeBlockRun, RightClickBlockEvent, Tag, Text,
};

/// The block that keeps a player's items.
const GRAVE_BLOCK: &str = "tombstone:grave";
/// Its block entity type (a mod block's own, named after the block).
const GRAVE_ENTITY: &str = "tombstone:grave";
/// Saved in the block entity's compound as vanilla saves a UUID: four ints.
const OWNER_KEY: &str = "Owner";
/// The damage type that killed the owner, saved in the grave's compound.
const CAUSE_KEY: &str = "DeathCause";
/// Six rows of nine (`minecraft:generic_9x6`): the 36 inventory slots, the
/// four armour slots and the off hand fit with room to spare.
const SLOTS: usize = 54;
const MENU: &str = "minecraft:generic_9x6";

pub fn hooks() -> Box<dyn Hooks> {
    Box::new(Tombstone)
}

/// What an operator may set in `config/tombstone.json`, beside `options.txt`
/// (on a dedicated server, beside `server.properties`). A missing or unreadable
/// file keeps these defaults and is written back so there is something to edit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Config {
    /// Only the owner (or an operator) opens or breaks a grave.
    owner_only: bool,
    /// A blast leaves graves standing.
    protect_from_explosions: bool,
    /// How far above the death spot a grave may look for room (0 to 8).
    above: i32,
    /// How far below it may look (0 to 4).
    below: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            owner_only: true,
            protect_from_explosions: true,
            above: 4,
            below: 2,
        }
    }
}

impl Config {
    fn load(host: &dyn Host) -> Self {
        let mut config = Self::default();
        let Ok(dir) = host.config_dir() else {
            return config;
        };
        let path = dir.join("tombstone.json");
        match std::fs::read_to_string(&path) {
            Ok(text) => config.read(&text),
            Err(_) => write_config(&path, &config),
        }
        config
    }

    /// Reads the keys it knows; unknown keys, other types and a broken file
    /// leave the defaults in place.
    fn read(&mut self, text: &str) {
        let Ok(value) = serde_json::from_str::<serde_json::Value>(text) else {
            return;
        };
        let bool_at = |key: &str, fallback: bool| {
            value
                .get(key)
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(fallback)
        };
        let int_at = |key: &str, fallback: i32, low: i32, high: i32| {
            value
                .get(key)
                .and_then(serde_json::Value::as_i64)
                .map(|n| n.clamp(i64::from(low), i64::from(high)) as i32)
                .unwrap_or(fallback)
        };
        self.owner_only = bool_at("owner_only", self.owner_only);
        self.protect_from_explosions =
            bool_at("protect_from_explosions", self.protect_from_explosions);
        self.above = int_at("grave_above", self.above, 0, 8);
        self.below = int_at("grave_below", self.below, 0, 4);
    }

    fn to_json(self) -> String {
        serde_json::json!({
            "owner_only": self.owner_only,
            "protect_from_explosions": self.protect_from_explosions,
            "grave_above": self.above,
            "grave_below": self.below,
        })
        .to_string()
    }
}

/// Writes the defaults where an operator will find them; a failure (a
/// read-only folder) only means the settings stay in memory.
fn write_config(path: &PathBuf, config: &Config) {
    let _ = std::fs::write(path, config.to_json());
}

struct Tombstone;

/// The death cause of a player who just died, waiting for the same tick's
/// `loot_drops` to make their grave. Keyed by the dying entity's id.
static PENDING_DEATHS: OnceLock<Mutex<HashMap<u32, String>>> = OnceLock::new();

fn pending_deaths() -> &'static Mutex<HashMap<u32, String>> {
    PENDING_DEATHS.get_or_init(|| Mutex::new(HashMap::new()))
}

impl Hooks for Tombstone {
    /// A player is about to die: keep the damage type that killed them so the
    /// grave made by the same tick's `loot_drops` can record it. Never stops
    /// the death.
    fn living_death(&self, _host: &mut dyn Host, event: &mut LivingDeathEvent<'_>) -> Flow {
        if event.kind == "minecraft:player"
            && let Ok(mut deaths) = pending_deaths().lock()
        {
            deaths.insert(event.entity, event.damage_type.to_string());
        }
        Flow::Continue
    }

    /// An explosion is about to break blocks: a grave is spared, so a creeper
    /// or a TNT blast cannot spill the items it keeps.
    fn explosion_detonate(&self, host: &mut dyn Host, event: &mut ExplosionDetonateEvent<'_>) {
        if event.blocks.is_empty() || !Config::load(&*host).protect_from_explosions {
            return;
        }
        event.blocks.retain(|pos| host.block(*pos) != GRAVE_BLOCK);
    }

    /// The grave's block entity: a mod entity with its own saved compound,
    /// created when the block is placed (by the death, or by an operator).
    fn block_entity(
        &self,
        _host: &mut dyn Host,
        event: &mut BlockEntityEvent<'_>,
    ) -> Option<(String, Compound)> {
        (event.block == GRAVE_BLOCK).then(|| (GRAVE_ENTITY.to_string(), Compound::new()))
    }

    /// The grave's slots and the chest menu that shows them when it is used.
    /// No face reaches them, so a hopper beside a grave cannot drain it.
    fn block_entity_container(
        &self,
        _host: &dyn Host,
        event: &mut BlockEntityContainerEvent<'_>,
    ) -> Option<ModContainer> {
        (event.block == GRAVE_BLOCK).then(|| ModContainer {
            slots: SLOTS,
            faces: Default::default(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            menu: Some(MENU.to_string()),
        })
    }

    /// The dying player's inventory, taken out of `drops` so nothing is
    /// thrown, and put into a grave at the place they died. Anything that
    /// stops the grave from being made leaves the drops to vanilla.
    fn loot_drops(&self, host: &mut dyn Host, event: &mut LootDropsEvent<'_>) {
        if event.context.entity.map(|(_, kind)| kind) != Some("minecraft:player") {
            return;
        }
        let (Some(entity_id), Some(owner)) = (
            event.context.entity.map(|(id, _)| id),
            event.context.player,
        ) else {
            return;
        };
        // Taken whether or not there are items, so a stale entry for a reused
        // entity id cannot outlive this death.
        let cause = pending_deaths()
            .lock()
            .ok()
            .and_then(|mut deaths| deaths.remove(&entity_id));
        if event.drops.is_empty() {
            return;
        }
        let dimension = event.context.dimension.to_string();
        // The player's feet where the level still holds them, else the point
        // the items would leave from (the eyes less 0.3).
        let at = host
            .entity(entity_id)
            .and_then(|entity| host.entity_position(entity))
            .unwrap_or(event.context.origin);
        let Some(pos) = grave_position(host, &dimension, at, Config::load(&*host)) else {
            return;
        };
        let stacks = event.drops.clone();
        if !place_grave(host, &dimension, pos, owner, &stacks, cause.as_deref()) {
            return;
        }
        event.drops.clear();
        host.tell(
            owner,
            Text::translate_with_fallback(
                "tombstone.message.grave_placed",
                "Your items rest in a grave at %s %s %s.",
                &[
                    Text::literal(pos[0].to_string()),
                    Text::literal(pos[1].to_string()),
                    Text::literal(pos[2].to_string()),
                ],
            ),
        );
        if cause.is_some() {
            tell_cause(host, owner, &cause);
        }
    }

    /// Only the grave's owner (and an operator) opens it.
    fn right_click_block(
        &self,
        host: &mut dyn Host,
        event: &mut RightClickBlockEvent,
    ) -> Flow {
        if host.block(event.pos) != GRAVE_BLOCK {
            return Flow::Continue;
        }
        let Some(entity) = host.mod_block_entity(event.pos) else {
            return Flow::Continue;
        };
        if entity.id != GRAVE_ENTITY {
            return Flow::Continue;
        }
        let Some(owner) = owner_of(&entity.nbt) else {
            return Flow::Continue;
        };
        let Some(player) = host.acting_player() else {
            return Flow::Cancel;
        };
        if player.uuid == owner || player.permission >= 2 {
            // The owner is told how they died, as the grave remembered it.
            tell_cause(host, player.uuid, &cause_of(&entity.nbt));
            return Flow::Continue;
        }
        if !Config::load(&*host).owner_only {
            return Flow::Continue;
        }
        host.action_bar(
            player.uuid,
            Text::translate_with_fallback(
                "tombstone.message.not_yours",
                "This grave belongs to someone else.",
                &[],
            ),
        );
        Flow::Cancel
    }

    /// Only the grave's owner, a creative player or an operator breaks it, so
    /// nobody else can empty it by mining.
    fn destroy_block(&self, host: &mut dyn Host, event: &mut DestroyBlockEvent) -> Flow {
        if host.block(event.pos) != GRAVE_BLOCK {
            return Flow::Continue;
        }
        let Some(entity) = host.mod_block_entity(event.pos) else {
            return Flow::Continue;
        };
        let Some(owner) = owner_of(&entity.nbt) else {
            return Flow::Continue;
        };
        let Some(player) = host.acting_player() else {
            return Flow::Cancel;
        };
        if player.uuid == owner || player.game_mode == 1 || player.permission >= 2 {
            return Flow::Continue;
        }
        if !Config::load(&*host).owner_only {
            return Flow::Continue;
        }
        host.action_bar(
            player.uuid,
            Text::translate_with_fallback(
                "tombstone.message.not_yours",
                "This grave belongs to someone else.",
                &[],
            ),
        );
        Flow::Cancel
    }
}

/// Writes the grave at `pos` and fills it. False, with the block put back if
/// it was written, when the level refused the block or its block entity.
fn place_grave(
    host: &mut dyn Host,
    dimension: &str,
    pos: [i32; 3],
    owner: u128,
    stacks: &[ItemStack],
    cause: Option<&str>,
) -> bool {
    if !write_block(host, dimension, pos, GRAVE_BLOCK) {
        return false;
    }
    let mut filled = false;
    host.change_mod_block_entity(pos, &mut |entity: &mut ModBlockEntity| {
        if entity.items.len() < SLOTS {
            entity.items.resize(SLOTS, None);
        }
        for (slot, stack) in stacks.iter().take(SLOTS).enumerate() {
            entity.items[slot] = Some(stack.clone());
        }
        entity.nbt.insert(OWNER_KEY.into(), uuid_tag(owner));
        if let Some(cause) = cause {
            entity.nbt.insert(CAUSE_KEY.into(), Tag::String(cause.into()));
        }
        filled = true;
    });
    if !filled {
        // No block entity to keep the items: nothing was taken from the
        // player yet, and the block must not stay behind empty.
        let _ = write_block(host, dimension, pos, "minecraft:air");
    }
    filled
}

fn write_block(host: &mut dyn Host, dimension: &str, pos: [i32; 3], block: &str) -> bool {
    let edit = NativeBlockEdit {
        palette: vec![block.to_string()],
        runs: vec![NativeBlockRun {
            x: pos[0],
            z: pos[2],
            y: pos[1],
            height: 1,
            surface: None,
            block: 0,
            only: None,
        }],
        ..Default::default()
    };
    matches!(host.edit_blocks(dimension, &edit), Ok(1))
}

/// Where a grave can stand: the block the player died in, then up to four
/// blocks above it, then the two below. Only a spot that is loaded in the
/// level and holds air, a fluid or a plant is picked.
fn grave_position(
    host: &dyn Host,
    dimension: &str,
    at: [f64; 3],
    config: Config,
) -> Option<[i32; 3]> {
    let level = host.level(dimension)?;
    let base = [
        at[0].floor() as i32,
        at[1].floor() as i32,
        at[2].floor() as i32,
    ];
    candidate_positions(base, config).into_iter().find(|pos| {
        host.level_is_loaded(level, *pos)
            && host
                .level_block(level, *pos)
                .is_some_and(|block| replaceable(&block))
    })
}

fn candidate_positions(base: [i32; 3], config: Config) -> Vec<[i32; 3]> {
    let mut out = vec![base];
    for dy in 1..=config.above {
        out.push([base[0], base[1] + dy, base[2]]);
    }
    for dy in 1..=config.below {
        out.push([base[0], base[1] - dy, base[2]]);
    }
    out
}

/// Blocks a grave may replace: air, a fluid, or a plant growing there.
fn replaceable(block: &str) -> bool {
    matches!(
        block,
        "minecraft:air"
            | "minecraft:cave_air"
            | "minecraft:void_air"
            | "minecraft:water"
            | "minecraft:short_grass"
            | "minecraft:tall_grass"
            | "minecraft:fern"
            | "minecraft:large_fern"
            | "minecraft:dead_bush"
            | "minecraft:snow"
            | "minecraft:vine"
    )
}

/// A UUID as vanilla saves one: four ints, most significant first.
fn uuid_tag(uuid: u128) -> Tag {
    Tag::IntArray(vec![
        (uuid >> 96) as u32 as i32,
        (uuid >> 64) as u32 as i32,
        (uuid >> 32) as u32 as i32,
        uuid as u32 as i32,
    ])
}

/// Tells a player how they died, when the grave knows.
fn tell_cause(host: &mut dyn Host, player: u128, cause: &Option<String>) {
    if let Some(cause) = cause {
        host.action_bar(
            player,
            Text::translate_with_fallback(
                "tombstone.message.grave_cause",
                "You died from %s.",
                &[Text::literal(cause.clone())],
            ),
        );
    }
}

/// The damage type a grave recorded, written for a player: `minecraft:fall`
/// reads as `fall`.
fn cause_of(nbt: &Compound) -> Option<String> {
    let (_, Tag::String(cause)) = nbt
        .iter()
        .find(|(key, _)| key.to_string_lossy() == CAUSE_KEY)?
    else {
        return None;
    };
    Some(pretty_cause(&cause.to_string_lossy()))
}

fn pretty_cause(damage_type: &str) -> String {
    damage_type
        .rsplit(':')
        .next()
        .unwrap_or(damage_type)
        .replace('_', " ")
}

/// The owner a grave's compound keeps, if it has one.
fn owner_of(nbt: &Compound) -> Option<u128> {
    let (_, Tag::IntArray(words)) = nbt
        .iter()
        .find(|(key, _)| key.to_string_lossy() == OWNER_KEY)?
    else {
        return None;
    };
    if words.len() != 4 {
        return None;
    }
    let mut uuid = 0u128;
    for word in words {
        uuid = (uuid << 32) | u128::from(*word as u32);
    }
    Some(uuid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_owner_uuid_survives_the_round_trip() {
        let uuid = 0x0011_2233_4455_6677_8899_aabb_ccdd_eeff_u128;
        let mut nbt = Compound::new();
        nbt.insert(OWNER_KEY.into(), uuid_tag(uuid));
        assert_eq!(owner_of(&nbt), Some(uuid));
        assert_eq!(owner_of(&Compound::new()), None);
        nbt.insert(OWNER_KEY.into(), Tag::IntArray(vec![1, 2]));
        assert_eq!(owner_of(&nbt), None);
    }

    #[test]
    fn a_grave_takes_a_whole_player_inventory() {
        // 36 inventory slots, 4 armour slots and the off hand.
        assert!(SLOTS >= 41);
    }

    #[test]
    fn the_death_cause_survives_the_round_trip_and_reads_plainly() {
        let mut nbt = Compound::new();
        nbt.insert(CAUSE_KEY.into(), Tag::String("minecraft:fall".into()));
        assert_eq!(cause_of(&nbt), Some("fall".to_owned()));
        nbt.insert(CAUSE_KEY.into(), Tag::String("tombstone:magic_blast".into()));
        assert_eq!(cause_of(&nbt), Some("magic blast".to_owned()));
        assert_eq!(cause_of(&Compound::new()), None);
        assert_eq!(pretty_cause("player_attack"), "player attack");
    }

    #[test]
    fn the_grave_is_looked_for_at_the_feet_first_then_above() {
        let positions = candidate_positions([4, 64, -7], Config::default());
        assert_eq!(
            positions,
            [
                [4, 64, -7],
                [4, 65, -7],
                [4, 66, -7],
                [4, 67, -7],
                [4, 68, -7],
                [4, 63, -7],
                [4, 62, -7],
            ]
        );
    }

    #[test]
    fn the_config_reads_what_it_knows_and_clamps_the_rest() {
        let mut config = Config::default();
        config.read(
            r#"{"owner_only": false, "protect_from_explosions": false,
               "grave_above": 40, "grave_below": -3, "unknown": 1}"#,
        );
        assert!(!config.owner_only);
        assert!(!config.protect_from_explosions);
        assert_eq!(config.above, 8);
        assert_eq!(config.below, 0);

        // A broken file and wrong types leave the defaults alone.
        let mut config = Config::default();
        config.read("not json");
        assert_eq!(config, Config::default());
        config.read(r#"{"owner_only": "yes", "grave_above": "high"}"#);
        assert_eq!(config, Config::default());

        // The written file reads back as the same settings.
        let written = Config::default().to_json();
        let mut again = Config::default();
        again.read(&written);
        assert_eq!(again, Config::default());
    }

    #[test]
    fn only_air_fluids_and_plants_are_replaced() {
        for block in ["minecraft:air", "minecraft:water", "minecraft:tall_grass"] {
            assert!(replaceable(block), "{block}");
        }
        for block in ["minecraft:stone", "minecraft:chest", "tombstone:grave"] {
            assert!(!replaceable(block), "{block}");
        }
    }
}

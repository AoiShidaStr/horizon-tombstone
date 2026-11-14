# Tombstone (Horizon port)

A clean-room port of the core of [Corail Tombstone](https://www.curseforge.com/minecraft/mc-mods/corail-tombstone)
to [Horizon](https://github.com/horizon-rust/horizon), for the Saison 2 server: **on death, a
grave keeps the player's inventory, and only its owner can get it back.**

The original mod is **All Rights Reserved** and its source is not public, so nothing was copied from
it: no code, no texture, no language file. The behaviour was rebuilt from its public description
("Keeps safe your inventory items in graves on death") and the block, its texture and all the code
here are original work.

## What it does

- A player who dies (with `keepInventory` off) leaves no items on the ground. Their 36 inventory
  slots, four armour slots and off hand go into a **grave** block placed where they died — at their
  feet, or up to four blocks above if that spot is solid, then two below.
- Chat tells the dead player where the grave is (`tombstone.message.grave_placed`).
- Right-clicking the grave opens a 54-slot chest menu (9x6). Only the grave's owner opens it; an
  operator (permission level 2 or more) may too.
- Breaking a grave is refused for anyone but its owner, a creative-mode player or an operator, so
  nobody else can empty it by mining. A grave broken by its owner empties onto the ground.
- Hoppers cannot reach a grave's slots.
- A grave is spared by explosions (a creeper or TNT blast leaves it standing); pistons already refuse to
  move a mod block-entity block, and a solid grave is not something a fluid or fire can flow into.
- The grave records the damage type that killed its owner and tells them when it is placed and when they
  open it (`tombstone.message.grave_cause`).
- A grave is an upright headstone with its own outline and collision, not a full cube: only the stone
  slab and the headstone itself stop a player or a projectile.
- The grave has an item, so `/give <player> tombstone:grave` works and it can be placed by hand; it
  drops itself when broken.
- Each death leaves its owner a **Grave's Key** (`tombstone:grave_key`), given on respawn and bound in
  its saved data to that grave's dimension and position. Using it in the air points at the grave
  (coordinates, dimension, distance). That also lets its holder open that one grave: an owner can hand
  the key to a friend, who right-clicks it in the air and then opens the grave once. A key made in
  creative is unbound and says so.
- The grave is craftable (eight stone bricks around a bone) and has its own creative tab
  (`itemGroup.tombstone.graves`, en/fr names) showing the four grave styles and the Grave's Key.
- If the grave cannot be written (chunk not loaded, an unexpected block), the items are left to
  vanilla: nothing is lost.
- `keepInventory` is respected (the game does not call the hook then, so no grave is made).

## Configuration

`config/tombstone.json`, beside `options.txt` (on a dedicated server, beside `server.properties`).
The file is written with these defaults the first time the mod runs; an unknown key or a broken file
leaves the defaults in place.

```json
{ "owner_only": true, "protect_from_explosions": true, "grave_above": 4, "grave_below": 2 }
```

| Key | Meaning |
|---|---|
| `owner_only` | Only the grave's owner (and an operator) opens or breaks it. `false` lets anyone take it. |
| `protect_from_explosions` | A blast leaves graves standing. `false` lets creeper and TNT blasts break them. |
| `grave_above` | How far above the death spot a grave may look for room, 0 to 8. |
| `grave_below` | How far below it may look, 0 to 4. |

## Files

```
content/                      the content mod (.hmod): the grave block, model, texture, language
  horizon-mod.toml
  data/tombstone/horizon/block/grave.json      the block (template minecraft:crafting_table:
                                               no state, opens a menu when used; headstone
                                               outline and collision)
  data/tombstone/horizon/item/grave_key.json   the Grave's Key item
  data/tombstone/loot_table/blocks/grave.json  the grave drops itself
  assets/tombstone/...                         blockstate, headstone model, 16x16 textures,
                                               the key's item model and texture, en_us/fr_fr
native/                       the native mod (.hnmod)
  horizon-native.json         .hnmod manifest, pinned to Horizon a85af177
  mixins/tombstone/
    horizon-mixin.toml        trait mod bundle: [api] path = "api"
    api/                      the Rust crate (mod-tombstone)
      src/lib.rs              the whole mod
tools/
  pack.py                     writes target/tombstone.hmod and .hnmod per docs/api/mod-files.md
  vps/sync.sh                 copy the mod folder to the build VPS and stage it
  vps/stage.sh                reproduce the launcher's native-mod API staging in a worktree
  vps/check.sh                mod crate check, tests, server build and client check
  vps/verify.sh               validate every build exit code and both packed bundles
  vps/pack_check.sh           pack locally, then run cargo-horizon-mod's reader over both files
```

### How it plugs into Horizon

| Step | Hook | API |
|---|---|---|
| The dying player's inventory, before it is thrown | `loot_drops` (`entity` is `minecraft:player`, `player` is the owner) | 405 |
| The damage type that killed them, recorded in the grave | `living_death` | 392 |
| The grave's key, given when the dead player respawns | `player_respawned` | 115 |
| A key used in the air points at its grave and unlocks it once | `use_item` | 13 |
| A grave is spared by a blast | `explosion_detonate` | 22 |
| The grave block is written and its block entity created | `edit_blocks` + `block_entity` | 126 / 15 |
| Its 54 slots and the 9x6 menu | `block_entity_container` (menu `minecraft:generic_9x6`) | 16 / 17 |
| The block entity is filled in place, with its owner | `Host::change_mod_block_entity` (`native_with_mod_block_entity`) | 18 (host) |
| Owner-only opening and breaking | `right_click_block`, `destroy_block` | 21 / 14 |

The content mod is `side = "both"`: a client needs it for the grave's model and texture, and the
server refuses a client whose file differs. The native part is server-side work; the launcher builds
it into every game it is installed in, where its hooks answer nothing on a client.

## Done

- Inventory kept in a grave on death; nothing dropped.
- `config/tombstone.json` with `owner_only`, `protect_from_explosions`, `grave_above` and `grave_below`,
  written with its defaults on first use and read at each death/open/break.
- Owner-only menu (54 slots) and owner-only breaking; operators and creative can clean up.
- Grave placement searches the death column (feet, +4 up, -2 down) and only replaces air, fluids
  and plants; an unloadable or refused spot falls back to vanilla drops.
- Original headstone block model and 16x16 textures; the grave drops itself when broken.
- 26.3 item definitions under `assets/tombstone/items/` for the grave and the key (a missing
  `items/<name>.json` leaves the creative slot blank).
- Grave's Key: granted on respawn, bound to its grave in `minecraft:custom_data`, locates the grave
  in the air and lets its holder open that grave once; unbound creative keys report they have no grave.
- Unit tests for the owner UUID round trip, the death cause round trip, the grave-key round trip, the
  placement order and the replaceable-block list.

## Missing / not done

- **Config**: only the four settings above (Corail Tombstone has a large config). No soulbound
  enchantment, no perks, no grave decay/expiry, no enchanted key or grave souls. The three
  decorative variants are static: no engraving, randomized appearance, fog or haunting soul.
- **Other original systems**: no abandoned-grave structures or their loot, Grave Guardian, undead
  companions, flying carpet, rituals or exploration scrolls/tablets.
- **Death cause / killer / XP**: the grave records the damage type (`minecraft:fall` reads as "fall") but
  not who dealt it (the `living_death` hook names a mob attacker, never a player) nor when. Experience is
  still dropped as vanilla orbs: no hook offers a dead player's XP to keep it.
- **Grave access**: the Grave's Key only locates and opens a grave; there is no enchanted key, no
  teleport to a grave, no `/tbs` style command and no grave list. Opening with a key is a two-step
  action (key in the air, then the grave), because the block-click hook cannot read the held stack's
  saved data. An unbound key cannot be crafted into a bound one.
- **Grave protection**: only player breaking and opening are gated; explosions are now spared, but a
  grave has no lock, no decay and no way to record or undo a blast that somehow removed it. Vanilla's
  `keepInventory` and `mobGriefing` are not consulted beyond the game's own rules.

- **Client UI**: the grave uses the vanilla chest screen; there is no custom grave screen or
  rotating soul effect on the static 3D headstone.
- **A mod block entity's items and a vanilla chest menu** are the only storage this uses; a hopper
  cannot reach them and a comparator reads nothing from a grave.
- Not tested on a real client/server session by this run: only `cargo check`/`cargo test` and the
  content checks below were run (see "Build and checks"). The headstone model, textures, creative
  tab contents and the key flow have therefore not been seen in a running game.

## Build and checks

Everything is built on the VPS (4 cores, 8 GB), never locally: a worktree of the pinned
Horizon clone at `/opt/build/wt/tombstone`, with the launcher's native-mod staging
reproduced by `tools/vps/stage.sh`. The scripts in `tools/vps/` are the ones actually used:

```sh
tools/vps/sync.sh          # from this PC: copy the mod to /opt/build/mods/tombstone and stage
# on the VPS:
bash /opt/build/mods/tombstone/tools/vps/check.sh          # cargo check -p mod-tombstone
bash /opt/build/mods/tombstone/tools/vps/check.sh full     # + tests, server and client flavours
tools/vps/pack_check.sh    # from this PC: pack, then cargo-horizon-mod check on both files
```

What the checks cover:

```sh
cargo check -p mod-tombstone                     # the mod, server flavour
cargo test  -p mod-tombstone                     # 7 unit tests
cargo build -p horizon-native-game --no-default-features   # dedicated server
cargo check -p horizon-native-game               # client flavour (default features)
cargo horizon-mod check target/tombstone.hmod    # content: API 0.2, side both, content only
cargo horizon-mod check target/tombstone.hnmod   # native: format native-1, pinned revisions
```

Latest run (14 Nov 2026), one queued full verification, `/opt/build/queue-tombstone-rebuild.log`,
started 08:21 and finished 08:32:

- `cargo check -p mod-tombstone` — exit 0 in 2m21s.
- `cargo test -p mod-tombstone` — **7 passed, 0 failed**, exit 0.
- `cargo build -p horizon-native-game --no-default-features` (dedicated server) — exit 0 in 9.59s.
- `cargo check -p horizon-native-game` (client flavour) — exit 0 in 2m30s.
- `cargo-horizon-mod horizon-mod check tombstone.hmod` — exit 0, content, API 0.2, both,
  20312 bytes, SHA-256 `a2e7233886064bc5…`.
- `cargo-horizon-mod horizon-mod check tombstone.hnmod` — exit 0, native-1, pinned to
  `a85af177a8…` / mixins `5a64af93…`, 10099 bytes, SHA-256 `5cd6d4d3ba0110c3…`.
- All 35 content JSON files parse, and all five creative-tab entries have their assets and a
  survival source (recipes for blocks, respawn grant for the key).

Prior run (14 Nov 2026), `/opt/build/queue-tombstone-buffy.log`, started 04:53 and finished 05:07:

- `cargo check -p mod-tombstone` — exit 0 in 2m45s (default features, so the client flavour of
  `horizon-client` is compiled too).
- `cargo test -p mod-tombstone` — **7 passed, 0 failed**, exit 0 (the six earlier tests plus the
  grave-key round trip).
- `cargo check -p horizon-native-game --no-default-features` (dedicated server) — exit 0 in 11s
  (the shared target was warm).
- `cargo check -p horizon-native-game` (client binary flavour) — exit 0 in 2m48s.
- `cargo horizon-mod check target/tombstone.hmod` — exit 0, content, API 0.2, both, 7193 bytes,
  SHA-256 `e70f5029c2535eaf…`.
- `cargo horizon-mod check target/tombstone.hnmod` — exit 0, native-1, pinned to
  `a85af177a8…` / mixins `5a64af93…`, 10099 bytes, SHA-256 `5cd6d4d3ba0110c3…`.

Nothing is launched in a game: the headstone model, the key's item icon, the creative tab and the
key flow are checked at compile and bundle level only. The grave was **not** exercised in a live
client/server session.

Runs (13 Nov 2026):

- Full pass on the first tree, `/opt/build/tombstone-check.log` — mod crate check, the four
  unit tests, the dedicated-server engine (no default features) and the client flavour all
  exited 0.
- Earlier re-check, `/opt/build/tombstone-verify.log` — `cargo check -p mod-tombstone` exited 0
  (14m21s, mostly waiting on the shared target lock other mods' builds hold);
  `/opt/build/tombstone-test.log` — 4 passed, 0 failed.
- Check of the tree with explosions, death cause, config and the grave item,
  `/opt/build/tombstone-check3.log` — `cargo check -p mod-tombstone` exited 0 at 23:14 (23m15s in
  the shared target). The six unit tests and the two engine flavours never got the build lock, so
  **only the crate check is confirmed for this tree**. Two edits came after it: the module doc
  comment, and moving the death-cause line from the action bar (hidden by the death screen) to
  chat. That second edit is ~12 lines reusing the calls already checked, but its own
  `cargo check` (`/opt/build/tombstone-check4.log`) was still waiting for the shared build
  directory when the hour ended. `check.sh full` has since passed on this tree (see the 14 Nov run
  above), so that edit is verified.
- `cargo-horizon-mod check` on both freshly packed files: `tombstone.hmod` reads as a content mod
  (API 0.2, side both, nothing to run, now carrying the grave item and its item model) and
  `tombstone.hnmod` as native code pinned to `a85af177a8…` / mixins `5a64af93…`; both exit 0.

The grave was **not** exercised in a live client/server session: no game was launched.

## Licence

- The port's own code, block, model and texture (everything in this folder): **MIT** —
  see [LICENSE](LICENSE).
- Corail Tombstone, the mod this one imitates, is **All Rights Reserved** and closed source
  (CurseForge project 243707, Modrinth project YT5K34AB). No file, code or asset from it is used
  here, and none may be redistributed from it. This port is an independent reimplementation of the
  publicly described behaviour, for private use on the Saison 2 server.
- Minecraft is a trademark of Mojang Studios; this mod is not approved by or associated with Mojang
  or Microsoft.

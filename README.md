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
- If the grave cannot be written (chunk not loaded, an unexpected block), the items are left to
  vanilla: nothing is lost.
- `keepInventory` is respected (the game does not call the hook then, so no grave is made).

## Files

```
content/                      the content mod (.hmod): the grave block, model, texture, language
  horizon-mod.toml
  data/tombstone/horizon/block/grave.json      the block (template minecraft:crafting_table:
                                               no state, opens a menu when used)
  assets/tombstone/...                         blockstate, model, 16x16 texture, en_us/fr_fr
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
  vps/check.sh                mod crate check, tests, server and client flavours
  vps/pack_check.sh           pack locally, then run cargo-horizon-mod's reader over both files
```

### How it plugs into Horizon

| Step | Hook | API |
|---|---|---|
| The dying player's inventory, before it is thrown | `loot_drops` (`entity` is `minecraft:player`, `player` is the owner) | 405 |
| The grave block is written and its block entity created | `edit_blocks` + `block_entity` | 126 / 15 |
| Its 54 slots and the 9x6 menu | `block_entity_container` (menu `minecraft:generic_9x6`) | 16 / 17 |
| The block entity is filled in place, with its owner | `Host::change_mod_block_entity` (`native_with_mod_block_entity`) | 18 (host) |
| Owner-only opening and breaking | `right_click_block`, `destroy_block` | 21 / 14 |

The content mod is `side = "both"`: a client needs it for the grave's model and texture, and the
server refuses a client whose file differs. The native part is server-side work; the launcher builds
it into every game it is installed in, where its hooks answer nothing on a client.

## Done

- Inventory kept in a grave on death; nothing dropped.
- Owner-only menu (54 slots) and owner-only breaking; operators and creative can clean up.
- Grave placement searches the death column (feet, +4 up, -2 down) and only replaces air, fluids
  and plants; an unloadable or refused spot falls back to vanilla drops.
- Original block, model, texture and en/fr language.
- Unit tests for the owner UUID round trip, the placement order and the replaceable-block list.

## Missing / not done

- **Config**: nothing is configurable (Corail Tombstone has a large config). No grave key item, no
  soulbound enchantment, no perks, no grave decay/expiry, no grave souls or decorations.
- **Death cause / killer / XP**: the grave keeps items only. Experience is still dropped as
  vanilla orbs, and the grave does not record the death message, the killer or the time.
- **Grave access**: no "grave key", no `/tbs` style commands, no grave list, no teleport to a grave.
- **Grave protection**: a grave can be destroyed by an explosion or a piston/fluid, not only by a
  player; only player breaking and opening are gated. Vanilla's `keepInventory` and `mobGriefing`
  are not consulted beyond the game's own rules.
- **No in-game way to obtain the grave item** (`"item": false`): it is only placed by a death, so it
  does not appear in the creative inventory and `/give` cannot make one.
- **Client UI**: the grave uses the vanilla chest screen; there is no custom grave screen, and the
  grave does not render as a 3D headstone with a rotating soul.
- **A mod block entity's items and a vanilla chest menu** are the only storage this uses; a hopper
  cannot reach them and a comparator reads nothing from a grave.
- Not tested on a real client/server session by this run: only `cargo check`/`cargo test` and the
  content checks below were run (see "Build and checks").

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
cargo test  -p mod-tombstone                     # 4 unit tests, all pass
cargo check -p horizon-native-game --no-default-features   # dedicated server
cargo check -p horizon-native-game               # client flavour (default features)
cargo horizon-mod check target/tombstone.hmod    # content: API 0.2, side both, content only
cargo horizon-mod check target/tombstone.hnmod   # native: format native-1, pinned revisions
```

Last full run (13 Nov 2026, `/opt/build/tombstone-check.log` and
`/opt/build/tombstone-verify.log`): every step exited 0, the four unit tests passed,
and both packages were read cleanly by `cargo-horizon-mod` 0.1.0 built from the pinned
Horizon revision. The grave was **not** exercised in a live client/server session.

## Licence

- The port's own code, block, model and texture (everything in this folder): **MIT** —
  see [LICENSE](LICENSE).
- Corail Tombstone, the mod this one imitates, is **All Rights Reserved** and closed source
  (CurseForge project 243707, Modrinth project YT5K34AB). No file, code or asset from it is used
  here, and none may be redistributed from it. This port is an independent reimplementation of the
  publicly described behaviour, for private use on the Saison 2 server.
- Minecraft is a trademark of Mojang Studios; this mod is not approved by or associated with Mojang
  or Microsoft.

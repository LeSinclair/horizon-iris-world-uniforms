# Iris World Uniforms (Horizon native mod)

> [!WARNING]
> **This mod was written by AI** (Claude, Anthropic's AI assistant), not by a person, and has had
> only light testing: horizon-mixins applies it, the game compiles with it and its values were
> checked in a unit test, but it hasn't been played with in game. Like every Horizon native mod it is **not sandboxed**: it compiles into the
> game and runs with your rights. Read the source before you build it, and use it at your own risk.

A native mod (a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle) for
[Horizon](https://github.com/horizon-rust/horizon) that gives Iris shader packs the world uniforms
Horizon reads as zero:

| Uniform | Value |
|---|---|
| `cloudHeight` | 192.33 in the Overworld, NaN elsewhere (BSL and Complementary drew their clouds at Y=0 without it) |
| `bedrockLevel`, `heightLimit`, `logicalHeightLimit` | the dimension's min Y, height and logical height |
| `isRightHanded` | the Main Hand option |
| `endFlashIntensity`, `previousEndFlashIntensity`, `endFlashPosition` | the End's sky flash (zero outside the End) |

## Second mod: Shader Pack World Reset

`shader_pack_world_reset/` is a separate mod in the same repository. Horizon kept an Iris shader
pack loaded from one world to the next, so whatever the last world left in buffers the pack keeps
between frames stayed on screen. With BSL, the block outline you had selected when you left a
world floated in the next one, in world space, until the pack was turned off and on. This mod does
what turning the pack off and on does whenever a world closes (single player, a LAN host, or
leaving a server), so every world starts with a fresh pack, as with Iris. The pack loads again
when the next world opens (about a quarter of a second for BSL).

Hooks: HEAD on `Game::finish_close_world` and `Game::leave_server`; support module `reset.rs` in
`game.rs`. Checked with horizon-mixins `apply` and `cargo check` (no warnings, together with
`iris_world`); not tried in game yet.

## Install

From the launcher's Mods page ("Add from GitHub") with this repository's link, or:

```sh
horizon-launcher mods add-github https://github.com/LeSinclair/horizon-iris-world-uniforms --all --trust-native-code
```

(`--all` installs both mods; `--mod iris_world` or `--mod shader_pack_world_reset` picks one), or from a clone:

```sh
horizon-launcher native-build path/to/horizon-iris-world-uniforms/iris_world --instance <name> --trust-native-code
```

Then build the game on the Mods page. The first native build downloads the launcher's Rust and
Horizon's source and takes 20 to 30 minutes.

## Horizon version

Pinned to Nightly 2026-11-01 (`4aa6854a88…`, `[compatibility] horizon_revision`). Horizon added
`cloudHeight`, `bedrockLevel`, `heightLimit`, `logicalHeightLimit` and `isRightHanded` itself just
after that Nightly (commit `6ad0a9df8`). From the next Nightly on, only the End flash uniforms are
new, and Horizon's own values win over the mod's. A newer Nightly needs the pin raised and the mod
checked again.

## How it works

No Horizon struct, constant or render code is changed. The mod adds the eight uniforms as if the
pack had written them in `shaders.properties` (`uniform.<type>.<name>`), in front of the pack's own
custom uniforms, so Horizon's existing custom-uniform path uploads them to every program and the
pack's own expressions can read them. Their values come from constants the game sets every frame.
A name the pack defines itself stays the pack's.

| File | What |
|---|---|
| `iris_world/horizon-mixin.toml` | the bundle: 3 support modules, 3 hooks |
| `state.rs` | module `horizon_shaders::iris_world`: this frame's values, the End flash maths, a test |
| `items.rs` | module in `horizon-shaders/src/custom.rs`: adds the uniforms, sets the constants |
| `flash.rs` | module in `game/end.rs`: reads the End flash |
| `add_uniforms.rs` | HEAD hook on `Custom::set_biomes` (the game calls it on every pack it loads, before the renderer reads the uniform names) |
| `frame_values.rs` | HEAD hook on `Custom::eval` |
| `game_values.rs` | HEAD hook on `Game::shader_pack_inputs` |

## Testing done

With horizon-mixins at the launcher's pinned commit (`a2af5c0`), `plan` and `apply` succeed on the
pinned Horizon source. On the staged source:

- `cargo check -p horizon-client` passes without warnings;
- a probe test parsed a pack with `uniform.float.myClouds = cloudHeight + 1` and
  `uniform.int.heightLimit = 7` and got every value right (the pack's own `heightLimit` kept,
  `myClouds` saw NaN, the End flash position straight up);
- four of Horizon's own `custom::tests` would fail, only because they count a pack's uniforms (the
  launcher doesn't run them).

Not done yet: playing with a pack in game. Issues and fixes welcome.

## License

MIT, see [LICENSE](LICENSE).

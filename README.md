# Iris World Uniforms (Horizon native mod)

> [!WARNING]
> **This mod was written by AI** (Claude, Anthropic's AI assistant), not by a person, and has had
> only light testing: horizon-mixins applies it, the game compiles with it and its values were
> checked in a unit test, but it hasn't been played with in game. Like every Horizon native mod it is **not sandboxed**: it compiles into the
> game and runs with your rights. Read the source before you build it, and use it at your own risk.

A native mod (a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle) for
[Horizon](https://github.com/horizon-rust/horizon) that gives Iris shader packs the world uniform
Horizon still doesn't provide:

| Uniform | Value |
|---|---|
| `previousEndFlashIntensity` | the End sky flash's intensity one tick ago (zero outside the End) |

Version 1.0.0 (for Nightly 2026-11-01) also added `cloudHeight`, `bedrockLevel`, `heightLimit`,
`logicalHeightLimit`, `isRightHanded`, `endFlashIntensity` and `endFlashPosition` (BSL and
Complementary drew their clouds at Y=0 without `cloudHeight`). Horizon gives all of those itself
since then, so 2.0.0 adds only the one it still lacks.

## Second mod: Shader Pack World Reset

`shader_pack_world_reset/` is a separate mod in the same repository. Horizon kept an Iris shader
pack loaded from one world to the next, so whatever the last world left in buffers the pack keeps
between frames stayed on screen. With BSL, the block outline you had selected when you left a
world floated in the next one, in world space, until the pack was turned off and on. This mod does
what turning the pack off and on does whenever a world closes (single player, a LAN host, or
leaving a server), so every world starts with a fresh pack, as with Iris. The pack loads again
when the next world opens (about a quarter of a second for BSL).

Hooks: HEAD on `Game::finish_close_world` and `Game::leave_server`; support module `reset.rs` in
`game.rs`. Horizon still keeps the pack between worlds in Nightly 2026-11-04, so the mod is still
needed; 1.1.0 also clears the translucent layers, as Horizon's own pack switch now does. Checked
with horizon-mixins `apply` and `cargo check` (no warnings, together with `iris_world`); not tried
in game yet.

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

Pinned to Nightly 2026-11-04 (`6475f62f3c…`, `[compatibility] horizon_revision`). A newer Nightly
needs the pin raised and the mods checked again. Older versions: 1.0.0 of both mods (commit
`af6038a`) is for Nightly 2026-11-01 (`4aa6854`).

## How it works

No Horizon struct, constant or render code is changed. The mod adds the uniform as if the pack had
written it in `shaders.properties` (`uniform.float.previousEndFlashIntensity`), in front of the
pack's own custom uniforms, so Horizon's existing custom-uniform path uploads it to every program
and the pack's own expressions can read it. Its value comes from a constant the game sets every
frame. A name the pack defines itself stays the pack's.

| File | What |
|---|---|
| `iris_world/horizon-mixin.toml` | the bundle: 3 support modules, 3 hooks |
| `state.rs` | module `horizon_shaders::iris_world`: this frame's value, a test |
| `items.rs` | module in `horizon-shaders/src/custom.rs`: adds the uniform, sets the constant |
| `flash.rs` | module in `game/end.rs`: reads the End flash |
| `add_uniforms.rs` | HEAD hook on `Custom::set_biomes` (the game calls it on every pack it loads, before the renderer reads the uniform names) |
| `frame_values.rs` | HEAD hook on `Custom::eval` |
| `game_values.rs` | HEAD hook on `Game::shader_pack_inputs` |

## Testing done

With the launcher's horizon-mixins (`a2af5c0`), `apply` succeeds on Nightly 2026-11-04's source
(both mods). On the staged source:

- `cargo check -p horizon-client` passes without warnings;
- a probe test parsed a pack with `uniform.float.mine = previousEndFlashIntensity * 2 + 1` and
  `uniform.float.endFlashIntensity = 3` and got every value right (the pack's own
  `endFlashIntensity` kept);
- one of Horizon's own `custom::tests` would fail, only because it counts a pack's uniforms (the
  launcher doesn't run them).

Not done yet: playing with a pack in game. Issues and fixes welcome.

## License

MIT, see [LICENSE](LICENSE).

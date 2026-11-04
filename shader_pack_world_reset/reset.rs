// Shader Pack World Reset (native mod `shader_pack_world_reset`).

use super::Game;

/// What turning the shader pack off and on does: the renderer lets go of the pack's pipelines
/// and buffers, and the next world's first frame loads the pack again (as Iris does per world).
pub(super) fn reset(game: &mut Game<'_>) {
    // The same steps as Horizon's own pack switch (shader_packs.rs, `shader_pack_inputs`).
    game.native_shaders = super::native_shaders::NativeShaders::picking("");
    game.set_translucent_layers(Vec::new());
    game.pack_ids = None;
    if let Some(renderer) = &mut game.renderer {
        renderer.drop_shader_packs();
    }
}

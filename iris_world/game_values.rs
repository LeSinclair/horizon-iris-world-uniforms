fn game_values(game: &crate::game::Game<'_>, partial: f32) {
    let (end_flash, previous_end_flash, end_flash_angles) =
        crate::game::end::iris_world_flash::get(game, partial);
    let shape = &game.world.shape;
    horizon_shaders::iris_world::set(horizon_shaders::iris_world::World {
        bedrock_level: shape.min_y,
        height_limit: shape.height,
        logical_height_limit: shape.logical_height,
        // The clouds sit at 192.33 (clouds.rs) and only the Overworld draws them.
        cloud_height: if game.dim == crate::game::Dim::Overworld {
            192.33
        } else {
            f32::NAN
        },
        right_handed: !game.settings.main_hand_left,
        end_flash,
        previous_end_flash,
        end_flash_angles,
    });
}

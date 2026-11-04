fn game_values(game: &crate::game::Game<'_>) {
    horizon_shaders::iris_world::set(horizon_shaders::iris_world::World {
        previous_end_flash: crate::game::end::iris_world_flash::previous(game),
    });
}

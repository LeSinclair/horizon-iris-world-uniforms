fn on_leave(game: &mut crate::game::Game<'_>) {
    crate::game::shader_pack_world_reset::reset(game);
}

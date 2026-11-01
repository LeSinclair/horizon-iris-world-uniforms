fn frame_values(custom: &mut Custom, builtin: &dyn Fn(&str) -> Option<Val>) {
    iris_world_items::frame(custom, builtin);
}

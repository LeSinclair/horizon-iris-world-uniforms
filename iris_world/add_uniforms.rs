fn add_uniforms(value: Custom) -> Custom {
    let mut custom = value;
    iris_world_items::add(&mut custom);
    custom
}

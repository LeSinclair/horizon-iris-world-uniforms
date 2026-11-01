// Iris World Uniforms (native mod `iris_world`): the uniforms as custom uniforms of the pack.

use super::{Custom, Expr, Item, Val};
use crate::iris_world::{UNIFORMS, constant, end_flash_position, get};

/// Puts the uniforms in front of the pack's own custom uniforms and variables, so the pack's
/// expressions can read them too. A name the pack defines itself stays the pack's.
pub(crate) fn add(custom: &mut Custom) {
    let mut ours = Vec::new();
    for (name, ty) in UNIFORMS {
        if custom.items.iter().any(|i| i.name == name) {
            continue;
        }
        let read = |c: usize| Expr::Name(constant(name, c), None);
        let expr = if ty == "vec3" {
            custom.calls += 1;
            Expr::Call("vec3".into(), (0..3).map(read).collect(), custom.calls)
        } else {
            read(0)
        };
        ours.push(Item {
            name: name.to_owned(),
            ty: ty.to_owned(),
            uniform: true,
            expr,
        });
    }
    ours.append(&mut custom.items);
    custom.items = ours;
}

/// Sets this frame's values as the constants the uniforms read.
pub(crate) fn frame(custom: &mut Custom, builtin: &dyn Fn(&str) -> Option<Val>) {
    let world = get();
    let view = match builtin("gbufferModelView") {
        Some(v) if v.n == 16 => v.v,
        _ => [0.0; 16],
    };
    let flash = end_flash_position(&world, &view);
    let values: [&[f64]; 8] = [
        &[f64::from(world.bedrock_level)],
        &[f64::from(world.height_limit)],
        &[f64::from(world.logical_height_limit)],
        &[f64::from(world.cloud_height)],
        &[f64::from(u8::from(world.right_handed))],
        &[f64::from(world.end_flash)],
        &[f64::from(world.previous_end_flash)],
        &flash,
    ];
    for ((name, _), value) in UNIFORMS.iter().zip(values) {
        for (c, &x) in value.iter().enumerate() {
            custom.constants.insert(constant(name, c), x);
        }
    }
}

// Iris World Uniforms (native mod `iris_world`): the uniform as a custom uniform of the pack.

use super::{Custom, Expr, Item, Val};
use crate::iris_world::{UNIFORMS, constant, get};

/// Puts the uniforms in front of the pack's own custom uniforms and variables, so the pack's
/// expressions can read them too. A name the pack defines itself stays the pack's.
pub(crate) fn add(custom: &mut Custom) {
    let mut ours = Vec::new();
    for (name, ty) in UNIFORMS {
        if custom.items.iter().any(|i| i.name == name) {
            continue;
        }
        ours.push(Item {
            name: name.to_owned(),
            ty: ty.to_owned(),
            uniform: true,
            expr: Expr::Name(constant(name, 0), None),
        });
    }
    ours.append(&mut custom.items);
    custom.items = ours;
}

/// Sets this frame's values as the constants the uniforms read.
pub(crate) fn frame(custom: &mut Custom, _builtin: &dyn Fn(&str) -> Option<Val>) {
    let world = get();
    let values = [f64::from(world.previous_end_flash)];
    for ((name, _), value) in UNIFORMS.iter().zip(values) {
        custom.constants.insert(constant(name, 0), value);
    }
}

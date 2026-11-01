// Iris World Uniforms (native mod `iris_world`): the End flash for shader packs.

use super::{Dim, Game};

/// `(intensity, previous intensity, [xAngle, yAngle])`; zero outside the End.
pub(crate) fn get(game: &Game<'_>, partial: f32) -> (f32, f32, [f32; 2]) {
    if game.dim != Dim::End {
        return (0.0, 0.0, [0.0; 2]);
    }
    let flash = &game.end.flash;
    (flash.intensity(partial), flash.old_intensity, [flash.x_angle, flash.y_angle])
}

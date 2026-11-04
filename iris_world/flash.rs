// Iris World Uniforms (native mod `iris_world`): the End flash for shader packs.

use super::{Dim, Game};

/// The previous tick's flash intensity; zero outside the End.
pub(crate) fn previous(game: &Game<'_>) -> f32 {
    if game.dim != Dim::End {
        return 0.0;
    }
    game.end.flash.old_intensity
}

// Iris World Uniforms (native mod `iris_world`): this frame's value of the Iris world uniform
// Horizon doesn't provide, set by the game, read when the pack's custom uniforms are evaluated.

use std::sync::Mutex;

/// The uniforms, their GLSL types and the constants their values are read from. Horizon gives
/// the other world uniforms itself since Nightly 2026-11-04.
pub const UNIFORMS: [(&str, &str); 1] = [("previousEndFlashIntensity", "float")];

/// What the game knows this frame (Iris: `EndFlashState`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct World {
    /// The previous tick's `EndFlashState` intensity; zero outside the End.
    pub previous_end_flash: f32,
}

pub const NONE: World = World { previous_end_flash: 0.0 };

static WORLD: Mutex<World> = Mutex::new(NONE);

pub fn set(world: World) {
    *WORLD.lock().unwrap_or_else(|e| e.into_inner()) = world;
}

pub fn get() -> World {
    *WORLD.lock().unwrap_or_else(|e| e.into_inner())
}

/// The constant a uniform's value is read from (named so no pack or builtin uses it).
pub fn constant(uniform: &str, component: usize) -> String {
    format!("hz_iris_world_{uniform}_{component}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_then_get() {
        set(World { previous_end_flash: 0.5 });
        assert_eq!(get().previous_end_flash, 0.5);
        set(NONE);
        assert_eq!(
            constant("previousEndFlashIntensity", 0),
            "hz_iris_world_previousEndFlashIntensity_0"
        );
    }
}

// Iris World Uniforms (native mod `iris_world`): this frame's values of the Iris world uniforms
// Horizon doesn't provide, set by the game, read when the pack's custom uniforms are evaluated.

use std::sync::Mutex;

/// The uniforms, their GLSL types and the constants their values are read from.
pub const UNIFORMS: [(&str, &str); 8] = [
    ("bedrockLevel", "int"),
    ("heightLimit", "int"),
    ("logicalHeightLimit", "int"),
    ("cloudHeight", "float"),
    ("isRightHanded", "bool"),
    ("endFlashIntensity", "float"),
    ("previousEndFlashIntensity", "float"),
    ("endFlashPosition", "vec3"),
];

/// What the game knows this frame (Iris: `IrisExclusiveUniforms.WorldInfoUniforms`,
/// `isRightHanded`, `EndFlashState`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct World {
    /// `DimensionType.minY`, `height` and `logicalHeight`.
    pub bedrock_level: i32,
    pub height_limit: i32,
    pub logical_height_limit: i32,
    /// The clouds' altitude, NaN where the level draws none (packs test `isnan(cloudHeight)`).
    pub cloud_height: f32,
    /// The Main Hand option.
    pub right_handed: bool,
    /// `EndFlashState.getIntensity(partial)` and the previous tick's; zero outside the End.
    pub end_flash: f32,
    pub previous_end_flash: f32,
    /// The flash's `(xAngle, yAngle)` in degrees.
    pub end_flash_angles: [f32; 2],
}

/// The Overworld's, until the game says otherwise.
pub const OVERWORLD: World = World {
    bedrock_level: -64,
    height_limit: 384,
    logical_height_limit: 384,
    cloud_height: 192.33,
    right_handed: true,
    end_flash: 0.0,
    previous_end_flash: 0.0,
    end_flash_angles: [0.0; 2],
};

static WORLD: Mutex<World> = Mutex::new(OVERWORLD);

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

/// `endFlashPosition` (Iris `CelestialUniforms.getEndFlashPosition`): `(0, 100, 0)` turned as
/// `SkyRenderer.renderEndFlash` turns the sprite, into view space with `view`
/// (`gbufferModelView`, column after column); zero without a flash.
pub fn end_flash_position(world: &World, view: &[f64; 16]) -> [f64; 3] {
    if world.end_flash <= 0.0 {
        return [0.0; 3];
    }
    let [x_angle, y_angle] = world.end_flash_angles.map(|a| f64::from(a).to_radians());
    // R_y(180° - yAngle) · R_x(-90° - xAngle) · (0, 100, 0).
    let (sx, cx) = (-std::f64::consts::FRAC_PI_2 - x_angle).sin_cos();
    let (sy, cy) = (std::f64::consts::PI - y_angle).sin_cos();
    let after_x = [0.0, 100.0 * cx, 100.0 * sx];
    let world_dir = [
        cy * after_x[0] + sy * after_x[2],
        after_x[1],
        -sy * after_x[0] + cy * after_x[2],
    ];
    std::array::from_fn(|r| (0..3).map(|c| view[c * 4 + r] * world_dir[c]).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_flash_is_zero_and_a_flash_straight_up_stays_up() {
        let identity: [f64; 16] = std::array::from_fn(|i| if i % 5 == 0 { 1.0 } else { 0.0 });
        assert_eq!(end_flash_position(&OVERWORLD, &identity), [0.0; 3]);
        // xAngle -90 turns (0, 100, 0) by 0 about x: it stays straight up.
        let world = World { end_flash: 1.0, end_flash_angles: [-90.0, 0.0], ..OVERWORLD };
        let p = end_flash_position(&world, &identity);
        assert!(p[0].abs() < 1e-9 && (p[1] - 100.0).abs() < 1e-9 && p[2].abs() < 1e-9, "{p:?}");
    }
}

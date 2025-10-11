pub use crate::bevy_prelude::*;
use crate::{areas::Area, editor::editor};

pub const DEVELOP_OVERRIDE: bool = true;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, load);

    if DEVELOP_OVERRIDE || crate::DEVELOP {
        editor(file!())(app);
    }
}

fn load(asset_server: Res<AssetServer>, mut commands: Commands) {
    let scene = asset_server.load("map/test_area.glb#Scene0");
    commands.spawn((SceneRoot(scene), Area {
        patch_function: old_patch,
    }));
}

#[rustfmt::skip]
pub fn old_patch(name: &str, mut transform: Mut<Transform>) {
        #[allow(clippy::match_same_arms)]
        #[allow(clippy::unreadable_literal)]
        #[allow(clippy::single_match)]
        match name {
            "block_loading_cable_cable_1" => {
                *transform = Transform {
                    translation: Vec3::new(-1.3475943, 0.47955698, -0.03334881),
                    rotation: Quat::from_array([0.5495246, -0.5478685, -0.44443572, -0.4475933]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_2" => {
                *transform = Transform {
                    translation: Vec3::new(-0.63684404, 0.479563, -0.030138576),
                    rotation: Quat::from_array([0.5495246, -0.5478685, -0.44443572, -0.4475933]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_3" => {
                *transform = Transform {
                    translation: Vec3::new(0.073906004, 0.47956714, -0.026922224),
                    rotation: Quat::from_array([0.5495246, -0.5478685, -0.44443572, -0.4475933]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_4" => {
                *transform = Transform {
                    translation: Vec3::new(0.78465617, 0.47956875, -0.023698466),
                    rotation: Quat::from_array([0.5495246, -0.5478685, -0.44443572, -0.4475933]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_5" => {
                *transform = Transform {
                    translation: Vec3::new(1.4954064, 0.4795673, -0.020466177),
                    rotation: Quat::from_array([0.5495246, -0.5478685, -0.44443572, -0.4475933]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_7" => {
                *transform = Transform {
                    translation: Vec3::new(2.9168935, 0.47979662, -0.0139723765),
                    rotation: Quat::from_array([0.5495246, -0.5478685, -0.44443572, -0.4475933]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_8" => {
                *transform = Transform {
                    translation: Vec3::new(3.6292863, 0.47996268, -0.010701739),
                    rotation: Quat::from_array([0.5495246, -0.5478685, -0.44443572, -0.4475933]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_9" => {
                *transform = Transform {
                    translation: Vec3::new(4.138556, 0.48009008, -0.008363046),
                    rotation: Quat::from_array([0.5495246, -0.5478685, -0.44443572, -0.4475933]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "cable" => {
                *transform = Transform {
                    translation: Vec3::new(-2.8457935, 0.47978082, -0.04013269),
                    rotation: Quat::from_array([-0.47649276, 0.4741884, 0.5224347, 0.524559]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "battery.001" => {
                *transform = Transform {
                    translation: Vec3::new(-3.8455796, 0.5, -0.044488907),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "battery.002" => {
                *transform = Transform {
                    translation: Vec3::new(-6.7904277, 0.5, 2.8107011),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_first_previous" => {
                *transform = Transform {
                    translation: Vec3::new(-2.0583308, 0.47955006, -0.03655433),
                    rotation: Quat::from_array([0.5495246, -0.5478685, -0.44443572, -0.4475933]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_6" => {
                *transform = Transform {
                    translation: Vec3::new(2.2061567, 0.47962743, -0.01722439),
                    rotation: Quat::from_array([0.5495246, -0.5478685, -0.44443572, -0.4475933]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_tail" => {
                *transform = Transform {
                    translation: Vec3::new(4.6669326, 0.4803723, -0.005967859),
                    rotation: Quat::from_array([0.07517434, 0.078369394, 0.70307994, -0.70277]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            _ => (),
        }
}

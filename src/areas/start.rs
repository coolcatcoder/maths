pub use crate::bevy_prelude::*;
use crate::{
    areas::Area,
    editor::editor,
};

pub const DEVELOP_OVERRIDE: bool = true;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, load);

    if DEVELOP_OVERRIDE || crate::DEVELOP {
        editor(file!())(app);
    }
}

fn load(asset_server: Res<AssetServer>, mut commands: Commands) {
    let scene = asset_server.load("map/test_area.glb#Scene0");
    commands.spawn((SceneRoot(scene), Area));
}

pub fn patch(names: Query<(&Name, &mut Transform), Added<Name>>) {
    for (name, mut transform) in names {
        #[allow(clippy::match_same_arms)]
        #[allow(clippy::unreadable_literal)]
        #[allow(clippy::single_match)]
        match name.as_str() {
            "block_loading_cable_first_previous" => {
                *transform = Transform {
                    translation: Vec3::new(6.568373, 0.89305323, -0.3073188),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_6" => {
                *transform = Transform {
                    translation: Vec3::new(6.5702558, 0.12592277, -0.37843215),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "cable" => {
                *transform = Transform {
                    translation: Vec3::new(6.5698957, 0.8065437, 0.30051267),
                    rotation: Quat::from_array([0.00081706874, -0.10087113, 0.9948645, 0.008303863]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_1" => {
                *transform = Transform {
                    translation: Vec3::new(6.568684, 0.76523083, -0.3192114),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_2" => {
                *transform = Transform {
                    translation: Vec3::new(6.568995, 0.63733333, -0.33108535),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_3" => {
                *transform = Transform {
                    translation: Vec3::new(6.569311, 0.50937116, -0.3429266),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_4" => {
                *transform = Transform {
                    translation: Vec3::new(6.5696273, 0.3813309, -0.35476214),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_5" => {
                *transform = Transform {
                    translation: Vec3::new(6.569941, 0.253206, -0.36660248),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_7" => {
                *transform = Transform {
                    translation: Vec3::new(6.570577, 0.109959275, -0.39026025),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_8" => {
                *transform = Transform {
                    translation: Vec3::new(6.5708838, 0.09394956, -0.40214062),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_9" => {
                *transform = Transform {
                    translation: Vec3::new(6.571191, 0.07786825, -0.41402486),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_tail" => {
                *transform = Transform {
                    translation: Vec3::new(6.571181, 0.40517983, -0.4158884),
                    rotation: Quat::from_array([-0.00436607, -0.005606618, 5.6503064e-5, 0.9999747]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            _ => (),
        }
    }
}

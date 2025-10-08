pub use crate::bevy_prelude::*;
use crate::{
    areas::Area,
    editor::editor,
    mouse::selection::{OutlineWhileSelected, Selected},
    render::ComesFromRootEntity,
};
use avian3d::prelude::RigidBody;
use bevy::{
    app::Propagate,
    feathers::{
        FeathersPlugins,
        controls::{ButtonProps, button},
        dark_theme::create_dark_theme,
        theme::{ThemeBackgroundColor, ThemedText, UiTheme},
        tokens,
    },
    input_focus::tab_navigation::TabGroup,
    scene::SceneInstance,
    ui_widgets::{Activate, observe},
};
use bevy_mod_outline::OutlineVolume;

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

// How do we overwrite files? I do not know...

pub fn patch(names: Query<(&Name, &mut Transform), Added<Name>>) {
    for (name, mut transform) in names {
        #[allow(clippy::match_same_arms)]
        #[allow(clippy::unreadable_literal)]
        #[allow(clippy::single_match)]
        match name.as_str() {
            "block_loading_cable_first_previous" => {
                *transform = Transform {
                    translation: Vec3::new(6.562654, 0.6097396, 0.5546866),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_6" => {
                *transform = Transform {
                    translation: Vec3::new(6.664108, 0.12583427, -0.074540034),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "cable" => {
                *transform = Transform {
                    translation: Vec3::new(6.573803, 0.5809511, 0.41309932),
                    rotation: Quat::from_array([-0.037924554, -0.37175363, 0.92743975, -0.0147136375]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_1" => {
                *transform = Transform {
                    translation: Vec3::new(6.580606, 0.53848577, 0.4485729),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_2" => {
                *transform = Transform {
                    translation: Vec3::new(6.59827, 0.4645929, 0.3426866),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_3" => {
                *transform = Transform {
                    translation: Vec3::new(6.6154547, 0.3862716, 0.23717783),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_4" => {
                *transform = Transform {
                    translation: Vec3::new(6.6321144, 0.30290413, 0.13225587),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_5" => {
                *transform = Transform {
                    translation: Vec3::new(6.6483083, 0.21520247, 0.028167536),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_7" => {
                *transform = Transform {
                    translation: Vec3::new(6.6678643, 0.13710801, -0.09703679),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_8" => {
                *transform = Transform {
                    translation: Vec3::new(6.6716595, 0.14813215, -0.11909562),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_cable_9" => {
                *transform = Transform {
                    translation: Vec3::new(6.675435, 0.1588448, -0.14077567),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            "block_loading_cable_tail" => {
                *transform = Transform {
                    translation: Vec3::new(6.664338, 0.4310713, -0.0033023967),
                    rotation: Quat::from_array([0.37189448, -0.050823685, 0.009587305, 0.9268331]),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                }
            }
            _ => (),
        }
    }
}

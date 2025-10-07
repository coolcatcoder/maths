pub use crate::bevy_prelude::*;
use crate::{
    editor::editor,
    mouse::selection::{OutlineWhileSelected, Selected},
};
use avian3d::prelude::RigidBody;
use bevy::{
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
        app.add_systems(Update, area_testing);
        editor(file!())(app);
    }
}

#[derive(Component)]
struct Area;

fn load(asset_server: Res<AssetServer>, mut commands: Commands) {
    let scene = asset_server.load("map/test_area.glb#Scene0");
    commands.spawn((SceneRoot(scene), Area));
}

fn area_testing(
    areas: Query<&Children, (Added<SceneInstance>, With<Area>)>,
    children: Query<&Children>,
    mut commands: Commands,
) {
    areas.iter().for_each(|scene_children| {
        if scene_children.len() != 1 {
            error!("There should only be one child for SceneInstance entities.");
            return;
        }
        let scene_child = scene_children.iter().next().else_return()?;
        let children = children.get(scene_child).else_return()?;

        children.iter().for_each(|child| {
            commands.entity(child).insert((
                Selected::<true>(false),
                OutlineWhileSelected::<true> {
                    colour: Color::srgb(1., 0., 0.),
                    width: 3.,
                },
            ));
        });
    });
}

pub fn patch(names: Query<(&Name, &mut Transform), Added<Name>>) {
    for (name, mut transform) in names {
        #[allow(clippy::match_same_arms)]
        #[allow(clippy::unreadable_literal)]
        #[allow(clippy::single_match)]
        match name.as_str() {
            "battery.001" => {
                *transform = Transform {
                    translation: Vec3::new(-3.0, 0.5, 0.0),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::ONE,
                }
            }
            "battery" => {
                *transform = Transform {
                    translation: Vec3::new(1.9353261, 0.5, 0.0),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::ONE,
                }
            }
            "Plane" => {
                *transform = Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    rotation: Quat::from_array([0.0, 0.0, 0.0, 1.0]),
                    scale: Vec3::ONE,
                }
            }
            _ => (),
        }
    }
}

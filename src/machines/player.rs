pub use crate::bevy_prelude::*;
use crate::{areas::AreaLoadedEntity, creatures::{BasicHorizontalControl, LandHandling, LandHandlingState}, mind_control::Controlled, render::CameraFollow};
use avian3d::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_observer(load);
}

fn load(
    on: On<AreaLoadedEntity>,
    name: Query<&Name>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let name = name.get(on.loaded).else_error("Couldn't get loaded entity's name.")?;
    name.starts_with("player").else_return()?;

    let scene = asset_server.load("machines/player.glb#Scene0");

    commands.entity(on.loaded).insert((
        CameraFollow,
        Controlled,
        SceneRoot(scene),
        RigidBody::Dynamic,
        Collider::cuboid(0.3, 1., 0.3),
        // Friction {
        //     dynamic_coefficient: 0.25,
        //     static_coefficient: 1.,
        //     ..default()
        // },
        LockedAxes::ROTATION_LOCKED,
        LandHandling {
            state: LandHandlingState::InControl,
            gain_control: 3.,
            lose_control: 13.,
            slowing: 0.05,
        },
        BasicHorizontalControl { speed: 2. },
    ));
}

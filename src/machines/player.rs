pub use crate::bevy_prelude::*;
use crate::{creatures::{BasicHorizontalControl, LandHandling, LandHandlingState}, mind_control::Controlled, render::CameraFollow};
use avian3d::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, load);
}

fn load(
    names: Query<(Entity, &Name), Added<Name>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for (entity, name) in names {
        name.starts_with("player").else_return()?;

        let scene = asset_server.load("machines/player.glb#Scene0");

        commands.entity(entity).insert((
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
}

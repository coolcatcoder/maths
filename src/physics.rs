use crate::{error_handling::ToUnwrapResult, plugin_module};
use avian3d::prelude::*;
use bevy::prelude::*;

plugin_module!(pub scene);

pub mod common_properties;

const DEVELOP_OVERRIDE: bool = false;

pub fn plugin(app: &mut App) {
    if DEVELOP_OVERRIDE || crate::DEVELOP {
        app.add_plugins(PhysicsDebugPlugin);
    }
    app.add_plugins((PhysicsPlugins::default(), plugins_in_modules))
        .add_systems(Update, load)
        .add_systems(Startup, pause);
}

fn pause(mut time: ResMut<Time<Physics>>) {
    time.pause();
}

#[derive(PhysicsLayer, Default)]
pub enum CollisionLayer {
    #[default]
    Default,
    Cable,
}

fn load(extras: Query<(&GltfExtras, Entity), Added<GltfExtras>>, mut commands: Commands) {
    extras.iter().for_each(|(extras, entity)| {
        let extras_json = serde_json::from_str::<serde_json::Value>(&extras.value)
            .else_error("Gltf extras was not json.")?;
        let collision = extras_json
            .get("collision")
            .else_return()?
            .as_bool()
            .else_return()?;

        let rigid_body = if collision {
            RigidBody::Dynamic
        } else {
            RigidBody::Static
        };

        commands
            .entity(entity)
            .insert((rigid_body, Collider::cuboid(1., 1., 1.)));
    });
}

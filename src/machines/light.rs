use std::num::NonZero;

use avian3d::prelude::{Collider, RigidBody, Sensor};
use bevy::{app::Propagate, prelude::*};

use crate::{
    machines::{
        outlet::{OutletSensor, OutletSensorEntity},
        power::{Powered, TakesPower},
    },
    render::SceneNotShadowCaster,
    sync::{SyncRotation, SyncTranslation},
};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (light, load));
}

#[derive(Component)]
#[require(Transform, RigidBody = RigidBody::Static)]
pub struct LightBulb;

pub fn load(
    names: Query<(Entity, &Name), Added<Name>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for (root_entity, name) in names {
        if name.starts_with("light bulb") {
            let scene = asset_server.load("machines/light.glb#Scene0");

            let outlet_sensor_entity = commands
                .spawn((
                    OutletSensor {
                        root: root_entity,
                        rest_length: 1.,
                        plugs: vec![],
                        max_plugs: NonZero::<u8>::new(1),
                    },
                    Collider::cuboid(2., 2., 2.),
                    SyncTranslation {
                        target: root_entity,
                        offset: Vec3::ZERO,
                    },
                    SyncRotation {
                        target: root_entity,
                    },
                ))
                .id();

            commands.entity(root_entity).insert((
                Propagate(SceneNotShadowCaster),
                SceneRoot(scene),
                Collider::cuboid(1., 1., 1.),
                PointLight {
                    intensity: 100_000.0,
                    range: 15.,
                    color: Color::WHITE,
                    shadows_enabled: true,
                    ..default()
                },
                TakesPower(1),
                Powered(false),
                OutletSensorEntity(outlet_sensor_entity),
                LightBulb,
            ));
        }
    }
}

fn light(mut light: Query<(&Powered, &mut Visibility), With<LightBulb>>) {
    light.iter_mut().for_each(|(powered, mut visibility)| {
        if powered.0 {
            *visibility = Visibility::Inherited;
        } else {
            *visibility = Visibility::Hidden;
        }
    });
}

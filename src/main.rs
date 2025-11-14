#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]

use bevy::{ecs::system::SystemParam, prelude::*};
use std::ops::{Deref, DerefMut};

mod l_systems;
mod linear_transformations_2d;

use l_systems as run;

mod prelude {
    pub(crate) use crate::{Draw, Vector};
    pub use bevy::{color::palettes::css::*, prelude::*};

    pub fn start() {}
    pub fn update() {}

    pub fn start_2d(mut commands: Commands) {
        commands.spawn((
            Camera2d,
            Projection::Orthographic(OrthographicProjection {
                scale: 0.01,
                ..OrthographicProjection::default_2d()
            }),
        ));
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, run::plugin))
        .add_systems(Startup, run::start)
        .add_systems(Update, run::update)
        .run();
}

#[derive(SystemParam)]
struct Draw<'w, 's>(Gizmos<'w, 's>);

impl<'w, 's> Deref for Draw<'w, 's> {
    type Target = Gizmos<'w, 's>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'w, 's> DerefMut for Draw<'w, 's> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, 'w, 's> Draw<'w, 's> {
    fn line(&mut self, start: [f32; 2], end: [f32; 2], colour: impl Into<Color>) {
        self.line_2d(start.into(), end.into(), colour);
    }

    fn vector(
        &'a mut self,
        vector: impl Into<Vec2> + Copy,
        colour: impl Into<Color>,
    ) -> Vector<'a, 'w, 's> {
        self.arrow_2d(Vec2::ZERO, vector.into(), colour);
        Vector {
            vector: vector.into(),
            gizmos: self,
        }
    }
}

struct Vector<'a, 'w, 's> {
    vector: Vec2,
    gizmos: &'a mut Gizmos<'w, 's>,
}

impl Vector<'_, '_, '_> {
    fn show_numbers(&mut self) {
        // TO DO: Once we get text gizmos, replace this with them.
        info!("{}", self.vector);
    }
}

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]

use bevy::{ecs::system::SystemParam, prelude::*};
use prelude::*;
use std::ops::{Deref, DerefMut};

mod l_systems;
mod linear_transformations_2d;
//mod fractal_tree;
//mod plant;
mod determinant;
mod entity_with;
mod linear_transformations_3d;
mod my_plant;
mod secret_santa;
mod solving_linear_equations;
mod dot_product;

use dot_product as run;

mod prelude {
    pub(crate) use crate::{Draw, Draw2d, Vector};
    pub use bevy::{color::palettes::css::*, prelude::*};

    #[allow(non_upper_case_globals)]
    pub const Vec3: fn(f32, f32, f32) -> Vec3 = |x, y, z| Vec3::new(x, y, z);
    #[allow(non_upper_case_globals)]
    pub const Vec2: fn(f32, f32) -> Vec2 = |x, y| Vec2::new(x, y);

    pub fn plugin(_: &mut App) {}
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

    pub fn start_3d(mut commands: Commands) {
        commands.spawn((
            Camera3d::default(),
            Transform::from_translation(Vec3::splat(100.)).looking_at(Vec3::ZERO, Dir3::Y),
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

#[derive(SystemParam)]
struct Draw2d<'w, 's>(Gizmos<'w, 's>);

impl<'w, 's> Deref for Draw2d<'w, 's> {
    type Target = Gizmos<'w, 's>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'w, 's> DerefMut for Draw2d<'w, 's> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, 'w, 's> Draw2d<'w, 's> {
    fn line(&mut self, start: Vec2, end: Vec2, colour: impl Into<Color>) {
        self.line_2d(start, end, colour);
    }

    fn vector(&mut self, vector: Vec2, colour: impl Into<Color>) {
        self.arrow_2d(Vec2::ZERO, vector, colour);
    }

    fn rectangle(
        &'a mut self,
        corner_1: Vec2,
        corner_2: Vec2,
        colour: impl Into<Color>,
    ) -> Rectangle<'a, 'w, 's> {
        Rectangle {
            corner_1,
            corner_2: Vec2(corner_1.x, corner_2.y),
            corner_3: corner_2,
            corner_4: Vec2(corner_2.x, corner_1.y),
            colour: colour.into(),
            draw: self,
        }
    }
}

struct Rectangle<'a, 'w, 's> {
    corner_1: Vec2,
    corner_2: Vec2,
    corner_3: Vec2,
    corner_4: Vec2,
    colour: Color,
    draw: &'a mut Draw2d<'w, 's>,
}
impl Rectangle<'_, '_, '_> {
    fn linear_transformation(&mut self, transformation: Mat2) -> &mut Self {
        self.corner_1 = transformation * self.corner_1;
        self.corner_2 = transformation * self.corner_2;
        self.corner_3 = transformation * self.corner_3;
        self.corner_4 = transformation * self.corner_4;
        self
    }
}
impl<'a, 'w, 's> Drop for Rectangle<'a, 'w, 's> {
    fn drop(&mut self) {
        self.draw.line(self.corner_1, self.corner_2, self.colour);
        self.draw.line(self.corner_2, self.corner_3, self.colour);
        self.draw.line(self.corner_3, self.corner_4, self.colour);
        self.draw.line(self.corner_4, self.corner_1, self.colour);
    }
}

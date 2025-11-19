use std::ops::Deref;

use bevy::ecs::{component::Mutable, query::QueryData};

pub use crate::prelude::*;

pub trait Lsystem: Copy + Component<Mutability = Mutable> {
    fn plugin(app: &mut App) {
        app.add_systems(
            Update,
            (draw_symbols::<Self>, update_symbols::<Self>).chain(),
        );
    }
    fn update(&self, update: impl FnOnce(&[Self]));
    fn draw(
        &self,
        translation: &mut Vec2,
        rotation: &mut f32,
        saved_transforms: &mut Vec<(Vec2, f32)>,
        draw: &mut Draw,
    );
}

#[derive(Component)]
pub struct Next(pub Option<Entity>);
#[derive(Component)]
pub struct Start;

fn update_symbols<T: Lsystem>(
    mut symbols: Query<(&mut T, &mut Next)>,
    mut iteration: Local<u8>,
    mut commands: Commands,
) {
    if *iteration == 7 {
        return;
    }
    *iteration += 1;

    symbols.iter_mut().for_each(|(mut symbol, mut next)| {
        let cloned_symbol = *symbol;

        let closure = |new_symbols: &[T]| {
            create_chain::<T>(
                (&mut symbol, &mut next),
                new_symbols.iter().copied(),
                &mut commands,
            );
        };
        cloned_symbol.update(closure);
    });
}

fn create_chain<T: Lsystem>(
    replace: (&mut T, &mut Next),
    mut symbols_iterator: impl std::iter::DoubleEndedIterator<Item = T>,
    commands: &mut Commands,
) {
    let Some(start) = symbols_iterator.next() else {
        error!(
            "Why did you enter an empty iterator? We don't account for this possibility at all!"
        );
        return;
    };

    let mut next = replace.1.0.take();
    for symbol in symbols_iterator.rev() {
        next = Some(commands.spawn((symbol, Next(next))).id());
    }

    replace.1.0 = next;
    *replace.0 = start;
}

fn draw_symbols<T: Lsystem>(
    mut draw: Draw,
    start: Query<(&T, &Next), With<Start>>,
    symbols: Query<(&T, &Next)>,
) {
    let mut translation = Vec2::new(0., -1.);
    let mut saved = vec![];
    let mut rotation = 0.;

    start.iter().for_each(|(symbol, next)| {
        symbol.draw(&mut translation, &mut rotation, &mut saved, &mut draw);

        let mut maybe_next = next.0;

        while let Some(next) = maybe_next {
            let Ok((symbol, next)) = symbols.get(next) else {
                error!("Failed to get next.");
                break;
            };
            symbol.draw(&mut translation, &mut rotation, &mut saved, &mut draw);

            maybe_next = next.0
        }
    });

    //info!("{}", debug_string);

    // draw.grid_2d(
    //     Isometry2d::IDENTITY,
    //     UVec2::new(100, 100),
    //     Vec2::splat(1.),
    //     GRAY,
    // );
}

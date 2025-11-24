pub use crate::prelude::*;

pub use start_2d as start;

pub fn update(mut draw: Draw2d) {
    draw.grid_2d(
        Isometry2d::IDENTITY,
        UVec2::new(100, 100),
        Vec2::splat(1.),
        GRAY,
    );

    let transform = Mat2::IDENTITY;

    for i in -5..=5 {
        draw.line(
            transform * Vec2::new(i as f32, -50.),
            transform * Vec2::new(i as f32, 50.),
            BLUE,
        );
        draw.line(
            transform * Vec2::new(-50., i as f32),
            transform * Vec2::new(50., i as f32),
            BLUE,
        );
    }

    // axis
    draw.line(Vec2(0., -50.), Vec2(0., 50.), RED);
    draw.line(Vec2(-50., 0.), Vec2(50., 0.), GREEN);

    // i hat
    draw.line(Vec2::ZERO, transform * Vec2::new(1., 0.), LIME);
    // j hat
    draw.line(Vec2::ZERO, transform * Vec2::new(0., 1.), MAGENTA);

    // 2x + 5y + 3z = -3
    // 4x + 0y + 8z = 0
    // 1x + 3y + 0z = 2
    //let a = Mat3::from_cols(Vec3(2., 4., 1.), Vec3(5., 0., 3.), Vec3(3., 8., 0.));
    //let v = Vec3(-3., 0., 2.);
    //let x = a.inverse() * v;
}

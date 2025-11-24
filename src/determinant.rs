pub use crate::prelude::*;

pub use start_2d as start;

pub fn update(mut draw: Draw2d) {
    draw.grid_2d(
        Isometry2d::IDENTITY,
        UVec2::new(100, 100),
        Vec2::splat(1.),
        GRAY,
    );

    let transform = Mat2::from_cols(Vec2(1., 1.), Vec2(2., -1.));

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

    draw.rectangle(Vec2::ZERO, Vec2::ONE, YELLOW).linear_transformation(transform);

    let determinant = transform.x_axis.x * transform.y_axis.y - transform.x_axis.y * transform.y_axis.x;
    info!("{determinant}");
}

pub use crate::prelude::*;

pub use start_2d as start;

pub fn update(mut draw: Draw) {
    draw.grid_2d(
        Isometry2d::IDENTITY,
        UVec2::new(100, 100),
        Vec2::splat(1.),
        GRAY,
    );

    // // let transform = Mat2::from_cols(Vec2::new(1., 1.), Vec2::new(-1., 0.));
    // let rotation = Mat2::from_cols(Vec2::new(0., 1.), Vec2::new(-1., 0.));
    // let shear = Mat2::from_cols(Vec2::new(1., 0.), Vec2::new(1., 1.));
    // // Read right to left.
    // // This will rotate, then shear.
    // let transform = shear * rotation;

    let m1 = Mat2::from_cols(Vec2::new(1., 1.), Vec2::new(-2., 0.));
    let m2 = Mat2::from_cols(Vec2::new(0., 1.), Vec2::new(2., 0.));

    // m1 then m2
    let correct_transform = m2 * m1;
    let i_hat = (m1.x_axis.x * m2.x_axis) + (m1.x_axis.y * m2.y_axis);
    let j_hat = (m1.y_axis.x * m2.x_axis) + (m1.y_axis.y * m2.y_axis);
    // Composition matrix!
    let transform = Mat2::from_cols(i_hat, j_hat);

    info!("correct: {correct_transform}");
    info!("mine: {transform}");

    for i in -5..=5 {
        draw.line_2d(
            transform * Vec2::new(i as f32, -50.),
            transform * Vec2::new(i as f32, 50.),
            BLUE,
        );
        draw.line_2d(
            transform * Vec2::new(-50., i as f32),
            transform * Vec2::new(50., i as f32),
            BLUE,
        );
    }

    // axis
    draw.line([0., -50.], [0., 50.], RED);
    draw.line([-50., 0.], [50., 0.], GREEN);

    // i hat
    draw.vector(transform * Vec2::new(1., 0.), LIME);
    // j hat
    draw.vector(transform * Vec2::new(0., 1.), MAGENTA);

    draw.vector(transform * Vec2::new(2., 3.), YELLOW);
}

fn linear_transformation_hand_written(
    vector: [f32; 2],
    transformed_i_hat: [f32; 2],
    transformed_j_hat: [f32; 2],
) -> [f32; 2] {
    let vector = Vec2::from_array(vector);
    let transformed_i_hat = Vec2::from_array(transformed_i_hat);
    let transformed_j_hat = Vec2::from_array(transformed_j_hat);
    let transformed = (vector.x * transformed_i_hat) + (vector.y * transformed_j_hat);
    transformed.into()
}

fn linear_transformation(
    vector: impl Into<Vec2>,
    linear_transformation: impl Into<Mat2>,
) -> [f32; 2] {
    (linear_transformation.into() * vector.into()).into()
}

fn multiply_mat3_by_vec3(mat3: Mat3, vec3: Vec3) {
    let output = vec3.x * mat3.x_axis + vec3.y * mat3.y_axis + vec3.z * mat3.z_axis;
    let check = mat3 * vec3;
    assert_eq!(output, check);
}

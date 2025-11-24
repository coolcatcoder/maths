pub use crate::prelude::*;

pub use start_3d as start;

pub fn update(mut draw: Gizmos) {
    // draw.grid_3d(
    //     Isometry3d::IDENTITY,
    //     UVec3::splat(5),
    //     Vec3::splat(1.),
    //     GRAY,
    // );

    let first_transformation =
        Mat3::from_cols(Vec3(0., 3., 6.), Vec3(1., 4., 7.), Vec3(2., 5., 8.));
    let second_transformation =
        Mat3::from_cols(Vec3(0., 5., 1.), Vec3(-2., 1., 4.), Vec3(2., 5., -1.));

    let result = second_transformation * first_transformation;
    let my_result = Mat3::from_cols(
        second_transformation * first_transformation.x_axis,
        second_transformation * first_transformation.y_axis,
        second_transformation * first_transformation.z_axis,
    );
    let my_result_expanded = Mat3::from_cols(
        (second_transformation.x_axis * first_transformation.x_axis.x)
            + (second_transformation.y_axis * first_transformation.x_axis.y)
            + (second_transformation.z_axis * first_transformation.x_axis.z),
        (second_transformation.x_axis * first_transformation.y_axis.x)
            + (second_transformation.y_axis * first_transformation.y_axis.y)
            + (second_transformation.z_axis * first_transformation.y_axis.z),
        (second_transformation.x_axis * first_transformation.z_axis.x)
            + (second_transformation.y_axis * first_transformation.z_axis.y)
            + (second_transformation.z_axis * first_transformation.z_axis.z),
    );
    let my_result_fully_expanded = Mat3::from_cols(
        Vec3(
            second_transformation.x_axis.x * first_transformation.x_axis.x,
            second_transformation.x_axis.y * first_transformation.x_axis.x,
            second_transformation.x_axis.z * first_transformation.x_axis.x,
        ) + Vec3(
            second_transformation.y_axis.x * first_transformation.x_axis.y,
            second_transformation.y_axis.y * first_transformation.x_axis.y,
            second_transformation.y_axis.z * first_transformation.x_axis.y,
        ) + Vec3(
            second_transformation.z_axis.x * first_transformation.x_axis.z,
            second_transformation.z_axis.y * first_transformation.x_axis.z,
            second_transformation.z_axis.z * first_transformation.x_axis.z,
        ),
        Vec3(
            second_transformation.x_axis.x * first_transformation.y_axis.x,
            second_transformation.x_axis.y * first_transformation.y_axis.x,
            second_transformation.x_axis.z * first_transformation.y_axis.x,
        ) + Vec3(
            second_transformation.y_axis.x * first_transformation.y_axis.y,
            second_transformation.y_axis.y * first_transformation.y_axis.y,
            second_transformation.y_axis.z * first_transformation.y_axis.y,
        ) + Vec3(
            second_transformation.z_axis.x * first_transformation.y_axis.z,
            second_transformation.z_axis.y * first_transformation.y_axis.z,
            second_transformation.z_axis.z * first_transformation.y_axis.z,
        ),
        Vec3(
            second_transformation.x_axis.x * first_transformation.z_axis.x,
            second_transformation.x_axis.y * first_transformation.z_axis.x,
            second_transformation.x_axis.z * first_transformation.z_axis.x,
        ) + Vec3(
            second_transformation.y_axis.x * first_transformation.z_axis.y,
            second_transformation.y_axis.y * first_transformation.z_axis.y,
            second_transformation.y_axis.z * first_transformation.z_axis.y,
        ) + Vec3(
            second_transformation.z_axis.x * first_transformation.z_axis.z,
            second_transformation.z_axis.y * first_transformation.z_axis.z,
            second_transformation.z_axis.z * first_transformation.z_axis.z,
        ),
    );

    info!("result: {result}");
    info!("my_result: {my_result}");
    info!("my_result_expanded: {my_result_expanded}");
    info!("my_result_fully_expanded: {my_result_fully_expanded}");

    // axis
    draw.line(Vec3(-50., 0., 0.), Vec3(50., 0., 0.), RED);
    draw.line(Vec3(0., -50., 0.), Vec3(0., 50., 0.), GREEN);
    draw.line(Vec3(0., 0., -50.), Vec3(0., 0., 50.), BLUE);

    // i hat
    draw.arrow(Vec3::ZERO, my_result_fully_expanded * Vec3::X, LIME);
    // j hat
    draw.arrow(Vec3::ZERO, my_result_fully_expanded * Vec3::Y, MAGENTA);
    // k hat
    draw.arrow(Vec3::ZERO, my_result_fully_expanded * Vec3::Z, YELLOW);
}

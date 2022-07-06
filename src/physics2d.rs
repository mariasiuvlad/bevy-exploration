use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct Physics2dPlugin;

impl Plugin for Physics2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(setup_graphics)
            .add_startup_system(setup_physics);
        // .add_system(print_ball_altitude);
    }
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn()
        .insert(Collider::cuboid(500.0, 50.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    commands
        .spawn()
        .insert(Collider::cuboid(50.0, 100.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(20.0))
        .insert(Restitution::coefficient(0.7))
        .insert(Velocity {
            linvel: Vec2::new(50.0, 50.0),
            angvel: 0.2,
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}

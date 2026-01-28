use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ground)
            .add_systems(Startup, setup_obstacles)
            .add_systems(Startup, setup_lighting);
    }
}

fn setup_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane
    let mesh = meshes.add(Cuboid::new(100.0, 1.0, 100.0));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.6, 0.2),
        ..default()
    });

    commands
        .spawn((
            Name::new("Ground"),
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_xyz(0.0, -2.0, 0.0),
            GlobalTransform::default(),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(50.0, 0.5, 50.0));

    // Walls
    let wall_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.4, 0.4),
        ..default()
    });

    let walls = vec![
        (Vec3::new(0.0, 5.0, -50.0), Vec3::new(100.0, 10.0, 1.0)), // Back wall
        (Vec3::new(0.0, 5.0, 50.0), Vec3::new(100.0, 10.0, 1.0)),  // Front wall
        (Vec3::new(-50.0, 5.0, 0.0), Vec3::new(1.0, 10.0, 100.0)), // Left wall
        (Vec3::new(50.0, 5.0, 0.0), Vec3::new(1.0, 10.0, 100.0)),  // Right wall
    ];

    for (position, size) in walls {
        let mesh = meshes.add(Cuboid::new(size.x, size.y, size.z));
        commands
            .spawn((
                Name::new("Wall"),
                Mesh3d(mesh),
                MeshMaterial3d(wall_material.clone()),
                Transform::from_translation(position),
                GlobalTransform::default(),
            ))
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(size.x / 2.0, size.y / 2.0, size.z / 2.0));
    }
}

fn setup_obstacles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let obstacle_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.3, 0.1),
        ..default()
    });

    // Create some cube obstacles
    for i in 0..6 {
        let x = (i as f32 % 3.0 - 1.0) * 15.0;
        let z = ((i as f32 / 3.0).floor() - 0.5) * 15.0;

        let mesh = meshes.add(Cuboid::new(4.0, 4.0, 4.0));
        commands
            .spawn((
                Name::new(format!("Obstacle {}", i)),
                Mesh3d(mesh),
                MeshMaterial3d(obstacle_material.clone()),
                Transform::from_xyz(x, 2.0, z),
                GlobalTransform::default(),
            ))
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(2.0, 2.0, 2.0));
    }

    // Create ramps
    let ramp_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.4, 0.3),
        ..default()
    });

    let mesh = meshes.add(Cuboid::new(20.0, 2.0, 10.0));
    commands
        .spawn((
            Name::new("Ramp"),
            Mesh3d(mesh),
            MeshMaterial3d(ramp_material),
            Transform::from_xyz(-30.0, 0.5, 0.0).with_rotation(Quat::from_rotation_z(0.3)),
            GlobalTransform::default(),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(10.0, 1.0, 5.0));
}

fn setup_lighting(mut commands: Commands) {
    // Sunlight
    commands.spawn((
        Name::new("Sun"),
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            std::f32::consts::PI * -0.25,
            std::f32::consts::PI * 0.25,
            0.0,
        )),
        GlobalTransform::default(),
    ));

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 500.0,
    });
}

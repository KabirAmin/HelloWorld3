use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Slime {
    pub mass: f32,
    pub merge_radius: f32,
    pub split_threshold: f32,
    pub color: Color,
}

#[derive(Component)]
pub struct SlimeVelocity {
    pub current: Vec3,
    pub target: Vec3,
}

pub struct SlimePlugin;

impl Plugin for SlimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_slime)
            .add_systems(Startup, spawn_enemy_slimes)
            .add_systems(Update, slime_movement)
            .add_systems(Update, slime_merge)
            .add_systems(Update, slime_split)
            .add_systems(Update, update_slime_scale);
    }
}

fn spawn_player_slime(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Sphere::new(1.0).mesh().uv(32, 32));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.8, 0.3),
        ..default()
    });

    commands
        .spawn((
            Name::new("Player Slime"),
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_xyz(0.0, 5.0, 0.0),
            GlobalTransform::default(),
            RigidBody::Dynamic,
            Collider::ball(1.0),
            Velocity {
                linvel: Vec3::ZERO,
                angvel: Vec3::ZERO,
            },
            Damping {
                linear_damping: 0.8,
                angular_damping: 0.8,
            },
            Restitution {
                coefficient: 0.3,
                combine_rule: CoefficientCombineRule::Min,
            },
            Slime {
                mass: 1.0,
                merge_radius: 2.5,
                split_threshold: 3.0,
                color: Color::srgb(0.3, 0.8, 0.3),
            },
            SlimeVelocity {
                current: Vec3::ZERO,
                target: Vec3::ZERO,
            },
            PlayerSlime,
        ))
        .insert(ColliderMassProperties::Mass(1.0));
}

fn spawn_enemy_slimes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let colors = [
        Color::srgb(0.8, 0.3, 0.3),
        Color::srgb(0.3, 0.3, 0.8),
        Color::srgb(0.8, 0.8, 0.3),
        Color::srgb(0.8, 0.3, 0.8),
    ];

    for i in 0..8 {
        let x = (i as f32 % 4.0 - 1.5) * 8.0;
        let z = ((i as f32 / 4.0).floor() - 0.5) * 8.0;
        let color = colors[i % colors.len()];

        let mesh = meshes.add(Sphere::new(0.7).mesh().uv(32, 32));
        let material = materials.add(StandardMaterial {
            base_color: color,
            ..default()
        });

        commands
            .spawn((
                Name::new(format!("Enemy Slime {}", i)),
                Mesh3d(mesh),
                MeshMaterial3d(material),
                Transform::from_xyz(x, 3.0, z),
                GlobalTransform::default(),
                RigidBody::Dynamic,
                Collider::ball(0.7),
                Velocity {
                    linvel: Vec3::ZERO,
                    angvel: Vec3::ZERO,
                },
                Damping {
                    linear_damping: 0.7,
                    angular_damping: 0.7,
                },
                Restitution {
                    coefficient: 0.2,
                    combine_rule: CoefficientCombineRule::Min,
                },
                Slime {
                    mass: 0.5,
                    merge_radius: 2.0,
                    split_threshold: 2.5,
                    color,
                },
                SlimeVelocity {
                    current: Vec3::ZERO,
                    target: Vec3::ZERO,
                },
                EnemySlime {
                    wander_timer: 0.0,
                    wander_direction: Vec3::new(
                        (i as f32 * 12.9898).sin(),
                        0.0,
                        (i as f32 * 78.233).sin(),
                    )
                    .normalize(),
                },
            ))
            .insert(ColliderMassProperties::Mass(0.5));
    }
}

#[derive(Component)]
pub struct PlayerSlime;

#[derive(Component)]
pub struct EnemySlime {
    pub wander_timer: f32,
    pub wander_direction: Vec3,
}

fn slime_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &Transform), (With<PlayerSlime>, Without<EnemySlime>)>,
    mut enemy_query: Query<(&mut Velocity, &Transform, &mut EnemySlime), Without<PlayerSlime>>,
    time: Res<Time>,
) {
    const SPEED: f32 = 15.0;
    const ENEMY_SPEED: f32 = 8.0;

    // Player movement
    if let Ok((mut vel, transform)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard.pressed(KeyCode::KeyW) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard.pressed(KeyCode::KeyS) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard.pressed(KeyCode::KeyA) {
            direction += Vec3::new(0.0, 0.0, -1.0);
        }
        if keyboard.pressed(KeyCode::KeyD) {
            direction += Vec3::new(0.0, 0.0, 1.0);
        }
        if keyboard.pressed(KeyCode::Space) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
            vel.linvel = direction * SPEED;
        }
    }

    // Enemy AI
    for (mut vel, transform, mut enemy) in enemy_query.iter_mut() {
        enemy.wander_timer += time.delta_secs();

        if enemy.wander_timer > 2.0 {
            enemy.wander_timer = 0.0;
            let angle = (transform.translation.x * 13.0 + transform.translation.z * 17.0).sin();
            enemy.wander_direction =
                Vec3::new(angle.cos(), 0.0, angle.sin()).normalize();
        }

        vel.linvel.x = enemy.wander_direction.x * ENEMY_SPEED;
        vel.linvel.z = enemy.wander_direction.z * ENEMY_SPEED;
    }
}

fn slime_merge(
    mut commands: Commands,
    slime_query: Query<(Entity, &Transform, &Slime, &Velocity)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut slimes_to_remove = Vec::new();
    let mut new_slimes = Vec::new();

    let slimes: Vec<_> = slime_query.iter().collect();

    for i in 0..slimes.len() {
        for j in (i + 1)..slimes.len() {
            let (entity_a, transform_a, slime_a, vel_a) = slimes[i];
            let (entity_b, transform_b, slime_b, vel_b) = slimes[j];

            let distance = transform_a.translation.distance(transform_b.translation);

            if distance < slime_a.merge_radius && distance > 0.1 {
                // Merge slimes
                slimes_to_remove.push(entity_a);
                slimes_to_remove.push(entity_b);

                let new_mass = slime_a.mass + slime_b.mass;
                let new_color = Color::srgb(
                    (slime_a.color.r() + slime_b.color.r()) * 0.5,
                    (slime_a.color.g() + slime_b.color.g()) * 0.5,
                    (slime_a.color.b() + slime_b.color.b()) * 0.5,
                );

                let new_position = (transform_a.translation + transform_b.translation) * 0.5;
                let new_velocity =
                    (vel_a.linvel * slime_a.mass + vel_b.linvel * slime_b.mass) / new_mass;

                let new_scale = (new_mass / slime_a.mass).powf(0.333);

                new_slimes.push((new_position, new_velocity, new_mass, new_color, new_scale));
            }
        }
    }

    for entity in slimes_to_remove {
        commands.entity(entity).despawn();
    }

    for (position, velocity, mass, color, scale) in new_slimes {
        let mesh = meshes.add(Sphere::new(1.0).mesh().uv(32, 32));
        let material = materials.add(StandardMaterial {
            base_color: color,
            ..default()
        });

        commands.spawn((
            Name::new("Merged Slime"),
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(position).with_scale(Vec3::splat(scale)),
            GlobalTransform::default(),
            RigidBody::Dynamic,
            Collider::ball(scale),
            Velocity {
                linvel: velocity,
                angvel: Vec3::ZERO,
            },
            Damping {
                linear_damping: 0.8,
                angular_damping: 0.8,
            },
            Slime {
                mass,
                merge_radius: 2.5,
                split_threshold: 3.0,
                color,
            },
            SlimeVelocity {
                current: velocity,
                target: Vec3::ZERO,
            },
        ));
    }
}

fn slime_split(
    mut commands: Commands,
    mut slime_query: Query<(Entity, &Transform, &Slime, &Velocity)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut to_split = Vec::new();

    for (entity, transform, slime, velocity) in slime_query.iter_mut() {
        if slime.mass > 2.5 {
            to_split.push((entity, *transform, slime.clone(), *velocity));
        }
    }

    for (entity, transform, slime, velocity) in to_split {
        commands.entity(entity).despawn();

        let half_mass = slime.mass * 0.5;
        let half_scale = (0.5_f32).powf(0.333);
        let offset = Vec3::new(0.5, 0.0, 0.5);

        for i in 0..2 {
            let position = transform.translation + if i == 0 { offset } else { -offset };
            let mesh = meshes.add(Sphere::new(1.0).mesh().uv(32, 32));
            let material = materials.add(StandardMaterial {
                base_color: slime.color,
                ..default()
            });

            commands.spawn((
                Name::new("Split Slime"),
                Mesh3d(mesh),
                MeshMaterial3d(material),
                Transform::from_translation(position).with_scale(Vec3::splat(half_scale)),
                GlobalTransform::default(),
                RigidBody::Dynamic,
                Collider::ball(half_scale),
                Velocity {
                    linvel: velocity.linvel + Vec3::new(if i == 0 { 3.0 } else { -3.0 }, 2.0, 0.0),
                    angvel: Vec3::ZERO,
                },
                Damping {
                    linear_damping: 0.8,
                    angular_damping: 0.8,
                },
                Slime {
                    mass: half_mass,
                    merge_radius: 2.5,
                    split_threshold: 2.5,
                    color: slime.color,
                },
                SlimeVelocity {
                    current: velocity.linvel,
                    target: Vec3::ZERO,
                },
            ));
        }
    }
}

fn update_slime_scale(mut slime_query: Query<(&Slime, &mut Transform)>) {
    for (slime, mut transform) in slime_query.iter_mut() {
        let scale = (slime.mass).powf(0.333);
        transform.scale = Vec3::splat(scale);
    }
}

impl Clone for Slime {
    fn clone(&self) -> Self {
        Slime {
            mass: self.mass,
            merge_radius: self.merge_radius,
            split_threshold: self.split_threshold,
            color: self.color,
        }
    }
}

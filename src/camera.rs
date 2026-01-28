use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, follow_player_camera);
    }
}

#[derive(Component)]
pub struct MainCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 15.0, 20.0).looking_at(Vec3::new(0.0, 5.0, 0.0), Vec3::Y),
        MainCamera,
    ));
}

fn follow_player_camera(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    player_query: Query<&Transform, (With<crate::slime::PlayerSlime>, Without<MainCamera>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            let target_position = player_transform.translation + Vec3::new(0.0, 15.0, 20.0);
            camera_transform.translation = camera_transform.translation.lerp(target_position, 0.05);
            camera_transform.look_at(player_transform.translation + Vec3::Y * 2.0, Vec3::Y);
        }
    }
}

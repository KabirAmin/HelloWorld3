use bevy::prelude::*;

pub struct GameStatePlugin;

#[derive(Resource, Default)]
pub struct GameStats {
    pub player_mass: f32,
    pub enemy_count: u32,
    pub score: u32,
}

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameStats>()
            .add_systems(Update, update_game_stats)
            .add_systems(Update, display_ui);
    }
}

fn update_game_stats(
    mut stats: ResMut<GameStats>,
    slime_query: Query<&crate::slime::Slime>,
    player_query: Query<&crate::slime::PlayerSlime>,
) {
    let player_slime = slime_query.iter().find(|_| player_query.iter().next().is_some());
    if let Some(slime) = player_slime {
        stats.player_mass = slime.mass;
    }

    let enemy_count = slime_query.iter().count() as u32 - 1; // Subtract player slime
    stats.enemy_count = enemy_count;
    stats.score = (stats.player_mass * 100.0) as u32;
}

fn display_ui(stats: Res<GameStats>, mut gizmos: Gizmos) {
    // UI is typically rendered with bevy_ui, but for simplicity we'll just display in console periodically
    // In a full implementation, this would use bevy_ui for on-screen UI
}

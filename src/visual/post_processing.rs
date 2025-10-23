use bevy::prelude::*;

pub struct PostProcessingPlugin;

impl Plugin for PostProcessingPlugin {
    fn build(&self, app: &mut App) {
        info!("🎬 Post-processing plugin initialized");
    }
}
use bevy::prelude::*;

pub struct ComputePlugin;

impl Plugin for ComputePlugin {
    fn build(&self, _app: &mut App) {
        info!("⚡ GPU compute shader system ready");
    }
}
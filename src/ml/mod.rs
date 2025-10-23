use bevy::prelude::*;

pub mod workflow;
pub mod native;

pub use workflow::*;
pub use native::*;

pub struct MlPlugin;

impl Plugin for MlPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MLWorkflowPlugin,
            NativeMLPlugin,
        ));

        info!("🤖 Machine learning system ready");
    }
}

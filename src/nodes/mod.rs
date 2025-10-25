use bevy::prelude::*;
use crate::core::NodeId;

pub mod generators;
pub mod effects;
pub mod outputs;
pub mod utilities;
pub mod motion_capture;
pub mod fractal_shaders;
pub mod vst3_plugins;
pub mod stream_diffusion;
// pub mod ui; // Temporarily disabled due to egui compatibility issues

pub use generators::*;
pub use effects::*;
pub use outputs::*;
pub use utilities::*;
pub use motion_capture::*;
pub use fractal_shaders::*;
pub use vst3_plugins::*;
pub use stream_diffusion::*;
// pub use ui::*; // Temporarily disabled due to egui compatibility issues

#[derive(Clone, Debug, PartialEq)]
pub enum NodeType {
    AudioGenerator,
    AudioEffect,
    VisualGenerator,
    VisualEffect,
    Output,
    Utility,
}

#[derive(Component)]
pub struct NodeInstance {
    pub id: NodeId,
    pub node_type: NodeType,
    pub name: String,
    pub enabled: bool,
}

/// Nodes plugin - manages all node types
pub struct NodesPlugin;

impl Plugin for NodesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                GeneratorNodesPlugin,
                EffectNodesPlugin,
                OutputNodesPlugin,
                UtilityNodesPlugin,
                // NodeGraphUIPlugin, // Temporarily disabled due to egui compatibility issues
            ));
    }
}

/// Placeholder plugins for node categories
pub struct GeneratorNodesPlugin;
impl Plugin for GeneratorNodesPlugin {
    fn build(&self, _app: &mut App) {}
}

pub struct EffectNodesPlugin;
impl Plugin for EffectNodesPlugin {
    fn build(&self, _app: &mut App) {}
}

pub struct OutputNodesPlugin;
impl Plugin for OutputNodesPlugin {
    fn build(&self, _app: &mut App) {}
}

pub struct UtilityNodesPlugin;
impl Plugin for UtilityNodesPlugin {
    fn build(&self, _app: &mut App) {}
}
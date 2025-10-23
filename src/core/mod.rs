use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use anyhow::Result;

pub mod graph;
pub mod resources;
pub mod events;
mod tests;

pub use graph::*;
pub use resources::*;
pub use events::*;

/// VJ system error types
#[derive(Debug, Clone)]
pub enum VjError {
    AudioError(String),
    VideoError(String),
    NetworkError(String),
    FileError(String),
    ConfigError(String),
    FeatureNotEnabled(&'static str),
    NodeError(String),
    ConnectionError(String),
    MlError(String),
    ScriptError(String),
}

impl std::fmt::Display for VjError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VjError::AudioError(msg) => write!(f, "Audio error: {}", msg),
            VjError::VideoError(msg) => write!(f, "Video error: {}", msg),
            VjError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            VjError::FileError(msg) => write!(f, "File error: {}", msg),
            VjError::ConfigError(msg) => write!(f, "Config error: {}", msg),
            VjError::FeatureNotEnabled(feature) => write!(f, "Feature '{}' is not enabled", feature),
            VjError::NodeError(msg) => write!(f, "Node error: {}", msg),
            VjError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            VjError::MlError(msg) => write!(f, "ML error: {}", msg),
            VjError::ScriptError(msg) => write!(f, "Script error: {}", msg),
        }
    }
}

impl std::error::Error for VjError {}

/// Core VJ system plugin
pub struct VjCorePlugin;

impl Plugin for VjCorePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<VjSystemState>()
            .init_resource::<PerformanceMetrics>()
            .add_message::<VjEvent>()
            .register_type::<NodeId>()
            .register_type::<ConnectionId>()
            .add_systems(Update, (
                update_performance_metrics,
                handle_vj_events,
            ).chain());
    }
}

/// Unique identifier for nodes in the VJ system
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub struct NodeId(pub Uuid);

impl NodeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

/// Unique identifier for connections between nodes
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub struct ConnectionId(pub Uuid);

impl ConnectionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ConnectionId {
    fn default() -> Self {
        Self::new()
    }
}

/// Data types that can flow through the node graph
#[derive(Debug, Clone, Serialize, Deserialize, Reflect, PartialEq)]
pub enum DataType {
    Float,
    Vector2,
    Vector3,
    Vector4,
    Color,
    AudioBuffer,
    Image, // Raw image data
    // Texture(Handle<Image>),  // Commented out due to serialization issues
    // Mesh(Handle<Mesh>),      // Commented out due to serialization issues
    String,
    Boolean,
    // ML-specific data types
    Model,          // ML model data
    Conditioning,   // Text conditioning data
    Latent,         // Latent space representation
    Clip,           // CLIP model data
    VAE,            // VAE model data
    Mask,           // Image mask data
    Audio,          // Audio data
    Array,          // Generic array data
    Integer,        // Integer values
    Mesh,           // 3D mesh data
    Scene,          // 3D scene data
    Transform,      // Transform data
}

impl Default for DataType {
    fn default() -> Self {
        Self::Float
    }
}

/// Node trait for processing nodes in the graph
pub trait Node: Send + Sync {
    fn id(&self) -> NodeId;
    fn name(&self) -> &str;
    fn inputs(&self) -> Vec<InputPort>;
    fn outputs(&self) -> Vec<OutputPort>;
    fn process(&mut self, inputs: HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>>;
}

/// Input port definition for nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPort {
    pub name: String,
    pub data_type: DataType,
    pub required: bool,
}

impl InputPort {
    pub fn new(name: &str, data_type: DataType) -> Self {
        Self {
            name: name.to_string(),
            data_type,
            required: true,
        }
    }

    pub fn optional(name: &str, data_type: DataType) -> Self {
        Self {
            name: name.to_string(),
            data_type,
            required: false,
        }
    }
}

/// Output port definition for nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputPort {
    pub name: String,
    pub data_type: DataType,
}

impl OutputPort {
    pub fn new(name: &str, data_type: DataType) -> Self {
        Self {
            name: name.to_string(),
            data_type,
        }
    }
}

/// System for updating performance metrics
fn update_performance_metrics(
    time: Res<Time>,
    mut metrics: ResMut<PerformanceMetrics>,
) {
    metrics.frame_time = time.delta_secs();
    metrics.fps = 1.0 / time.delta_secs();
    metrics.total_time = time.elapsed_secs();
}

/// System for handling VJ events
fn handle_vj_events(
    mut events: MessageReader<VjEvent>,
) {
    for event in events.read() {
        match event {
            VjEvent::NodeCreated { node_id, node_type } => {
                info!("✨ Node created: {} ({})", node_id.0, node_type);
            }
            VjEvent::NodeDestroyed { node_id } => {
                info!("🗑️ Node destroyed: {}", node_id.0);
            }
            VjEvent::ConnectionEstablished { from, to } => {
                info!("🔌 Connection: {} -> {}", from.0, to.0);
            }
            VjEvent::ConnectionRemoved { from, to } => {
                info!("🔗 Connection removed: {} -x-> {}", from.0, to.0);
            }
            VjEvent::ParameterChanged { node_id, parameter, old_value, new_value } => {
                debug!("🎛️ Parameter changed on {}: {} = {} -> {}", 
                       node_id.0, parameter, old_value, new_value);
            }
            VjEvent::AudioEvent { event_type } => {
                debug!("🎵 Audio event: {:?}", event_type);
            }
            VjEvent::VisualEvent { event_type } => {
                debug!("🎨 Visual event: {:?}", event_type);
            }
            VjEvent::ScriptEvent { script_id, event_type } => {
                debug!("📜 Script event on {}: {:?}", script_id, event_type);
            }
            VjEvent::McpEvent { server_id, event_type } => {
                debug!("🔌 MCP server {} event: {:?}", server_id, event_type);
            }
            VjEvent::PerformanceEvent { event_type } => {
                debug!("⚡ Performance event: {:?}", event_type);
            }
        }
    }
}
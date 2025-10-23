use bevy::prelude::*;
use crate::core::NodeId;

/// Main event system for VJ operations
#[derive(Message, Debug, Clone)]
pub enum VjEvent {
    /// Node lifecycle events
    NodeCreated {
        node_id: NodeId,
        node_type: String,
    },
    NodeDestroyed {
        node_id: NodeId,
    },
    
    /// Connection events
    ConnectionEstablished {
        from: NodeId,
        to: NodeId,
    },
    ConnectionRemoved {
        from: NodeId,
        to: NodeId,
    },
    
    /// Parameter changes
    ParameterChanged {
        node_id: NodeId,
        parameter: String,
        old_value: String,
        new_value: String,
    },
    
    /// Audio system events
    AudioEvent {
        event_type: AudioEventType,
    },
    
    /// Visual system events
    VisualEvent {
        event_type: VisualEventType,
    },
    
    /// Script execution events
    ScriptEvent {
        script_id: String,
        event_type: ScriptEventType,
    },
    
    /// MCP server events
    McpEvent {
        server_id: String,
        event_type: McpEventType,
    },
    
    /// Performance events
    PerformanceEvent {
        event_type: PerformanceEventType,
    },
}

/// Audio-specific event types
#[derive(Debug, Clone)]
pub enum AudioEventType {
    DeviceConnected { device_name: String },
    DeviceDisconnected { device_name: String },
    BufferUnderrun,
    BufferOverrun,
    MidiNote { channel: u8, note: u8, velocity: u8 },
    MidiControlChange { channel: u8, controller: u8, value: u8 },
    BeatDetected { bpm: f32, confidence: f32 },
    BeatTriggered,
}

/// Visual-specific event types
#[derive(Debug, Clone)]
pub enum VisualEventType {
    ShaderCompiled { shader_name: String, success: bool },
    ShaderHotReloaded { shader_path: String },
    TextureLoaded { texture_name: String },
    RenderTargetCreated { target_name: String, size: (u32, u32) },
    ComputeShaderDispatch { shader_name: String, groups: (u32, u32, u32) },
    EffectTriggered,
}

/// Script execution event types
#[derive(Debug, Clone)]
pub enum ScriptEventType {
    ScriptLoaded { language: String },
    ScriptExecuted { duration_ms: u64, success: bool },
    ScriptError { error: String },
    FunctionCalled { function_name: String, args: Vec<String> },
}

/// MCP server event types
#[derive(Debug, Clone)]
pub enum McpEventType {
    ServerStarted { port: u16 },
    ServerStopped,
    ClientConnected { client_id: String },
    ClientDisconnected { client_id: String },
    MessageReceived { message_type: String, data: String },
    MessageSent { message_type: String, data: String },
}

/// Performance monitoring event types
#[derive(Debug, Clone)]
pub enum PerformanceEventType {
    FpsDropped { from: f32, to: f32 },
    MemoryUsageHigh { usage_mb: u64 },
    GpuMemoryLow { available_mb: u64 },
    AudioLatencyHigh { latency_ms: f32 },
    NodeProcessingTime { node_id: NodeId, time_ms: f32 },
}

/// Scene management events
#[derive(Message, Debug, Clone)]
pub enum SceneEvent {
    SceneLoaded { scene_name: String },
    SceneSaved { scene_name: String, path: String },
    SceneExported { scene_name: String, format: String },
    SceneSwitched { from: String, to: String },
}

/// UI interaction events
#[derive(Message, Debug, Clone)]
pub enum UiEvent {
    NodeSelected { node_id: NodeId },
    NodeDeselected { node_id: NodeId },
    NodeDragged { node_id: NodeId, position: Vec2 },
    ConnectionDragStarted { node_id: NodeId, port: usize },
    ConnectionDragCompleted { from: NodeId, to: NodeId },
    ParameterAdjusted { node_id: NodeId, parameter: String, value: f32 },
}

/// ComfyUI integration events
#[derive(Message, Debug, Clone)]
pub enum ComfyUiEvent {
    WorkflowStarted { workflow_id: String },
    WorkflowCompleted { workflow_id: String, outputs: Vec<String> },
    WorkflowFailed { workflow_id: String, error: String },
    ImageGenerated { prompt: String, image_path: String },
    ModelLoaded { model_name: String, model_type: String },
}

/// Machine learning events
#[derive(Message, Debug, Clone)]
pub enum MlEvent {
    ModelLoaded { model_name: String, model_type: String },
    InferenceStarted { model_name: String, input_shape: Vec<usize> },
    InferenceCompleted { model_name: String, duration_ms: u64 },
    TrainingStarted { model_name: String, dataset_size: usize },
    TrainingEpochCompleted { model_name: String, epoch: usize, loss: f32 },
}

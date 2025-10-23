use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Global VJ system state
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct VjSystemState {
    pub is_playing: bool,
    pub bpm: f32,
    pub current_scene: String,
    pub master_volume: f32,
    pub master_brightness: f32,
    pub tempo_sync_enabled: bool,
}

impl Default for VjSystemState {
    fn default() -> Self {
        Self {
            is_playing: false,
            bpm: 120.0,
            current_scene: "Default".to_string(),
            master_volume: 1.0,
            master_brightness: 1.0,
            tempo_sync_enabled: true,
        }
    }
}

/// Performance metrics for monitoring system health
#[derive(Resource, Debug, Default)]
pub struct PerformanceMetrics {
    pub fps: f32,
    pub frame_time: f32,
    pub total_time: f32,
    pub audio_latency: f32,
    pub gpu_memory_usage: u64,
    pub cpu_usage: f32,
    pub active_nodes: usize,
    pub active_connections: usize,
}

/// Global configuration settings
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct VjConfig {
    pub audio_buffer_size: usize,
    pub audio_sample_rate: u32,
    pub max_nodes: usize,
    pub max_connections: usize,
    pub enable_gpu_profiling: bool,
    pub enable_hot_reload: bool,
    pub mcp_server_port: u16,
    pub comfyui_endpoint: String,
}

impl Default for VjConfig {
    fn default() -> Self {
        Self {
            audio_buffer_size: 512,
            audio_sample_rate: 44100,
            max_nodes: 1000,
            max_connections: 5000,
            enable_gpu_profiling: true,
            enable_hot_reload: true,
            mcp_server_port: 8080,
            comfyui_endpoint: "http://localhost:8188".to_string(),
        }
    }
}

/// Scene management resource
#[derive(Resource, Debug, Default)]
pub struct SceneManager {
    pub scenes: HashMap<String, SceneData>,
    pub current_scene: Option<String>,
}

impl SceneManager {
    pub fn add_scene(&mut self, name: String, scene: SceneData) {
        self.scenes.insert(name, scene);
    }
    
    pub fn switch_scene(&mut self, name: &str) -> Result<(), String> {
        if self.scenes.contains_key(name) {
            self.current_scene = Some(name.to_string());
            Ok(())
        } else {
            Err(format!("Scene '{}' not found", name))
        }
    }
}

/// Data for a complete VJ scene
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneData {
    pub name: String,
    pub description: String,
    pub nodes: Vec<SavedNodeData>,
    pub connections: Vec<SavedConnectionData>,
    pub parameters: HashMap<String, f32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub modified_at: chrono::DateTime<chrono::Utc>,
}

/// Saved node data for scene serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedNodeData {
    pub id: uuid::Uuid,
    pub node_type: String,
    pub position: Vec2,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Saved connection data for scene serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedConnectionData {
    pub id: uuid::Uuid,
    pub from_node: uuid::Uuid,
    pub from_port: usize,
    pub to_node: uuid::Uuid,
    pub to_port: usize,
    pub data_type: String,
}
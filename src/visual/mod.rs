use bevy::prelude::*;
use bevy::render::render_resource::*;
#[cfg(feature = "profiling")]
use bevy::render::renderer::RenderDevice;
#[cfg(feature = "profiling")]
use wgpu_profiler::GpuProfiler;
use std::collections::HashMap;
#[cfg(feature = "profiling")]
use std::path::Path;

pub mod shader_loader;
pub mod effects;
pub mod post_processing;

use shader_loader::*;
use effects::*;
use post_processing::*;

#[derive(Resource, Default)]
pub struct ShaderRegistry {
    pub shaders: HashMap<String, Handle<Shader>>,
    pub hot_reload_enabled: bool,
}

#[derive(Resource, Default)] 
pub struct VisualEffectsState {
    pub active_effects: Vec<EffectInstance>,
    pub effect_chain: Vec<String>,
}

#[derive(Clone)]
pub struct EffectInstance {
    pub id: String,
    pub effect_type: EffectType,
    pub parameters: HashMap<String, f32>,
    pub enabled: bool,
}

#[derive(Clone, Debug)]
pub enum EffectType {
    ColorGrade,
    Blur,
    ChromaticAberration,
    Distortion,
    Custom(String),
}

pub struct VisualPlugin;

impl Plugin for VisualPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ShaderRegistry>()
            .init_resource::<VisualEffectsState>()
            .add_plugins((
                ShaderLoaderPlugin,
                EffectsPlugin, 
                PostProcessingPlugin,
            ))
            .add_systems(Startup, setup_visual_system)
            .add_systems(Update, (
                update_shader_hot_reload,
                update_visual_effects,
            ));
        
        info!("🎨 Visual system initialized");
    }
}

fn setup_visual_system(
    mut commands: Commands,
    mut shader_registry: ResMut<ShaderRegistry>,
    asset_server: Res<AssetServer>,
) {
    info!("🚀 Setting up visual system...");
    
    // Enable hot reload in debug mode
    if cfg!(debug_assertions) {
        shader_registry.hot_reload_enabled = true;
        info!("🔥 Shader hot reload enabled");
    }
    
    // Load default shaders
    let default_shaders = vec![
        ("basic_fragment", "shaders/basic_fragment.wgsl"),
        ("uv_gradient", "shaders/uv_gradient.wgsl"), 
        ("time_animation", "shaders/time_animation.wgsl"),
    ];
    
    for (name, path) in default_shaders {
        let handle = asset_server.load(path);
        shader_registry.shaders.insert(name.to_string(), handle);
    }
    
    info!("✅ Visual system ready");
}

fn update_shader_hot_reload(
    shader_registry: Res<ShaderRegistry>,
    asset_server: Res<AssetServer>,
) {
    if !shader_registry.hot_reload_enabled {
        return;
    }
    
    // Hot reload logic would go here - watching file system changes
    // For now, this is a placeholder
}

fn update_visual_effects(
    mut effects_state: ResMut<VisualEffectsState>,
    time: Res<Time>,
) {
    // Update effect parameters based on time or other inputs
    for effect in &mut effects_state.active_effects {
        if !effect.enabled {
            continue;
        }
        
        match effect.effect_type {
            EffectType::ColorGrade => {
                // Animate color grading parameters
                if let Some(intensity) = effect.parameters.get_mut("intensity") {
                    *intensity = 0.5 + 0.3 * (time.elapsed_secs() * 0.5).sin();
                }
            }
            EffectType::Distortion => {
                // Animate distortion
                if let Some(amount) = effect.parameters.get_mut("amount") {
                    *amount = 0.1 * (time.elapsed_secs() * 2.0).sin();
                }
            }
            _ => {}
        }
    }
}

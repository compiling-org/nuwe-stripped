use bevy::prelude::*;
use bevy::asset::{AssetLoader, LoadedAsset, AssetPath};
use bevy::render::render_resource::*;
use std::path::Path;
use std::collections::HashMap;

pub struct ShaderLoaderPlugin;

impl Plugin for ShaderLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<HotReloadState>()
            .add_systems(Update, check_shader_changes);
        
        info!("🔧 Shader loader plugin initialized");
    }
}

#[derive(Resource, Default)]
pub struct HotReloadState {
    pub watched_files: HashMap<String, std::time::SystemTime>,
    pub enabled: bool,
}

pub fn check_shader_changes(
    mut hot_reload_state: ResMut<HotReloadState>,
    asset_server: Res<AssetServer>,
) {
    if !hot_reload_state.enabled {
        return;
    }

    let shader_dirs = vec!["assets/shaders", "shaders"];
    
    for dir in shader_dirs {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "wgsl" || extension == "glsl" {
                        if let Ok(metadata) = entry.metadata() {
                            if let Ok(modified) = metadata.modified() {
                                let path_str = path.to_string_lossy().to_string();
                                
                                let needs_reload = hot_reload_state
                                    .watched_files
                                    .get(&path_str)
                                    .map_or(true, |&last_modified| modified > last_modified);
                                
                                if needs_reload {
                                    info!("🔄 Reloading shader: {}", path_str);
                                    hot_reload_state.watched_files.insert(path_str, modified);
                                    // Trigger asset reload
                                    // asset_server.reload(&path); // Uncomment when available
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// Shader compilation utilities
pub struct ShaderCompiler;

impl ShaderCompiler {
    pub fn compile_wgsl(source: String, _name: &str) -> Result<Shader, String> {
        // Basic WGSL validation
        if !source.contains("@vertex") && !source.contains("@fragment") && !source.contains("@compute") {
            return Err("Shader must contain at least one entry point".to_string());
        }
        
        // In a real implementation, this would use naga or wgpu for proper compilation
        Ok(Shader::from_wgsl(source, file!()))
    }
    
    pub fn create_default_shaders() -> HashMap<String, String> {
        let mut shaders = HashMap::new();
        
        // Basic fragment shader
        shaders.insert("basic_fragment".to_string(), 
            r#"@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(uv, 0.5, 1.0);
}
"#.to_string());

        // UV gradient shader
        shaders.insert("uv_gradient".to_string(),
            r#"struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let gradient = mix(vec3<f32>(1.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 1.0), input.uv.x);
    return vec4<f32>(gradient, 1.0);
}
"#.to_string());

        // Time-based animation shader
        shaders.insert("time_animation".to_string(),
            r#"@group(0) @binding(0) var<uniform> time: f32;

@fragment 
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let center = vec2<f32>(0.5, 0.5);
    let dist = distance(uv, center);
    let wave = sin(dist * 10.0 - time * 3.0) * 0.5 + 0.5;
    let color = vec3<f32>(wave, wave * 0.5, 1.0 - wave);
    return vec4<f32>(color, 1.0);
}
"#.to_string());

        // Color grading shader
        shaders.insert("color_grade".to_string(),
            r#"@group(0) @binding(0) var input_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
@group(0) @binding(2) var<uniform> params: ColorGradeParams;

struct ColorGradeParams {
    contrast: f32,
    brightness: f32,
    saturation: f32,
    hue_shift: f32,
}

@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    var color = textureSample(input_texture, texture_sampler, uv);
    
    // Apply brightness
    color = vec4<f32>(color.rgb + vec3<f32>(params.brightness), color.a);
    
    // Apply contrast  
    color = vec4<f32>((color.rgb - 0.5) * params.contrast + 0.5, color.a);
    
    // Apply saturation
    let luminance = dot(color.rgb, vec3<f32>(0.299, 0.587, 0.114));
    color = vec4<f32>(mix(vec3<f32>(luminance), color.rgb, params.saturation), color.a);
    
    return color;
}
"#.to_string());

        // Blur shader
        shaders.insert("blur".to_string(),
            r#"@group(0) @binding(0) var input_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
@group(0) @binding(2) var<uniform> blur_radius: f32;

@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let texel_size = 1.0 / vec2<f32>(textureDimensions(input_texture));
    var color = vec4<f32>(0.0);
    let samples = 9;
    
    for (var x = -1; x <= 1; x++) {
        for (var y = -1; y <= 1; y++) {
            let offset = vec2<f32>(f32(x), f32(y)) * texel_size * blur_radius;
            color += textureSample(input_texture, texture_sampler, uv + offset);
        }
    }
    
    return color / f32(samples);
}
"#.to_string());
        
        shaders
    }
}
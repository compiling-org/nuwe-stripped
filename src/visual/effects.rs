use bevy::prelude::*;
use std::collections::HashMap;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, process_visual_effects);
        
        info!("✨ Effects plugin initialized");
    }
}

#[derive(Component)]
pub struct EffectChain {
    pub effects: Vec<Effect>,
    pub enabled: bool,
}

#[derive(Clone)]
pub struct Effect {
    pub id: String,
    pub effect_type: EffectType,
    pub parameters: EffectParameters,
    pub enabled: bool,
    pub blend_mode: BlendMode,
}

#[derive(Clone, Debug)]
pub enum EffectType {
    ColorGrade,
    Blur { radius: f32 },
    ChromaticAberration { strength: f32 },
    Distortion { amount: f32, frequency: f32 },
    Feedback { decay: f32, offset: Vec2 },
    Kaleidoscope { segments: u32, angle: f32 },
    Custom { shader_name: String },
}

#[derive(Clone)]
pub struct EffectParameters {
    pub floats: HashMap<String, f32>,
    pub vectors: HashMap<String, Vec3>,
    pub colors: HashMap<String, Color>,
}

impl Default for EffectParameters {
    fn default() -> Self {
        Self {
            floats: HashMap::new(),
            vectors: HashMap::new(), 
            colors: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum BlendMode {
    Normal,
    Add,
    Multiply,
    Screen,
    Overlay,
    SoftLight,
}

impl EffectChain {
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
            enabled: true,
        }
    }
    
    pub fn add_effect(&mut self, effect: Effect) {
        self.effects.push(effect);
    }
    
    pub fn remove_effect(&mut self, id: &str) {
        self.effects.retain(|e| e.id != id);
    }
    
    pub fn get_effect_mut(&mut self, id: &str) -> Option<&mut Effect> {
        self.effects.iter_mut().find(|e| e.id == id)
    }
    
    pub fn create_color_grade_effect(id: String) -> Effect {
        let mut params = EffectParameters::default();
        params.floats.insert("contrast".to_string(), 1.0);
        params.floats.insert("brightness".to_string(), 0.0);
        params.floats.insert("saturation".to_string(), 1.0);
        params.floats.insert("hue_shift".to_string(), 0.0);
        
        Effect {
            id,
            effect_type: EffectType::ColorGrade,
            parameters: params,
            enabled: true,
            blend_mode: BlendMode::Normal,
        }
    }
    
    pub fn create_blur_effect(id: String, radius: f32) -> Effect {
        let mut params = EffectParameters::default();
        params.floats.insert("radius".to_string(), radius);
        
        Effect {
            id,
            effect_type: EffectType::Blur { radius },
            parameters: params,
            enabled: true,
            blend_mode: BlendMode::Normal,
        }
    }
    
    pub fn create_distortion_effect(id: String, amount: f32, frequency: f32) -> Effect {
        let mut params = EffectParameters::default();
        params.floats.insert("amount".to_string(), amount);
        params.floats.insert("frequency".to_string(), frequency);
        
        Effect {
            id,
            effect_type: EffectType::Distortion { amount, frequency },
            parameters: params,
            enabled: true,
            blend_mode: BlendMode::Normal,
        }
    }
    
    pub fn create_kaleidoscope_effect(id: String, segments: u32, angle: f32) -> Effect {
        let mut params = EffectParameters::default();
        params.floats.insert("segments".to_string(), segments as f32);
        params.floats.insert("angle".to_string(), angle);
        
        Effect {
            id,
            effect_type: EffectType::Kaleidoscope { segments, angle },
            parameters: params,
            enabled: true,
            blend_mode: BlendMode::Normal,
        }
    }
}

fn process_visual_effects(
    mut query: Query<&mut EffectChain>,
    time: Res<Time>,
) {
    for mut effect_chain in query.iter_mut() {
        if !effect_chain.enabled {
            continue;
        }
        
        for effect in &mut effect_chain.effects {
            if !effect.enabled {
                continue;
            }
            
            // Update time-based parameters
            match &effect.effect_type {
                EffectType::Distortion { .. } => {
                    // Animate distortion over time
                    if let Some(phase) = effect.parameters.floats.get_mut("phase") {
                        *phase = time.elapsed_secs() * 2.0;
                    } else {
                        effect.parameters.floats.insert("phase".to_string(), time.elapsed_secs() * 2.0);
                    }
                }
                EffectType::Kaleidoscope { .. } => {
                    // Slowly rotate kaleidoscope
                    if let Some(angle) = effect.parameters.floats.get_mut("angle") {
                        *angle += time.delta_secs() * 0.1;
                    }
                }
                _ => {}
            }
        }
    }
}

// Preset effect chains for common VJ scenarios
pub struct EffectPresets;

impl EffectPresets {
    pub fn psychedelic_chain() -> EffectChain {
        let mut chain = EffectChain::new();
        
        chain.add_effect(EffectChain::create_kaleidoscope_effect(
            "kaleidoscope".to_string(), 6, 0.0
        ));
        
        chain.add_effect(EffectChain::create_distortion_effect(
            "distortion".to_string(), 0.05, 4.0
        ));
        
        chain.add_effect(EffectChain::create_color_grade_effect(
            "color_grade".to_string()
        ));
        
        chain
    }
    
    pub fn dreamy_chain() -> EffectChain {
        let mut chain = EffectChain::new();
        
        chain.add_effect(EffectChain::create_blur_effect(
            "soft_blur".to_string(), 2.0
        ));
        
        let mut color_grade = EffectChain::create_color_grade_effect(
            "dreamy_grade".to_string()
        );
        color_grade.parameters.floats.insert("saturation".to_string(), 1.3);
        color_grade.parameters.floats.insert("brightness".to_string(), 0.1);
        chain.add_effect(color_grade);
        
        chain
    }
    
    pub fn glitch_chain() -> EffectChain {
        let mut chain = EffectChain::new();
        
        let mut distortion = EffectChain::create_distortion_effect(
            "glitch_distort".to_string(), 0.1, 8.0
        );
        distortion.blend_mode = BlendMode::Add;
        chain.add_effect(distortion);
        
        let mut aberration_params = EffectParameters::default();
        aberration_params.floats.insert("strength".to_string(), 0.02);
        
        chain.add_effect(Effect {
            id: "chromatic_ab".to_string(),
            effect_type: EffectType::ChromaticAberration { strength: 0.02 },
            parameters: aberration_params,
            enabled: true,
            blend_mode: BlendMode::Normal,
        });
        
        chain
    }
}
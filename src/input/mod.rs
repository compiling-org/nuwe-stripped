use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;
use glam::{Vec2, Vec3, Quat};

pub mod midi_advanced;
pub mod osc;

pub use midi_advanced::*;
pub use osc::*;

// Unified Input Event System
#[derive(Debug, Clone, Message)]
pub enum InputEvent {
    // MIDI
    MidiNoteOn { channel: u8, note: u8, velocity: u8 },
    MidiNoteOff { channel: u8, note: u8 },
    MidiControlChange { channel: u8, controller: u8, value: u8 },
    MidiPitchBend { channel: u8, value: u16 },
    MidiProgramChange { channel: u8, program: u8 },

    // OSC
    OscMessage { address: String, args: Vec<OscArg> },

    // Lighting Protocols
    ArtNet { universe: u16, data: Vec<u8> },
    Sacn { universe: u16, data: Vec<u8> },
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OscArg {
    Float(f32),
    Double(f64),
    Int(i32),
    Long(i64),
    String(String),
    Bool(bool),
    Blob(Vec<u8>),
}


// Input Processing Systems
#[derive(Resource)]
pub struct InputSystemManager {
    pub midi_enabled: bool,
    pub osc_enabled: bool,
    pub input_mappings: HashMap<String, InputMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMapping {
    pub source: InputSource,
    pub target: String,
    pub transform: Option<InputTransform>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputSource {
    MidiControl { channel: u8, controller: u8 },
    OscAddress { address: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Axis {
    X,
    Y,
    Z,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputTransform {
    pub scale: f32,
    pub offset: f32,
    pub invert: bool,
    pub smooth: Option<f32>, // Smoothing factor 0-1
    pub deadzone: Option<f32>, // Deadzone threshold
}

impl Default for InputSystemManager {
    fn default() -> Self {
        Self {
            midi_enabled: true,
            osc_enabled: true,
            input_mappings: HashMap::new(),
        }
    }
}

impl InputSystemManager {
    pub fn enable_system(&mut self, system: InputSystemType) -> Result<()> {
        match system {
            InputSystemType::Midi => {
                self.midi_enabled = true;
                info!("🎹 Enhanced MIDI input system enabled");
            }
            InputSystemType::Osc => {
                self.osc_enabled = true;
                info!("🌐 OSC input system enabled");
            }
        }
        Ok(())
    }

    pub fn add_mapping(&mut self, name: String, mapping: InputMapping) {
        self.input_mappings.insert(name, mapping);
    }

    pub fn process_input(&self, event: &InputEvent) -> Vec<(String, f32)> {
        let mut outputs = Vec::new();
        
        for (target_name, mapping) in &self.input_mappings {
            if !mapping.enabled {
                continue;
            }
            
            if let Some(value) = self.extract_value(event, &mapping.source) {
                let transformed_value = self.apply_transform(value, &mapping.transform);
                outputs.push((target_name.clone(), transformed_value));
            }
        }
        
        outputs
    }

    fn extract_value(&self, event: &InputEvent, source: &InputSource) -> Option<f32> {
        match (event, source) {
            (InputEvent::MidiControlChange { controller, value, .. },
             InputSource::MidiControl { controller: target_controller, .. }) => {
                if *controller == *target_controller {
                    Some(*value as f32 / 127.0) // Normalize to 0-1
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn apply_transform(&self, value: f32, transform: &Option<InputTransform>) -> f32 {
        let mut result = value;
        
        if let Some(transform) = transform {
            // Apply deadzone
            if let Some(deadzone) = transform.deadzone {
                if result.abs() < deadzone {
                    result = 0.0;
                }
            }
            
            // Apply invert
            if transform.invert {
                result = -result;
            }
            
            // Apply scale and offset
            result = result * transform.scale + transform.offset;
            
            // Note: Smoothing would require state management, implemented in systems
        }
        
        result
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputSystemType {
    Midi,
    Osc,
}

// Events for input system
#[derive(Event, Message)]
pub struct InputMappingEvent {
    pub mapping_name: String,
    pub value: f32,
}


// Bevy systems
pub fn process_input_events(
    mut input_events: MessageReader<InputEvent>,
    mut mapping_events: MessageWriter<InputMappingEvent>,
    input_manager: Res<InputSystemManager>,
) {
    for event in input_events.read() {
        let mappings = input_manager.process_input(event);
        for (mapping_name, value) in mappings {
            mapping_events.write(InputMappingEvent {
                mapping_name,
                value,
            });
        }
    }
}

pub fn handle_input_mappings(
    mut mapping_events: MessageReader<InputMappingEvent>,
    // Add your target systems here (audio, visual, etc.)
) {
    for event in mapping_events.read() {
        info!("🎛️ Input mapping '{}' = {:.3}", event.mapping_name, event.value);

        // Route to appropriate systems based on mapping name
        match event.mapping_name.as_str() {
            "volume" => {
                // Update audio volume
            }
            "color_hue" => {
                // Update visual color
            }
            "effect_strength" => {
                // Update effect parameters
            }
            _ => {
                // Handle custom mappings
            }
        }
    }
}

pub struct InputSystemPlugin;

impl Plugin for InputSystemPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputSystemManager::default())
            .add_message::<InputEvent>()
            .add_message::<InputMappingEvent>()
            .add_systems(Update, (
                process_input_events,
                handle_input_mappings,
            ));

        info!("🎮 Advanced input system initialized");
    }
}
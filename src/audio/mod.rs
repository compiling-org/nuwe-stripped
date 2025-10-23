use bevy::prelude::*;

pub mod glicol_integration;
pub mod midi_handler;
pub mod audio_analysis;
pub mod synthesis;
pub mod ui;

pub use glicol_integration::*;
pub use midi_handler::*;
pub use audio_analysis::*;
pub use synthesis::*;
pub use ui::*;

/// Audio system plugin integrating Glicol, MIDI, and analysis
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                GlicolPlugin,
                MidiPlugin,
                AudioAnalysisPlugin,
                SynthesisPlugin,
                AudioUiPlugin,
            ))
            .init_resource::<AudioSettings>()
            .add_systems(Startup, setup_audio_system)
            .add_systems(Update, (
                update_audio_metrics,
                process_audio_events,
            ));

    }
}

/// Audio system settings and configuration
#[derive(Resource, Debug)]
pub struct AudioSettings {
    pub sample_rate: f32,
    pub buffer_size: usize,
    pub channels: usize,
    pub master_volume: f32,
    pub latency_ms: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            sample_rate: 44100.0,
            buffer_size: 512,
            channels: 2,
            master_volume: 1.0,
            latency_ms: 12.0,
        }
    }
}

/// Audio metrics for monitoring
#[derive(Resource, Debug, Default)]
pub struct AudioMetrics {
    pub current_bpm: f32,
    pub peak_level_left: f32,
    pub peak_level_right: f32,
    pub rms_level_left: f32,
    pub rms_level_right: f32,
    pub spectrum: Vec<f32>, // FFT frequency bins
    pub beats_detected: u32,
    pub last_beat_time: f64,
}

/// Setup audio system
fn setup_audio_system(
    mut commands: Commands,
    audio_settings: Res<AudioSettings>,
) {
    info!("🎵 Initializing audio system...");
    info!("Sample rate: {} Hz", audio_settings.sample_rate);
    info!("Buffer size: {} samples", audio_settings.buffer_size);
    info!("Channels: {}", audio_settings.channels);
    
    // Initialize audio metrics resource
    commands.init_resource::<AudioMetrics>();
    
    info!("✅ Audio system initialized");
}

/// Update audio performance metrics
fn update_audio_metrics(
    mut metrics: ResMut<AudioMetrics>,
    time: Res<Time>,
) {
    // This would be populated by actual audio processing
    // For now, just demonstrate the structure
    
    // Simulate beat detection
    let current_time = time.elapsed_secs_f64();
    if current_time - metrics.last_beat_time > 60.0 / 120.0 { // 120 BPM
        metrics.beats_detected += 1;
        metrics.last_beat_time = current_time;
        metrics.current_bpm = 120.0;
    }
}

/// Process audio-related events
fn process_audio_events() {
    // Handle audio events from MIDI, beat detection, etc.
}

/// Placeholder plugins for audio subsystems
pub struct GlicolPlugin;
impl Plugin for GlicolPlugin {
    fn build(&self, app: &mut App) {
        info!("🎼 Glicol live coding audio system ready");
    }
}

pub struct MidiPlugin;
impl Plugin for MidiPlugin {
    fn build(&self, app: &mut App) {
        info!("🎹 MIDI input system ready");
    }
}

pub struct AudioAnalysisPlugin;
impl Plugin for AudioAnalysisPlugin {
    fn build(&self, app: &mut App) {
        info!("📊 Audio analysis system ready");
    }
}

pub struct SynthesisPlugin;
impl Plugin for SynthesisPlugin {
    fn build(&self, app: &mut App) {
        info!("🎛️ Audio synthesis system ready");
    }
}
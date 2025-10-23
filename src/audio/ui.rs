#[cfg(feature = "ui")]
use bevy::prelude::*;
#[cfg(feature = "ui")]
use bevy_egui::{egui, EguiContexts, EguiPlugin};
#[cfg(feature = "ui")]
use egui_tiles::{Container, Linear, Tabs, Tile, TileId, Tiles, Tree};


/// UI plugin for audio system controls
#[cfg(feature = "ui")]
pub struct AudioUiPlugin;

#[cfg(feature = "ui")]
impl Plugin for AudioUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(EguiPlugin::default())
            .init_resource::<AudioUiState>()
            .add_systems(Update, audio_ui_system);
    }
}

/// UI state for audio controls
#[cfg(feature = "ui")]
#[derive(Resource)]
pub struct AudioUiState {
    pub show_audio_panel: bool,
    pub selected_patch: Option<String>,
    pub patch_parameters: std::collections::HashMap<String, f32>,
    pub audio_metrics: bool,
}

#[cfg(feature = "ui")]
impl Default for AudioUiState {
    fn default() -> Self {
        Self {
            show_audio_panel: true,
            selected_patch: None,
            patch_parameters: std::collections::HashMap::new(),
            audio_metrics: true,
        }
    }
}

/// Main audio UI system
#[cfg(feature = "ui")]
fn audio_ui_system(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<AudioUiState>,
    plugdata: Option<()>,
    audio_settings: Res<crate::audio::AudioSettings>,
    audio_metrics: Res<crate::audio::AudioMetrics>,
) {
    let ctx = contexts.ctx_mut().unwrap();

    // Main audio control window
    if ui_state.show_audio_panel {
        egui::Window::new("🎵 Audio Control")
            .default_width(400.0)
            .show(ctx, |ui| {
                ui.collapsing("🎛️ Audio Synthesis", |ui| {
                    ui.label("🎼 Glicol live coding synthesis");
                    ui.label("🎹 MIDI input/output support");
                    ui.label("📊 Real-time audio analysis");
                });

                ui.collapsing("📊 Audio Metrics", |ui| {
                    show_audio_metrics_ui(ui, &audio_metrics);
                });

                ui.collapsing("⚙️ Audio Settings", |ui| {
                    show_audio_settings_ui(ui, &audio_settings);
                });
            });
    }
}

/// Show PlugData patch controls
#[cfg(all(feature = "ui", feature = "plugdata-integration"))]
fn show_plugdata_ui(
    ui: &mut egui::Ui,
    plugdata: &mut PlugDataRuntime,
    ui_state: &mut AudioUiState,
) {
    // Patch list
    ui.label("Loaded Patches:");
    let patch_info = plugdata.get_patch_info();

    if patch_info.is_empty() {
        ui.label("(No patches loaded)");
    } else {
        for (key, value) in &patch_info {
            if key.ends_with("_path") {
                let patch_name = key.trim_end_matches("_path");
                let selected = ui_state.selected_patch.as_ref() == Some(patch_name);

                if ui.selectable_label(selected, patch_name).clicked() {
                    ui_state.selected_patch = Some(patch_name.to_string());
                }
            }
        }
    }

    ui.separator();

    // Patch controls
    if let Some(ref patch_name) = ui_state.selected_patch {
        ui.label(format!("🎛️ Controls for: {}", patch_name));

        // Parameter controls (example parameters)
        let params = vec![
            ("volume", "Volume", 0.0..=1.0),
            ("frequency", "Frequency", 20.0..=20000.0),
            ("resonance", "Resonance", 0.0..=1.0),
            ("filter_cutoff", "Filter Cutoff", 20.0..=20000.0),
        ];

        for (param_key, param_name, range) in params {
            let osc_addr = format!("/{}/{}", patch_name, param_key);
            let current_value = ui_state.patch_parameters
                .get(&osc_addr)
                .copied()
                .unwrap_or(0.5);

            let mut new_value = current_value;
            ui.add(
                egui::Slider::new(&mut new_value, range.clone())
                    .text(param_name)
            );

            if new_value != current_value {
                ui_state.patch_parameters.insert(osc_addr.clone(), new_value);
                // Send OSC parameter update
                let _ = plugdata.send_parameter(&osc_addr, new_value);
            }
        }

        ui.separator();

        // Patch actions
        if ui.button("🔄 Reload Patch").clicked() {
            // This would need patch path storage
            ui.label("Reload functionality needs patch path storage");
        }

        if ui.button("🗑️ Unload Patch").clicked() {
            if let Err(e) = plugdata.unload_patch(patch_name) {
                ui.label(format!("❌ Failed to unload: {:?}", e));
            } else {
                ui_state.selected_patch = None;
                ui_state.patch_parameters.clear();
            }
        }
    }

    ui.separator();

    // Load new patch
    ui.label("Load New Patch:");
    static mut PATCH_PATH: String = String::new();
    static mut PATCH_NAME: String = String::new();

    unsafe {
        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut PATCH_NAME);
        });

        ui.horizontal(|ui| {
            ui.label("Path:");
            ui.text_edit_singleline(&mut PATCH_PATH);
        });

        if ui.button("📁 Load Patch").clicked() && !PATCH_NAME.is_empty() && !PATCH_PATH.is_empty() {
            match plugdata.load_patch(&PATCH_NAME, &PATCH_PATH) {
                Ok(_) => {
                    ui_state.selected_patch = Some(PATCH_NAME.clone());
                    PATCH_NAME.clear();
                    PATCH_PATH.clear();
                }
                Err(e) => {
                    ui.label(format!("❌ Failed to load: {:?}", e));
                }
            }
        }
    }
}

/// Show audio metrics display
#[cfg(feature = "ui")]
fn show_audio_metrics_ui(ui: &mut egui::Ui, metrics: &crate::audio::AudioMetrics) {
    ui.label(format!("🎵 BPM: {:.1}", metrics.current_bpm));
    ui.label(format!("📊 Peak L/R: {:.3}/{:.3}", metrics.peak_level_left, metrics.peak_level_right));
    ui.label(format!("📈 RMS L/R: {:.3}/{:.3}", metrics.rms_level_left, metrics.rms_level_right));
    ui.label(format!("🥁 Beats Detected: {}", metrics.beats_detected));

    if !metrics.spectrum.is_empty() {
        ui.label("Spectrum (first 10 bins):");
        ui.horizontal(|ui| {
            for (i, &bin) in metrics.spectrum.iter().take(10).enumerate() {
                ui.label(format!("{:.2}", bin));
                if i < 9 {
                    ui.label("|");
                }
            }
        });
    }
}

/// Show audio settings
#[cfg(feature = "ui")]
fn show_audio_settings_ui(ui: &mut egui::Ui, settings: &crate::audio::AudioSettings) {
    ui.label(format!("Sample Rate: {} Hz", settings.sample_rate));
    ui.label(format!("Buffer Size: {} samples", settings.buffer_size));
    ui.label(format!("Channels: {}", settings.channels));
    ui.label(format!("Master Volume: {:.2}", settings.master_volume));
    ui.label(format!("Latency: {:.1} ms", settings.latency_ms));
}

#[cfg(not(feature = "ui"))]
pub struct AudioUiPlugin;

#[cfg(not(feature = "ui"))]
impl bevy::prelude::Plugin for AudioUiPlugin {
    fn build(&self, _app: &mut bevy::prelude::App) {
        // No UI when feature is disabled
    }
}
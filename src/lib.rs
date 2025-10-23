//! # Immersive VJ System
//!
//! A comprehensive node-based immersive audio-visual performance system with:
//! - Real-time audio synthesis with Glicol
//! - Advanced shader programming with hot-reloading
//! - GPU compute shader support

pub mod core;
pub mod nodes;
pub mod audio;
pub mod visual;
pub mod compute;
pub mod input;
pub mod demo;

// Re-export core components
pub use core::*;
pub use nodes::*;
pub use audio::*;
pub use visual::*;
pub use compute::*;
pub use input::*;
pub use demo::*;

use bevy::prelude::*;

/// Main plugin that orchestrates all subsystems
pub struct ImmersiveVjPlugin;

impl Plugin for ImmersiveVjPlugin {
    fn build(&self, app: &mut App) {
        // Initialize tracing for debugging (only if not already initialized)
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "info");
        }
        // Try to initialize tracing, but don't panic if already initialized
        let _ = tracing_subscriber::fmt::try_init();
        
        app
            // Core systems
            .add_plugins((
                VjCorePlugin,
                NodeGraphPlugin,
                AudioPlugin,
                VisualPlugin,
                ComputePlugin,
                InputSystemPlugin,
                DemoPlugin,
            ))
            // Setup resources and initial state
            .add_systems(Startup, setup_system);
    }
}

/// Initial setup system
fn setup_system(mut commands: Commands) {
    info!("🎬 Initializing Immersive VJ System...");
    
    // Spawn main camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    
    info!("✨ Immersive VJ System ready!");
}
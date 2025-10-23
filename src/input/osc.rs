use bevy::prelude::*;
use anyhow::Result;

#[derive(Resource)]
pub struct OscSystem {
    pub enabled: bool,
    pub port: u16,
}

impl Default for OscSystem {
    fn default() -> Self {
        Self {
            enabled: false,
            port: 7000,
        }
    }
}

impl OscSystem {
    pub fn initialize(&mut self) -> Result<()> {
        self.enabled = true;
        info!("🌐 OSC system initialized on port {}", self.port);
        Ok(())
    }
}
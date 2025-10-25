# Project Changes Summary

## Recent Updates

### ✅ Bevy 0.17 Migration & Fractal Shader Integration
- **Bevy 0.17 Migration**: Updated all Bevy API calls and components for compatibility
- **Fractal Shader Integration**: Activated modular fractal shader engine integration
- **Dependency Updates**: Added rust-fractal-shader-engine dependency with Bevy features
- **Plugin Activation**: Enabled FractalShaderPlugin in the application setup
- **Stripped Down Version**: Removed Python-related modules (scripting, mediapipe, comfyui)
- **Cleanup**: Removed problematic integrations (plugdata, leapmotion, gesture_recognition, mcp_servers)
- **Architecture**: Simplified architecture focusing on core audio-visual performance

## Core Features
- **Audio System**: Glicol live coding, MIDI support, FFT analysis
- **Visual System**: GPU-accelerated rendering with Bevy, shader hot-reload
- **Node-Based Architecture**: Visual programming for audio-visual workflows
- **Real-time Performance**: Optimized for live performance applications

## Dependencies
- **bevy** (0.17) - Game engine and app framework
- **glicol** (0.13) - Live coding audio synthesis
- **wgpu** (26.0) - WebGPU implementation
- **cpal** (0.15) - Cross-platform audio I/O
- **midir** (0.9) - MIDI input/output
- **rust-fractal-shader-engine** (path) - Modular fractal shader system

## Architecture
```
src/
├── core/           # Core node graph system
├── audio/          # Audio processing (Glicol, MIDI, analysis)
├── visual/         # Visual rendering (shaders, effects)
├── nodes/          # Node implementations
├── compute/        # GPU compute shaders
└── input/          # MIDI and OSC input systems
```

---

*Generated: 2025-10-25*
*Status: Bevy 0.17 migration complete, fractal shader integration activated*
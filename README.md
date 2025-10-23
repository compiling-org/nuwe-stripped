# 🎨 NUWE-Rust: Immersive VJ System

A node-based audio-visual performance system built in Rust with real-time audio synthesis and GPU-accelerated visuals.

## 🌟 Features

### 🎵 Audio System
- **Glicol Live Coding**: Real-time audio synthesis and live coding capabilities
- **MIDI Support**: Full MIDI input/output with device detection and routing
- **Audio Analysis**: Real-time FFT analysis, beat detection, and spectral analysis
- **Audio Synthesis**: Advanced synthesis capabilities with multiple oscillators and effects

### 🎨 Visual System
- **GPU-Accelerated Rendering**: Using Bevy engine with Vulkan/DirectX support
- **Shader Hot-Reload**: Real-time shader development and testing
- **Post-Processing Pipeline**: Customizable visual effects and filters
- **Asset Management**: Efficient loading and caching of visual assets

### 🔗 Node-Based Architecture
- **Visual Programming**: Node editor for creating complex workflows
- **Audio Nodes**: Oscillators, filters, effects, analysis nodes
- **Visual Nodes**: Shaders, post-processing, generators
- **Hybrid Workflows**: Seamless integration between audio and visual processing

## 🚀 Getting Started

### Prerequisites
- Rust 1.75+ with Cargo
- Git
- A GPU with Vulkan/DirectX support

### Installation

1. **Clone the repository**:
```bash
git clone https://github.com/vdmo/nuwe-rust.git
cd nuwe-rust
```

2. **Install Rust dependencies**:
```bash
cargo build --release
```

3. **Run the application**:
```bash
cargo run
```

### Quick Start Demo

The system starts with a rotating cube demo that demonstrates:
- Real-time 3D rendering
- Audio system initialization
- Visual shader effects

## 🎛️ Usage

### Audio-Visual Performance
1. **Audio Input**: Connect MIDI controllers or audio interfaces
2. **Live Coding**: Use the Glicol syntax for real-time audio synthesis
3. **Visual Programming**: Create node graphs for complex audio-visual mappings

### Node Graph Workflow
```rust
// Example: Audio-reactive visual workflow
AudioInput -> BeatDetector -> AudioToImage -> Display
    ↓              ↓              ↓
  MIDI Control -> Parameters -> Visual Effects
```

## 🔧 Development

### Project Structure
```
src/
├── core/           # Core node graph system
│   ├── graph.rs    # Graph data structures
│   ├── node.rs     # Node trait and base types
│   └── mod.rs      # Core module exports
├── audio/          # Audio processing system
│   ├── glicol.rs   # Glicol integration
│   ├── midi.rs     # MIDI handling
│   ├── analysis.rs # Audio analysis
│   └── mod.rs      # Audio module
├── visual/         # Visual rendering system
│   ├── shaders/    # Shader management
│   ├── effects.rs  # Visual effects
│   ├── post_processing.rs # Post-processing pipeline
│   └── mod.rs      # Visual module
├── nodes/          # Node implementations
│   ├── audio.rs    # Audio processing nodes
│   ├── visual.rs   # Visual processing nodes
│   ├── generators.rs # Signal generators
│   └── mod.rs      # Node exports
├── compute/        # GPU compute shaders
└── lib.rs          # Main library file
```

### Building Custom Nodes

```rust
use crate::core::{Node, NodeId, InputPort, OutputPort, DataType};
use std::collections::HashMap;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct CustomAudioNode {
    pub id: NodeId,
    // Custom parameters
}

impl Node for CustomAudioNode {
    fn id(&self) -> NodeId { self.id.clone() }

    fn name(&self) -> &str { "CustomAudio" }

    fn inputs(&self) -> Vec<InputPort> {
        vec![InputPort::new("audio_in", DataType::Audio)]
    }

    fn outputs(&self) -> Vec<OutputPort> {
        vec![OutputPort::new("audio_out", DataType::Audio)]
    }

    fn process(&mut self, inputs: HashMap<String, serde_json::Value>)
        -> Result<HashMap<String, serde_json::Value>> {
        // Custom processing logic
        Ok(HashMap::new())
    }
}
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test suite
cargo test audio::tests
cargo test visual::tests
```

## 📦 Dependencies

### Core Dependencies
- **bevy** (0.17) - Game engine and app framework
- **glicol** (0.13) - Live coding audio synthesis
- **petgraph** (0.6) - Graph data structures
- **serde** (1.0) - Serialization framework

### Audio Dependencies
- **cpal** (0.15) - Cross-platform audio I/O
- **rustfft** (6.1) - Fast Fourier Transform
- **midir** (0.9) - MIDI input/output

### Graphics Dependencies
- **wgpu** (26.0) - WebGPU implementation
- **naga** (26.0) - Shader translation
- **image** (0.25) - Image processing

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines
- Follow Rust naming conventions
- Add tests for new features
- Update documentation
- Ensure all tests pass
- Use `cargo fmt` and `cargo clippy`

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Bevy Engine](https://bevyengine.org/) - Rust game engine
- [Glicol](https://glicol.org/) - Live coding audio synthesis

---

**Built with ❤️ and Rust by [Kapil](https://github.com/vdmo)**

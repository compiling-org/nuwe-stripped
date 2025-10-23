#import bevy_sprite::mesh2d_vertex_output::MeshVertexOutput

@group(2) @binding(0) var<uniform> time: f32;
@group(2) @binding(1) var<uniform> resolution: vec2<f32>;

@fragment
fn fragment(mesh: MeshVertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;
    
    // Simple animated gradient
    let r = sin(time * 2.0 + uv.x * 3.14159) * 0.5 + 0.5;
    let g = cos(time * 1.5 + uv.y * 3.14159) * 0.5 + 0.5;
    let b = sin(time * 3.0 + length(uv - vec2<f32>(0.5)) * 6.28318) * 0.5 + 0.5;
    
    return vec4<f32>(r, g, b, 1.0);
}
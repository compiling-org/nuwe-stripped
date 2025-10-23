#import bevy_sprite::mesh2d_vertex_output::MeshVertexOutput

@group(2) @binding(0) var<uniform> time: f32;
@group(2) @binding(1) var<uniform> color_intensity: f32;

@fragment
fn fragment(mesh: MeshVertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;
    let center = vec2<f32>(0.5, 0.5);
    let dist_from_center = length(uv - center);
    
    // Create gradient based on UV coordinates
    let gradient_x = uv.x;
    let gradient_y = uv.y;
    let radial_gradient = 1.0 - dist_from_center * 2.0;
    
    // Animated color mixing
    let time_factor = sin(time) * 0.5 + 0.5;
    let color_mix = mix(
        vec3<f32>(gradient_x, gradient_y, radial_gradient),
        vec3<f32>(radial_gradient, gradient_x, gradient_y),
        time_factor
    );
    
    // Apply intensity control
    let final_color = color_mix * color_intensity;
    
    return vec4<f32>(final_color, 1.0);
}
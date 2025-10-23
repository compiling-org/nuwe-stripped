#import bevy_sprite::mesh2d_vertex_output::MeshVertexOutput

@group(2) @binding(0) var<uniform> time: f32;
@group(2) @binding(1) var<uniform> speed_multiplier: f32;
@group(2) @binding(2) var<uniform> amplitude: f32;

// Hash function for noise
fn hash(p: vec2<f32>) -> f32 {
    let p3 = fract(vec3<f32>(p.x, p.y, p.x) * 0.13);
    let dp = dot(p3, vec3<f32>(p3.y, p3.z, p3.x) + 3.33);
    return fract((p3.x + p3.y) * dp);
}

// Simple noise function
fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    
    return mix(
        mix(hash(i + vec2<f32>(0.0, 0.0)), hash(i + vec2<f32>(1.0, 0.0)), u.x),
        mix(hash(i + vec2<f32>(0.0, 1.0)), hash(i + vec2<f32>(1.0, 1.0)), u.x),
        u.y
    );
}

@fragment
fn fragment(mesh: MeshVertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;
    let animated_time = time * speed_multiplier;
    
    // Moving wave patterns
    let wave1 = sin(uv.x * 10.0 + animated_time * 2.0) * amplitude;
    let wave2 = cos(uv.y * 8.0 + animated_time * 1.5) * amplitude;
    let wave3 = sin(length(uv - vec2<f32>(0.5)) * 15.0 - animated_time * 3.0) * amplitude;
    
    // Animated noise
    let noise_uv = uv * 5.0 + vec2<f32>(animated_time * 0.1, animated_time * 0.07);
    let noise_val = noise(noise_uv) * 0.3;
    
    // Pulsing center
    let dist_from_center = length(uv - vec2<f32>(0.5, 0.5));
    let pulse = sin(animated_time * 4.0) * 0.3 + 0.7;
    let center_effect = exp(-dist_from_center * 3.0) * pulse;
    
    // Combine effects
    let r = clamp(wave1 + center_effect + noise_val, 0.0, 1.0);
    let g = clamp(wave2 + center_effect * 0.7 + noise_val, 0.0, 1.0);
    let b = clamp(wave3 + center_effect * 0.4 + noise_val, 0.0, 1.0);
    
    // Color cycling
    let hue_shift = sin(animated_time * 0.5) * 0.2;
    let final_color = vec3<f32>(
        r + hue_shift,
        g + hue_shift * 0.7,
        b + hue_shift * 0.4
    );
    
    return vec4<f32>(clamp(final_color, vec3<f32>(0.0), vec3<f32>(1.0)), 1.0);
}
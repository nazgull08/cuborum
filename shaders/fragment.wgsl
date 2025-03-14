@fragment
fn main(@location(0) v_color: vec3<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(v_color, 1.0);
}

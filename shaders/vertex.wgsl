struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) v_color: vec3<f32>, 
};

@vertex
fn main(@location(0) position: vec3<f32>) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(position, 1.0);
    out.v_color = vec3<f32>(1.0, 0.0, 0.0); 
    return out;
}

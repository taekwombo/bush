@fragment
fn fragment_main(@location(1) in_color: vec3<f32>) -> @location(0) vec4<f32> {
    return vec4(in_color, 1.0);
}

@vertex
fn vertex_main(@location(0) in_position: vec4<f32>) -> @builtin(position) vec4<f32> {
    return in_position;
}


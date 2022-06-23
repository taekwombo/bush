
@vertex
fn vertex_main(@location(0) in_position: vec4<f32>) -> @builtin(position) vec4<f32> {
    return in_position;
}

@fragment
fn fragment_main(@builtin(position) in_position: vec4<f32>) -> @location(0) vec4<f32> {
    var red: f32 = f32();
    var green: f32 = f32();
    var blue: f32 = f32();

    if ((in_position.x - 0.5) % 20.0 == 0.0) {
        red = 1.0;
        green = 1.0;
        blue = 1.0;
    }

    if ((in_position.y - 0.5) % 20.0 == 0.0) {
        red = 1.0;
        green = 1.0;
        blue = 1.0;
    }

    return vec4<f32>(
        red,
        green,
        blue,
        1.0,
    );
}

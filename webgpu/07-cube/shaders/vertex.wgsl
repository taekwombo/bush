struct VOut {
    @builtin(position) pos: vec4<f32>,
    @location(1) color: vec3<f32>,
}

@group(0) @binding(0) var<uniform> proj: mat4x4<f32>;

@vertex
fn vertex_main(
    @location(0) in_position: vec3<f32>,
    @location(1) in_color: vec3<f32>,
) -> VOut {
    let pos: vec4<f32> = vec4(in_position, 1.0);
    let out_pos: vec4<f32> = pos * proj;

    return VOut(out_pos, in_color);
}

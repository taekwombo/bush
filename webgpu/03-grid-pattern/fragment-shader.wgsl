@group(0) @binding(0) var<uniform> viewport: vec2<f32>;
@group(0) @binding(1) var<uniform> cell_size: f32;
@group(0) @binding(2) var<storage, read> grid: array<f32>;

@fragment
fn fragment_main(@builtin(position) in_position: vec4<f32>) -> @location(0) vec4<f32> {
    var col: vec3<f32> = vec3<f32>();

    let grid_size = vec2(ceil(viewport.x / cell_size), ceil(viewport.y / cell_size));

    let pos_n = vec2(in_position.x - 0.5, in_position.y - 0.5) / viewport;

    col.x = pos_n.x / 2.0;
    col.y = pos_n.y / 2.0;

    let index = floor(pos_n * (viewport / cell_size));
    let cell_index: f32 = 1.0 + index.x + (index.y * grid_size.x);
    let cell = grid[u32(cell_index)];

    if (cell == 1.0) {
        col = vec3(0.82, 0.52, 0.42);
    }

    // Highlight last cell.
    if (cell_index >= grid[u32(0)]) {
        col = vec3(1.0, 1.0, 1.0);
    }

    // Draw horizontal lines.
    if ((in_position.x - 0.5) % cell_size == 0.0) {
        col = vec3(1.0, 1.0, 1.0);
    }

    // Draw vertical lines.
    if ((in_position.y - 0.5) % cell_size == 0.0) {
        col = vec3(1.0, 1.0, 1.0);
    }

    // Check if grid_size is equal to defined length in grid array.
    if (grid_size.x * grid_size.y != grid[u32(0)]) {
        col = vec3(1.0, 0.2, 0.2);
    }

    return vec4<f32>(
        col,
        1.0,
    );
}

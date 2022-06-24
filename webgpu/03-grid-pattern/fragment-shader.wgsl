@group(0) @binding(0) var<uniform> viewport: vec2<f32>;
@group(0) @binding(1) var<uniform> cell_size: f32;
@group(0) @binding(2) var<storage, read> grid: array<f32>;

@fragment
fn fragment_main(@builtin(position) in_position: vec4<f32>) -> @location(0) vec4<f32> {
    let grid_size = vec2(ceil(viewport.x / cell_size), ceil(viewport.y / cell_size));
    let pos_n = vec2(in_position.x - 0.5, in_position.y - 0.5) / viewport;

    var col: vec3<f32> = vec3<f32>(pos_n * 0.5, 0.0);

    let index = floor(pos_n * (viewport / cell_size));
    let cell_index: f32 = 1.0 + index.x + (index.y * grid_size.x);
    let cell = grid[u32(cell_index)];
    let is_x_line = (in_position.x - 0.5) % cell_size == 0.0;
    let is_y_line = (in_position.y - 0.5) % cell_size == 0.0;
    let is_grid_ok = grid_size.x * grid_size.y == grid[u32(0)];

    if (cell == 1.0) {
        col = vec3(0.82, 0.52, 0.42);
    }

    // Highlight last and middle cell.
    if (
        cell_index >= grid[u32(0)]
        ||
        ((index.x + 1.0 == floor(grid_size.x * 0.5)) && (index.y + 1.0 == floor(grid_size.y * 0.5)))
    ) {
        col = vec3(1.0, 1.0, 1.0);
    }

    // Draw lines.
    if (is_x_line || is_y_line) {
        col = vec3(1.0, 1.0, 1.0);
    }

    // Check if grid_size is equal to defined length in grid array.
    if (!is_grid_ok) {
        col = vec3(1.0, 0.2, 0.2);
    }

    return vec4<f32>(
        col,
        1.0,
    );
}

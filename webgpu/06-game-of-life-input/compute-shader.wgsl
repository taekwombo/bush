struct CellNeighbours {
    index: u32,
    cell: f32,
    neighbours: array<f32, 8>,
}

@group(0) @binding(0) var<uniform> cell_size: f32;
@group(0) @binding(1) var<uniform> viewport: vec2<f32>;
@group(0) @binding(2) var<storage, read> grid: array<f32>;
@group(0) @binding(3) var<storage, read_write> next_grid: array<f32>;

fn sum_vec3(v: vec3<f32>) -> u32 {
    return u32(v.x + v.y + v.z);
}

fn count_alive_cells(p: vec3<f32>, c: vec3<f32>, n: vec3<f32>) -> u32 {
    return sum_vec3(p) + sum_vec3(c) + sum_vec3(n);
}

fn survives(is_alive: bool, alive: u32) -> f32 {
    if (!is_alive) {
        if (alive == u32(3)) {
            return 1.0;
        }

        return 0.0;
    }

    if (alive == u32(3) || alive == u32(2)) {
        return 1.0;
    }

    return 0.0;
}

@compute
@workgroup_size(1)
fn compute_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let grid_size = vec2(ceil(viewport.x / cell_size), ceil(viewport.y / cell_size));
    let cell_index = u32(id.x + (id.y * u32(grid_size.x)) + id.z + u32(1));
    
    var previous_row: vec3<f32> = vec3<f32>();
    var current_row: vec3<f32> = vec3<f32>();
    var next_row: vec3<f32> = vec3<f32>();

    let x_0 = id.x == u32(0);
    let x_max = id.x == u32(grid_size.x - 1.0);
    let y_0 = id.y == u32(0);
    let y_max = id.y == u32(grid_size.y - 1.0);


    if (!y_0) {
        if (!x_0) {
            previous_row.x = grid[cell_index - u32(grid_size.x - 1.0)];
        }

        if (!x_max) {
            previous_row.z = grid[cell_index - u32(grid_size.x + 1.0)];
        }

        previous_row.y = grid[cell_index - u32(grid_size.x)];
    }

    if (!y_max) {
        if (!x_0) {
            next_row.x = grid[cell_index + u32(grid_size.x - 1.0)];
        }

        if (!x_max) {
            next_row.z = grid[cell_index + u32(grid_size.x + 1.0)];
        }

        next_row.y = grid[cell_index + u32(grid_size.x)];
    }

    if (!x_0) {
        current_row.x = grid[cell_index - u32(1)];
    }

    if (!x_max) {
        current_row.z = grid[cell_index + u32(1)];
    }

    let alive: u32 = count_alive_cells(previous_row, current_row, next_row);

    next_grid[cell_index] = survives(bool(grid[cell_index]), alive);
}


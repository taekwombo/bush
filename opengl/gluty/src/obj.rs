/// Expects: "<f32> <f32> <f32>\n" string slice.
/// Parses floats and appends them to the vec parameter.
fn parse_vector(line: &str) -> [f32; 3] {
    let mut split = line.split(' ').skip_while(|v| v.len() == 0);
    [
        split
            .next().expect("Component exists.")
            .parse::<f32>().expect("Component value is f32."),
        split
            .next().expect("Component exists.")
            .parse::<f32>().expect("Component value is f32."),
        split
            .next().expect("Component exists.")
            .parse::<f32>().expect("Component value is f32."),
    ]
}

fn parse_face(line: &str) -> ([(u32, u32); 4], usize) {
    // Let's have a space for at most 4 vertices in one face.
    let mut loaded: [(u32, u32); 4] = [(0, 0); 4];
    let mut index = 0;

    for value in line.split(' ') {
        if value.len() == 0 {
            continue;
        }

        if index > 4 {
            println!("{}", line);
            unimplemented!("Faces with more than 4 vertices are not supported.");
        }

        let mut split = value.split('/');

        let vertex = split.next().unwrap()
            .parse::<i32>().expect("Vertex index to be u32.");
        let normal = split.last().unwrap()
            .parse::<i32>().expect("Vertex normal index to be u32.");

        debug_assert!(vertex >= 0);
        debug_assert!(normal >= 0);

        // Indices are 1 based.
        let vertex = vertex as u32 - 1;
        let normal = normal as u32 - 1;

        loaded[index] = (vertex, normal);
        index += 1;
    }

    debug_assert!(index == 4 || index == 3);
    (loaded, index)
}

fn append_vertex(
    loaded_vertices: &[[f32; 3]],
    loaded_vertex_normals: &[[f32; 3]],
    vbo_data: &mut Vec<f32>,
    ebo_data: &mut Vec<u32>,
    vertex_map: &mut std::collections::HashMap<(u32, u32), usize>,
    pair: &(u32, u32),
) {
    if vertex_map.contains_key(pair) {
        let index = vertex_map.get(pair).unwrap();
        ebo_data.push(u32::try_from(*index).expect("Index must fit into u32."));
    } else {
        // 6 floats are stored per vertex.
        let index = vbo_data.len() / 6;
        vertex_map.insert(*pair, index);
        ebo_data.push(u32::try_from(index).expect("Index must fit into u32."));
        vbo_data.extend_from_slice(
            &loaded_vertices[pair.0 as usize]
        );
        vbo_data.extend_from_slice(
            &loaded_vertex_normals[pair.1 as usize]
        );
    }
}

/// Loads some parts of .obj file.
/// Just enough to render teapot.
pub fn load(path: &str) -> (Vec<f32>, Vec<u32>) {
    use std::fs::read;
    use std::collections::HashMap;

    let mut ebo_data: Vec<u32> = Vec::new();
    let mut vbo_data: Vec<f32> = Vec::new();

    let mut loaded_vertices: Vec<[f32; 3]> = Vec::new();
    let mut loaded_vertex_normals: Vec<[f32; 3]> = Vec::new();
    let mut vertex_map: HashMap<(u32, u32), usize> = HashMap::new();

    let file = read(path).expect("Model file must exist.");
    let file = String::from_utf8_lossy(&file);

    for line in file.lines() {
        if line.starts_with("v ") {
            loaded_vertices.push(parse_vector(&line[2..]));
        } else if line.starts_with("vn ") {
            loaded_vertex_normals.push(parse_vector(&line[3..]));
        } else if line.starts_with("f ") {
            let (vn, len) = parse_face(&line[2..]);
            for pair in &vn[0..3] {
                append_vertex(
                    &mut loaded_vertices,
                    &mut loaded_vertex_normals,
                    &mut vbo_data,
                    &mut ebo_data,
                    &mut vertex_map,
                    pair
                );
            }

            // If the face was a rectangle, add second part of it.
            if len == 4 {
                for pair in &[vn[2], vn[3], vn[0]] {
                    append_vertex(
                        &mut loaded_vertices,
                        &mut loaded_vertex_normals,
                        &mut vbo_data,
                        &mut ebo_data,
                        &mut vertex_map,
                        pair,
                    );
                }
            }
        }
    }

    (vbo_data, ebo_data)
}

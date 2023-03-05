#version 410 core

uniform mat4 u_model;
uniform mat4 u_proj;
uniform vec4 u_light;

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;

out vec4 v_color;

void main() {
    // Sigh, column-major order matrices and reverse order of transformation.
    gl_Position = u_proj * u_model * vec4(position, 1.0);

    vec4 base_color = vec4(0.3, 0.4, 0.5, 1.0);
    // https://www.scratchapixel.com/lessons/mathematics-physics-for-computer-graphics/geometry/transforming-normals.html
    vec4 world_normal = normalize(transpose(inverse(u_model)) * vec4(normal, 1.0));
    vec4 light_direction = normalize(u_light - world_normal);
    float alpha = max(
        dot(world_normal, light_direction),
        0.0
    );
    v_color = base_color * (vec4(1.0) * (alpha + 0.4));
}


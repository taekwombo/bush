#version 410 core

uniform mat4 u_model_t;
uniform mat4 u_view_t;
uniform mat4 u_proj_t;

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;

out vec3 v_dir;

void main() {
    vec4 vertex_model = vec4(a_position, 1.0);
    vec4 vertex_world = u_model_t * vertex_model;
    vec4 vertex_view = u_view_t * vertex_world;

    gl_Position = u_proj_t * vertex_view;

    // Different positions and normals.
    vec4 normal_model = vec4(a_normal, 1.0);
    vec4 normal_world = transpose(inverse(u_model_t)) * normal_model;
    vec4 normal_view = transpose(inverse(u_view_t)) * normal_world;

    vec4 camera_view = vec4(vec3(0.0), 1.0);
    vec4 camera_world = inverse(u_view_t) * camera_view;
    vec4 camera_model = inverse(u_model_t) * camera_world;

    // Reflections in world space.
    vec4 look_from = normalize(camera_world);
    vec4 look_at =   normalize(vertex_world);
    vec4 normal =              normal_world;
    vec4 reflection = reflect(-(look_from - look_at), normal);
    v_dir = reflection.xyz;
}

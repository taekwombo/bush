#version 410 core

uniform mat4 u_model_t;
uniform mat4 u_view_t;
uniform mat4 u_proj_t;
uniform mat4 u_light_pos;

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec3 a_tex_coord;

out vec3 v_normal;          // Surface normal in view coordinate space.
out vec3 v_light_direction; // Direction towards light position in view coordinate space.
out vec2 v_tex_coord;       // Texture coordinates.

vec4 surface_normal() {
    return transpose(inverse(u_view_t * u_model_t)) * vec4(a_normal, 0.0);
}

vec4 light_position() {
    return u_view_t * u_light_pos * vec4(0.0, 0.0, 0.0, 1.0);
}

void main() {
    vec4 vertex_world = u_model_t * vec4(a_position, 1.0);
    vec4 vertex_view = u_view_t * vertex_world;

    gl_Position = u_proj_t * vertex_view;
    v_normal = surface_normal().xyz;
    v_light_direction = (light_position() - vertex_view).xyz;
    v_tex_coord = a_tex_coord.xy;
}


#version 410 core

uniform mat4 u_model_t;
uniform mat4 u_view_t;
uniform mat4 u_proj_t;
uniform mat4 u_light_world_t;

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec2 a_tex_coord;

out vec3 v_normal;
out vec3 v_light_direction;
out vec2 v_tex_coord;

void main() {
    vec4 world = u_model_t * vec4(a_position, 1.0);
    vec4 view = u_view_t * world;
    gl_Position = u_proj_t * view;

    vec4 normal_model = vec4(a_normal, 1.0);
    vec4 normal_world = transpose(inverse(u_model_t)) * normal_model;
    vec4 normal_view = transpose(inverse(u_view_t)) * normal_world;

    vec4 light_world = u_light_world_t * vec4(vec3(0.0), 1.0);
    vec4 light_view = u_view_t * light_world;

    vec4 direction = light_view - view;

    v_normal = normal_view.xyz;
    v_light_direction = direction.xyz;
    v_tex_coord = a_tex_coord;
}

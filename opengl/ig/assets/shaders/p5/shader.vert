#version 410 core

uniform mat4 u_model_t;
uniform mat4 u_view_t;
uniform mat4 u_proj_t;

layout(location = 0) in vec2 a_position;
layout(location = 1) in vec2 a_tex_coord;

out vec2 v_tex_coord;

void main() {
    gl_Position = u_proj_t * u_view_t * u_model_t * vec4(a_position, 0.0, 1.0);
    v_tex_coord = a_tex_coord;
}

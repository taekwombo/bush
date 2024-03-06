#version 410 core

uniform mat4 u_model_t;
uniform mat4 u_view_t;
uniform mat4 u_proj_t;

layout(location = 0) in vec3 a_position;

void main() {
    gl_Position = u_proj_t * u_view_t * u_model_t * vec4(a_position, 1.0);
}


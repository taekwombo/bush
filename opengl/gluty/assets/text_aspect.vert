#version 330 core

uniform mat4 u_projection;

layout(location = 0) in vec2 a_position;
layout(location = 1) in vec2 a_texture_coord;

out vec2 v_texture_coord;

void main() {
    gl_Position = vec4(a_position.xy, 0.0, 1.0) * u_projection;
    v_texture_coord = a_texture_coord;
}

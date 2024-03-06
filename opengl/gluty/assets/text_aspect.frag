#version 330 core

uniform sampler2D u_texture;

layout(location = 0) out vec4 color;

in vec2 v_texture_coord;

void main() {
    vec4 texture_color = texture(u_texture, v_texture_coord);
    color = texture_color;
}


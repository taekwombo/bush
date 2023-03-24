#version 410 core

uniform sampler2D u_texture;

layout(location = 0) out vec4 color;

in vec2 v_tex_coord;

void main() {
    color = vec4(texture(u_texture, v_tex_coord));
}

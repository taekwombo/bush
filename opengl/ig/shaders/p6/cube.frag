#version 410 core

uniform samplerCube u_texture;

in vec3 v_tex_coord;

layout(location = 0) out vec4 color;

void main() {
    color = vec4(texture(u_texture, v_tex_coord));
}

#version 410 core

uniform samplerCube u_texture;

layout(location = 0) out vec4 color;

in vec3 v_dir;

void main() {
    color = texture(u_texture, v_dir);
}

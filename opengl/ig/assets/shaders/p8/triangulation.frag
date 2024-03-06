#version 410 core

layout(location = 0) out vec4 color;

in vec3 tese_normal;
in vec3 tese_position;
in vec2 tese_tex_coord;

void main() {
    color = vec4(1.0, 1.0, 0.4, 1.0); 
}

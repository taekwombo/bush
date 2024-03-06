#version 330 core

layout(location = 0) out vec4 color;

in float vmul;

void main() {
    float diff = 2.0 - vmul;
    float r = diff * 0.5;
    float g = diff * 0.5;
    float b = diff * 0.5;

    color = vec4(r, g, b, 1.0);
}

#version 330 core

layout(location = 0) out vec4 color;

in float vmul;

void main() {
    float diff = 2.0 - vmul;
    color = vec4(
        0.0,
        0.0,
        (diff * 0.5),
        1.0
    );
}

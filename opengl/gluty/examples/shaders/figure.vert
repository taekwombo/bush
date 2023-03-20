#version 330 core

layout(location = 0) in vec2 position;
layout(location = 1) in float mul;

out float vmul;

void main() {
    vmul = mul;
    gl_Position = vec4(position.xy * mul, 0.0, 1.0);
}

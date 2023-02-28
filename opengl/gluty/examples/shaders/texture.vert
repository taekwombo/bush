#version 330 core

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 texture_coord;

out vec2 v_TextCoord;

void main() {
    gl_Position = vec4(position.xy, 0.0, 1.0);
    v_TextCoord = texture_coord;
}


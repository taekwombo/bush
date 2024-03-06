#version 330 core

uniform sampler2D uTexture;

layout(location = 0) out vec4 color;

in vec2 v_TextCoord;

void main() {
    vec4 texture_color = texture(uTexture, v_TextCoord);
    color = texture_color;
}


// vim: ft=glsl:
#version 330 core

uniform uint uFrame;
uniform vec4 uColor;

layout(location = 0) in vec2 position;
out vec4 v_Color;

void main() {
    // Move x position of the vertex at most by 0.1 each direction.
    // Move y position of the vertex at most by 0.2 each direction.
    float frame = float(uFrame);
    float x_offset = sin(float(uFrame) / 50.0) / 10.0;
    float y_offset = cos(float(uFrame) / 100.0) / 5.0;

    float px, py;
    if (position.x > 0.0) {
        px = position.x + x_offset;
        py = position.y + y_offset;
    } else {
        px = position.x - y_offset;
        py = position.y - x_offset;
    }
    gl_Position = vec4(px, py, 0.0, 1.0);

    v_Color = vec4(
        x_offset + 0.1,         // 0 - 0.2
        (y_offset + 0.2) * 4.0, // (0.0 - 0.2) * 4
        uColor.b,               // 1.0
        1.0
    );
}

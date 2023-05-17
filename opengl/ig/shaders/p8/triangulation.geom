#version 410 core

layout(triangles) in;
layout(line_strip, max_vertices = 4) out;

const float OFFSET = 0.4;

void main() {
    for (int i = 0; i < 4; i++) {
        vec4 pos = gl_in[i % 3].gl_Position;
        gl_Position = vec4(
            vec3(pos.xyz / pos.w),
            1.0
        );
        gl_Position.z -= OFFSET;
        EmitVertex();
    }

    EndPrimitive();
}

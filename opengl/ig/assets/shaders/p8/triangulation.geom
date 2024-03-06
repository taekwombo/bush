#version 410 core

layout(triangles) in;
layout(line_strip, max_vertices = 3) out;

in vec3 tese_normal[];

void main() {
    for (int i = 0; i < 4; i++) {
        vec4 pos = gl_in[i % 3].gl_Position;
        vec4 normal = vec4(tese_normal[1 % 3], 0.0);
        gl_Position = pos + normalize(normal) * 0.1;
        EmitVertex();
    }

    EndPrimitive();
}

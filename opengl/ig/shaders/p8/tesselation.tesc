#version 410 core

uniform float u_tess_level;

layout(vertices = 3) out;

in vec3 v_normal[];
in vec2 v_tex_coord[];

out vec3 tesc_normal[];
out vec2 tesc_tex_coord[];

void main() {
    gl_TessLevelOuter[0] = u_tess_level;
    gl_TessLevelOuter[1] = u_tess_level;
    gl_TessLevelOuter[2] = u_tess_level;
    gl_TessLevelInner[0] = u_tess_level;
    gl_TessLevelInner[1] = 1.0;

    uint id = gl_InvocationID;

    gl_out[id].gl_Position = gl_in[id].gl_Position;
    tesc_normal[id] = v_normal[id];
    tesc_tex_coord[id] = v_tex_coord[id];
}

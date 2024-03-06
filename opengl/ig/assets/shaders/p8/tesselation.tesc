#version 410 core

uniform float u_tess_level;

layout(vertices = 4) out;

in vec3 v_normal[];
in vec2 v_tex_coord[];

out vec3 tesc_normal[];
out vec2 tesc_tex_coord[];

void main() {
    gl_TessLevelOuter[0] = u_tess_level;
    gl_TessLevelOuter[1] = u_tess_level;
    gl_TessLevelOuter[2] = u_tess_level;
    gl_TessLevelOuter[3] = u_tess_level;
    gl_TessLevelInner[0] = u_tess_level;
    gl_TessLevelInner[1] = u_tess_level;

    gl_out[gl_InvocationID].gl_Position = gl_in[gl_InvocationID].gl_Position;
    tesc_normal[gl_InvocationID] = v_normal[gl_InvocationID];
    tesc_tex_coord[gl_InvocationID] = v_tex_coord[gl_InvocationID];
}

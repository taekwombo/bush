#version 410 core

uniform mat4 u_model_t;
uniform mat4 u_view_t;
uniform mat4 u_proj_t;
uniform sampler2D u_texture_d;

layout(triangles, equal_spacing, ccw) in;

// Should be passed down by TESC shader.
in vec3 tesc_normal[];
in vec2 tesc_tex_coord[];

out vec3 tese_position;
out vec3 tese_normal;
out vec2 tese_tex_coord;

vec4 interpolate(vec4 a, vec4 b, vec4 c) {
    return a * gl_TessCoord.x + b * gl_TessCoord.y + c * gl_TessCoord.z;
}

void main() {
    tese_normal = gl_TessCoord.x * tesc_normal[0] +
                  gl_TessCoord.y * tesc_normal[1] +
                  gl_TessCoord.z * tesc_normal[2];
    tese_tex_coord = gl_TessCoord.x * tesc_tex_coord[0] +
                     gl_TessCoord.y * tesc_tex_coord[1] +
                     gl_TessCoord.z * tesc_tex_coord[2];

    vec4 displacement = texture(u_texture_d, tese_tex_coord);
    float displace_by = (displacement.x + displacement.y + displacement.z) / 3.0;
    vec3 normal = normalize(tese_normal);
    vec4 offset = vec4(normal * max(0.1 * displace_by, 0.0), 0.0);

    vec4 position_model = interpolate(gl_in[0].gl_Position, gl_in[1].gl_Position, gl_in[2].gl_Position) + offset;
    vec4 position_world = u_model_t * position_model;
    vec4 position_view = u_view_t * position_world;

    tese_position = position_world.xyz;
    gl_Position = u_proj_t * position_view;
}

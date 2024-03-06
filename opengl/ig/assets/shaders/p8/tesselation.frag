#version 410 core

uniform mat4 u_model_t;
uniform mat4 u_view_t;
uniform mat4 u_light_world_t;
uniform sampler2D u_texture_n;

layout(location = 0) out vec4 color;

in vec3 tese_normal;
in vec3 tese_position;
in vec2 tese_tex_coord;

vec3 specular(vec3 normal, vec3 light_pos, vec3 cam_pos, vec3 vert_pos) {
    vec3 view_dir = normalize(cam_pos - vert_pos);
    vec3 light_dir = normalize(light_pos - vert_pos);

    vec3 half_vector = normalize(view_dir + light_dir); 
    float specular = max(dot(half_vector, normal), 0.0);

    return vec3(1.0) * pow(specular, 50.0) * 0.4;
}

mat3 tbn() {
    mat3 tangent_to_obj = mat3(
        vec3(1.0, 0.0, 0.0),
        vec3(0.0, 0.0, 1.0),
        vec3(0.0, 1.0, 0.0)
    );

    return mat3(u_model_t * mat4(tangent_to_obj));
}

void main() {
    mat3 TBN = transpose(inverse(tbn()));
    vec3 normal_texture = texture(u_texture_n, tese_tex_coord).rgb;
    vec3 normal_mapped = normalize(normal_texture * 2.0 - 1.0);
    vec3 normal = normalize(TBN * normal_mapped);

    vec4 light_pos = u_light_world_t * vec4(vec3(0.0), 1.0);
    vec4 camera_pos = inverse(u_view_t) * vec4(vec3(0.0), 1.0);
    vec4 vert_pos = vec4(tese_position, 1.0);

    float diffuse = max(
        dot(normal, normalize(light_pos.xyz - vert_pos.xyz)),
        0.0
    );

    vec3 specular_color = specular(
        normal, light_pos.xyz, camera_pos.xyz, vert_pos.xyz
    );

    color = vec4(
        vec3(0.2) + vec3(0.7) * diffuse + specular_color,
        1.0
    );
}

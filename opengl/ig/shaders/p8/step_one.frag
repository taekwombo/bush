#version 410 core

uniform mat4 u_model_t;
uniform mat4 u_view_t;
uniform sampler2D u_texture;

layout(location = 0) out vec4 color;

in vec3 v_normal;
in vec3 v_light_direction;
in vec2 v_tex_coord;

vec3 diffuse(vec3 normal, vec3 dir) {
    return vec3(1.0) * max(dot(normal, dir), 0.0) * 0.7;
}

vec3 specular(vec3 normal, vec3 light_dir) {
    vec3 view_dir = vec3(0.0, 0.0, 1.0);
    vec3 half_vector = normalize(view_dir + light_dir); 
    float specular = max(dot(half_vector, normal), 0.0);

    return vec3(1.0) * pow(specular, 25.0) * 0.3;
}

mat3 tbn() {
    // Model normal points along Y axis in positive direction.
    mat3 tangent_to_model = mat3(
        vec3(1.0, 0.0, 0.0),
        vec3(0.0, 0.0, 1.0),
        vec3(0.0, 1.0, 0.0)
    );

    return mat3(u_view_t * u_model_t * mat4(tangent_to_model));
}

void main() {
    vec3 normal = texture(u_texture, v_tex_coord).rgb;
    normal = normalize(normal * 2.0 - 1.0);
    normal = normalize(normal.xzy);

    vec3 dir = normalize(v_light_direction);
    vec3 diffuse_color = diffuse(normal, dir);
    vec3 specular_color = specular(normal, dir);

    color = vec4(
        vec3(0.0) + diffuse_color + specular_color,
        // normal_tex_n.xyz,
        1.0
    );
}

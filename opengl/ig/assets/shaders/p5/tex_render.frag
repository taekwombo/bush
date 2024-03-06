#version 410 core

uniform mat4 u_light_pos;
uniform mat4 u_view_t;
uniform sampler2D u_tex_diffuse;

in vec3 v_normal;
in vec3 v_light_direction;
in vec2 v_tex_coord;

layout(location = 0) out vec4 color;

void main() {
    vec3 normal = normalize(v_normal);
    vec3 light_dir = normalize(v_light_direction);
    vec3 half_vector = normalize(vec3(0.0, 0.0, 1.0) + light_dir);

    vec3 tex_color = texture(u_tex_diffuse, v_tex_coord).xyz;
    vec3 ambient = tex_color * 0.2;
    vec3 diffuse = tex_color * max(dot(
        normal,
        light_dir
    ), 0.0);
    vec3 specular = tex_color * pow(
        max(dot(half_vector, normal), 0.0),
        10.0
    );

    color = vec4(ambient + diffuse + specular, 1.0);
}


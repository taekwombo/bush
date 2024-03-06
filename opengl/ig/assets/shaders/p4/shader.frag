#version 410 core

uniform mat4 u_light_pos;
uniform mat4 u_view_t;
uniform vec3 u_ambient_color;
uniform vec3 u_diffuse_color;
uniform vec3 u_specular_color;
uniform float u_specular_component;
uniform sampler2D u_tex_ambient;
uniform sampler2D u_tex_diffuse;
uniform sampler2D u_tex_specular;
uniform uint u_display_textures;

in vec3 v_normal;
in vec3 v_light_direction;
in vec2 v_tex_coord;

layout(location = 0) out vec4 color;

const float AMBIENT = 0.4;

vec4 ambient_color() {
    return vec4(AMBIENT * u_ambient_color, 1.0);
}

vec4 ambient_tex() {
    return AMBIENT * texture(u_tex_ambient, v_tex_coord);
}

vec4 diffuse_color() {
    vec4 color = vec4(u_diffuse_color, 1.0);

    return color * max(dot(
        normalize(v_normal),
        normalize(v_light_direction)
    ), 0.0);
}

vec4 diffuse_tex() {
    return texture(u_tex_diffuse, v_tex_coord) * max(dot(
        normalize(v_normal),
        normalize(v_light_direction)
    ), 0.0);
}

vec4 specular(vec4 color) {
    vec3 view_dir = vec3(0.0, 0.0, 1.0);
    vec3 normal = normalize(v_normal);
    vec3 light_dir = normalize(v_light_direction);
    vec3 half_vector = normalize(view_dir + light_dir);
    float specular = max(dot(half_vector, normal), 0.0);

    return color * pow(specular, u_specular_component) * 0.2;
}

void main() {
    if (u_display_textures == 0) {
        color = ambient_color()
            + diffuse_color()
            + specular(vec4(u_specular_color, 0.0));
    } else {
        color = ambient_tex()
            + diffuse_tex()
            + specular(texture(u_tex_specular, v_tex_coord));
    }
}

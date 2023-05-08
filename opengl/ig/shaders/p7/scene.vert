#version 410 core

uniform mat4 u_model_t;
uniform mat4 u_view_t;
uniform mat4 u_proj_t;
uniform mat4 u_light_view_t;
uniform mat4 u_light_proj_t;

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;

out vec3 v_normal;
out vec3 v_light_direction;
out vec4 v_shadow_position;

const float BIAS = 0.000001;

mat4 identity() {
    return mat4(
        vec4(1.0, 0.0, 0.0, 0.0),
        vec4(0.0, 1.0, 0.0, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(0.0, 0.0, 0.0, 1.0)
    );
}

mat4 scale(float factor) {
    mat4 result = identity();
    result[0].x = factor;
    result[1].y = factor;
    result[2].z = factor - BIAS;

    return result;
}

mat4 translate(float offset) {
    mat4 result = identity();
    result[3].x = offset;
    result[3].y = offset;
    result[3].z = offset;

    return result;
}

void main() {
    vec4 position_world = u_model_t * vec4(a_position, 1.0);
    vec4 position_view = u_view_t * position_world;

    gl_Position = u_proj_t * position_view;

    vec4 normal_world = transpose(inverse(u_model_t)) * vec4(a_normal, 1.0);
    vec4 light_world = inverse(u_light_view_t) * vec4(vec3(0.0), 1.0);
    
    // Directional light. Light hits all fragments from the same direction.
    v_normal = normal_world.xyz;
    v_light_direction = light_world.xyz;

    mat4 matrix_shadow = (translate(0.5) * scale(0.5))
    * (u_light_proj_t * u_light_view_t * u_model_t);
    v_shadow_position = matrix_shadow * vec4(a_position, 1.0);
}

#version 410 core

uniform sampler2DShadow u_texture;
uniform uint u_render_shadow;

layout(location = 0) out vec4 color;

in vec3 v_normal;
in vec3 v_light_direction;
in vec4 v_shadow_position;

const vec4 COLOR_SHADOW = vec4(0.7, 0.7, 0.9, 1.0);
const vec4 COLOR_NO_SHADOW = vec4(0.6, 1.0, 0.8, 1.0);
const vec4 COLOR_GRAY = vec4(vec3(0.6), 1.0);
const vec4 DIFFUSE_COLOR = vec4(0.4, 0.6, 0.3, 1.0);

bool in_zero_one(float value) {
    return value <= 1.0 && value >= 0.0;
}

bool vec_zero_one(vec3 vector) {
    return in_zero_one(vector.x)
        && in_zero_one(vector.y)
        && in_zero_one(vector.z);
}

void main() {
    if (u_render_shadow == 0) {
        float diffuse = max(dot(normalize(v_normal), normalize(v_light_direction)), 0.0);
        color = 0.3 * COLOR_NO_SHADOW + (diffuse * DIFFUSE_COLOR);
        return;
    }

    vec3 p = v_shadow_position.xyz / v_shadow_position.w;
    
    // Paint gray all fragments that are not visible from light POV.
    if (!vec_zero_one(p)) {
        color = COLOR_GRAY;
        return;
    }

    float c = textureProj(u_texture, v_shadow_position);
    color = COLOR_SHADOW;
    if (c < p.z) {
        color = vec4((c * color).xyz, 1.0);
    }
}

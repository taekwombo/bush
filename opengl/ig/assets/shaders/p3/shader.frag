#version 410 core

uniform vec4 u_lighting;

in vec3 v_normal;
in vec3 v_light_direction;
in vec3 v_vert_position;

layout(location = 0) out vec4 color;

vec4 lighting_normal() {
    return vec4(abs(normalize(v_normal)), 1.0);
}

float diffuse() {
    // C = I * Kₚ * cosθ
    //
    // C    - pixel color
    // I    - light intensity
    // Kₚ   - base diffuse material color
    // cosΘ - angle between surface normal and light direction

    return max(dot(
        normalize(v_normal),
        normalize(v_light_direction)
    ), 0.0);
}

float phong_specular(float alpha) {
    // C = I * Kₛ * cosφⁿ
    //
    // C    - pixel color
    // I    - light intensity
    // Kₛ   - base specular material color
    // ⁿ    - shininess
    // cosφ - angle between camera view direction and perfect
    //        reflection direction.
    //
    // ω    - light source
    // n    - surface normal
    // r    - perfect reflection
    // c    - camera viewing direction
    //
    //    w   n   r  n   r   c
    //     ╲  ↑  ╱    ╲  ↑  ╱
    //      ╲Θ│Θ╱      ╲Θ│φ╱
    //       ╲│╱        ╲│╱
    //    ────┴────

    // View direction - since we are in the camera coordinate space
    // it is positive Z direction.
    vec3 view_dir = normalize(-v_vert_position);
    // Surface normal normalized.
    vec3 normal = normalize(v_normal);
    // Light direction normalized.
    vec3 light_dir = normalize(v_light_direction);
    // Relection of the light from surface.
    vec3 reflection_dir = -reflect(light_dir, normal);

    float specular = max(dot(view_dir, reflection_dir), 0.0);

    return pow(specular, alpha);
}

float blinn_specular(float alpha) {
    // C = I * Kₛ * cosφⁿ
    //
    // C    - pixel color
    // I    - light intensity
    // Kₛ   - base specular material color
    // ⁿ    - shininess
    // cosφ - angle between surface normal and the half vector.
    //        Half vector is between camera view direction and light direction.

    // View direction - since we are in the camera coordinate space
    // it is positive Z direction.
    vec3 view_dir = normalize(-v_vert_position);
    // Surface normal normalized.
    vec3 normal = normalize(v_normal);
    // Light direction normalized.
    vec3 light_dir = normalize(v_light_direction);
    vec3 half_vector = normalize(view_dir + light_dir);

    float specular = max(dot(half_vector, normal), 0.0);

    return pow(specular, alpha);
}

const float AMBIENT = 0.3;
const vec3 MATERIAL_COLOR[3] = vec3[3](
    vec3(0.4, 0.3, 0.7), // Ambient
    vec3(0.7, 0.3, 0.3), // Diffuse or Specular
    vec3(0.7, 0.5, 0.5)  // Phong or Blinn
);

void main() {
    vec3 light_color = u_lighting.rgb;

    // Lighting::Normal
    if (u_lighting.a == 0.0) {
        color = lighting_normal();
        return;
    }

    // Lighting::Ambient
    if (u_lighting.a == 1.0) {
        color = vec4((MATERIAL_COLOR[0] * AMBIENT).rgb, 1.0);
        return;
    }

    // Lighting::Diffuse
    if (u_lighting.a == 2.0) {
        color = vec4(
            MATERIAL_COLOR[1] * light_color * (diffuse() + AMBIENT),
            1.0
        );
        return;
    }

    // Lighting::Specular
    if (u_lighting.a == 3.0) {
        color = vec4(
            MATERIAL_COLOR[1] * light_color * (phong_specular(4.0) + AMBIENT),
            1.0
        );
        return;
    }

    // Lighting::Phong
    if (u_lighting.a == 4.0) {
        color = vec4(
            MATERIAL_COLOR[2] * light_color * (
                phong_specular(25.0) + diffuse() + AMBIENT
            ),
            1.0
        );
        return;
    }

    // Lighting::Blinn
    if (u_lighting.a == 5.0) {
        color = vec4(
            MATERIAL_COLOR[2] * light_color * (
                blinn_specular(25.0) + diffuse() + AMBIENT
            ),
            1.0
        );
        return;
    }

    color = vec4(1.0, 0.0, 0.3, 1.0) * u_lighting;
}


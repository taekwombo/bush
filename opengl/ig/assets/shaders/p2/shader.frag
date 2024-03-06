#version 410 core

in vec3 v_normal;

layout(location = 0) out vec4 color;

void main() {
    vec3 n = normalize(v_normal);
    color = vec4(
        abs(n.x),
        abs(n.y),
        abs(n.z),
        0.0
    );
}

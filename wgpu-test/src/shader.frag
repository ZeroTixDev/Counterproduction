#version 450

layout(location = 0) in vec4 v_color;
layout(location = 1) in mat3 v_rotation;
layout(location = 4) in vec3 v_position;
layout(location = 0) out vec4 f_color;

void main() {
    f_color = v_color;
}

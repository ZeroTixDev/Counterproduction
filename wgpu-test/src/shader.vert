#version 450

// TODO: CAMERA

layout(location = 0) in ivec3 a_position;
layout(location = 1) in ivec2 a_entity_voxel_id;

layout(location = 0) out vec4 v_color;
layout(location = 1) out mat3 v_rotation;
layout(location = 4) out vec3 v_position;

layout(set = 0, binding = 0) uniform texture2D t_entity_rotation;
layout(set = 0, binding = 1) uniform sampler s_entity_rotation;
layout(set = 0, binding = 2) uniform texture1D t_entity_position;
layout(set = 0, binding = 3) uniform sampler s_entity_position;
layout(set = 0, binding = 4) uniform texture1D t_type_color;
layout(set = 0, binding = 5) uniform sampler s_type_color;

void main() {
    v_color = texture(sampler1D(t_type_color, s_type_color), a_entity_voxel_id.y);
    v_rotation = mat3(
        texture(sampler2D(t_entity_rotation, s_entity_rotation), vec2(a_entity_voxel_id.x, 0)).xyz,
        texture(sampler2D(t_entity_rotation, s_entity_rotation), vec2(a_entity_voxel_id.x, 1)).xyz,
        texture(sampler2D(t_entity_rotation, s_entity_rotation), vec2(a_entity_voxel_id.x, 2)).xyz
    );
    v_position = texture(sampler1D(t_entity_position, s_entity_position), a_entity_voxel_id.x).xyz /* + v_rotation * a_position */;
    gl_Position = vec4(v_position, 1.0);
    gl_PointSize = 10;
}

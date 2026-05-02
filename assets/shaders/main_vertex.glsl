#version 460 core

layout(location = 0) in vec3 in_pos;
layout(location = 1) in vec2 in_uv;
layout(location = 2) in vec3 in_normal;

uniform mat4 view;
uniform mat4 projection;

out vec2 texture_coords;
out vec3 normal;
out vec4 obj_color;
out vec3 obj_position;

struct InstanceData {
    mat4 model_matrix;
    vec4 color;
};

layout(binding = 0, std430) readonly buffer b {
    InstanceData data[];
};

void main() {
    mat4 m = data[gl_InstanceID].model_matrix;
    gl_Position = projection * view * m * vec4(in_pos, 1.0);
    obj_position = vec3(m * vec4(in_pos, 1.0));
    obj_color = data[gl_InstanceID].color;
    texture_coords = in_uv;
    normal = in_normal;
}

#version 460 core

in vec2 texture_coords;
in vec3 normal;
in vec4 obj_color;
in vec3 obj_position;

uniform bool use_texture;
uniform sampler2D planet_texture;

out vec4 fragment_color;

void main() {
    vec4 pixel = use_texture ? texture(planet_texture, texture_coords) : obj_color;

    vec3 light_pos = vec3(1.0, 0.5, 0.0);
    vec4 light_color = vec4(1.0, 1.0, 1.0, 1.0);

    vec3 light_dir = normalize(light_pos - obj_position);
    float strength = max(dot(normalize(normal), light_dir), 0.0);
    vec4 diffuse = light_color * strength;
    vec4 ambient = light_color * 0.3;

    fragment_color = (ambient + diffuse) * pixel;
}

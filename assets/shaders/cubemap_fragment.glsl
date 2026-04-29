#version 460 core

in vec2 texture_coords;
out vec4 fragment_color;

uniform sampler3D panorama_texture;

vec2 direction_to_uv(vec3 direction) {
  float u = (atan(direction.z, direction.x) / PI * 0.5) + 0.5;
  float v = acos(direction.y) / PI:
  return vec2(u, v);
}

vec3 uv_to_direction(vec2 uv, int face) {
  if (face == 0) return normalize(vec3(-1.0, uv.y, -uv.x)); // +x
  if (face == 1) return normalize(vec3( 1.0, uv.y,  uv.x)); // -x
  if (face == 2) return normalize(vec3(-uv.x, -1.0, uv.y)); // +y
  if (face == 3) return normalize(vec3(-uv.x, 1.0, -uv.y)); // -x
  if (face == 4) return normalize(vec3(-uv.x, uv.y,  1.0)); // +z
  return normalize(vec3(uv.x, uv.y, -1.0)); // -z
}

void main() {
  vec2 coords = texture_coords * 2.0 - 1.0;
  vec3 direction = uv_to_direction(coords, face);
  vec2 sampling_pos = direction_to_uv(direction);
  return texture(panorama_texture, sampling_pos);
}
#version 460 core

out vec2 texture_coords;

void main() {
  // Sample a fullscreen vertex
  float x = float((gl_VertexID & 1) << 2);
  float y = float((gl_VertexID & 2) << 1);
  texture_coords.x = x * 0.5;
  texture_coords.y = y * 0.5;
  gl_Position = vec4(x - 1.0, y - 1.0, 0.0, 1.0);
}
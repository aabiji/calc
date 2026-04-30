#pragma once

#include <glm/glm.hpp>

class Camera {
public:
  explicit Camera();

  void rotate_orientation(float dx, float dy, float sensitivity);
  void rotate_position(bool left);
  void move_vertically(bool up);
  void zoom(bool inwards);
  glm::mat4 view_matrix();

private:
  glm::vec3 position;
  float yaw, pitch;
  float orbit_distance, orbit_angle;
  glm::mat4 projection;
};

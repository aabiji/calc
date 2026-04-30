#include <cmath>
#include <glm/common.hpp>
#include <glm/gtc/matrix_transform.hpp>

#include "camera.h"

Camera::Camera() {
  // Nice, predetermined starting perspective
  orbit_distance = 8.0;
  position = glm::vec3(-5.47636, 2.0, -5.83177);
  yaw = -0.0123633;
  pitch = -0.247973;
  orbit_angle = 3.95841;
}

void Camera::rotate_orientation(float dx, float dy, float sensitivity) {
  yaw += dx * sensitivity;
  pitch = glm::clamp(pitch + dy * sensitivity, -1.0f, 1.5f); // +- 85 degrees
}

void Camera::rotate_position(bool left) {
  float steps = 100;
  orbit_angle += (M_PI * 2.0 * (left ? -1 : 1)) / steps;
  if (orbit_angle > 2.0 * M_PI)
    orbit_angle -= 2 * M_PI;
  if (orbit_angle < 0)
    orbit_angle += 2 * M_PI;
  position.x = orbit_distance * std::cos(orbit_angle);
  position.z = orbit_distance * std::sin(orbit_angle);
}

void Camera::zoom(bool inwards) {
  orbit_distance =
      glm::clamp(orbit_distance + (inwards ? -1.0 : 1.0), 3.0, 12.0);
}

void Camera::move_vertically(bool up) {
  position.y = glm::clamp(position.y + (up ? 0.1 : -0.1), -2.0, 2.0);
}

glm::mat4 Camera::view_matrix() {
  // Orbit the camera around the origin, while preserving some free rotation
  glm::vec3 world_up = glm::vec3(0.0, 1.0, 0.0);
  glm::vec3 base_front =
      glm::normalize(glm::vec3(-position.x, 0.0, -position.z));
  glm::vec3 base_right = glm::normalize(glm::cross(base_front, world_up));

  glm::mat4 yaw_rot = glm::rotate(glm::mat4(1.0f), yaw, world_up);
  base_front = glm::vec3(yaw_rot * glm::vec4(base_front, 0.0));
  base_right = glm::normalize(glm::cross(base_front, world_up));

  glm::mat4 pitch_rot = glm::rotate(glm::mat4(1.0f), pitch, base_right);
  glm::vec3 front = glm::vec3(pitch_rot * glm::vec4(base_front, 0.0));

  glm::vec3 up = glm::normalize(glm::cross(base_right, front));
  return glm::lookAt(position, position + front, up);
}

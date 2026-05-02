#include <cmath>
#include <glm/common.hpp>
#include <glm/gtc/matrix_transform.hpp>

#include "camera.h"

Camera::Camera() {
  // Nice, predetermined starting perspective
  yaw = -0.0194961;
  pitch = -0.00847298;
  orbit_angle = 4.08407;
  position = glm::vec3(-4.11448, 0.4, -5.66313);
  orbit_distance = 7.0;
}

void Camera::rotate_orientation(glm::vec2 delta, float sensitivity) {
  yaw += delta.x * sensitivity;
  pitch =
      glm::clamp(pitch + delta.y * sensitivity, -1.0f, 1.5f); // +- 85 degrees
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
  float step = inwards ? -0.5 : 0.5;
  orbit_distance = glm::clamp(orbit_distance + step, 3.0f, 12.0f);
  position.x = orbit_distance * std::cos(orbit_angle);
  position.z = orbit_distance * std::sin(orbit_angle);
}

void Camera::move_vertically(bool up) {
  position.y = glm::clamp(position.y + (up ? 0.1 : -0.1), -5.0, 5.0);
}

glm::mat4 Camera::view_matrix() {
  // Orbit the camera around the origin, while preserving some free rotation
  glm::vec3 world_up = glm::vec3(0.0, 1.0, 0.0);
  glm::vec3 base_front =
      glm::normalize(glm::vec3(-position.x, -position.y, -position.z));
  glm::vec3 base_right = glm::normalize(glm::cross(base_front, world_up));

  glm::mat4 yaw_rot = glm::rotate(glm::mat4(1.0f), yaw, world_up);
  base_front = glm::vec3(yaw_rot * glm::vec4(base_front, 0.0));
  base_right = glm::normalize(glm::cross(base_front, world_up));

  glm::mat4 pitch_rot = glm::rotate(glm::mat4(1.0f), pitch, base_right);
  glm::vec3 front = glm::vec3(pitch_rot * glm::vec4(base_front, 0.0));

  glm::vec3 up = glm::normalize(glm::cross(base_right, front));
  return glm::lookAt(position, position + front, up);
}

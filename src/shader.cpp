#include <filesystem>
#include <fstream>
#include <glad/glad.h>
#include <glm/glm.hpp>
#include <glm/gtc/type_ptr.hpp>

#include "debug.h"
#include "shader.h"

unsigned int load_shader(const char *path, int type) {
  auto size = std::filesystem::file_size(path);
  std::string contents(size, '\0');
  std::ifstream file(path);
  assert(file.good() && file.is_open());
  file.read(&contents[0], size);

  const char *data = contents.c_str();
  unsigned int id = glCreateShader(type);
  glShaderSource(id, 1, &data, nullptr);
  glCompileShader(id);

  int success = 0;
  char info_log[1024];
  glGetShaderiv(id, GL_COMPILE_STATUS, &success);
  if (!success) {
    glGetShaderInfoLog(id, 1024, nullptr, info_log);
    THROW_ERROR("{} | {}", path, info_log);
  }

  return id;
}

Shader::Shader(const char *vshader_path, const char *fshader_path) {
  unsigned int vertex = load_shader(vshader_path, GL_VERTEX_SHADER);
  unsigned int fragment = load_shader(fshader_path, GL_FRAGMENT_SHADER);

  program = glCreateProgram();
  glAttachShader(program, vertex);
  glAttachShader(program, fragment);
  glLinkProgram(program);

  int success = 0;
  char info_log[1024];
  glGetProgramiv(program, GL_LINK_STATUS, &success);
  if (!success) {
    glGetProgramInfoLog(program, 1024, nullptr, info_log);
    THROW_ERROR("Link error: {}", info_log);
  }

  glDeleteShader(vertex);
  glDeleteShader(fragment);
}

Shader::~Shader() { glDeleteProgram(program); }

void Shader::use() { glUseProgram(program); }

template <typename T> void Shader::set(const char *name, T value) {
  int location = glGetUniformLocation(program, name);
  if constexpr (std::is_same_v<T, bool>)
    glUniform1i(location, value);

  if constexpr (std::is_same_v<T, glm::vec3>)
    glUniform3fv(location, 1, glm::value_ptr(value));

  if constexpr (std::is_same_v<T, glm::mat4>)
    glUniformMatrix4fv(glGetUniformLocation(program, name), 1, GL_FALSE,
                       glm::value_ptr(value));
}
template void Shader::set<glm::vec3>(const char *, glm::vec3);
template void Shader::set<glm::mat4>(const char *, glm::mat4);
template void Shader::set<bool>(const char *, bool);
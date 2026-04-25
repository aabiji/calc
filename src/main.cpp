#include <assert.h>
#include <cmath>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <unordered_map>

#define GLFW_INCLUDE_NONE
#include <GLFW/glfw3.h>
#include <glad/glad.h>

#include <glm/glm.hpp>
#include <glm/gtc/type_ptr.hpp>

// TODO:
// - Generate a sphere by subdividing an icosohedron
//   https://danielsieger.com/blog/2021/03/27/generating-spheres.html
// - Map a texture to the vertices

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
    std::cout << path << " -> " << info_log << "\n";
    std::exit(-1);
  }

  return id;
}

class Shader {
public:
  explicit Shader(const char *vshader_path, const char *fshader_path) {
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
      std::cout << "Link error -> " << info_log << "\n";
      std::exit(-1);
    }

    glDeleteShader(vertex);
    glDeleteShader(fragment);
  }

  ~Shader() { glDeleteProgram(program); }

  void use() { glUseProgram(program); }

  void set_vec3(const char *name, glm::vec3 &value) {
    glUniform3fv(glGetUniformLocation(program, name), 1, glm::value_ptr(value));
  }

private:
  unsigned int program;
};

struct vec3hash {
  std::size_t operator()(const glm::vec3 &v) const {
    return std::hash<float>()(v.x) ^ (std::hash<float>()(v.y) << 1) ^
           (std::hash<float>()(v.z) << 2);
  }
};

struct Vertex {
  glm::vec3 position;
};

class Mesh {
public:
  ~Mesh();
  explicit Mesh(std::vector<Vertex> v, std::vector<unsigned int> i);
  void render();

private:
  std::vector<Vertex> vertices;
  std::vector<unsigned int> indices;
  unsigned int vao, vbo, ebo;

  void generate_buffers();
};

Mesh::Mesh(std::vector<Vertex> v, std::vector<unsigned int> i)
    : vertices(v), indices(i) {
  glGenVertexArrays(1, &vao);
  glGenBuffers(1, &vbo);
  glGenBuffers(1, &ebo);

  glBindBuffer(GL_ARRAY_BUFFER, vbo);
  glBufferData(GL_ARRAY_BUFFER, vertices.size() * sizeof(Vertex),
               vertices.data(), GL_STATIC_DRAW);
  glBindVertexArray(vao);

  glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);
  glBufferData(GL_ELEMENT_ARRAY_BUFFER, indices.size() * sizeof(unsigned int),
               indices.data(), GL_STATIC_DRAW);

  glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, sizeof(Vertex),
                        (void *)offsetof(Vertex, position));
  glEnableVertexAttribArray(0); // position
}

Mesh::~Mesh() {
  glDeleteVertexArrays(1, &vao);
  glDeleteBuffers(1, &vbo);
  glDeleteBuffers(1, &ebo);
}

void Mesh::render() {
  glBindVertexArray(vao);
  glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);
  glDrawElements(GL_TRIANGLES, indices.size(), GL_UNSIGNED_INT, 0);
}

Mesh generate_sphere(int depth) {
  std::vector<Vertex> vertices;
  std::unordered_map<glm::vec3, unsigned int, vec3hash> vertex_map;

  auto add_vertex = [&](glm::vec3 v) {
    int index = vertices.size();
    vertices.push_back(Vertex{v});
    vertex_map.insert({v, index});
    return index;
  };

  auto midpoint = [&](int p1, int p2) {
    glm::vec3 v =
        glm::normalize((vertices[p1].position + vertices[p2].position) / 2.0f);
    return vertex_map.count(v) ? vertex_map[v] : add_vertex(v);
  };

  // Initial icosahedron points
  double p = (1.0 + std::sqrt(5.0)) / 2.0;
  std::vector<glm::vec3> initial_vertices = {
      glm::vec3(-1, p, 0),  glm::vec3(1, p, 0),    glm::vec3(-1, -p, 0),
      glm::vec3(1, -p, 0),  glm::vec3(0, -1, p),   glm::vec3(0, 1, p),
      glm::vec3(0, -1, -p), glm::vec3(0, 1, -p),   glm::vec3(p, 0, -1),
      glm::vec3(p, 0, 1),   glm::vec3(-p, -0, -1), glm::vec3(-p, -0, 1)};
  for (glm::vec3 v : initial_vertices)
    add_vertex(v);

  // Each triangle is grouped by 3 indices
  std::vector<unsigned int> indices = {
      0, 11, 5,  0, 5,  1, 0, 1, 7, 0, 7,  10, 0, 10, 11, 1, 5, 9, 5, 11,
      4, 11, 10, 2, 10, 7, 6, 7, 1, 8, 3,  9,  4, 3,  4,  2, 3, 2, 6, 3,
      6, 8,  3,  8, 9,  4, 9, 5, 2, 4, 11, 6,  2, 10, 8,  6, 7, 9, 8, 1};

  for (int j = 0; j < depth; j++) {
    std::vector<unsigned int> new_indices;

    for (size_t i = 0; i < indices.size(); i += 3) {
      int a = midpoint(indices[0], indices[1]);
      int b = midpoint(indices[1], indices[2]);
      int c = midpoint(indices[2], indices[0]);

      new_indices.push_back(indices[0]);
      new_indices.push_back(a);
      new_indices.push_back(c);

      new_indices.push_back(indices[1]);
      new_indices.push_back(b);
      new_indices.push_back(a);

      new_indices.push_back(indices[2]);
      new_indices.push_back(c);
      new_indices.push_back(b);

      new_indices.push_back(a);
      new_indices.push_back(b);
      new_indices.push_back(c);
    }

    indices = new_indices;
  }

  return Mesh(vertices, indices);
}

int main() {
  // auto satellites = read_satellite_data("../data/starlink.csv");

  glfwInit();
  glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
  glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 6);
  glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

  int window_width = 900, window_height = 700;
  GLFWwindow *window = glfwCreateWindow(window_width, window_height,
                                        "LEO Visualization", nullptr, nullptr);
  assert(window != nullptr);

  glfwMakeContextCurrent(window);
  assert(gladLoadGLLoader((GLADloadproc)glfwGetProcAddress) != 0);

  glViewport(0, 0, window_width, window_height);

  {
    Shader shader("../src/vertex.glsl", "../src/fragment.glsl");
    Mesh sphere = generate_sphere(0);

    while (!glfwWindowShouldClose(window)) {
      if (glfwGetKey(window, GLFW_KEY_SPACE) == GLFW_PRESS)
        break;

      glClearColor(0.0, 0.0, 0.0, 1.0);
      glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

      shader.use();
      sphere.render();

      glfwSwapBuffers(window);
      glfwPollEvents();
    }
  }

  glfwTerminate();
  return 0;
}

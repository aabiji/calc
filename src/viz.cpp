#include <glad/glad.h>

#include "GLFW/glfw3.h"
#include "debug.h"
#include "mesh.h"
#include "viz.h"

// auto satellites = read_satellite_data("../assets/starlink.csv");
// std::transform(satellites.begin(), satellites.end(),
//                std::back_inserter(circles.data), satellite_to_model);
// InstanceData satellite_to_model(Satellite s) {
//  s.propagate(0);
//  glm::mat4 scale = glm::scale(glm::mat4(1.0), glm::vec3(0.01, 0.01, 0.01));
//  glm::mat4 translate = glm::translate(glm::mat4(1.0), s.position * 0.0001f);
//  InstanceData instance;
//  instance.model_matrix = translate * scale;
//  instance.color = glm::vec4(0.0, 1.0, 0.0, 1.0);
//  return instance;
//}

GLFWContext::GLFWContext() { glfwInit(); }
GLFWContext::~GLFWContext() { glfwTerminate(); }

Visualizer::Visualizer(int width, int height) {
  create_window(width, height);
  set_callbacks();
  init_scene_objects();
}

void Visualizer::create_window(int width, int height) {
  glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
  glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 6);
  glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

  window =
      glfwCreateWindow(width, height, "LEO Visualization", nullptr, nullptr);
  if (window == nullptr)
    THROW_ERROR("Failed to create window");

  glfwMakeContextCurrent(window);
  if (gladLoadGLLoader((GLADloadproc)glfwGetProcAddress) == 0)
    THROW_ERROR("Failed to load the OpenGL context");

  state.projection = glm::perspective(
      (float)M_PI / 4.0f, (float)width / (float)height, 0.1f, 100.0f);
  glEnable(GL_DEPTH_TEST);
}

void Visualizer::set_callbacks() {
  glfwSetWindowUserPointer(window, &state);

  glfwWindowHint(GLFW_OPENGL_DEBUG_CONTEXT, true);
  glEnable(GL_DEBUG_OUTPUT);
  glEnable(GL_DEBUG_OUTPUT_SYNCHRONOUS);
  glDebugMessageCallback(gl_debug_callback, nullptr);
  glDebugMessageControl(GL_DONT_CARE, GL_DONT_CARE, GL_DONT_CARE, 0, nullptr,
                        GL_TRUE);

  glfwSetMouseButtonCallback(
      window, [](GLFWwindow *window, int button, int action, int mods) {
        InputState *state =
            static_cast<InputState *>(glfwGetWindowUserPointer(window));
        if (button == GLFW_MOUSE_BUTTON_LEFT)
          state->mouse_pressed = action == GLFW_PRESS;
        (void)mods;
      });

  glfwSetKeyCallback(window, [](GLFWwindow *window, int key, int scancode,
                                int action, int mods) {
    (void)scancode;
    (void)mods;
    InputState *state =
        static_cast<InputState *>(glfwGetWindowUserPointer(window));
    if (action == GLFW_PRESS || action == GLFW_REPEAT)
      state->keys.insert(key);
    else
      state->keys.erase(key);
  });

  glfwSetWindowSizeCallback(window, [](GLFWwindow *window, int w, int h) {
    InputState *state =
        static_cast<InputState *>(glfwGetWindowUserPointer(window));
    state->projection =
        glm::perspective((float)M_PI / 4.0f, (float)w / (float)h, 0.1f, 100.0f);
    glViewport(0, 0, w, h);
  });

  glfwSetCursorPosCallback(window, [](GLFWwindow *window, double x, double y) {
    InputState *state =
        static_cast<InputState *>(glfwGetWindowUserPointer(window));
    state->cursor_delta =
        glm::vec2(x - state->prev_cursor.x, y - state->prev_cursor.y);
    state->prev_cursor = glm::vec2(x, y);
  });

  glfwSetScrollCallback(
      window, [](GLFWwindow *window, double xoffset, double yoffset) {
        InputState *state =
            static_cast<InputState *>(glfwGetWindowUserPointer(window));
        state->yscroll = yoffset;
        (void)xoffset;
      });
}

void Visualizer::init_scene_objects() {
  main_shader.init("../assets/shaders/main_vertex.glsl",
                   "../assets/shaders/main_fragment.glsl");
  cubemap_shader.init("../assets/shaders/cubemap_vertex.glsl",
                      "../assets/shaders/cubemap_fragment.glsl");
  cubemap_texture.init(
      {"../assets/textures/cubemap/px.png", "../assets/textures/cubemap/nx.png",
       "../assets/textures/cubemap/py.png", "../assets/textures/cubemap/ny.png",
       "../assets/textures/cubemap/pz.png",
       "../assets/textures/cubemap/nz.png"});
  earth_texture.init({"../assets/textures/earth/daymap.jpg"});

  circles = create_circle_mesh(10);
  globe = create_unit_sphere(32, 32);
  skybox.init();

  globe_instances.push_back(InstanceData(
      glm::scale(glm::mat4(1.0), glm::vec3(2.0, 2.0, 2.0)), glm::vec4(0.0)));
  satellite_instances = {}; // TODO!
}

void Visualizer::run() {
  double x, y;
  glfwGetCursorPos(window, &x, &y);
  state.prev_cursor = glm::vec2(x, y);

  while (!glfwWindowShouldClose(window)) {
    glClearColor(0.0, 0.0, 0.0, 1.0);
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

    // clang-format off
    if (state.keys.contains(GLFW_KEY_W)) camera.move_vertically(true);
    if (state.keys.contains(GLFW_KEY_S)) camera.move_vertically(false);
    if (state.keys.contains(GLFW_KEY_A)) camera.rotate_position(false);
    if (state.keys.contains(GLFW_KEY_D)) camera.rotate_position(true);
    if (state.mouse_pressed) camera.rotate_orientation(state.cursor_delta, 0.001);
    if (state.yscroll != 0) camera.zoom(state.yscroll < 0);
    // clang-format on

    state.yscroll = 0;
    state.cursor_delta = glm::vec2(0.0);
    render_scene();

    glfwSwapBuffers(window);
    glfwPollEvents();
  }
}

void Visualizer::render_scene() {
  glm::mat4 v = camera.view_matrix();
  glm::mat4 v_no_translation = glm::mat4(glm::mat3(v));

  main_shader.use();
  main_shader.set<glm::mat4>("projection", state.projection);
  main_shader.set<glm::mat4>("view", v);

  earth_texture.use();
  main_shader.set<bool>("use_texture", true);
  globe.render(globe_instances);

  // glDisable(GL_DEPTH_TEST); // For drawing the 2D shapes
  // main_shader.set<bool>("use_texture", false);
  // circles.render();
  // glEnable(GL_DEPTH_TEST);

  // Render the skybox
  cubemap_shader.use();
  cubemap_shader.set<glm::mat4>("projection", state.projection);
  cubemap_shader.set<glm::mat4>("view", v_no_translation);
  glDepthFunc(GL_LEQUAL);
  cubemap_texture.use();
  skybox.render();
  glDepthFunc(GL_LESS);
}

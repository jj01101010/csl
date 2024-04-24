#version 330 core

layout (location = 0) in vec2 pos;

uniform vec2 offset;
uniform mat4 transform;

void main() {
  gl_Position = transform * vec4(pos - offset, 0.0, 1.0);
}

#version 330 core

layout (location = 0) in vec3 pos;

out vec3 outPos;

uniform mat4 transform;

void main() {
  outPos = pos;
  gl_Position = transform * vec4(pos.xyz, 1.0);
}

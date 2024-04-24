#version 330 core

layout (location = 0) in vec3 pos;

out vec3 outPos;

uniform vec2 offset;
uniform mat4 transform;

void main() {
  outPos = (transform * vec4(pos + vec3(offset, 0.0), 1.0)).xyz;
  gl_Position = transform * vec4(pos, 1.0);
}

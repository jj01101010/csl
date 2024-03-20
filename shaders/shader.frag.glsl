#version 330 core

out vec4 final_color;

in vec3 outPos;

uniform vec2 offset;
uniform vec2 pitch;

void main() {
  float lX = outPos.x;
  float lY = outPos.y;

  // TODO: Make this pixel perfect
  float scaleFactor = 300.0;

  float offX = (lX + offset[0]) * scaleFactor;
  float offY = (lY + offset[1]) * scaleFactor;

  if (int(mod(offX, pitch[0])) == 0 ||
      int(mod(offY, pitch[1])) == 0) {
    final_color = vec4(0.0, 0.0, 0.0, 1.0);
  } else {
    final_color = vec4(1.0, 1.0, 1.0, 1.0);
  }
}
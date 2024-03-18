#version 330 core

out vec4 final_color;

in vec3 outPos;

uniform vec2 offset;
uniform vec2 pitch;

void main() {

  // Normalize coodinates
  float lX = (outPos.x + 1.0) / 2.0;
  float lY = 1 - (outPos.y + 1.0) / 2.0;

  float scaleFactor = 200.0;

  float offX = (lX + offset[0]) * scaleFactor;
  float offY = (lY + offset[1]) * scaleFactor;

  if (int(mod(offX, pitch[0])) == 0 ||
      int(mod(offY, pitch[1])) == 0) {
    final_color = vec4(0.0, 0.0, 0.0, 0.5);
  } else {
    final_color = vec4(lX, lY, 0.0, 1.0);
  }
}
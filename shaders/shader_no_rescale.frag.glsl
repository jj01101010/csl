#version 330 core

out vec4 final_color;

uniform vec2 vp;
uniform vec2 offset;
uniform vec2 pitch;

void main() {

  float lX = gl_FragCoord.x / vp[0];
  float lY = gl_FragCoord.y / vp[1];

  float scaleFactor = 200.0;

  float offX = (scaleFactor * offset[0]) + lX * scaleFactor;
  float offY = (scaleFactor * offset[1]) + (1.0 - lY) * scaleFactor;

  if (int(mod(offX, pitch[0])) == 0 ||
      int(mod(offY, pitch[1])) == 0) {
    final_color = vec4(0.0, 0.0, 0.0, 0.5);
  } else {
    final_color = vec4(lX, lY, 0.0, 1.0);
  }
}
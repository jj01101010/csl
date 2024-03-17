#version 330 core

out vec4 final_color;

void main() {

  float vpw = 100;
  float vph = 100;

  vec2 offset = vec2(0);
  vec2 pitch = vec2(50, 50);

  float lX = gl_FragCoord.x / vpw;
  float lY = gl_FragCoord.y / vph;

  float scaleFactor = 10000.0;

  float offX = (scaleFactor * offset[0]) + gl_FragCoord.x;
  float offY = (scaleFactor * offset[1]) + (1.0 - gl_FragCoord.y);

  if (int(mod(offX, pitch[0])) == 0 ||
      int(mod(offY, pitch[1])) == 0) {
    final_color = vec4(0.0, 0.0, 0.0, 0.5);
  } else {
    final_color = vec4(1.0, 1.0, 1.0, 1.0);
  }
}
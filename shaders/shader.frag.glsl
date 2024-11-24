#version 330 core

out vec4 final_color;

in vec3 outPos;

uniform vec2 offset;

#define PI 3.1415

void main() {
  vec2 pos = outPos.xy + offset;

  // TODO: Make this pixel perfect
  float scaleFactor = 1.0;

  //float offX = (lX) * scaleFactor;
  //float offY = (lY) * scaleFactor;

  // if (int(mod(offX, pitch[0])) == 0 ||
  //     int(mod(offY, pitch[1])) == 0) {
  //   final_color = vec4(0.0, 0.0, 0.0, 1.0);
  // } else {
  //   
  // }

  final_color = vec4(abs(sin(2 * PI * (outPos.x + offset.x))), abs(cos(2 * PI * (outPos.y + offset.y))), 0.0, 1.0);
}
#version 330 core

out vec4 FragColor;

uniform float time;

void main() {
    float color = 0.5 + 0.5 * sin(time);
    FragColor = vec4(color, color, 1.0, 1.0);
}

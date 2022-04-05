#version 330 core

uniform vec4 uColor;

out vec4 final_color;

void main() {
    final_color = uColor;
}
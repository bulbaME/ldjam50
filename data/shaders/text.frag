#version 330 core

in vec2 fTextureCoord;

uniform sampler2D uTexture;

out vec4 final_color;

void main() {
    final_color = vec4(1.0, 1.0, 1.0, texture(uTexture, fTextureCoord).r);
}
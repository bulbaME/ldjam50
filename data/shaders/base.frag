#version 330 core

in vec2 fTextureCoord;
out vec4 final_color;

uniform sampler2D uTexture;

void main() {
    final_color = texture(uTexture, fTextureCoord) * vec4(1.0, 0.5, 0.5, 1.0);
}
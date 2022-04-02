#version 330 core

in vec2 fTextureCoord;
out vec4 final_color;

uniform sampler2D final_texture;

void main() {
    final_color = texture(final_texture, fTextureCoord) * vec4(1.0, 1.0, 1.0, 1.0);
}
#version 330 core

layout (location = 0) in vec3 aPosition;

out vec2 fTextureCoord;

uniform mat4 uMVP;

void main() {
    gl_Position = uMVP * vec4(aPosition, 1.0);
    fTextureCoord = vec2(aPosition.xy);
}
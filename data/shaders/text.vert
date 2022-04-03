#version 330 core

layout (location = 0) in vec3 aPosition;
layout (location = 1) in float aIndex;

out vec2 fTextureCoord;

uniform mat4 uMVP;
uniform mat4 uSource;

void main() {
    gl_Position = uMVP * vec4(aPosition, 1.0);

    int index = int(aIndex);
    fTextureCoord = vec2(uSource[0][index], uSource[1][index]);
}
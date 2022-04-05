#version 330 core

layout (location = 0) in vec3 aPosition;
layout (location = 1) in float aIndex;

uniform mat4 uMVP;

void main() {
    int index = int(aIndex);
    gl_Position = uMVP * vec4(aPosition, 1.0);
}
#version 300 es
precision highp float;
in vec4 position;
out vec2 uv;

void main() {
    uv = (position.xy + 1.0) * 0.5;
    gl_Position = vec4(position.xyz * 0.5, 1.0);
}
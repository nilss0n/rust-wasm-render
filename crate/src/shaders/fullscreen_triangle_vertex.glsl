precision highp float;
attribute vec4 position;
varying vec2 uv;

void main() {
    uv = (position.xy + 1.0) * 0.5;
    gl_Position = position;
}
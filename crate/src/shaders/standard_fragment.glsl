#version 300 es
precision highp float;
in vec2 uv;

float seed = 0.0;

out vec4 outColor;

float rand(){
    float v = fract(sin(seed) * 43758.5453123);
    seed = seed * v + v;
    return v;
}

void main() {
    seed = uv.x * 37.0 + uv.y + 91.0 - uv.x * uv.y * 23.0;
    vec3 col = vec3(1.);
    //xp(col, vec3(1.0 / 2.2));
    col += vec3(rand(), rand(), rand()) * 0.0039;
    outColor = vec4(col, 1.0);
}
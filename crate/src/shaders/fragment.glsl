precision highp float;
varying vec2 uv;

float seed = 0.0;

float rand(){
    float v = fract(sin(seed) * 43758.5453123);
    seed = seed * v + v;
    return v;
}

void main() {
    seed = uv.x * 37.0 + uv.y + 91.0 - uv.x * uv.y * 23.0;
    vec3 col = vec3(uv.x + uv.y, 0, 0) / 8.0;
    //xp(col, vec3(1.0 / 2.2));
    col += vec3(rand(), rand(), rand()) * 0.0039;
    gl_FragColor = vec4(col, 1.0);
}
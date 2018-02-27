#version 150

uniform float timer;

attribute vec4 position;

varying vec2 texcoord;
varying float fade_factor;

void main()
{
    gl_Position = position;
    texcoord = position.xy * vec2(0.5) + vec2(0.5);
    fade_factor = 0.5 * sin(timer) + 0.5;
}

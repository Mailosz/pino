#version 300 es
//#pragma debug(on) 
// an attribute will receive data from a buffer
in vec2 a_pos;
uniform vec2 u_res;

in vec2 gradient_coord;

vec2 normalizuj(in vec2 pos, in vec2 res);
vec2 normalizuj(in vec2 pos, in vec2 res) {
    // convert the position from pixels to 0.0 to 1.0
    vec2 zeroToOne = pos / res;

    // convert from 0->1 to 0->2
    vec2 zeroToTwo = zeroToOne * 2.0;

    // convert from 0->2 to -1->+1 (clip space)
    vec2 clipSpace = zeroToTwo - 1.0;

    return clipSpace * vec2(1, -1);
}


void main() {
    
    gl_Position = vec4(normalizuj(a_pos, u_res), 0, 1);

}
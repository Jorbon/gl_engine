#version 150

in vec2 p;
out vec4 color;

uniform vec2 u;
uniform float step_num;
uniform sampler2D main_tex;
uniform sampler2D aux_tex;
uniform sampler2D depth_tex;

void main() {
	color = texture(main_tex, p);
}


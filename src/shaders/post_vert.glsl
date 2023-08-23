#version 150

in vec2 pos;
out vec2 p;

void main() {
	p = (pos + 1.0) * 0.5;
	gl_Position = vec4(pos, 0.0, 1.0);
}


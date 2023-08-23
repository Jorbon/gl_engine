#version 150

in vec3 position;
uniform mat4 matrix;
uniform mat4 model_matrix;

void main() {
	gl_Position = matrix * (model_matrix * vec4(position, 1.0));
}

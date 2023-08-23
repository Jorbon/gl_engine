#version 150

in vec3 position;
in vec3 normal;
out vec3 pos;
out vec3 sm_pos;
out vec3 cam_pos;
out vec3 norm;
out vec3 cam_norm;

uniform mat4 sm_matrix;
uniform mat4 matrix;
uniform mat4 model_matrix;
uniform mat4 perspective;

void main() {
	pos = vec3(model_matrix * vec4(position, 1.0));
	sm_pos = (vec3(sm_matrix * vec4(pos, 1.0)) + 1.0) * 0.5;
	norm = normal;
	cam_norm = mat3(matrix) * normal;
	vec4 cp = matrix * vec4(pos, 1.0);
	cam_pos = vec3(cp);
	gl_Position = perspective * cp;
}
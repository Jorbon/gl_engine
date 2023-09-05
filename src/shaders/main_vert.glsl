#version 150

in vec3 position;
in vec3 normal;
//in vec2 uv;
out vec3 pos;
out vec3 sm_pos;
out vec3 norm;
out vec2 uvf;
//out vec3 cam_pos;
//out vec3 cam_norm;

uniform mat4 sm_matrix;
uniform mat4 matrix;
uniform mat4 model_matrix;
uniform mat4 perspective;

void main() {
	pos = vec3(model_matrix * vec4(position, 1.0));
	vec4 cp = matrix * vec4(pos, 1.0);
	gl_Position = perspective * cp;
	
	sm_pos = vec3(sm_matrix * vec4(pos, 1.0)) * 0.5 + 0.5;
	norm = normal;
	uvf = pos.xy * 0.5 + 0.5;//uv;
	//cam_pos = vec3(cp);
	//cam_norm = mat3(matrix) * normal;
}
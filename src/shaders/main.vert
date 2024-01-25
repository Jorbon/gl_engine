#version 150

in vec3 position; // model position
out vec3 world_position;
out vec3 shadowmap_position;

uniform mat4 shadowmap_transform;
uniform mat4 camera_transform;
uniform mat4 model_transform;
uniform mat4 perspective_matrix;

void main() {
	world_position = vec3(model_transform * vec4(position, 1.0));
	vec3 camera_position = vec3(camera_transform * vec4(world_position, 1.0));
	gl_Position = perspective_matrix * vec4(camera_position, 1.0);
	
	shadowmap_position = vec3(shadowmap_transform * vec4(world_position, 1.0)) * 0.5 + 0.5;
}
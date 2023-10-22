#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
uniform mat4 cam;
uniform float time;

out vec3 v_normal;

void main()
{
	v_normal = transpose(inverse(mat3(cam))) * normal;
	vec3 transformed = position + vec3(sin(time * position.x), cos(time * position.y), sin(cos(time * position.x) + sin(time * position.y))) * time / 10;
	gl_Position = cam * vec4(transformed, 1);
}

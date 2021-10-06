#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
uniform mat4 cam;

out vec3 v_normal;

void main()
{
	v_normal = transpose(inverse(mat3(cam))) * normal;
	gl_Position = cam * vec4(position, 1);
}

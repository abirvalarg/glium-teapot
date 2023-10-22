#version 330 core
in vec3 v_normal;
out vec4 color;
uniform vec3 light;
uniform float time;

void main()
{
	float red = (sin(time) + 1) / 2;
	float green = (sin(time / 2) + 1) / 2;
	float blue = (cos(time) + 1) / 2;
	vec3 shadowColor = vec3(red / 2, green / 2, blue / 2);
	vec3 regularColor = vec3(red, green, blue);
	float brightness = dot(normalize(v_normal), normalize(light)) * 10;
	color = vec4(mix(shadowColor, regularColor, brightness), 1);
}

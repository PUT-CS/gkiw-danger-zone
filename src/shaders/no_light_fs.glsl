#version 330 core
in vec2 TexCoords;
out vec4 FragColor;

uniform vec4 ParticleColor;
uniform sampler2D texture1;

void main()
{
    FragColor = (texture(texture1, TexCoords) * ParticleColor); 
}  

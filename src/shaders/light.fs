#version 330 core
out vec4 FragColor;

struct Material {
    vec3 diffuse;
    vec3 specular;
    float shininess;
}; 

struct DirLight {
    vec3 direction;
	
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

struct PointLight {
    vec3 position;
    
    float constant;
    float linear;
    float quadratic;
	
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoords;

//#define NR_POINT_LIGHTS 4

uniform sampler2D texture_diffuse1;
uniform vec3 viewPos;
uniform DirLight dirLight;
//uniform PointLight pointLights[NR_POINT_LIGHTS];
uniform Material material;

// function prototypes
vec4 CalcDirLight(DirLight light, vec3 normal, vec3 viewDir);
//vec3 CalcPointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir);

void main() {    
    vec3 norm = normalize(Normal);
    vec3 viewDir = normalize(viewPos - FragPos);

    vec4 result = CalcDirLight(dirLight, norm, viewDir);
    
    // for (int i=0; < 4; NR_POINT_LIGHTS; i++)
    //     result += CalcPointLight(pointLights[i], norm, FragPos, viewDir);
    FragColor = result;
}

vec4 CalcDirLight(DirLight light, vec3 normal, vec3 viewDir) {
    vec3 lightDir = normalize(-light.direction);

    float diff = max(dot(normal, lightDir), 0.0);

    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);

    vec4 ambient = vec4(light.ambient, 1.0) * vec4(texture(texture_diffuse1, TexCoords));
    vec4 diffuse = vec4(light.diffuse, 1.0) * diff * vec4(texture(texture_diffuse1, TexCoords));
    vec4 specular = vec4(light.specular, 1.0) * spec * vec4(texture(texture_diffuse1, TexCoords));
    return ambient + diffuse + specular;
}

// vec3 CalcPointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir)
// {
//     vec3 lightDir = normalize(light.position - fragPos);
//     // diffuse shading
//     float diff = max(dot(normal, lightDir), 0.0);
//     // specular shading
//     vec3 reflectDir = reflect(-lightDir, normal);
//     float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
//     // attenuation
//     float distance = length(light.position - fragPos);
//     float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));    
//     // combine results
//     vec3 ambient = light.ambient * vec3(texture(material.diffuse, TexCoords));
//     vec3 diffuse = light.diffuse * diff * vec3(texture(material.diffuse, TexCoords));
//     vec3 specular = light.specular * spec * vec3(texture(material.specular, TexCoords));
//     ambient *= attenuation;
//     diffuse *= attenuation;
//     specular *= attenuation;
//     return (ambient + diffuse + specular);
// }

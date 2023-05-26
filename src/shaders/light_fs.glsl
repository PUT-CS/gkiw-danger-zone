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

uniform sampler2D texture_diffuse1;
uniform vec3 viewPos;
uniform DirLight dirLight;
uniform PointLight pointLight;
uniform Material material;

// function prototypes
vec4 CalcDirLight(DirLight light, vec3 normal, vec3 viewDir);
vec4 CalcPointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir);

void main() {    
    vec3 norm = normalize(Normal);
    vec3 viewDir = normalize(viewPos - FragPos);
    //calculate pixel color with light (Phong model)
    vec4 result = CalcDirLight(dirLight, norm, viewDir);
    
    result += CalcPointLight(pointLight, norm, FragPos, viewDir);
    FragColor = result;
}

// calculate sun's light
vec4 CalcDirLight(DirLight light, vec3 normal, vec3 viewDir) {
    vec3 lightDir = normalize(-light.direction); // get a negative light vector (facing towards light)

    float diff = max(dot(normal, lightDir), 0.0); //dot product cannot be negative 

    vec3 reflectDir = reflect(-lightDir, normal); //vector calculated by reflecting light vector around the normal vector
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess); // dot product of vectors from model to view raised to the power of material shininess 

    // calculate what a texture would look like 
    vec4 ambient = vec4(light.ambient, 1.0) * vec4(texture(texture_diffuse1, TexCoords));
    vec4 diffuse = vec4(light.diffuse, 1.0) * diff * vec4(texture(texture_diffuse1, TexCoords));
    vec4 specular = vec4(light.specular, 1.0) * spec * vec4(texture(texture_diffuse1, TexCoords));
    return ambient + diffuse + specular;
}

//calculate pixel color from a point light based on distance, 
vec4 CalcPointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir)
{
    vec3 lightDir = normalize(light.position - fragPos); // get a negative light vector (facing towards light), depends on view direction

    float diff = max(dot(normal, lightDir), 0.0); //dot product cannot be negative

    vec3 reflectDir = reflect(-lightDir, normal); //vector calculated by reflecting light vector around the normal vector 
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess); // dot product of vectors from model to view raised to the power of material shininess

    // calculate light strenght based on distance from the source
    float distance = length(light.position - fragPos);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));    

    // calculate what a texture would look like 
    vec4 ambient = vec4(light.ambient, 1.0) * vec4(texture(texture_diffuse1, TexCoords));
    vec4 diffuse = vec4(light.diffuse, 1.0) * diff * vec4(texture(texture_diffuse1, TexCoords));
    vec4 specular = vec4(light.specular, 1.0) * spec * vec4(texture(texture_diffuse1, TexCoords));

    // multiply pixel look by the distance from source
    ambient *= attenuation;
    diffuse *= attenuation;
    specular *= attenuation;
    return ambient + diffuse + specular;
}

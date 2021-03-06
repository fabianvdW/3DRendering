#version 460 core
out vec4 FragColor;
in vec3 ourColor;
in vec2 TexCoord;

uniform sampler2D texture0;
uniform sampler2D texture1;
uniform float mix_p;

void main() {
    FragColor = mix(texture(texture0, TexCoord), texture(texture1,vec2(1- TexCoord.x, TexCoord.y)), mix_p) ;
}

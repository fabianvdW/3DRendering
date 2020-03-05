#version 460 core
layout (location = 0) in vec2 aPos;
out vec3 ourColor;

uniform int iterations;
uniform float noise;
uniform mat4 rotationMatrix;

void GetSierpinskiTriangle(in vec2 pos, out bool isInTriangle, out vec2 bottom, out vec2 right, out vec2 left){
    bottom = vec2(0.,0.);
    right = vec2(0.,0.);
    left = vec2(0.,0.);
    isInTriangle = false;
    vec2 general_bias = vec2(0.);
    float scale = 1.;
    for(int i=0; i<iterations;i++){
        if(pos.y <= 2 * pos.x +1 && pos.y <= -2.*pos.x +1){
            if (pos.x <0. && pos.y <= -2.*pos.x -1 && (abs(pos.x+0.5)>0.00001|| pos.y < -2.*pos.x -1)){
                //Bottom left triangle
                pos  = 2 * pos + vec2(1., 1.);
                general_bias += vec2(0.5 / scale, 0.5/scale);
                scale = 2 * scale;
            } else if (pos.x > 0. && pos.y <= 2. * pos.x -1 && (pos.x != 0.5 || pos.y < 2. * pos.x -1)){
                //Bottom right triangle
                pos = 2 * pos - vec2(1.,-1.);
                general_bias += vec2(-0.5/ scale, 0.5/scale);
                scale = 2 * scale;
            } else if (pos.x > -0.5 && pos.x <0.5 && pos.y >= 0.){
                //Upper triangle
                pos = 2 * pos - vec2(0., 1.);
                general_bias += vec2(0., -0.5 /scale);
                scale = 2 * scale;
            } else if (pos.x >= -0.5 && pos.x <=0.5){
                //in curr triangle
                isInTriangle = true;
                bottom = vec2(0./scale, -1/scale) - general_bias;
                right = vec2(0.5/scale, 0./scale) - general_bias;
                left = vec2(-0.5/scale, 0./scale) - general_bias;
                break;
            } else{
                break;
            }
        }
    }
}

void main(){
    vec2 in1 = vec2(aPos);
    bool out1 = false;
    vec2 bottom =  vec2(0.);
    vec2 right = vec2(0.);
    vec2 left = vec2(0.);
    GetSierpinskiTriangle(in1, out1, bottom, right, left);
    if (out1){
        vec4 middle = vec4(0.5 * (bottom + 0.5* (right + left)),0.,0.);
        vec4 newPos = vec4(0.);
        if(middle.x != 0.){
            newPos = rotationMatrix * vec4(aPos.x-middle.x, aPos.y- middle.y, 0.,1.) + vec4(middle.x,middle.y,0., 0);
        }else{
            newPos = vec4(aPos.xy, 0., 1.);
        }
        gl_Position = newPos;
        float mult = 3.;
        float color1 =  sin(mult*(mult * newPos.x + noise)) * 0.5 +0.5;
        float color2 =  cos(mult*(mult * newPos.y + noise)) * 0.5 +0.5;
        float color3 =  sin(1.41*(newPos.x + noise)) * 0.5 +0.5;
        ourColor = vec3(color1,color2,color3);
    }else{
        gl_Position = vec4(-2.0,0,0.,1.);
    }
}
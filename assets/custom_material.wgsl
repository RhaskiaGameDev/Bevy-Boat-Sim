// vertex shader ----------------------------------

uniform float t;
uniform float freq;
uniform float amp;

varying vec3 normal;
varying vec2 uv;

void main()
{
    float h = amp * sin(freq * gl_Vertex.y + t);

    vec3 n = gl_Normal;
    n.y -= h;
    normal = gl_NormalMatrix * normalize(n);

    uv = gl_MultiTexCoord0.xy;

    vec4 p = gl_Vertex;
    p.z += h;
    gl_Position = gl_ModelViewProjectionMatrix * p;
}


// fragment shader ----------------------------------

uniform sampler2D tex;
uniform vec3 light_dir;

varying vec3 normal;
varying vec2 uv;

void main()
{
    vec3 n = normalize(normal);
    vec3 l = normalize(light_dir);

    float kd = clamp(dot(n, -l), 0.0, 1.0);

    vec3 tex_color = texture2D(tex, uv).rgb;
    vec3 color = kd * tex_color;

    gl_FragColor = vec4(color, 1.0);
}
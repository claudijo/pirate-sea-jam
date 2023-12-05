#define_import_path pirate_sea_jam::water_dynamics

const PI: f32 = 3.1415926538;
const GRAVITY: f32 = 9.807;

fn gerstner_wave(
    wave: vec4<f32>,
    p: vec3<f32>,
    tangent: ptr<function,vec3<f32>>,
    binormal: ptr<function,vec3<f32>>,
    time: f32,
) -> vec3<f32> {
    let steepness = wave.z;
    let wave_length = wave.w;

   let k: f32 = 2. * PI / wave_length;
   let c: f32 = sqrt(GRAVITY / k);
   let d: vec2<f32> = normalize(wave.xy);
   let f: f32 = k * (dot(d, p.xz) - c * time);
   let a: f32 = steepness / k;

    *tangent += vec3<f32>(
        -d.x * d.x * (steepness * sin(f)),
        d.x * (steepness * cos(f)),
        -d.x * d.y * (steepness * sin(f))
    );

    *binormal += vec3<f32>(
        -d.x * d.y * (steepness * sin(f)),
        d.y * (steepness * cos(f)),
        -d.y * d.y * (steepness * sin(f))
    );

    return vec3<f32>(
        d.x * (a * cos(f)),
        a * sin(f),
        d.y * (a * cos(f))
    );
}
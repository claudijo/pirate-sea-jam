#define_import_path pirate_sea_jam::water_dynamics

const PI: f32 = 3.14159265358979323846264338327950288;
const GRAVITY: f32 = 10.;

// `wave`: Vec4 containing direction x, direction z, steepness, wave_length
fn gerstner_wave(wave: vec4<f32>, p: vec3<f32>, time: f32) -> vec3<f32> {
    let steepness = wave.z;
    let wave_length = wave.w;

   let k: f32 = 2. * PI / wave_length;
   let c: f32 = sqrt(GRAVITY / k);
   let d: vec2<f32> = normalize(wave.xy);
   let f: f32 = k * (dot(d, p.xz) - c * time);
   let a: f32 = steepness / k;

    return vec3<f32>(
        d.x * (a * cos(f)),
        a * sin(f),
        d.y * (a * cos(f))
    );
}
#define_import_path pirate_sea_jam::water_dynamics

#import pirate_sea_jam::ocean_material_bindings::ImpactPoint

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

fn wave_height(k: f32, x: f32, omega: f32, t: f32) -> f32 {
    return sin(k * x - omega * t);
}

fn wave_height_impact_point(impact_point: ImpactPoint, world_position: vec4<f32>, k: f32, t: f32) -> f32 {
    let omega = sqrt(2 * PI * GRAVITY * k) * 0.4;
    let x = distance(world_position.xz, impact_point.position.xz);
    let damping = 1. / pow(x, 2.);
    return wave_height(k, x, omega, t - impact_point.time) * damping;
}
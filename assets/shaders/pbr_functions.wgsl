#define_import_path pirate_sea_jam::pbr_functions

#import bevy_pbr::{
    pbr_types,
    pbr_bindings,
    mesh_view_bindings as view_bindings,
    mesh_view_types,
    lighting,
    transmission,
    clustered_forward as clustering,
    shadows,
    ambient,
    mesh_types::{MESH_FLAGS_SHADOW_RECEIVER_BIT, MESH_FLAGS_TRANSMITTED_SHADOW_RECEIVER_BIT},
    utils::E,
}

#ifdef ENVIRONMENT_MAP
#import bevy_pbr::environment_map
#endif

#import bevy_core_pipeline::tonemapping::{screen_space_dither, powsafe, tone_mapping}


fn apply_pbr_lighting(
    in: pbr_types::PbrInput,
) -> vec4<f32> {
    var output_color: vec4<f32> = in.material.base_color;

    // TODO use .a for exposure compensation in HDR
    let emissive = in.material.emissive;

    // calculate non-linear roughness from linear perceptualRoughness
    let metallic = in.material.metallic;
    let perceptual_roughness = in.material.perceptual_roughness;
    let roughness = lighting::perceptualRoughnessToRoughness(perceptual_roughness);
    let ior = in.material.ior;
    let thickness = in.material.thickness;
    let diffuse_transmission = in.material.diffuse_transmission;
    let specular_transmission = in.material.specular_transmission;

    let specular_transmissive_color = specular_transmission * in.material.base_color.rgb;

    let occlusion = in.occlusion;

    // Neubelt and Pettineo 2013, "Crafting a Next-gen Material Pipeline for The Order: 1886"
    let NdotV = max(dot(in.N, in.V), 0.0001);

    // Remapping [0,1] reflectance to F0
    // See https://google.github.io/filament/Filament.html#materialsystem/parameterization/remapping
    let reflectance = in.material.reflectance;
    let F0 = 0.16 * reflectance * reflectance * (1.0 - metallic) + output_color.rgb * metallic;

    // Diffuse strength is inversely related to metallicity, specular and diffuse transmission
    let diffuse_color = output_color.rgb * (1.0 - metallic) * (1.0 - specular_transmission) * (1.0 - diffuse_transmission);

    // Diffuse transmissive strength is inversely related to metallicity and specular transmission, but directly related to diffuse transmission
    let diffuse_transmissive_color = output_color.rgb * (1.0 - metallic) * (1.0 - specular_transmission) * diffuse_transmission;

    // Calculate the world position of the second Lambertian lobe used for diffuse transmission, by subtracting material thickness
    let diffuse_transmissive_lobe_world_position = in.world_position - vec4<f32>(in.world_normal, 0.0) * thickness;

    let R = reflect(-in.V, in.N);

    let f_ab = lighting::F_AB(perceptual_roughness, NdotV);

    var direct_light: vec3<f32> = vec3<f32>(0.0);

    // Transmitted Light (Specular and Diffuse)
    var transmitted_light: vec3<f32> = vec3<f32>(0.0);

    let view_z = dot(vec4<f32>(
        view_bindings::view.inverse_view[0].z,
        view_bindings::view.inverse_view[1].z,
        view_bindings::view.inverse_view[2].z,
        view_bindings::view.inverse_view[3].z
    ), in.world_position);
    let cluster_index = clustering::fragment_cluster_index(in.frag_coord.xy, view_z, in.is_orthographic);
    let offset_and_counts = clustering::unpack_offset_and_counts(cluster_index);

    // Point lights (direct)
    for (var i: u32 = offset_and_counts[0]; i < offset_and_counts[0] + offset_and_counts[1]; i = i + 1u) {
        let light_id = clustering::get_light_id(i);
        var shadow: f32 = 1.0;
        if ((in.flags & MESH_FLAGS_SHADOW_RECEIVER_BIT) != 0u
                && (view_bindings::point_lights.data[light_id].flags & mesh_view_types::POINT_LIGHT_FLAGS_SHADOWS_ENABLED_BIT) != 0u) {
            shadow = shadows::fetch_point_shadow(light_id, in.world_position, in.world_normal);
        }
        let light_contrib = lighting::point_light(in.world_position.xyz, light_id, roughness, NdotV, in.N, in.V, R, F0, f_ab, diffuse_color);
        direct_light += light_contrib * shadow;

        if diffuse_transmission > 0.0 {
            // NOTE: We use the diffuse transmissive color, the second Lambertian lobe's calculated
            // world position, inverted normal and view vectors, and the following simplified
            // values for a fully diffuse transmitted light contribution approximation:
            //
            // roughness = 1.0;
            // NdotV = 1.0;
            // R = vec3<f32>(0.0) // doesn't really matter
            // f_ab = vec2<f32>(0.1)
            // F0 = vec3<f32>(0.0)
            var transmitted_shadow: f32 = 1.0;
            if ((in.flags & (MESH_FLAGS_SHADOW_RECEIVER_BIT | MESH_FLAGS_TRANSMITTED_SHADOW_RECEIVER_BIT)) == (MESH_FLAGS_SHADOW_RECEIVER_BIT | MESH_FLAGS_TRANSMITTED_SHADOW_RECEIVER_BIT)
                    && (view_bindings::point_lights.data[light_id].flags & mesh_view_types::POINT_LIGHT_FLAGS_SHADOWS_ENABLED_BIT) != 0u) {
                transmitted_shadow = shadows::fetch_point_shadow(light_id, diffuse_transmissive_lobe_world_position, -in.world_normal);
            }
            let light_contrib = lighting::point_light(diffuse_transmissive_lobe_world_position.xyz, light_id, 1.0, 1.0, -in.N, -in.V, vec3<f32>(0.0), vec3<f32>(0.0), vec2<f32>(0.1), diffuse_transmissive_color);
            transmitted_light += light_contrib * transmitted_shadow;
        }
    }

    // Spot lights (direct)
    for (var i: u32 = offset_and_counts[0] + offset_and_counts[1]; i < offset_and_counts[0] + offset_and_counts[1] + offset_and_counts[2]; i = i + 1u) {
        let light_id = clustering::get_light_id(i);

        var shadow: f32 = 1.0;
        if ((in.flags & MESH_FLAGS_SHADOW_RECEIVER_BIT) != 0u
                && (view_bindings::point_lights.data[light_id].flags & mesh_view_types::POINT_LIGHT_FLAGS_SHADOWS_ENABLED_BIT) != 0u) {
            shadow = shadows::fetch_spot_shadow(light_id, in.world_position, in.world_normal);
        }
        let light_contrib = lighting::spot_light(in.world_position.xyz, light_id, roughness, NdotV, in.N, in.V, R, F0, f_ab, diffuse_color);
        direct_light += light_contrib * shadow;

        if diffuse_transmission > 0.0 {
            // NOTE: We use the diffuse transmissive color, the second Lambertian lobe's calculated
            // world position, inverted normal and view vectors, and the following simplified
            // values for a fully diffuse transmitted light contribution approximation:
            //
            // roughness = 1.0;
            // NdotV = 1.0;
            // R = vec3<f32>(0.0) // doesn't really matter
            // f_ab = vec2<f32>(0.1)
            // F0 = vec3<f32>(0.0)
            var transmitted_shadow: f32 = 1.0;
            if ((in.flags & (MESH_FLAGS_SHADOW_RECEIVER_BIT | MESH_FLAGS_TRANSMITTED_SHADOW_RECEIVER_BIT)) == (MESH_FLAGS_SHADOW_RECEIVER_BIT | MESH_FLAGS_TRANSMITTED_SHADOW_RECEIVER_BIT)
                    && (view_bindings::point_lights.data[light_id].flags & mesh_view_types::POINT_LIGHT_FLAGS_SHADOWS_ENABLED_BIT) != 0u) {
                transmitted_shadow = shadows::fetch_spot_shadow(light_id, diffuse_transmissive_lobe_world_position, -in.world_normal);
            }
            let light_contrib = lighting::spot_light(diffuse_transmissive_lobe_world_position.xyz, light_id, 1.0, 1.0, -in.N, -in.V, vec3<f32>(0.0), vec3<f32>(0.0), vec2<f32>(0.1), diffuse_transmissive_color);
            transmitted_light += light_contrib * transmitted_shadow;
        }
    }

    // directional lights (direct)
    let n_directional_lights = view_bindings::lights.n_directional_lights;
    for (var i: u32 = 0u; i < n_directional_lights; i = i + 1u) {
        var shadow: f32 = 1.0;
        if ((in.flags & MESH_FLAGS_SHADOW_RECEIVER_BIT) != 0u
                && (view_bindings::lights.directional_lights[i].flags & mesh_view_types::DIRECTIONAL_LIGHT_FLAGS_SHADOWS_ENABLED_BIT) != 0u) {
            shadow = shadows::fetch_directional_shadow(i, in.world_position, in.world_normal, view_z);
        }
        var light_contrib = lighting::directional_light(i, roughness, NdotV, in.N, in.V, R, F0, f_ab, diffuse_color);
#ifdef DIRECTIONAL_LIGHT_SHADOW_MAP_DEBUG_CASCADES
        light_contrib = shadows::cascade_debug_visualization(light_contrib, i, view_z);
#endif
        direct_light += light_contrib * shadow;

        if diffuse_transmission > 0.0 {
            // NOTE: We use the diffuse transmissive color, the second Lambertian lobe's calculated
            // world position, inverted normal and view vectors, and the following simplified
            // values for a fully diffuse transmitted light contribution approximation:
            //
            // roughness = 1.0;
            // NdotV = 1.0;
            // R = vec3<f32>(0.0) // doesn't really matter
            // f_ab = vec2<f32>(0.1)
            // F0 = vec3<f32>(0.0)
            var transmitted_shadow: f32 = 1.0;
            if ((in.flags & (MESH_FLAGS_SHADOW_RECEIVER_BIT | MESH_FLAGS_TRANSMITTED_SHADOW_RECEIVER_BIT)) == (MESH_FLAGS_SHADOW_RECEIVER_BIT | MESH_FLAGS_TRANSMITTED_SHADOW_RECEIVER_BIT)
                    && (view_bindings::lights.directional_lights[i].flags & mesh_view_types::DIRECTIONAL_LIGHT_FLAGS_SHADOWS_ENABLED_BIT) != 0u) {
                transmitted_shadow = shadows::fetch_directional_shadow(i, diffuse_transmissive_lobe_world_position, -in.world_normal, view_z);
            }
            let light_contrib = lighting::directional_light(i, 1.0, 1.0, -in.N, -in.V, vec3<f32>(0.0), vec3<f32>(0.0), vec2<f32>(0.1), diffuse_transmissive_color);
            transmitted_light += light_contrib * transmitted_shadow;
        }
    }

    // Ambient light (indirect)
    var indirect_light = ambient::ambient_light(in.world_position, in.N, in.V, NdotV, diffuse_color, F0, perceptual_roughness, occlusion);

    if diffuse_transmission > 0.0 {
        // NOTE: We use the diffuse transmissive color, the second Lambertian lobe's calculated
        // world position, inverted normal and view vectors, and the following simplified
        // values for a fully diffuse transmitted light contribution approximation:
        //
        // perceptual_roughness = 1.0;
        // NdotV = 1.0;
        // F0 = vec3<f32>(0.0)
        // occlusion = vec3<f32>(1.0)
        transmitted_light += ambient::ambient_light(diffuse_transmissive_lobe_world_position, -in.N, -in.V, 1.0, diffuse_transmissive_color, vec3<f32>(0.0), 1.0, vec3<f32>(1.0));
    }

    // Environment map light (indirect)
#ifdef ENVIRONMENT_MAP
    let environment_light = environment_map::environment_map_light(perceptual_roughness, roughness, diffuse_color, NdotV, f_ab, in.N, R, F0);
    indirect_light += (environment_light.diffuse * occlusion) + environment_light.specular;

    // we'll use the specular component of the transmitted environment
    // light in the call to `specular_transmissive_light()` below
    var specular_transmitted_environment_light = vec3<f32>(0.0);

    if diffuse_transmission > 0.0 || specular_transmission > 0.0 {
        // NOTE: We use the diffuse transmissive color, inverted normal and view vectors,
        // and the following simplified values for the transmitted environment light contribution
        // approximation:
        //
        // diffuse_color = vec3<f32>(1.0) // later we use `diffuse_transmissive_color` and `specular_transmissive_color`
        // NdotV = 1.0;
        // R = T // see definition below
        // F0 = vec3<f32>(1.0)
        // occlusion = 1.0
        //
        // (This one is slightly different from the other light types above, because the environment
        // map light returns both diffuse and specular components separately, and we want to use both)

        let T = -normalize(
            in.V + // start with view vector at entry point
            refract(in.V, -in.N, 1.0 / ior) * thickness // add refracted vector scaled by thickness, towards exit point
        ); // normalize to find exit point view vector

        let transmitted_environment_light = bevy_pbr::environment_map::environment_map_light(perceptual_roughness, roughness, vec3<f32>(1.0), 1.0, f_ab, -in.N, T, vec3<f32>(1.0));
        transmitted_light += transmitted_environment_light.diffuse * diffuse_transmissive_color;
        specular_transmitted_environment_light = transmitted_environment_light.specular * specular_transmissive_color;
    }
#else
    // If there's no environment map light, there's no transmitted environment
    // light specular component, so we can just hardcode it to zero.
    let specular_transmitted_environment_light = vec3<f32>(0.0);
#endif

    let emissive_light = emissive.rgb * output_color.a;

    if specular_transmission > 0.0 {
        transmitted_light += transmission::specular_transmissive_light(in.world_position, in.frag_coord.xyz, view_z, in.N, in.V, F0, ior, thickness, perceptual_roughness, specular_transmissive_color, specular_transmitted_environment_light).rgb;
    }

    if (in.material.flags & pbr_types::STANDARD_MATERIAL_FLAGS_ATTENUATION_ENABLED_BIT) != 0u {
        // We reuse the `atmospheric_fog()` function here, as it's fundamentally
        // equivalent to the attenuation that takes place inside the material volume,
        // and will allow us to eventually hook up subsurface scattering more easily
        var attenuation_fog: mesh_view_types::Fog;
        attenuation_fog.base_color.a = 1.0;
        attenuation_fog.be = pow(1.0 - in.material.attenuation_color.rgb, vec3<f32>(E)) / in.material.attenuation_distance;
        // TODO: Add the subsurface scattering factor below
        // attenuation_fog.bi = /* ... */
        transmitted_light = bevy_pbr::fog::atmospheric_fog(
            attenuation_fog, vec4<f32>(transmitted_light, 1.0), thickness,
            vec3<f32>(0.0) // TODO: Pass in (pre-attenuated) scattered light contribution here
        ).rgb;
    }

    // Total light
    output_color = vec4<f32>(
        transmitted_light + direct_light + indirect_light + emissive_light,
        output_color.a
    );

    output_color = clustering::cluster_debug_visualization(
        output_color,
        view_z,
        in.is_orthographic,
        offset_and_counts,
        cluster_index,
    );

//    output_color = vec4<f32>(0., 0., 1., 1.);

    return output_color;
}


use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct StartAimArtilleryAnimationClips {
    pub handles: HashMap<&'static str, Handle<AnimationClip>>,
}

#[derive(Resource)]
pub struct EndAimArtilleryAnimationClips {
    pub handles: HashMap<&'static str, Handle<AnimationClip>>,
}

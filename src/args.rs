use crate::args::resources::Args;
use bevy::app::{App, Plugin};
use bevy::log::info;
use clap::Parser;

pub mod resources;
pub mod run_conditions;

pub struct ArgsPlugin;

impl Plugin for ArgsPlugin {
    fn build(&self, app: &mut App) {
        let args = Args::parse();
        info!("{args:?}");

        app.insert_resource(args);
    }
}

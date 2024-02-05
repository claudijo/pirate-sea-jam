use crate::args::resources::Args;
use bevy::prelude::*;

pub fn sync_test_mode(args: Res<Args>) -> bool {
    args.sync_test
}

pub fn p2p_mode(args: Res<Args>) -> bool {
    !args.sync_test
}

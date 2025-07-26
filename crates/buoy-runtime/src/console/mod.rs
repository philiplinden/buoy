pub mod repl;

use bevy::prelude::*;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        repl::setup_repl(app);
    }
}

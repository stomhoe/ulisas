use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

use crate::game::{enemy, star};

mod game;
mod menu;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[states(scoped_entities)]
pub enum AppState {
    #[default]
    Menu,
    Game,
    GameOver,
}
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, menu::plugin, star::plugin, enemy::plugin))
        .init_state::<AppState>()
        .run()
    ;
}



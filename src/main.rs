use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

mod game;
mod menus;

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
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .run()
    ;
}



use bevy::prelude::*;


pub mod enemy_components;
mod enemy_systems;



pub fn plugin(app: &mut App) {
    app
    .add_plugins(())
    /*
    .add_systems(Update, (
    ))
     */
    ;
}
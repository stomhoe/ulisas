use bevy::prelude::*;
use bevy_replicon::prelude::*;

mod star_systems;
pub mod star_resources;
pub mod star_components;
pub mod star_messages;



#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PluginNameSystems;


pub fn plugin(app: &mut App) {
    app
    .add_plugins(())
    /*
    .add_systems(Update, (
    ))
     */
    ;
}
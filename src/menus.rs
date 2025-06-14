use bevy::prelude::*;
pub(crate) mod main_menu;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, main_menu)
            //.add_systems(Update, )
        ;
    }
}

pub fn main_menu(){
    println!("main menu");
}




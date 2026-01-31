use bevy::prelude::*;


pub fn plugin(app: &mut App) {
    app
        .add_systems(Startup, main_menu)
        //.add_systems(Update, )
    ;
}

pub fn main_menu(){
    println!("main menu");
}

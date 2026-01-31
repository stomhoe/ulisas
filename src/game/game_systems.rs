#[allow(unused_imports)] use bevy::prelude::*;
#[allow(unused_imports)] use bevy_replicon::prelude::*;



#[allow(unused_parens)]
pub fn game_init_system(mut cmd: Commands, 
) {
    cmd.spawn(());
}




// ----------------------> NO OLVIDARSE DE AGREGARLO AL Plugin DEL MÓDULO <-----------------------------

#[allow(unused_parens)]
pub fn first_update_game_system(mut cmd: Commands, 
    mut query: Query<(&Name),()>
) {
    for mut item in query.iter() {
        
    }
}



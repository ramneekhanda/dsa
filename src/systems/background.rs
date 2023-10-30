use bevy::prelude::*;


pub fn update_background(ccommand: Commands,
                         mut window: Query<&mut Window>,) {

    let ht = window.single().height();
    let wd = window.single().width();

    let grid_sz = 20; // draw line after every 20 pixel

}

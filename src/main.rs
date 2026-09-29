use raylib::prelude::*;
mod components; 
use components::StringDisplayComponent; 
const LAYER1: Color = Color::new(54, 58, 79, 255);

fn main(){

    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Hello this is my app")
        .transparent()
        .build();

    let Test = components::StringDisplayComponent::create("Hello World".to_string(), 20, Color::RED, 400, 400, 200, 100);

    handle_interface_initialization(&mut rl);
    while !rl.window_should_close() {  
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(36, 39, 58, 220));
        d.draw_rectangle_rounded(Rectangle::new(20.0, 20.0, 200.0, 150.0), 0.1, 10, LAYER1);
        Test.draw(&mut d);
    }

}

fn handle_interface_initialization(rl: &mut RaylibHandle ) {
    // set the screen to be fullscreen 
    let monitor = raylib::core::window::get_current_monitor();
    if !rl.is_window_fullscreen() {
    let width = raylib::core::window::get_monitor_width(monitor);
    let height = raylib::core::window::get_monitor_height(monitor);
    rl.set_window_size(width, height);
    }
    rl.toggle_fullscreen();

}

use raylib::prelude::*;

fn main(){

    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Hello this is my app")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }

}

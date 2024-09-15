use raylib::prelude::*;

mod spacial;
mod block;
mod worm;

fn main() {
    let (mut rl, thread) = init()
        .size(640, 480)
        .title("puzzle game")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
    }
}

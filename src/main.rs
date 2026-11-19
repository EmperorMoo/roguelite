use macroquad::prelude::*;

#[macroquad::main("Number 15: Burger King Foot lettuce.")]
async fn main() {
    // test to see if macroquad is working
    loop {
        clear_background(BLUE);
        draw_text("Hello Macroquad!", 20.0, 20.0, 30.0, WHITE);
        next_frame().await

    }
}
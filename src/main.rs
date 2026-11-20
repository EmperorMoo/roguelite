use macroquad::prelude::*;

#[macroquad::main("Number 15: Burger King Foot lettuce.")]
async fn main() {
    let mut player_health = 100;
    let mut player_x: f32 = 400.0;
    let mut player_y: f32 = 300.0;
    let player_speed: f32 = 200.0;
    // test to see if macroquad is working
    loop {
        clear_background(BLUE);

        draw_text("Can Speedy Peedy deal a bagiggle wiggle do giggles in spiggle spoo giggles?", 20.0, 20.0, 30.0, WHITE);
        
        draw_rectangle(player_x, player_y, 32.0, 32.0, RED);

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            player_y -= player_speed * get_frame_time();
        }

        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            player_y += player_speed * get_frame_time();
            
        }

        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            player_x -= player_speed * get_frame_time();
            
        }

        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            player_x += player_speed * get_frame_time();
            
        }


        if player_x < 0.0 {
            player_x = 0.0;
        }

        if player_x > screen_width() - 32.0 {
            player_x = screen_width() - 32.0;
        }

        if player_y < 0.0 {
            player_y = 0.0;
        }

        if player_y > screen_height() - 32.0 {
            player_y = screen_height() - 32.0;
        }
        
        next_frame().await;

    }
}

//676767676767676767676767676767676767676767676767676767676767676767676767676767
// I'm doing a lot of planning right now for the game, and I want to make sure that I have a good idea of what I want to do before I start coding. I want to make sure that the game is fun and engaging, and that it has a good balance of difficulty and reward. I also want to make sure that the game is visually appealing and has a good soundtrack. I'm also thinking about adding some multiplayer features, so that players can compete against each other or work together to achieve a common goal. Overall, I want to create a game that is both challenging and rewarding, and that will keep players coming back for more.
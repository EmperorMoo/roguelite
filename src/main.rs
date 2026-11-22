use macroquad::prelude::*;

#[macroquad::main("Number 15: Burger King Foot lettuce.")]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as u64);

    let mut player_health = 100;
    let mut player_x: f32 = 400.0;
    let mut player_y: f32 = 300.0;
    let player_speed: f32 = 200.0;
    let mut player_facing_left = false;
    let player_texture = load_texture("assets/characters/warrior/Warrior V1.png").await.unwrap();
    let tile_size: f32 = 32.0;
    let dungeon_width: usize = 25;
    let dungeon_height: usize = 18;
    let mut dungeon = vec![vec![0; dungeon_width]; dungeon_height];


    let combat_rooms = vec![
        RoomTemplate {
            width: 12,
            height: 8,
            shape: RoomShape::Rectangle,
            doors: vec![DoorDirection::North, DoorDirection::South],
        },
        RoomTemplate {
            width: 10,
            height: 10,
            shape: RoomShape::Square,
            doors: vec![DoorDirection::East, DoorDirection::West],
        },
        RoomTemplate {
            width: 16,
            height: 10,
            shape: RoomShape::Rectangle,
            doors: vec![DoorDirection::North, DoorDirection::South, DoorDirection::East],
        },
        RoomTemplate {
            width: 12,
            height: 12,
            shape: RoomShape::Circular,
            doors: vec![DoorDirection::North, DoorDirection::South, DoorDirection::East, DoorDirection::West],
        },
    ];

    let room_index = rand::gen_range(0, combat_rooms.len());
    let selected_room = &combat_rooms[room_index];
    let room_width = selected_room.width;
    let room_height = selected_room.height;
    let room_x = rand::gen_range(0,dungeon_width - room_width + 1);
    let room_y = rand::gen_range(0,dungeon_height - room_height + 1);

    generate_room(
        &mut dungeon, 
        room_x, 
        room_y, 
        room_width, 
        room_height,
        &selected_room.shape,
        &selected_room.doors,
    );

    // test to see if macroquad is working
    loop {
        clear_background(BLACK);

        for y in 0..dungeon_height {
            for x in 0..dungeon_width {

                let tile_color = match dungeon[y][x] {
                    0 => BLACK,
                    1 => GRAY,
                    2 => DARKGRAY,
                    _ => BLACK
                };

                draw_rectangle(
                    x as f32 * tile_size,
                    y as f32 * tile_size,
                    tile_size,
                    tile_size,
                    tile_color,
                );

                draw_rectangle_lines(
                    x as f32 * tile_size,
                    y as f32 * tile_size,
                    tile_size,
                    tile_size,
                    1.0,
                    DARKGRAY,
                );
            }
        }

        //draw_text("Can Speedy Peedy deal a bagiggle wiggle do giggles in spiggle spoo giggles?", 20.0, 20.0, 30.0, WHITE);
        
        draw_texture_ex(
            &player_texture,
            player_x,
            player_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(64.0, 64.0)),
                flip_x: player_facing_left,
                ..Default::default()
            },
        );

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            player_y -= player_speed * get_frame_time();
        }

        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            player_y += player_speed * get_frame_time();
            
        }

        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            player_x -= player_speed * get_frame_time();
            player_facing_left = true;
        }

        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            player_x += player_speed * get_frame_time();
            player_facing_left = false;
        }


        if player_x < 0.0 {
            player_x = 0.0;
        }

        if player_x > screen_width() - 64.0 {
            player_x = screen_width() - 64.0;
        }

        if player_y < 0.0 {
            player_y = 0.0;
        }

        if player_y > screen_height() - 64.0 {
            player_y = screen_height() - 64.0;
        }
        
        next_frame().await;

    }
}

enum RoomShape {
    Rectangle,
    Square,
    Circular,
    Irregular,
}

enum DoorDirection {
    North,
    South,
    East,
    West,
}

struct RoomTemplate {
    width: usize,
    height: usize,
    shape: RoomShape,
    doors: Vec<DoorDirection>,
}

fn generate_room(
    dungeon: &mut Vec<Vec<i32>>,
    start_x: usize,
    start_y: usize,
    room_width: usize,
    room_height: usize,
    room_shape: &RoomShape,
    room_doors: &[DoorDirection],
) {

let (room_width, room_height) = match room_shape {
    RoomShape::Rectangle => (room_width, room_height),
    RoomShape::Square => {
        let size = room_width.min(room_height);
        (size, size)
    }
    RoomShape::Circular => (room_width, room_height),
    RoomShape::Irregular => (room_width, room_height),
};

for y in start_y..start_y + room_height {
        for x in start_x..start_x + room_width {
            let center_x = room_width as f32 / 2.0;
            let center_y = room_height as f32 / 2.0;

            let distance_x = (x as f32 + 0.5) - (start_x as f32 + center_x);
            let distance_y = (y as f32 + 0.5) - (start_y as f32 + center_y);

            let distance = (distance_x * distance_x + distance_y * distance_y).sqrt();

            if matches!(room_shape, RoomShape::Circular) {
                let radius = center_x.min(center_y) - 0.5;

                if distance <= radius {
                    if distance >= radius -1.0 {
                        dungeon[y][x] = 2;
                    } else {
                        dungeon[y][x] = 1;
                    }
                }
            }
            else if y == start_y || y == start_y + room_height - 1 || x == start_x || x == start_x + room_width - 1 {
                dungeon[y][x] = 2;
            } else {
                dungeon[y][x] = 1;
            }
        }
    }

    //let door_y = if matches!(room_shape, RoomShape::Circular) {
        //start_y + room_height - 2
    //} else {
        //start_y + room_height - 1
    //};

    for door in doors{
        match door {
            DoorDirection::North => {
                dungeon[start_y][start_x + room_width / 2] = 1;
            }
            DoorDirection::South => {
                let door_y = if matches! (room_shape, RoomShape::Circular) {
                    start_y + room_height - 2
                } else {
                    start_y + room_height - 1
                };

                dungeon[door_y][start_x + room_width / 2] = 1;
            }
            DoorDirection::East =>{
                dungeon[start_y + room_height / 2][start_x + room_width -1] = 1;
            }
            DoorDirection::West => {
                dungeon[start_y + room_height / 2][start_x] = 1;
            }
        }
    }

    for &door_direction in room_doors {

        dungeon[door_y][door_x] = 1; //create a door in the dungeon
    }
}

//676767676767676767676767676767
// I'm doing a lot of planning right now for the game.
use macroquad::prelude::*;
use std::env;
use std::path::PathBuf;

// This is a helper function that fixes the path for relative paths
fn resolve_path(relative_path: &str) -> PathBuf {
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let candidate = exe_dir.join(relative_path);
            if candidate.exists() {
                return candidate;
            }
        }
    }
    PathBuf::from(relative_path)
}
#[macroquad::main("Number 15: Burger King Foot lettuce.")]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as u64);

    let mut player_health = 100;
    let mut player_x: f32 = 400.0;
    let mut player_y: f32 = 300.0;
    let mut camera = Camera2D {
        target: vec2(player_x + 32.0, player_y + 32.0),
        zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
        ..Default::default()
    };
    let player_speed: f32 = 600.0;
    let mut player_facing_left = false;
    let player_texture = load_texture(resolve_path("assets/characters/warrior/Warrior V2.png").to_str().unwrap()).await.unwrap();
    player_texture.set_filter(FilterMode::Nearest);
    let tile_size: f32 = 64.0;
    let dungeon_width: usize = 100;
    let dungeon_height: usize = 100;
    let mut dungeon = vec![vec![0; dungeon_width]; dungeon_height];


    let combat_rooms: Vec<RoomTemplate> = Vec::from([
        RoomTemplate {
            width: 40,
            height: 30,
            shape: RoomShape::Rectangle,
            doors: vec![
                DoorDirection::North, 
                DoorDirection::South, 
                DoorDirection::East, 
                DoorDirection::West
            ],
            
            obstacles: vec![
                // Corner pillars
                Obstacle { 
                    x: 6, 
                    y: 5, 
                    width: 3, 
                    height: 3,
                    obstacle_type:
                ObstacleType::Pillar,
                },
                Obstacle { 
                    x: 31, 
                    y: 5, 
                    width: 3, 
                    height: 3,
                    obstacle_type:
                ObstacleType::Pillar
                },

                // Lower pillars
                Obstacle { 
                    x: 6, 
                    y: 22, 
                    width: 3, 
                    height: 3,
                    obstacle_type:
                ObstacleType::Pillar,
                },
                Obstacle { 
                    x: 31, 
                    y: 22, 
                    width: 3, 
                    height: 3, 
                    obstacle_type:
                ObstacleType::Pillar,
                },

                // Central side barriers
                Obstacle { 
                    x: 12, 
                    y: 12, 
                    width: 2, 
                    height: 5, 
                    obstacle_type:
                ObstacleType::Pillar,
                },
                Obstacle { 
                    x: 26, 
                    y: 12, 
                    width: 2, 
                    height: 5, 
                    obstacle_type:
                ObstacleType::Pillar,
                },

                // Smaller pillars
                Obstacle { 
                    x: 18, 
                    y: 5, 
                    width: 3, 
                    height: 3, 
                    obstacle_type:
                ObstacleType::Pillar,
                },
                Obstacle { 
                    x: 18, 
                    y: 23, 
                    width: 3, 
                    height: 3, 
                    obstacle_type:
                ObstacleType::Pillar,
                },
            ],
        },

        RoomTemplate {
            width: 30,
            height: 40,
            shape: RoomShape::Rectangle,
            doors: vec![
                DoorDirection::North,
                DoorDirection::South,
                DoorDirection::East,
                DoorDirection::West,
            ],
            obstacles: vec![
                // Upper pillars
                Obstacle { 
                    x: 5, 
                    y: 7, 
                    width: 3, 
                    height: 3, 
                    obstacle_type:
                ObstacleType::Pillar
                },
                Obstacle { 
                    x: 22, 
                    y: 7, 
                    width: 3, 
                    height: 3, 
                    obstacle_type:
                ObstacleType::Pillar,
                },

                // Central barriers
                Obstacle { 
                    x: 9, 
                    y: 17, 
                    width: 3, 
                    height: 6, 
                    obstacle_type:
                ObstacleType::Pillar,
                },
                Obstacle { 
                    x: 9, 
                    y: 17, 
                    width: 3, 
                    height: 6, 
                    obstacle_type:
                ObstacleType::Pillar,
                },

                // Lower pillars
                Obstacle { 
                    x: 5, 
                    y: 30, 
                    width: 3, 
                    height: 3, 
                    obstacle_type:
                ObstacleType::Pillar,
                },
                Obstacle { 
                    x: 22, 
                    y: 30, 
                    width: 3, 
                    height: 3, 
                    obstacle_type:
                ObstacleType::Pillar,
                },
            ],
        },
    ]);

        //RoomTemplate {
            //width: 10,
            //height: 10,
            //shape: RoomShape::Square,
            //doors: vec![DoorDirection::East, DoorDirection::West],
        //},
        //RoomTemplate {
            //width: 16,
            //height: 10,
            //shape: RoomShape::Rectangle,
            //doors: vec![DoorDirection::North, DoorDirection::South, DoorDirection::East],
        //},
        //RoomTemplate {
            //width: 12,
            //height: 12,
            //shape: RoomShape::Circular,
            //doors: vec![DoorDirection::North, DoorDirection::South, DoorDirection::East, DoorDirection::West],
        //},
        //RoomTemplate {

        //}

    let room_index = rand::gen_range(0, combat_rooms.len());
    let selected_room = &combat_rooms[room_index];
    let room_width = selected_room.width;
    let room_height = selected_room.height;
    let room_x = (dungeon_width - room_width) / 2;
    let room_y = (dungeon_height - room_height) / 2;
    //let room_x = rand::gen_range(0,dungeon_width - room_width + 1);
    //let room_y = rand::gen_range(0,dungeon_height - room_height + 1);

    generate_room(
        &mut dungeon, 
        room_x, 
        room_y, 
        room_width, 
        room_height,
        &selected_room.shape,
        &selected_room.doors,
    );

    generate_obstacles(
        &mut dungeon, 
        room_x, 
        room_y,
        &selected_room.obstacles,
    );

    fn is_solid_tile(
        dungeon: &Vec<Vec<i32>>,
        tile_x: usize,
        tile_y: usize,
    ) -> bool {
        dungeon[tile_y][tile_x] == 2 || dungeon[tile_y][tile_x] == 3
    }

    fn check_collision(
        dungeon: &Vec<Vec<i32>>,
        player_x: f32,
        player_y: f32,
        player_size: f32,
        tile_size: f32,
    ) -> bool {
        let corners = [
            (player_x, player_y),
            (player_x + player_size, player_y),
            (player_x, player_y + player_size),
            (player_x +player_size, player_y + player_size),
        ];

        for (x, y) in corners {
            let tile_x = (x / tile_size) as usize;
            let tile_y = (y / tile_size) as usize;

            if tile_y >= dungeon.len() || tile_x >= dungeon[0].len() {
                return true;
            }

            if is_solid_tile(dungeon, tile_x, tile_y) {
                return true;
            }
        }

        false
    }

    player_x = (room_x as f32 + room_width as f32 / 2.0) * tile_size - 64.0;
    player_y = (room_y as f32 + room_height as f32 / 2.0) * tile_size - 64.0;

    loop {
        clear_background(BLACK);

        let mut move_x = 0.0;
        let mut move_y = 0.0;

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            move_y -= 1.0;
        }

        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            move_y += 1.0;
            
        }

        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            move_x -= 1.0;
            player_facing_left = true;
        }

        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            move_x += 1.0;
            player_facing_left = false;
        }

        let movement = vec2(move_x, move_y).normalize_or_zero() * player_speed * get_frame_time();

        let next_x = player_x + movement.x;

        if !check_collision(&dungeon, next_x, player_y, 128.0, tile_size) {
            player_x = next_x;
        }

        let next_y = player_y + movement.y;

        if !check_collision(&dungeon, player_x, next_y, 128.0, tile_size) {
            player_y = next_y;
        }

        let view_width = screen_width();
        let view_height = screen_height();

        camera.zoom = vec2(
            2.0 / view_width,
            2.0 / view_height,
        );

        let half_view_width = view_width / 2.0;
        let half_view_height = view_height / 2.0;

        let world_width = dungeon_width as f32 * tile_size;
        let world_height = dungeon_height as f32 * tile_size;

        let camera_x = if world_width > view_width {
            (player_x + 32.0).clamp(half_view_width, world_width - half_view_width,)
        } else {
            world_width / 2.0
        };

        let camera_y = if world_height > view_height {
            (player_y + 32.0).clamp(half_view_height, world_height - half_view_height,)
        } else {
            world_height / 2.0
        };

        camera.target = vec2(camera_x, camera_y);

        set_camera(&camera);

        for y in 0..dungeon_height {
            for x in 0..dungeon_width {

                let tile_color = match dungeon[y][x] {
                    0 => BLACK,
                    1 => GRAY,
                    2 => DARKGRAY,
                    3 => Color::new(0.35, 0.30, 0.25, 1.0),
                    _ => BLACK,
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
                dest_size: Some(vec2(128.0, 128.0)),
                flip_x: player_facing_left,
                ..Default::default()
            },
        );

        set_default_camera();

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
    obstacles: Vec<Obstacle>,
}

enum ObstacleType {
    Pillar,
    Barrier,
}

struct Obstacle {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    obstacle_type: ObstacleType,
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

    for door in room_doors{
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

    //for &door_direction in room_doors {

        //dungeon[door_y][door_x] = 1; //create a door in the dungeon
    //}
}

fn generate_obstacles(
    dungeon: &mut Vec<Vec<i32>>, 
    room_x: usize,
    room_y: usize,
    obstacles: &[Obstacle],
) {
    for obstacle in obstacles {
        let start_x = room_x + obstacle.x;
        let start_y = room_y + obstacle.y;

        for y in start_y..start_y + obstacle.height{

            for x in start_x..start_x + obstacle.width {
                dungeon[y][x] = 3;
            }
        }
    }
}


// When I add pit obstacles that damage the player upon falling into them, I can use just the lower pixels of the character.
//676767676767676767676767676767
// I'm doing a lot of planning right now for the game.
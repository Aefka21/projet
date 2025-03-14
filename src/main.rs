use bevy::prelude::*;
//use rand::prelude::*;
use noise::{NoiseFn, Perlin};

#[derive(PartialEq, Clone)]
enum TileType {
    Empty,
    Mud,
    Ground,
    Steel,
}

struct TileMap {
    tiles: Vec<Vec<TileType>>,
    width: usize,
    height: usize,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Add a 2D camera
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true, // HDR is required for the bloom effect
            ..default()
        },
        //Bloom::default(),
    ));

    
    // Generate noise map
    let width = 100;
    let height = 100;
    let scale = 5.0;
    let tile_map = generate_tile_map(width, height, scale);
    
    // Create a grid of sprites
    spawn_tile_grid(&mut commands, &tile_map);
}

fn generate_tile_map(width: usize, height: usize, scale: f64) -> TileMap {
    //let rng = rand::rng();
    let perlin = Perlin::new(43);
    let mut noise_array = vec![vec![TileType::Empty; width]; height];
    
    for y in 0..height {
        for x in 0..width {
            // Convert grid coordinates to appropriate noise coordinates
            let nx = x as f64 / width as f64 * scale;
            let ny = y as f64 / height as f64 * scale;
            
            // Get noise value and scale from [-1, 1] to [0, 1]
            let noise_value = perlin.get([nx, ny]) as f64;
            /*if noise_value > 0.5 {
                println!("Steel!!!");
            }*/
            noise_array[y][x] = get_tile((noise_value + 1.0) / 2.0);
        }
    }
    
    TileMap{tiles: noise_array, width: width, height: height}
}

fn get_tile(val: f64) -> TileType {
    match val.abs() {
        v if v < 0.2 => TileType::Mud,
        v if v < 0.7 => TileType::Ground,
        _ => TileType::Steel,
    }
}

fn print_noise_array(array: &Vec<Vec<TileType>>) {
    let height = array.len();
    let width = if height > 0 { array[0].len() } else { 0 };
    
    println!("Perlin Noise Array ({}x{}):", width, height);
    for y in 0..30 /* height */ {
        for x in 0..30 /* width */{
            let tile = array[y][x].clone();
            // Format to 4 decimal places for readability
            if tile == TileType::Mud {
                print!("0 ");
            } else if tile == TileType::Ground {
                print!("1 ");
            } else {
                print!("2 ");
            }
        }
        println!(); // New line after each row
    }
}

fn spawn_tile_grid(commands: &mut Commands, tile_map: &TileMap) {
    // Define the size of each tile
    let tile_size = 6.0;
    
    // Calculate total grid size
    let grid_width = tile_map.width as f64 * tile_size;
    let grid_height = tile_map.height as f64 * tile_size;
    
    // Center the grid
    let offset_x = -grid_width / 2.0 + tile_size / 2.0;
    let offset_y = -grid_height / 2.0 + tile_size / 2.0;
    
    // Spawn a sprite for each tile
    for y in 0..tile_map.height {
        for x in 0..tile_map.width {
            let tile_type = &tile_map.tiles[y][x];
            
            // Choose color based on tile type
            let color = match tile_type {
                TileType::Mud => Color::srgb(0., 0., 1.),
                TileType::Ground => Color::srgb(1., 0., 0.),
                TileType::Steel => Color::srgb(1., 1., 1.),
                TileType::Empty => Color::srgb(0., 0., 0.),
            };
            
            // Calculate position
            let position = Vec3::new(
                (offset_x + x as f64 * tile_size) as f32,
                (offset_y + y as f64 * tile_size) as f32,
                0.0,
            );
            
            commands.spawn((
                // Transform will automatically add GlobalTransform
                Transform::from_translation(position),
                // Default visibility components will be added automatically
                Visibility::Inherited,
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(tile_size as f32, tile_size as f32)),
                    ..default()
                },
            ));
        }
    }
}

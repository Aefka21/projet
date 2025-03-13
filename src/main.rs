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

fn main() {
    App::new()
        //.add_plugins(DefaultPlugins)
        .add_systems(Startup, generate_ground)
        .run();
}

fn generate_noise_map() -> Vec<Vec<TileType>> {
    let height = 100;
    let width = 100;
    let scale: f64 = 5.;
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
    
    noise_array
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

fn generate_ground() {
    let map = generate_noise_map();
    print_noise_array(&map);
}


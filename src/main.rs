use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

const TILE_SIZE: f64 = 6.;
const SIDE: usize = 100;

#[derive(PartialEq, Clone)]
enum TileType {
    Empty,
    Air,
    Mud,
    Ground,
    Steel,
    Cave,
    Player,
}

#[derive(Resource)]
struct Mining {
    is_mining: bool,
}

struct TileMap {
    pub tiles: Vec<Vec<TileType>>,
}

impl TileMap {
    pub fn get_tile_type(&self, x: usize, y: usize) -> TileType {
        self.tiles[y][x].clone()
    }

    pub fn update_terrain(&mut self, x: usize, y: usize, tile_type: TileType) {
        self.tiles[y][x] = tile_type; 
    }
}

impl Resource for TileMap {}

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_input)
        //.add_systems(Update, test)
        .run();
}

/*fn test(
    mut commands: Commands,
    map: Res<TileMap>,
    mut mine: ResMut<Mining>,
    query: Query<(Entity, &Transform), With<Sprite>>
) {
    // Calculate total grid size
    let grid_width = SIDE as f64 * TILE_SIZE;
    let grid_height = SIDE as f64 * TILE_SIZE;
    
    // Center the grid
    let offset_x = (-grid_width / 2.0 + TILE_SIZE / 2.0) as f32;
    let offset_y = (-grid_height / 2.0 + TILE_SIZE / 2.0) as f32;
    let target_position = Vec3::new(offset_x, offset_y + 6. * 32., 0.0);
    if mine.is_mining {
        for (entity, transform) in query.iter() {
            println!("{}", transform.translation);
            if transform.translation.truncate() == target_position.truncate() {
                commands.entity(entity).despawn(); // Supprime le sprite
                println!("Sprite supprimé à {:?}", transform.translation);
            }
        }
        //spawn_tile_grid(&mut commands, &map);
        mine.is_mining = false;
    }
}*/

fn setup(
   mut commands: Commands,
) {
    // Add a 2D camera
    commands.spawn((
        Camera2d,
    ));

    // Generate noise map
    let scale = 2.5;
    let tile_map = generate_tile_map(scale);
    
    // Create a grid of sprites
    spawn_tile_grid(&mut commands, &tile_map);
    commands.insert_resource(tile_map);

    commands.insert_resource(Mining {is_mining: false});

    // Calculate total grid size
    let grid_width = SIDE as f64 * TILE_SIZE;
    let grid_height = SIDE as f64 * TILE_SIZE;
    
    // Center the grid
    let offset_x = (-grid_width / 2.0 + TILE_SIZE / 2.0) as f32;
    let offset_y = (-grid_height / 2.0 + TILE_SIZE / 2.0) as f32;

    commands.spawn((
        Player,
        Transform::from_xyz(offset_x + 6., offset_y + 32. * 6., 0.0),
        GlobalTransform::default(),
        Sprite {
            color: Color::srgb(1., 0., 0.),
            custom_size: Some(Vec2::splat(TILE_SIZE as f32 * 2.)),
            ..Default::default()
        },
    ));
}

fn generate_tile_map(scale: f64) -> TileMap {
    let perlin = Perlin::new(42);
    let mut array = vec![vec![TileType::Empty; SIDE]; SIDE];
    
    for y in 0..SIDE {
        for x in 0..SIDE {
            // Convert grid coordinates to appropriate noise coordinates
            let nx = x as f64 / SIDE as f64 * scale;
            let ny = y as f64 / SIDE as f64 * scale;
            
            // Get noise value and scale from [-1, 1] to [0, 1]
            let noise_value = perlin.get([nx, ny]) as f64;
            if (y == 0 && x == 0) || (y == 1 && x == 0) {
                array[y][x] = TileType::Player;
            } else if y > SIDE - 20 {
                array[y][x] = TileType::Air;
            } else {
                array[y][x] = get_tile((noise_value + 1.0) / 2.0);
            }
        }
    }
    
    TileMap{tiles: array}
}

fn get_tile(val: f64) -> TileType {
    match val.abs() {
        v if v < 0.3 => TileType::Mud,
        v if v < 0.6 => TileType::Ground,
        v if v < 0.8 => TileType::Cave,
        _ => TileType::Steel,
    }
}

fn get_color(tile: TileType) -> Color {
    match tile {
        TileType::Mud => Color::srgb(0.8, 0.4, 0.16),
        TileType::Ground => Color::srgb(0.6, 0.2, 0.2),
        TileType::Steel => Color::srgb(0.8, 0.8, 0.9),
        TileType::Empty => Color::srgb(0., 0., 0.),
        TileType::Air => Color::srgb(0.6, 0.6, 1.),
        TileType::Cave => Color::srgb(0.2, 0.0, 0.01),
        TileType::Player => Color::srgb(0., 1., 0.),
    }
}

fn spawn_tile_grid(commands: &mut Commands, tile_map: &TileMap) {
    // Calculate total grid size
    let grid_width = SIDE as f64 * TILE_SIZE;
    let grid_height = SIDE as f64 * TILE_SIZE;
    
    // Center the grid
    let offset_x = -grid_width / 2.0 + TILE_SIZE / 2.0;
    let offset_y = -grid_height / 2.0 + TILE_SIZE / 2.0;
    
    // Spawn a sprite for each tile
    for y in 0..SIDE {
        for x in 0..SIDE {
            let tile_type = tile_map.tiles[y][x].clone();
            
            // Choose color based on tile type
            let color = get_color(tile_type);
            
            // Calculate position
            let position = Vec3::new(
                (offset_x + x as f64 * TILE_SIZE) as f32,
                (offset_y + y as f64 * TILE_SIZE) as f32,
                0.0,
            );
            
            // Spawn the tile
            commands.spawn((
                Transform::from_translation(position),
                Visibility::Inherited,
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
                    ..default()
                },
            ));
        }
    }
}

fn player_input(
    key: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    mut tile_map: ResMut<TileMap>,
    mut mining: ResMut<Mining>,
    //commands: &mut Commands,
) {
    for mut transform in query.iter_mut() {
        let mut movement = Vec2::ZERO;

        if key.pressed(KeyCode::Space) {
            /*for y in 0..SIDE {
                for x in 0..SIDE {
                    if tile_map.get_tile_type(x, y) == TileType::Ground {
                        tile_map.update_terrain(x, y, TileType::Cave);
                        mining.is_mining = true;
                    }
                }
            }*/
        }
        if key.pressed(KeyCode::KeyA) {
            movement.x -= TILE_SIZE as f32; // Left
        }
        if key.pressed(KeyCode::KeyD) {
            movement.x += TILE_SIZE as f32; // Right
        }

        // Appliquer le mouvement (case par case)
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;

        transform.translation.x = transform.translation.x.clamp((-TILE_SIZE * SIDE as f64 / 2.) as f32 + TILE_SIZE as f32, (TILE_SIZE * SIDE as f64 / 2.) as f32 - TILE_SIZE as f32);
    }
}
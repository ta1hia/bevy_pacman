use bevy::{
    prelude::*,
    log::{Level, LogSettings},
    input::{
        keyboard::KeyboardInput,
    },
};
use std::time::Duration;
use rand::seq::SliceRandom; 

fn main() {
    let mut app = App::build();
    app.add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(PacmanMovementTimer(Timer::new(
                    Duration::from_millis(1. as u64),
                    true,
        )))
        .add_resource(GhostMovementTimer(Timer::new(
                    Duration::from_millis(150. as u64),
                    true,
        )))
        .add_resource(GhostModeTimer(Timer::new(
                    Duration::from_millis(10000. as u64),
                    true,
        )))
        .add_resource(Game{mode:Mode::Scatter})
        .add_plugins(DefaultPlugins);
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.add_startup_system(setup.system())
        .add_startup_system(ghost_setup.system())
        .add_system(position_translation.system())
        .add_system(size_scaling.system())
        .add_resource(LogSettings {
            filter: "bevy_webgl2=warn".into(),
            level: Level::INFO,
        })
        .add_system(sprite_timer.system())
        .add_system(pacman_animate.system())
        .add_system(pacman_movement.system())
        .add_system(pacman_eating.system())
        .add_system(pacman_energy_boost.system())
        .add_system(ghost_timer.system())
        .add_system(ghost_mode_timer.system())
        .add_system(ghost_mode.system())
        .add_system(ghost_movement.system())
        .add_system(ghost_animate.system())
        .add_system(ghost_next_target.system())
        .run();
}

const ARENA_WIDTH: i32 = 27;
const ARENA_HEIGHT: i32 = 31;

const WORLD_MAP: [[i32; 27]; 31] = [
  [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
  [1, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 1],
  [1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1], 
  [1, 2, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 2, 1],
  [1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1], 
  [1, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 4, 1], 
  [1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1], 
  [1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1],
  [1, 0, 0, 0, 0, 0, 4, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 4, 0, 0, 0, 0, 0, 1], 
  [1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1], 
  [9, 9, 9, 9, 9, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 9, 9, 9, 9, 9], 
  [9, 9, 9, 9, 9, 1, 0, 1, 1, 0, 0, 5, 0, 0, 0, 5, 0, 0, 1, 1, 0, 1, 9, 9, 9, 9, 9], 
  [9, 9, 9, 9, 9, 1, 0, 1, 1, 0, 1, 1, 1, 3, 1, 1, 1, 0, 1, 1, 0, 1, 9, 9, 9, 9, 9], 
  [1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 3, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1], 
  [0, 0, 0, 0, 0, 0, 4, 0, 0, 4, 1, 1, 9, 9, 9, 1, 1, 4, 0, 0, 4, 0, 0, 0, 0, 0, 0], 
  [1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1], 
  [9, 9, 9, 9, 9, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 9, 9, 9, 9, 9], 
  [9, 9, 9, 9, 9, 1, 0, 1, 1, 4, 0, 0, 0, 0, 0, 0, 0, 4, 1, 1, 0, 1, 9, 9, 9, 9, 9], 
  [9, 9, 9, 9, 9, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 9, 9, 9, 9, 9], 
  [1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1], 
  [1, 0, 0, 0, 0, 0, 4, 0, 0, 4, 0, 0, 1, 1, 1, 0, 0, 4, 0, 0, 4, 0, 0, 0, 0, 0, 1], 
  [1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1], 
  [1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1], 
  [1, 2, 0, 0, 1, 1, 4, 0, 0, 4, 0, 5, 0, 0, 0, 5, 0, 4, 0, 0, 4, 1, 1, 0, 0, 2, 1], 
  [1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1], 
  [1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1], 
  [1, 0, 0, 4, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 4, 0, 0, 1], 
  [1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1], 
  [1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1], 
  [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
  [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];


#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)] 
struct Position {
    x: i32,
    y: i32,
}
impl Position {
    fn euclid_distance(self, x:i32, y:i32) -> f32 {
        (((self.y-y).pow(2) + (self.x-x).pow(2)) as f32).sqrt()
    }
    fn choose_next_tile(self, direction: Direction, target_: Position) -> (Position, Direction){
  	let mut tile: Position = self;
	let mut shortest: f32 = 99999.;
        let mut dir: Direction = direction;
        let mut target = target_;

        if WORLD_MAP[self.y as usize][self.x as usize] == 9 ||
            WORLD_MAP[self.y as usize][self.x as usize] == 3{
                target = Position{x: 13, y: 11};
        }

        //up
        if self.y-1 > -1 && 
            direction != Direction::Down &&
            WORLD_MAP[(self.y-1) as usize][self.x as usize] != 1 &&
            WORLD_MAP[(self.y-1) as usize][self.x as usize] != 5 {
            let distance = target.euclid_distance(self.x, self.y-1);
            if distance < shortest {
                shortest = distance;
                tile = Position{x:self.x, y:self.y-1};
                dir = Direction::Up;
            }
	}		
        //left
        if self.x-1 > -1 && 
            direction != Direction::Right &&
            WORLD_MAP[self.y as usize][(self.x-1) as usize] != 1 {
            let distance = target.euclid_distance(self.x-1, self.y);
            if distance < shortest {
                shortest = distance;
                tile = Position{x:self.x-1, y:self.y};
                dir = Direction::Left;
            }
	}		
        //down
        if self.y+1 < ARENA_HEIGHT && 
            direction != Direction::Up &&
            WORLD_MAP[(self.y+1) as usize][self.x as usize] != 1 &&
            WORLD_MAP[(self.y+1) as usize][self.x as usize] != 3 {
            let distance = target.euclid_distance(self.x, self.y+1);
            if distance < shortest {
                shortest = distance;
                tile = Position{x:self.x, y:self.y+1};
                dir = Direction::Down;
            }
	}		
        //right
        if self.x+1 < ARENA_WIDTH && 
            direction != Direction::Left &&
            WORLD_MAP[self.y as usize][(self.x+1) as usize] != 1 {
            let distance = target.euclid_distance(self.x+1, self.y);
            if distance < shortest {
                tile = Position{x:self.x+1, y:self.y};
                dir = Direction::Right;
            }
	}		
        (tile, dir)
    }
 
}

struct Size {
    width: f32,
    height: f32,
}
impl Size {
    fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}
impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
    fn quarter_cw(self) -> Self {
        match self {
            Self::Left => Self::Up,
            Self::Right => Self::Down,
            Self::Up => Self::Right,
            Self::Down => Self::Left,
        }
    }
    fn quarter_ccw(self) -> Self {
        match self {
            Self::Left => Self::Down,
            Self::Right => Self::Up,
            Self::Up => Self::Left,
            Self::Down => Self::Right,
        }
    }
}
#[derive(PartialEq, Copy, Clone, Debug)]
enum Mode {
    Chase1,
    Chase2,
    Scatter,
    // Scared,
}
impl Mode {
    fn next(self) -> Self {
        // let vs = vec![Self::Chase1, Self::Scatter];
        match self {
            Self::Chase1 => Self::Chase2,
            Self::Chase2 => Self::Scatter,
            _ => Self::Chase1,
            // Self::Scared => *vs.choose(&mut rand::thread_rng()).unwrap(),
        }
    }
}

struct Game{
    mode: Mode,
}
struct Pacman {
    direction: Direction,
    last: Position,
}

struct Ghost {
    direction: Direction,
    target: Position,
    scatter_target: Position,
}
struct Food {}
struct Energy {}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
    
    let wall_material = materials.add(Color::rgb(0.2, 0.6, 1.0).into());
    let food_material = materials.add(Color::rgb(1.0, 1.0, 1.0).into());
    let energy_material = materials.add(Color::rgb(1.0, 1.0, 1.0).into());
    let gate_material = materials.add(Color::rgb(0.5, 0.5, 0.5).into());
    for j in 0..31 {
        for i in 0..27 {
            if WORLD_MAP[j][i] == 0 ||
                WORLD_MAP[j][i] == 4 ||  
                WORLD_MAP[j][i] == 5 {
                commands
                    .spawn(SpriteBundle {
                        material: food_material.clone(),
                        ..Default::default()
                    })
                    .with(Food{})
                    .with(Position{x:i as i32, y:j as i32})
                    .with(Size::square(0.1));
            } else if WORLD_MAP[j][i] == 1 {
                commands
                    .spawn(SpriteBundle {
                        material: wall_material.clone(),
                        ..Default::default()
                    })
                    .with(Position{x:i as i32, y:j as i32})
                    .with(Size::square(1.0));
            } else if WORLD_MAP[j][i] == 2 {
                commands
                    .spawn(SpriteBundle {
                        material: energy_material.clone(),
                        ..Default::default()
                    })
                    .with(Energy{})
                    .with(Position{x:i as i32, y:j as i32})
                    .with(Size::square(0.4));
            } else if WORLD_MAP[j][i] == 3 {
                commands
                    .spawn(SpriteBundle {
                        material: gate_material.clone(),
                        ..Default::default()
                    })
                    .with(Position{x:i as i32, y:j as i32})
                    .with(Size::square(1.0));
            }
        }
    }
    let texture_handle = asset_server.load("pacman-sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(20.0, 20.0), 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            ..Default::default()
        })
        .with(Pacman{direction:Direction::Right, last: Position{x:13 as i32, y:23 as i32}})
        .with(Position{x:13 as i32, y:23 as i32})
        .with(Size::square(1.0))
        .with(Timer::from_seconds(0.1, true));
}


fn ghost_setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
    

    let pink_texture = asset_server.load("pinkghost-sheet.png");
    let pink_atlas = TextureAtlas::from_grid(pink_texture, Vec2::new(20.0, 20.0), 4, 1);
    let pink_atlas_handle = texture_atlases.add(pink_atlas);
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: pink_atlas_handle,
            ..Default::default()
        })
        .with(Ghost{direction: Direction::Up, target: Position{x:25, y:1}, scatter_target: Position{x:25, y:1}})
        .with(Position{x:13 as i32, y:14 as i32})
        .with(Size::square(1.0));

    let blue_texture = asset_server.load("blueghost-sheet.png");
    let blue_atlas = TextureAtlas::from_grid(blue_texture, Vec2::new(20.0, 20.0), 4, 1);
    let blue_atlas_handle = texture_atlases.add(blue_atlas);
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: blue_atlas_handle,
            ..Default::default()
        })
        .with(Ghost{direction: Direction::Down, target: Position{x:1, y:29}, scatter_target: Position{x:1, y:29}})
        .with(Position{x:12 as i32, y:14 as i32})
        .with(Size::square(1.0));

    let orange_texture = asset_server.load("orangeghost-sheet.png");
    let orange_atlas = TextureAtlas::from_grid(orange_texture, Vec2::new(20.0, 20.0), 4, 1);
    let orange_atlas_handle = texture_atlases.add(orange_atlas);
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: orange_atlas_handle,
            ..Default::default()
        })
        .with(Ghost{direction: Direction::Down, target: Position{x:25, y:29}, scatter_target: Position{x:25, y:29}})
        .with(Position{x:14 as i32, y:14 as i32})
        .with(Size::square(1.0));

    let red_texture = asset_server.load("redghost-sheet.png");
    let red_atlas = TextureAtlas::from_grid(red_texture, Vec2::new(20.0, 20.0), 4, 1);
    let red_atlas_handle = texture_atlases.add(red_atlas);
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: red_atlas_handle,
            ..Default::default()
        })
        .with(Ghost{direction: Direction::Left, target: Position{x:1, y:1}, scatter_target: Position{x:1, y:1}})
        .with(Position{x:13, y:11})
        .with(Size::square(1.0));
}


fn position_translation(mut q: Query<(&Position, &mut Transform)>) {
    fn translation(x: i32, y: i32) -> (i32, i32) {
        let (x2, y2): (i32, i32);
        if x < ARENA_WIDTH/2  {
            x2 = ((ARENA_WIDTH/2 - x ) * 20  + (20/2))  * -1
        } else {
            x2 = (x - ARENA_WIDTH/2) * 20  - (20/2)
        }
        if y < ARENA_HEIGHT/2  {
            y2 = (ARENA_HEIGHT/2 - y ) * 20  + (20/2)
        } else {
            y2 = ((y - ARENA_HEIGHT/2) * 20  - (20/2)) * -1
        }
        (x2, y2)
    }
    for (pos, mut transform) in q.iter_mut() {
        let (x, y): (i32, i32) = translation(pos.x, pos.y);
        transform.translation = Vec3::new(
            x as f32,
            y as f32,
            0.0,
        );
    }
}

fn size_scaling(mut q: Query<(&Size, &mut Sprite)>) {
    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            20 as f32 *sprite_size.width ,
            20 as f32 *sprite_size.height ,
        );
    }
}

struct PacmanMovementTimer(Timer);
fn sprite_timer(
    time: Res<Time>, 
    mut sprite_timer: ResMut<PacmanMovementTimer>
) {
    sprite_timer.0.tick(time.delta_seconds());
}

struct GhostMovementTimer(Timer);
fn ghost_timer(
    time: Res<Time>, 
    mut sprite_timer: ResMut<GhostMovementTimer>
) {
    sprite_timer.0.tick(time.delta_seconds());
}

struct GhostModeTimer(Timer);
fn ghost_mode_timer(
    time: Res<Time>, 
    mut ghost_mode_timer: ResMut<GhostModeTimer>
) {
    ghost_mode_timer.0.tick(time.delta_seconds());
}

fn pacman_eating(
    commands: &mut Commands,
    foods: Query<(Entity, &Position), With<Food>>,
    pacmans: Query<&Pacman>, 
){
    if let Some(pacman) = pacmans.iter().next() {
        for (ent, food_pos) in foods.iter() {
            if food_pos == &pacman.last {
                commands.despawn(ent);
            }
        }
    }
}

fn pacman_energy_boost(
    commands: &mut Commands,
    foods: Query<(Entity, &Position), With<Energy>>,
    pacmans: Query<(Entity, &Pacman)>, 
){
    if let Some((_, pacman)) = pacmans.iter().next() {
        for (ent, food_pos) in foods.iter() {
            if food_pos == &pacman.last {
                commands.despawn(ent);
            }
        }
    }
}


fn pacman_movement(
    keyboard_input: Res<Input<KeyCode>>,
    pacman_timer: ResMut<PacmanMovementTimer>,
    mut pacmans: Query<(Entity, &mut Pacman)>,
    mut positions: Query<&mut Position>,
    mut sprites: Query<(&TextureAtlasSprite, &mut Transform)>
) {
    if let Some((entity, mut pacman)) = pacmans.iter_mut().next() {
        let mut pos = positions.get_mut(entity).unwrap();   // when would i retrieve pacman like this vs querying directly in `sprites`?
        let (_, mut transform)= sprites.get_mut(entity).unwrap();
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            pacman.direction
        };

        if !pacman_timer.0.finished() {
            return;
        }

        if dir == pacman.direction.opposite() {
            transform.rotate(Quat::from_rotation_z(std::f32::consts::PI));
            pacman.direction = dir;
        } else if dir == pacman.direction.quarter_cw() {
            transform.rotate(Quat::from_rotation_z(-1. * std::f32::consts::PI / 2.));
            pacman.direction = dir;
        } else if dir == pacman.direction.quarter_ccw() {
            transform.rotate(Quat::from_rotation_z(std::f32::consts::PI / 2.));
            pacman.direction = dir;
        }

        pacman.last = *pos;
        if keyboard_input.pressed(KeyCode::Down) {
            if pos.y + 1 < 31 && WORLD_MAP[(pos.y+1) as usize][pos.x as usize] != 1 {
                pos.y += 1;
            }
        }
        if keyboard_input.pressed(KeyCode::Up) {
            if pos.y - 1 > -1 && WORLD_MAP[(pos.y-1) as usize][pos.x as usize] != 1 {
                pos.y -= 1;
            }
        }
        if keyboard_input.pressed(KeyCode::Right) {
            if pos.x + 1 < 27 && WORLD_MAP[pos.y as usize][(pos.x+1) as usize] != 1 {
                pos.x += 1;
            } else if pos.y == 14 && pos.x + 1 == 27 {
                pos.x = 0
            }
        }
        if keyboard_input.pressed(KeyCode::Left) {
            if pos.x - 1 > -1 && WORLD_MAP[pos.y as usize][(pos.x-1) as usize] != 1 {
                pos.x -= 1;
            } else if pos.y == 14 && pos.x - 1 == -1 {
                pos.x = 26
            }
        }
    }
}

fn pacman_animate(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&Pacman, &mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (_, mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta_seconds());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

fn ghost_movement(
    ghost_timer: ResMut<GhostMovementTimer>,
    mut ghosts: Query<(Entity, &mut Ghost)>,
    mut positions: Query<&mut Position>,
) {
    for (entity, mut ghost) in ghosts.iter_mut() {
        let mut pos = positions.get_mut(entity).unwrap();
        if !ghost_timer.0.finished() {
            return;
        }
        let (next_tile, next_dir) = pos.choose_next_tile(ghost.direction, ghost.target);
        ghost.direction = next_dir;

        if next_tile.y == 14 && next_tile.x + 1 == 27 {
            pos.x = 0
        } else if next_tile.y == 14 && next_tile.x -1 == -1 {
            pos.x = 26
        } else {
            pos.x = next_tile.x;
            pos.y = next_tile.y;
        }

    }
}


fn ghost_animate(
    mut query: Query<(&Ghost, &mut TextureAtlasSprite)>,
) {
    for (ghost, mut sprite) in query.iter_mut() {
        match ghost.direction {
            Direction::Left => sprite.index = 0,
            Direction::Up => sprite.index = 1,
            Direction::Right => sprite.index = 2,
            Direction::Down => sprite.index = 3,
        }
    }
}

fn ghost_mode(
    mut game: ResMut<Game>,
    ghost_mode_timer: ResMut<GhostModeTimer>,
){
    if !ghost_mode_timer.0.finished() {
        return;
    }
    game.mode = game.mode.next();
}


fn ghost_next_target(
    game: ResMut<Game>,
    mut ghosts: Query<(Entity, &mut Ghost)>,
    pacmans: Query<(Entity, &Pacman)>,
    ghost_timer: ResMut<GhostMovementTimer>,
    positions: Query<&Position>,
) {
    for (_entity, mut ghost) in ghosts.iter_mut() {
        if !ghost_timer.0.finished() {
            return;
        }
        if game.mode == Mode::Scatter {
            ghost.target = ghost.scatter_target; 
        }
        else if game.mode == Mode::Chase1 || game.mode == Mode::Chase2 {
            if let Some((pacman_entity, _)) = pacmans.iter().next() {
                ghost.target = *positions.get(pacman_entity).unwrap();
            }
        } 
    }
}

use bevy::prelude::*;
use std::cmp::min;


fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(WindowDescriptor{  // https://docs.rs/bevy/0.3.0/bevy/prelude/struct.WindowDescriptor.html
            title: "pacman".to_string(),
            ..Default::default()  // what is up with this syntax?
        })
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.system())
        .add_system(position_translation.system())
        .add_system(size_scaling.system())
        .add_system(animate_sprite_system.system())
        .run();
}

const ARENA_WIDTH: i32 = 27;
const ARENA_HEIGHT: i32 = 31;

const WORLD_MAP: [[i32; 27]; 31] = [
  [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
  [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
  [1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1], 
  [1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1], 
  [1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1], 
  [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], 
  [1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1], 
  [1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1], 
  [1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1], 
  [1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1], 
  [2, 2, 2, 2, 2, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 2, 2, 2, 2, 2], 
  [2, 2, 2, 2, 2, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 2, 2, 2, 2, 2], 
  [2, 2, 2, 2, 2, 1, 0, 1, 1, 0, 1, 1, 1, 3, 1, 1, 1, 0, 1, 1, 0, 1, 2, 2, 2, 2, 2], 
  [1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 3, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1], 
  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
  [1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1], 
  [2, 2, 2, 2, 2, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 2, 2, 2, 2, 2], 
  [2, 2, 2, 2, 2, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 2, 2, 2, 2, 2], 
  [2, 2, 2, 2, 2, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 2, 2, 2, 2, 2], 
  [1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1], 
  [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], 
  [1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1], 
  [1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1], 
  [1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1], 
  [1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1], 
  [1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1], 
  [1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1], 
  [1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1], 
  [1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1], 
  [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
  [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

// const WORLD_MAP: [[i32; 29]; 31] = [
//   [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1 ,1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
//   [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ,1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
//   [1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1 ,1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1], 
//   [1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1 ,1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1], 
//   [1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1 ,1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1], 
//   [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ,0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], 
//   [1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1 ,1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1], 
//   [1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1 ,1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1], 
//   [1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1 ,1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1], 
//   [1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1 ,1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1], 
//   [0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1 ,1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0], 
//   [0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0 ,0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0], 
//   [0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1 ,1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0], 
//   [1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1 ,1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1], 
//   [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0 ,0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
//   [1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1 ,1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1], 
//   [0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1 ,1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0], 
//   [0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0 ,0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0], 
//   [0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1 ,1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0], 
//   [1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1 ,1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1], 
//   [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ,1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], 
//   [1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1 ,1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1], 
//   [1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1 ,1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1], 
//   [1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0 ,0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1], 
//   [1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1 ,1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1], 
//   [1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1 ,1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1], 
//   [1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1 ,1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1], 
//   [1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1 ,1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1], 
//   [1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1 ,1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1], 
//   [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ,0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
//   [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1 ,1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
// ];

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)] // what the hell is this
struct Position {
    x: i32,
    y: i32,
}

struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

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
    let gate_material = materials.add(Color::rgb(0.5, 0.5, 0.5).into());
    for i in 0..31 {
        for j in 0..27 {
            if WORLD_MAP[i][j] == 1 {
                commands
                    .spawn(SpriteBundle {
                        material: wall_material.clone(),
                        ..Default::default()
                    })
                    .with(Position{x:i as i32, y:j as i32})
                    .with(Size::square(1.0));
            } else if WORLD_MAP[i][j] == 0 {
                commands
                    .spawn(SpriteBundle {
                        material: food_material.clone(),
                        ..Default::default()
                    })
                    .with(Position{x:i as i32, y:j as i32})
                    .with(Size::square(0.1));
            } else if WORLD_MAP[i][j] == 3 {
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
    let texture_handle = asset_server.load("textures/pacman-sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(20.0, 20.0), 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            ..Default::default()
        })
        .with(Position{x:23 as i32, y:13 as i32})
        .with(Size::square(1.0))
        .with(Timer::from_seconds(0.1, true));
}

// fn translation(x: i32, y: i32) -> (i32, i32) {
//     let (mut x2, y2): (i32, i32);
//     if x < ARENA_WIDTH/2  {
//         x2 = ((ARENA_WIDTH/2 - x ) * 20  + (20/2)) 
//     } else {
//         x2 = ((x - ARENA_WIDTH/2) * 20  - (20/2))  * -1
//     }
//     if y < ARENA_HEIGHT/2  {
//         y2 = ((ARENA_HEIGHT/2 - y ) * 20  + (20/2)) 
//     } else {
//         y2 = ((y - ARENA_HEIGHT/2) * 20  - (20/2))  * -1
//     }
//     (x2, y2)
// }

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        let (mut x, y): (i32, i32);
        if pos.x < ARENA_WIDTH/2  {
            x = ((ARENA_WIDTH/2 - pos.x ) * 20  + (20/2)) 
        } else {
            x = ((pos.x - ARENA_WIDTH/2) * 20  - (20/2))  * -1
        }
        if pos.y < ARENA_HEIGHT/2  {
            y = ((ARENA_HEIGHT/2 - pos.y ) * 20  + (20/2)) 
        } else {
            y = ((pos.y - ARENA_HEIGHT/2) * 20  - (20/2))  * -1
        }
        transform.translation = Vec3::new(
            y as f32,
            x as f32,
            0.0,
        );
    }
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();  // unwrap?
    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            20 as f32 *sprite_size.width ,
            20 as f32 *sprite_size.height ,
        );
    }
}

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta_seconds());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

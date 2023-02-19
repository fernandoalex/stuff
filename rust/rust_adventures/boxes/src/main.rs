// Where I stopped?
// Sorting tiles using ordering
use bevy::prelude::*;
use itertools::Itertools;
use rand::prelude::*;
use std::convert::TryFrom;

const TILE_SIZE: f32 = 40.0;
const TILE_SPACER: f32 = 10.0;

#[derive(Default, Component)]
struct Board {
    size: u8,
    physical_size: f32,
}

impl Board {
    fn new(size: u8) -> Self {
        let physical_size = f32::from(size) * TILE_SIZE
            + f32::from(size + 1) * TILE_SPACER;

        Board {
            size,
            physical_size,
        }
    }

    fn cell_position_to_physical(&self, pos: u8) -> f32 {
        let offset = 
            -self.physical_size / 2.0 + 0.5 * TILE_SIZE;

        offset
            + f32::from(pos) * TILE_SIZE
            + f32::from(pos + 1) * TILE_SPACER
    }
}

struct Materials {
    pub board: Color,
    pub tile_placeholder: Color,
    pub tile: Color,
    // pub none: Color,
}

const MATERIALS: Materials = Materials {
    board: Color::rgb(0.7, 0.7, 0.8),
    tile_placeholder: Color::rgb(0.65, 0.65, 0.9),
    tile: Color::rgb(0.9, 0.9, 1.0),
    // none: Color::NONE,
};

#[derive(Default, Component)]
struct Points {
    value: u32,
}

#[derive(Default, Component)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(Default, Component)]
struct TileText;

#[derive(Resource)]
struct FontSpec {
    family: Handle<Font>,
}

impl FromWorld for FontSpec {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource_mut::<AssetServer>()
            .unwrap();

        FontSpec {
            family: asset_server
                .load("fonts/FiraSans-Bold.ttf"),
        }
    }

}
enum BoardShift {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<&KeyCode> for BoardShift {
    type Error = &'static str;

    fn try_from(value: &KeyCode) -> Result<Self, Self::Error> {
        match value {
            KeyCode::H => Ok(BoardShift::Left),
            KeyCode::J => Ok(BoardShift::Up),
            KeyCode::L => Ok(BoardShift::Right),
            KeyCode::K => Ok(BoardShift::Down),
            KeyCode::Left => Ok(BoardShift::Left),
            KeyCode::Up => Ok(BoardShift::Up),
            KeyCode::Right => Ok(BoardShift::Right),
            KeyCode::Down => Ok(BoardShift::Down),
            _ => Err("not a valid board_shift key"),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // setup here is the `fn setup` reason we pass it this way is because
        // of "extension traits", bevy uses the type signature of the function
        // to "add" some stuff to it
        .init_resource::<FontSpec>()
        .add_startup_system(setup)
        .add_startup_system(spawn_board)
        .add_startup_system_to_stage(
            StartupStage::PostStartup, 
            spawn_tiles
        )
        .add_system(render_tile_points)
        .add_system(board_shift)
        .run();
}

fn setup(mut commands: Commands) {
    // This will send a command to the "world" struct
    commands
        .spawn(Camera2dBundle::default());
}

fn spawn_board(mut commands: Commands) {
    let board = Board::new(4);

    commands
        .spawn(SpriteBundle {
            sprite:  Sprite {
                color: MATERIALS.board,
                custom_size: Some(Vec2::new(
                    board.physical_size,
                    board.physical_size,
                )),
                // ..Default::default() is a trait that will fill the rest with default values
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            // let offset = -board.physical_size / 2.0
            //     + 0.5 * TILE_SIZE;

            for tile in (0..board.size)
                .cartesian_product(0..board.size)
            {
                    builder.spawn(SpriteBundle {
                        sprite: Sprite { 
                            color: MATERIALS.tile_placeholder, 
                            custom_size: Some(Vec2::new(
                                TILE_SIZE, TILE_SIZE
                            )),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(
                            board.cell_position_to_physical(tile.0), 
                            board.cell_position_to_physical(tile.1), 
                            1.0
                        ),
                        ..Default::default()
                    });
            }
        })
        .insert(board);
}

fn spawn_tiles (
    mut commands: Commands,
    query_board: Query<&Board>,
    font_spec: Res<FontSpec>,
) {
    let board = query_board.single();

    let mut rng = rand::thread_rng();

    let starting_tiles: Vec<(u8, u8)> = (0..board.size)
        .cartesian_product(0..board.size)
        .choose_multiple(&mut rng, 2);

    for (x, y) in starting_tiles.iter() {
        let pos = Position { x: *x, y: *y };

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: MATERIALS.tile,
                    custom_size: Some(Vec2::new(
                        TILE_SIZE, TILE_SIZE
                    )),
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    board.cell_position_to_physical(pos.x),
                    board.cell_position_to_physical(pos.y),
                    1.0,
                ),
                ..Default::default()
            })
            .with_children(|child_builder| {
                child_builder
                    .spawn(Text2dBundle {
                    text: Text::from_section(
                        "2", 
                        TextStyle {
                            font: font_spec
                                .family
                                .clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                            ..Default::default()
                        })
                        .with_alignment(TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                        }),
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    ..Default::default()
                })
                .insert(TileText);
            })
            .insert(Points { value: 2 })
            .insert(pos);
    }
}

fn render_tile_points(
    mut texts: Query<&mut Text, With<TileText>>,
    tiles: Query<(&Points, &Children)>,
) {
    for (points, children) in tiles.iter() {
        if let Some(entity) = children.first() {
           let mut text = texts
                .get_mut(*entity)
                .expect("expected Text to exist");

            let mut text_section = text
                .sections
                .first_mut()
                .expect("expect first section to be accessible as mutable");

            text_section.value = points.value.to_string()
        }
    }
}

fn board_shift (keyboard_input: Res<Input<KeyCode>>) {
    let shift_direction = 
        keyboard_input.get_just_pressed().find_map(
            |key_code| BoardShift::try_from(key_code).ok(),
        );
    
    match shift_direction {
        Some(BoardShift::Left) => {
            dbg!("left");
        }
        Some(BoardShift::Right) => {
            dbg!("right");
        }
        Some(BoardShift::Up) => {
            dbg!("up");
        }
        Some(BoardShift::Down) => {
            dbg!("down");
        }
        None => (),
    }
}

use bevy::prelude::*;

const TILE_SIZE: f32 = 40.0;

#[derive(Default, Component)]
struct Board {
    size: u8,
}

struct Materials {
    pub board: Color,
    // pub tile_placeholder: Color,
    // pub tile: Color,
    // pub none: Color,
}

const MATERIALS: Materials = Materials {
    board: Color::rgb(0.7, 0.7, 0.8),
    // tile_placeholder: Color::rgb(0.75, 0.75, 0.9),
    // tile: Color::rgb(0.9, 0.9, 1.0),
    // none: Color::NONE,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // setup here is the `fn setup` reason we pass it this way is because
        // of "extension traits", bevy uses the type signature of the function
        // to "add" some stuff to it
        .add_startup_system(setup)
        .add_startup_system(spawn_board)
        .run();
}

fn setup(mut commands: Commands) {
    // This will send a command to the "world" struct
    commands
        .spawn(Camera2dBundle::default());
}

fn spawn_board(mut commands: Commands) {
    let board = Board { size: 4 };
    let physical_board_size =
        f32::from(board.size) * TILE_SIZE;

    commands
        .spawn(SpriteBundle {
            sprite:  Sprite {
                color: MATERIALS.board,
                custom_size: Some(Vec2::new(
                    physical_board_size,
                    physical_board_size,
                )),
                // ..Default::default() is a trait that will fill the rest with default values
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(board);
}

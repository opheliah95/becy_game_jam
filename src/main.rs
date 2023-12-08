use bevy::{prelude::*, render::camera::ScalingMode};

#[derive(Component)]
pub struct Player {
    pub speed: f32,  
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("character.png");
    
    let mut camera = Camera2dBundle::default();
    // setup min scale
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    // spawn a sprite
    commands.spawn(
       (
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            texture,
            ..default()
        },
        Player {speed: 100.0},
       )
    );
}

fn character_movement (
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    for(mut transform, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();
        if input.pressed(KeyCode::W) {
            transform.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= movement_amount;
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= movement_amount;
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += movement_amount;
        }
    }
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Jam Game".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                ..default()
                })
                .build())
    .add_systems(Startup, setup)
    .add_systems(Update, character_movement)
    .run();
}

use std::f32::consts::PI;
use bevy::input::ElementState;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

use crate::bullet::Bullet;
use crate::Direction;

#[derive(Component)]
pub struct Guy {
    pub direction: Direction
}

fn init_guy(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Initializing guy");
    let sprite = asset_server.load("images/person-circle.png");

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(SpriteBundle {
            texture: sprite,
            transform: Transform {
                scale: Vec3::new(0.1, 0.1,  0.1),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Guy { direction: Direction::Down });
}

fn rotate_guy(
    mut keyboard_input: EventReader<KeyboardInput>,
    mut query: Query<(&mut Guy, &mut Transform)>
) {
    let (mut guy, mut transform) = query.single_mut();

    for key in keyboard_input.iter() {
        if key.state == ElementState::Pressed {
            match key.key_code {
                Some(KeyCode::W) | Some(KeyCode::Up) => {
                    guy.direction = Direction::Up;
                    transform.rotation = Quat::from_rotation_z(0.)
                }
                Some(KeyCode::A) | Some(KeyCode::Left) => {
                    guy.direction = Direction::Left;
                    transform.rotation = Quat::from_rotation_z(PI / 2.)
                }
                Some(KeyCode::S) | Some(KeyCode::Down) => {
                    guy.direction = Direction::Down;
                    transform.rotation = Quat::from_rotation_z(PI)
                }
                Some(KeyCode::D) | Some(KeyCode::Right) => {
                    guy.direction = Direction::Right;
                    transform.rotation = Quat::from_rotation_z(-PI / 2.)
                }
                _ => {}
            }
        }
    }
}

fn fire_bullet (
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    guy_query: Query<(&mut Guy, &mut Transform)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("Firing bullet");
        let (guy, transform) = guy_query.single();

        let position = Vec2::new(
            transform.translation.x,
            transform.translation.y
        );
    
        let mut bullet = commands.spawn();
    
        bullet
            .insert(Bullet {
                direction: guy.direction
            })
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(5., 5.)),
                    ..Default::default()
                },
                ..Default::default()
            });

        println!("Bullet position: {:?}", position);
    }
    
}

pub struct GuyPlugin;

impl Plugin for GuyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(init_guy)
            .add_system(rotate_guy)
            .add_system(fire_bullet);
    }
}
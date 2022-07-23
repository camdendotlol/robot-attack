use std::f32::consts::PI;
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
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Guy, &mut Transform)>
) {
    let (mut guy, mut transform) = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::W) | keyboard_input.just_pressed(KeyCode::Up) {
        println!("Rotating guy up");
        guy.direction = Direction::Up;
        transform.rotation = Quat::from_rotation_z(0.)
    }

    if keyboard_input.just_pressed(KeyCode::A) | keyboard_input.just_pressed(KeyCode::Left) {
        println!("Rotating guy left");
        guy.direction = Direction::Left;
        transform.rotation = Quat::from_rotation_z(PI / 2.)
    }

    if keyboard_input.just_pressed(KeyCode::S) | keyboard_input.just_pressed(KeyCode::Down) {
        println!("Rotating guy down");
        guy.direction = Direction::Down;
        transform.rotation = Quat::from_rotation_z(PI)
    }

    if keyboard_input.just_pressed(KeyCode::D) | keyboard_input.just_pressed(KeyCode::Right) {
        println!("Rotating guy right");
        guy.direction = Direction::Right;
        transform.rotation = Quat::from_rotation_z(PI + PI / 2.)
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
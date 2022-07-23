use bevy::prelude::*;

use crate::Direction;

#[derive(Component)]
pub struct Bullet {
    pub direction: Direction
}

fn move_bullet(
    mut bullet_query: Query<(Entity, &Bullet, &mut Transform)>,
    mut commands: Commands,
    window: Res<WindowDescriptor>
) {
    for (entity, bullet, mut transform) in bullet_query.iter_mut() {
        match bullet.direction {
            Direction::Up => {
                transform.translation.y += 3.;
            }
            Direction::Down => {
                transform.translation.y -= 3.;
            }
            Direction::Left => {
                transform.translation.x -= 3.;
            }
            Direction::Right => {
                transform.translation.x += 3.;
            }
        }

        if transform.translation.x.abs() > window.width / 2. || transform.translation.y.abs() > window.height / 2. {
            println!("Bullet {} out of bounds, destroying", entity.id());
            commands.entity(entity).despawn();
        }
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(move_bullet);
    }
}
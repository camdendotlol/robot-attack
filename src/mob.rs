use bevy::prelude::*;
use rand::Rng;

enum MobType {
    Bug,
    Robot
}

enum Origin {
    Top,
    Bottom,
    Left,
    Right
}

#[derive(Component)]
pub struct Mob {
    mob_type: MobType,
    health: u32,
    origin: Origin
}

fn pick_direction() -> Origin {
    let mut rng = rand::thread_rng();

    let num: u8 = rng.gen_range(0..4);

    return match num {
        0 => Origin::Top,
        1 => Origin::Bottom,
        2 => Origin::Left,
        3 => Origin::Right,
        _ => Origin::Top
    };
}

fn get_spawn_point(origin: &Origin, window: Res<WindowDescriptor>) -> Vec3 {
    return match origin {
        Origin::Top => Vec3::new(0., (window.height / 2.) -1., 0.),
        Origin::Bottom => Vec3::new(0., ((window.height / 2.) -1.) * -1., 0.),
        Origin::Left => Vec3::new(((window.width / 2.) -1.) * -1., 0., 0.),
        Origin::Right => Vec3::new((window.width / 2.) -1., 0., 0.)
    };
}

fn spawn_mobs(mut commands: Commands, asset_server: Res<AssetServer>, window: Res<WindowDescriptor>) {
    let mut rng = rand::thread_rng();
    let num: u16 = rng.gen_range(0..1000);

    // 0.4% spawn chance per frame
    if num < 996 {
        return;
    }

    let mob_type = match rand::random() {
        true => MobType::Bug,
        false => MobType::Robot,
    };

    let sprite = match mob_type {
        MobType::Bug => asset_server.load("images/bug.png"),
        MobType::Robot => asset_server.load("images/robot.png"),
    };

    let health = match mob_type {
        MobType::Bug => 10,
        MobType::Robot => 20,
    };

    let origin = pick_direction();

    commands.spawn_bundle(SpriteBundle {
        texture: sprite,
        transform: Transform {
            scale: Vec3::new(0.2, 0.2,  0.2),
            translation: get_spawn_point(&origin, window),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Mob {
        mob_type,
        health,
        origin
    });
}

fn move_mob(
    mut mob_query: Query<(Entity, &Mob, &mut Transform)>,
    mut commands: Commands,
    window: Res<WindowDescriptor>
) {
    for (entity, mob, mut transform) in mob_query.iter_mut() {
        match mob.origin {
            Origin::Top => transform.translation.y -= 0.1,
            Origin::Bottom => transform.translation.y += 0.1,
            Origin::Left => transform.translation.x += 0.1,
            Origin::Right => transform.translation.x -= 0.1
        }

        if transform.translation.x == 0. && transform.translation.y == 0. {
            println!("Ouch! Player hit!");
            commands.entity(entity).despawn();
        }

        if transform.translation.x.abs() > window.width / 2. || transform.translation.y.abs() > window.height / 2. {
            println!("Mob {} out of bounds, destroying", entity.id());
            commands.entity(entity).despawn();
        }
    }
}

pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_mobs)
            .add_system(move_mob);
    }
}
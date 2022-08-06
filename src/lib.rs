use bevy::prelude::*;
use mob::MobPlugin;

use crate::guy::GuyPlugin;
use crate::ui::UiPlugin;
use crate::bullet::BulletPlugin;

mod bullet;
mod guy;
mod mob;
mod ui;

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(BulletPlugin)
            .add_plugin(GuyPlugin)
            .add_plugin(MobPlugin)
            .add_plugin(UiPlugin);
    }
}

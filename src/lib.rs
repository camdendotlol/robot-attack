use bevy::prelude::*;

use crate::guy::GuyPlugin;
use crate::ui::UiPlugin;
use crate::bullet::BulletPlugin;

mod guy;
mod ui;
mod bullet;

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
            .add_plugin(UiPlugin)
            .add_plugin(GuyPlugin)
            .add_plugin(BulletPlugin);
    }
}

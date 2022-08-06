use bevy::prelude::*;

pub struct Health {
    pub hp: usize,
    pub extra: usize,
}

pub struct GameState {
    health: Health,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            health: Health { hp: 100, extra: 0 },
        }
    }
}

#[derive(Component)]
pub struct HealthText;

fn init_health(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    game_state: Res<GameState>,
) {
    println!("Initializing health");
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    right: Val::Px(10.),
                    bottom: Val::Px(10.),
                    ..Default::default()
                },
                margin: UiRect {
                    right: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: format!("Health: {}", game_state.health.hp),
                                style: TextStyle {
                                    font_size: 40.0,
                                    color: Color::BEIGE,
                                    font: font.clone(),
                                },
                            },
                            TextSection {
                                value: "  ".to_string(),
                                style: TextStyle {
                                    font_size: 40.0,
                                    color: Color::NONE,
                                    font: font.clone(),
                                },
                            },
                            TextSection {
                                value: format!("Extra: {}", game_state.health.extra),
                                style: TextStyle {
                                    font_size: 30.0,
                                    color: Color::BEIGE,
                                    font,
                                },
                            },
                        ],
                        alignment: Default::default(),
                    },
                    ..Default::default()
                })
                .insert(HealthText);
        });
}

#[derive(Default)]
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState::default())
            .add_startup_system(init_health);
    }
}

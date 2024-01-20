

use bevy::{app::AppExit, prelude::*};

use crate::{utils::{despawn_with_component, common_button_system}, constants::{BACKGROUND, TEXT_COLOR}, common_entity::EntitySpawner, GameState, game::GameScoresRes};


#[derive(Component)]
struct GameOverMenuScreen;

// All actions that can be triggered from a button click
#[derive(Component)]
enum GameOverMenuButtonAction {
    Back,
    Quit,
}

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::GameOver), gameover_menu_setup)
            .add_systems(
                OnExit(GameState::GameOver),
                despawn_with_component::<GameOverMenuScreen>,
            )
            .add_systems(Update, (menu_action, common_button_system));
    }
}

fn gameover_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>, game_scores_stored: Res<GameScoresRes>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            GameOverMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::px(20., 20., 10., 30.),
                        ..default()
                    },
                    background_color: BACKGROUND.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(
                        TextBundle::from_section(
                            "GAME OVER",
                            TextStyle {
                                font: font.clone(),
                                font_size: 80.0,
                                color: TEXT_COLOR,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );

                    let game_score = format!("Score : {:}   Level : {:}   Lines : {:}", game_scores_stored.score, game_scores_stored.level, game_scores_stored.lines);
                    parent.spawn(
                        TextBundle::from_section(
                            game_score,
                            TextStyle {
                                font: font.clone(),
                                font_size: 30.0,
                                color: TEXT_COLOR,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        }),
                    );

                    parent.spawn_button(GameOverMenuButtonAction::Back, "right.png", "Main Menu", &asset_server);
                    parent.spawn_button(GameOverMenuButtonAction::Quit, "exitRight.png", "Quit", &asset_server);
                });
        });
}

#[allow(unused_mut)]
#[allow(unused_variables)]
fn menu_action(
    interaction_query: Query<
        (&Interaction, &GameOverMenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                GameOverMenuButtonAction::Quit => {
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        app_exit_events.send(AppExit);
                    }
                }
                GameOverMenuButtonAction::Back => {
                    game_state.set(GameState::Menu);
                }
            }
        }
    }
}
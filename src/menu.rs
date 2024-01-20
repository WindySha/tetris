

use bevy::{app::AppExit, prelude::*};

use crate::{
    common_entity::EntitySpawner,
    constants::{BACKGROUND, TEXT_COLOR},
    utils::{common_button_system, despawn_with_component},
    GameState,
};

pub struct MenuPlugin;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum GameSelectedLevel {
    Easy,
    Normal,
    Hard,
}

#[derive(Resource)]
pub struct GameLevelRes(pub GameSelectedLevel);

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameLevelRes(GameSelectedLevel::Easy))
            .add_systems(OnEnter(GameState::Menu), main_menu_setup)
            .add_systems(
                OnExit(GameState::Menu),
                despawn_with_component::<OnMainMenuScreen>,
            )
            .add_systems(Update, (menu_action, common_button_system));
    }
}

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMainMenuScreen;

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    EasyPlay,
    NormalPlay,
    HardPlay,
    Help,
    Quit,
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::px(120., 120., 10., 30.),
                        ..default()
                    },
                    background_color: BACKGROUND.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(
                        TextBundle::from_section(
                            "TETRIS",
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

                    // Display three buttons for each action available from the main menu:
                    // - Easy Mode
                    // - Normal Mode
                    // - Hard Mode
                    // - Help
                    // - quit
                    parent.spawn_button(
                        MenuButtonAction::EasyPlay,
                        "right.png",
                        "Easy",
                        &asset_server,
                    );
                    parent.spawn_button(
                        MenuButtonAction::NormalPlay,
                        "right.png",
                        "Normal",
                        &asset_server,
                    );
                    parent.spawn_button(
                        MenuButtonAction::HardPlay,
                        "right.png",
                        "Hard",
                        &asset_server,
                    );
                    parent.spawn_button(
                        MenuButtonAction::Help,
                        "wrench.png",
                        "How To Play",
                        &asset_server,
                    );
                    parent.spawn_button(
                        MenuButtonAction::Quit,
                        "exitRight.png",
                        "Quit",
                        &asset_server,
                    );
                });
        });
}

#[allow(unused_mut)]
#[allow(unused_variables)]
fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
    mut game_level: ResMut<GameLevelRes>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit =>
                {
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        app_exit_events.send(AppExit);
                    }
                }
                MenuButtonAction::EasyPlay => {
                    game_state.set(GameState::Game);
                    game_level.0 = GameSelectedLevel::Easy;
                }
                MenuButtonAction::NormalPlay => {
                    game_state.set(GameState::Game);
                    game_level.0 = GameSelectedLevel::Normal;
                }
                MenuButtonAction::HardPlay => {
                    game_state.set(GameState::Game);
                    game_level.0 = GameSelectedLevel::Hard;
                }
                MenuButtonAction::Help => {
                    game_state.set(GameState::HelpMenu);
                }
            }
        }
    }
}

use bevy::{prelude::*};

use crate::{utils::{despawn_with_component, common_button_system}, constants::{BACKGROUND, TEXT_COLOR}, common_entity::EntitySpawner, GameState};


#[derive(Component)]
struct MenuHelpScreen;

#[derive(Component)]
enum GameOverMenuHelpButtonAction {
    Back,
}

pub struct MenuHelpPlugin;

impl Plugin for MenuHelpPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::HelpMenu), help_menu_setup)
            .add_systems(
                OnExit(GameState::HelpMenu),
                despawn_with_component::<MenuHelpScreen>,
            )
            .add_systems(Update, (menu_action, common_button_system));
    }
}

fn help_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            MenuHelpScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::px(20., 20., 10., 10.),
                        ..default()
                    },
                    background_color: BACKGROUND.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(
                        TextBundle::from_section(
                            "CONTROLS",
                            TextStyle {
                                font: font.clone(),
                                font_size: 60.0,
                                color: TEXT_COLOR,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        }),
                    );

                    let game_score = format!("
                    Left : move left    \n
                    Right : move right                 \n
                    Up : rotate \n
                    Down : soft drop     \n
                    Space : hard drop      \n
                    Esc : pause game       \n");
                    parent.spawn(
                        TextBundle::from_section(
                            game_score,
                            TextStyle {
                                font: font.clone(),
                                font_size: 40.0,
                                color: TEXT_COLOR,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(0.0)),
                            ..default()
                        }),
                    );

                    parent.spawn_button(GameOverMenuHelpButtonAction::Back, "right.png", "Back", &asset_server);
                });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &GameOverMenuHelpButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                GameOverMenuHelpButtonAction::Back => {
                    game_state.set(GameState::Menu);
                }
            }
        }
    }
}
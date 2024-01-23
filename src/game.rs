use bevy::{
    prelude::{OnEnter, OnExit, Plugin, Update},
    window::WindowFocused,
};

use crate::{
    board::Board,
    brick::{Brick, BrickType},
    constants::{BOARD_VIEW_X, BOARD_VIEW_Y, BOARD_X, BOARD_Y, GAME_DATA_TEXT_COLOR},
    data::PauseStateRes,
    menu::{GameLevelRes, GameSelectedLevel},
    position::Position,
    utils::{despawn_with_component, get_level, get_score, get_speed},
    GameState,
};

use lazy_static::*;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
    time::Duration,
};

use bevy::{prelude::*, sprite::Anchor};

pub(crate) const BLOCK_INSET: f32 = 1.;
pub(crate) const BLOCK_WIDTH: f32 = 36.;

pub(crate) const DEFAULT_EASY_FALLING_SPEED: f32 = 1.1; // easy mode speed
pub(crate) const DEFAULT_NORMAL_FALLING_SPEED: f32 = 0.725; // normal mode speed
pub(crate) const DEFAULT_HARD_FALLING_SPEED: f32 = 0.4; // hard mode speed
pub(crate) const MAX_FALLING_SPEED: f32 = 0.1; // max speed

#[derive(Resource)]
pub struct GameScoresRes {
    pub level: usize,
    pub score: usize,
    pub lines: usize,
}

static ENABLE_SHOWING_SHADOW_BRICK: AtomicBool = AtomicBool::new(true);
static ENABLE_SHOWING_BOARD_LINES: AtomicBool = AtomicBool::new(true);
static ENABLE_7_BAG_RANDOMIZATION: AtomicBool = AtomicBool::new(true);

lazy_static! {
    pub static ref BRICK_COLOR_MAP: HashMap<BrickType, String> = HashMap::from([
        (BrickType::O, "#CDCD00".to_string()),
        (BrickType::I, "#00CDCD".to_string()),
        (BrickType::J, "#0000CD".to_string()),
        (BrickType::L, "#CD6600".to_string()),
        (BrickType::S, "#00CD00".to_string()),
        (BrickType::Z, "#CD0000".to_string()),
        (BrickType::T, "#9A00CD".to_string()),
        (BrickType::None, "#484848".to_string()),
    ]);
    static ref BRICK_FALLING_SPEED: Mutex<f32> = Mutex::new(1.0);
}

#[derive(Component)]
struct BoardBundle(Board);

#[derive(Component)]
struct GameInfoBundle;

#[derive(Component)]

struct NextBrickTitleBundle;

#[derive(Component)]
struct MovingBrickBundle {
    moving_pos: Position,
    brick: Brick,
    movable: bool,
}

#[derive(Component)]
struct ShadowBrickBundle;

#[derive(Component)]
struct NextBrickBundle(Brick);

#[derive(Component)]
struct ScoreText(usize);

#[derive(Component)]
struct LinesText(usize);

#[derive(Component)]
struct LevelText(usize);

#[derive(Debug, Resource)]
struct DropTimer(Timer);

pub struct GamePlugin;

// https://strategywiki.org/wiki/Tetris/Rotation_systems
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameScoresRes {
            level: 0,
            score: 0,
            lines: 0,
        })
        .insert_resource(PauseStateRes::new(false, false))
        .add_systems(
            Update,
            ((
                brick_fall_down_system,
                apply_deferred,
                rotate_brick_key_event,
                apply_deferred,
                hard_drop_key_event,
                apply_deferred,
                move_brick_key_event,
                apply_deferred,
                soft_drop_key_event,
            )
                .chain(),)
                .run_if(is_not_pause_state.and_then(in_state(GameState::Game))),
        )
        .add_systems(
            Update,
            pause_state_changed_event.run_if(in_state(GameState::Game)),
        )
        .add_systems(
            OnEnter(GameState::Game),
            (setup_game_data, apply_deferred, setup_tetris).chain(),
        )
        .add_systems(
            OnExit(GameState::Game),
            despawn_with_component::<BoardBundle>,
        )
        .add_systems(
            OnExit(GameState::Game),
            despawn_with_component::<GameInfoBundle>,
        )
        .add_systems(
            OnExit(GameState::Game),
            despawn_with_component::<NextBrickTitleBundle>,
        )
        .add_systems(
            OnExit(GameState::Game),
            despawn_with_component::<NextBrickBundle>,
        )
        .add_systems(
            OnExit(GameState::Game),
            despawn_with_component::<ShadowBrickBundle>,
        )
        .insert_resource(DropTimer(Timer::new(
            Duration::from_secs_f32(DEFAULT_NORMAL_FALLING_SPEED),
            TimerMode::Repeating,
        )));
    }
}

// init some game datas
fn setup_game_data(
    game_level: Res<GameLevelRes>,
    mut drop_timer: ResMut<DropTimer>,
    mut game_scores_stored: ResMut<GameScoresRes>,
) {
    let duration: f32;
    match game_level.0 {
        GameSelectedLevel::Easy => {
            duration = DEFAULT_EASY_FALLING_SPEED;
            let _ = ENABLE_SHOWING_SHADOW_BRICK.compare_exchange(
                false,
                true,
                Ordering::Relaxed,
                Ordering::Relaxed,
            );
            let _ = ENABLE_SHOWING_BOARD_LINES.compare_exchange(
                false,
                true,
                Ordering::Relaxed,
                Ordering::Relaxed,
            );
            let _ = ENABLE_7_BAG_RANDOMIZATION.compare_exchange(
                false,
                true,
                Ordering::Relaxed,
                Ordering::Relaxed,
            );
        }
        GameSelectedLevel::Normal => {
            duration = DEFAULT_NORMAL_FALLING_SPEED;
            let _ = ENABLE_SHOWING_SHADOW_BRICK.compare_exchange(
                true,
                false,
                Ordering::Relaxed,
                Ordering::Relaxed,
            );
            let _ = ENABLE_SHOWING_BOARD_LINES.compare_exchange(
                false,
                true,
                Ordering::Relaxed,
                Ordering::Relaxed,
            );
            let _ = ENABLE_7_BAG_RANDOMIZATION.compare_exchange(
                false,
                true,
                Ordering::Relaxed,
                Ordering::Relaxed,
            );
        }
        GameSelectedLevel::Hard => {
            duration = DEFAULT_HARD_FALLING_SPEED;
            let _ = ENABLE_SHOWING_SHADOW_BRICK.compare_exchange(
                true,
                false,
                Ordering::Relaxed,
                Ordering::Relaxed,
            );
            let _ = ENABLE_SHOWING_BOARD_LINES.compare_exchange(
                true,
                false,
                Ordering::Relaxed,
                Ordering::Relaxed,
            );
            let _ = ENABLE_7_BAG_RANDOMIZATION.compare_exchange(
                true,
                false,
                Ordering::Relaxed,
                Ordering::Relaxed,
            );
        }
    }

    *BRICK_FALLING_SPEED.lock().unwrap() = duration;

    drop_timer.0.set_duration(Duration::from_secs_f32(
        *BRICK_FALLING_SPEED.lock().unwrap(),
    ));

    // reset score data
    game_scores_stored.level = 1;
    game_scores_stored.score = 0;
    game_scores_stored.lines = 0;
}

fn setup_tetris(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_board(&mut commands, &Board::new());
    spawn_game_info(&mut commands, &asset_server);
    spawn_next_brick_title(&mut commands, &asset_server);
    spawn_next_brick(&mut commands, Brick::new(ENABLE_7_BAG_RANDOMIZATION.load(Ordering::Relaxed)));
}

#[inline]
fn create_brick_start_position(brick_type: &BrickType) -> Position {
    let mut pos = Position {
        x: (BOARD_X / 2) as i32 - 2,
        y: BOARD_Y as i32 - 6,
    };
    match brick_type {
        BrickType::Z | BrickType::S | BrickType::I => pos.y += 1,
        _ => (),
    }
    pos
}

fn brick_fall_down_system(
    mut commands: Commands,
    time: Res<Time>,
    mut drop_timer: ResMut<DropTimer>,
    mut moving_brick_query: Query<(Entity, &mut MovingBrickBundle, &mut Transform)>,
    shadow_brick_query: Query<Entity, With<ShadowBrickBundle>>,
    mut board_query: Query<(Entity, &mut BoardBundle)>,
    next_brick_query: Query<(Entity, &mut NextBrickBundle)>,
    mut text_query: ParamSet<(
        Query<(&mut Text, &mut ScoreText)>,
        Query<(&mut Text, &mut LinesText)>,
        Query<(&mut Text, &mut LevelText)>,
    )>,
    mut play_state: ResMut<NextState<GameState>>,
    mut game_scores_stored: ResMut<GameScoresRes>,
) {
    if !drop_timer.0.tick(time.delta()).finished() {
        return;
    }
    let mut board = board_query.get_single_mut().unwrap();
    let Ok(next_brick) = next_brick_query.get_single() else {
        return;
    };

    let create_new_next_brick_func = |commands: &mut Commands| {
        commands.entity(next_brick.0).despawn_recursive();
        spawn_next_brick(commands, Brick::new(ENABLE_7_BAG_RANDOMIZATION.load(Ordering::Relaxed)));
    };

    let Ok(mut moving_brick) = moving_brick_query.get_single_mut() else {
        // in the initial state, no moving exists, so create a new one and the shadow brick
        let brick = &next_brick.1 .0;
        let init_brick_position = create_brick_start_position(&brick.0);

        let bottom_pos = board
            .1
             .0
            .get_bottom_valid_brick_pos(brick, &init_brick_position);

        spawn_brick(&mut commands, brick, &init_brick_position);
        spawn_shadow_brick(&mut commands, brick, &bottom_pos);

        // recreate the next brick
        create_new_next_brick_func(&mut commands);
        return;
    };

    let is_next_moving_valid = board
        .1
         .0
        .is_valid_brick(&moving_brick.1.brick, &moving_brick.1.moving_pos.down());
    if is_next_moving_valid {
        // do the falling down
        moving_brick.1.moving_pos.down_assign();
        moving_brick.2.translation.y -= BLOCK_WIDTH;
    } else {
        // make the brick occupy the board.
        board
            .1
             .0
            .occupy_brick(moving_brick.1.brick, moving_brick.1.moving_pos);
        let cleaned_lines = board.1 .0.clean_lines();

        commands.entity(moving_brick.0).despawn_recursive();
        commands.entity(board.0).despawn_recursive();

        if let Ok(shadow_bricks) = shadow_brick_query.get_single() {
            commands.entity(shadow_bricks).despawn_recursive();
        }

        spawn_board(&mut commands, &board.1 .0);

        let init_brick_position = create_brick_start_position(&next_brick.1 .0 .0);

        let is_new_created_brick_valid = board
            .1
             .0
            .is_valid_brick(&next_brick.1 .0, &init_brick_position);
        if !is_new_created_brick_valid {
            // game over
            play_state.set(GameState::GameOver);
            return;
        }

        let bottom_pos = board
            .1
             .0
            .get_bottom_valid_brick_pos(&next_brick.1 .0, &init_brick_position);

        spawn_brick(&mut commands, &next_brick.1 .0, &init_brick_position);
        spawn_shadow_brick(&mut commands, &next_brick.1 .0, &bottom_pos);

        create_new_next_brick_func(&mut commands);

        if cleaned_lines > 0 {
            let mut all_cleaned_lines = 0;
            let mut level = 1;
            let mut scores = 0;
            if let Ok(mut text) = text_query.p1().get_single_mut() {
                text.1 .0 += cleaned_lines;
                text.0.sections[0].value = format!("{:}", text.1 .0);
                all_cleaned_lines = text.1 .0;
            }

            if let Ok(mut text) = text_query.p2().get_single_mut() {
                text.1 .0 = get_level(all_cleaned_lines);
                text.0.sections[0].value = format!("{:}", text.1 .0);
                level = text.1 .0;
            }

            if let Ok(mut text) = text_query.p0().get_single_mut() {
                text.1 .0 += get_score(level, cleaned_lines);
                text.0.sections[0].value = format!("{:}", text.1 .0);
                scores = text.1 .0;
            }

            // store the data, it will be used when the game is finished.
            game_scores_stored.level = level;
            game_scores_stored.score = scores;
            game_scores_stored.lines = all_cleaned_lines;
        }
        // reset falling speed to default
        if let Ok(text) = text_query.p2().get_single() {
            let level = text.1 .0;
            let speed = *BRICK_FALLING_SPEED.lock().unwrap();
            drop_timer
                .0
                .set_duration(Duration::from_secs_f32(get_speed(level, speed)));
        }
    }
}

fn soft_drop_key_event(
    keyboard_input: Res<Input<KeyCode>>,
    level_query: Query<&LevelText>,
    mut drop_timer: ResMut<DropTimer>,
) {
    if keyboard_input.just_pressed(KeyCode::J) || keyboard_input.just_pressed(KeyCode::Down) {
        if drop_timer.0.duration().as_secs_f32() > MAX_FALLING_SPEED {
            drop_timer
                .0
                .set_duration(Duration::from_secs_f32(MAX_FALLING_SPEED));
        }
    } else if keyboard_input.just_released(KeyCode::J)
        || keyboard_input.just_released(KeyCode::Down)
    {
        let level: usize = level_query.get_single().unwrap().0;
        let seted_speed = *BRICK_FALLING_SPEED.lock().unwrap();
        let speed = get_speed(level, seted_speed);

        drop_timer.0.set_duration(Duration::from_secs_f32(speed));
    }
}

fn hard_drop_key_event(
    keyboard_input: Res<Input<KeyCode>>,
    mut moving_brick_query: Query<(Entity, &mut MovingBrickBundle, &mut Transform)>,
    board_query: Query<&mut BoardBundle>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let Ok(mut moving_brick) = moving_brick_query.get_single_mut() else {
            return;
        };
        let Ok(board) = board_query.get_single() else {
            return;
        };
        let current_brick = moving_brick.1.brick;
        let moving_pos = &mut moving_brick.1.moving_pos;

        while board.0.is_valid_brick(&current_brick, &moving_pos.down()) {
            moving_pos.down_assign();
            moving_brick.2.translation.y -= BLOCK_WIDTH;
        }
        // hard dropped, then it cannot be moved again
        moving_brick.1.movable = false;
    }
}

fn rotate_brick_key_event(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut moving_brick_query: Query<(Entity, &mut MovingBrickBundle, &mut Transform)>,
    shadow_brick_query: Query<Entity, With<ShadowBrickBundle>>,
    board_query: Query<&mut BoardBundle>,
) {
    if !(keyboard_input.just_pressed(KeyCode::Up)) {
        return;
    }

    // use "let else"
    let Ok(mut moving_brick) = moving_brick_query.get_single_mut() else {
        return;
    };
    let Ok(board) = board_query.get_single() else {
        return;
    };

    // when the brick fast dropped down, it become non movable.
    if !moving_brick.1.movable {
        return;
    }

    let rotated_brick = moving_brick.1.brick.rotate_right();
    let moving_pos = &mut moving_brick.1.moving_pos;

    if board
        .0
        .is_valid_brick_for_rotation(&rotated_brick, moving_pos)
    {
        let bottom_pos = board
            .0
            .get_bottom_valid_brick_pos(&rotated_brick, moving_pos);
        spawn_brick(&mut commands, &rotated_brick, moving_pos);
        spawn_shadow_brick(&mut commands, &rotated_brick, &bottom_pos);
        if let Ok(shadow_bricks) = shadow_brick_query.get_single() {
            commands.entity(shadow_bricks).despawn_recursive();
        }
        commands.entity(moving_brick.0).despawn_recursive();
    }
}

fn move_brick_key_event(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut moving_brick_query: Query<(Entity, &mut MovingBrickBundle, &mut Transform)>,
    shadow_brick_query: Query<Entity, With<ShadowBrickBundle>>,
    board_query: Query<&mut BoardBundle>,
) {
    if !(keyboard_input.just_pressed(KeyCode::Left) || keyboard_input.just_pressed(KeyCode::Right))
    {
        return;
    }

    // use "let else"
    let Ok(mut moving_brick) = moving_brick_query.get_single_mut() else {
        return;
    };
    let Ok(board) = board_query.get_single() else {
        return;
    };

    // when the brick fast dropped down, it become non movable.
    if !moving_brick.1.movable {
        return;
    }

    let is_left_moving = keyboard_input.just_pressed(KeyCode::Left);

    let current_brick = moving_brick.1.brick;
    let moving_pos = moving_brick.1.moving_pos;

    let next_pos = if is_left_moving {
        moving_pos.left()
    } else {
        moving_pos.right()
    };

    if board.0.is_valid_brick(&current_brick, &next_pos) {
        moving_brick.1.moving_pos = next_pos;
        if is_left_moving {
            moving_brick.2.translation.x -= BLOCK_WIDTH;
        } else {
            moving_brick.2.translation.x += BLOCK_WIDTH;
        }
        // dismiss shadow brick
        if let Ok(shadow_bricks) = shadow_brick_query.get_single() {
            commands.entity(shadow_bricks).despawn_recursive();
        }
        let bottom_pos = board
            .0
            .get_bottom_valid_brick_pos(&current_brick, &next_pos);
        // redraw shadow brick
        spawn_shadow_brick(&mut commands, &current_brick, &bottom_pos);
    }
}

#[inline]
fn spawn_brick(commands: &mut Commands, brick: &Brick, moving_pos: &Position) {
    let board_width = BLOCK_WIDTH * BOARD_VIEW_X as f32;
    let board_height = BLOCK_WIDTH * BOARD_VIEW_Y as f32;
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(
                moving_pos.x as f32 * BLOCK_WIDTH - board_width / 2.0 + BLOCK_WIDTH / 2.,
                moving_pos.y as f32 * BLOCK_WIDTH - board_height / 2.0 + BLOCK_WIDTH / 2.,
                0.1,
            ),
            ..default()
        })
        .insert(MovingBrickBundle {
            moving_pos: *moving_pos,
            brick: brick.clone(),
            movable: true,
        })
        .with_children(|parent| {
            for pos in brick.1 {
                let color = Color::hex(&BRICK_COLOR_MAP[&brick.0]).unwrap();
                parent.spawn(sprite_bundle(
                    BLOCK_WIDTH,
                    color,
                    position_to_vec2(&pos, 0.3),
                ));
            }
        });
}

#[inline]
fn spawn_shadow_brick(commands: &mut Commands, brick: &Brick, shadow_pos: &Position) {
    if !ENABLE_SHOWING_SHADOW_BRICK.load(Ordering::Relaxed) {
        return;
    }

    let board_width = BLOCK_WIDTH * BOARD_VIEW_X as f32;
    let board_height = BLOCK_WIDTH * BOARD_VIEW_Y as f32;
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(
                shadow_pos.x as f32 * BLOCK_WIDTH - board_width / 2.0 + BLOCK_WIDTH / 2.,
                shadow_pos.y as f32 * BLOCK_WIDTH - board_height / 2.0 + BLOCK_WIDTH / 2.,
                0.0,
            ),
            ..default()
        })
        .insert(ShadowBrickBundle)
        .with_children(|parent| {
            for pos in brick.1 {
                let color = Color::rgb_u8(90, 90, 90);
                parent.spawn(sprite_bundle(
                    BLOCK_WIDTH,
                    color,
                    position_to_vec2(&pos, 0.2),
                ));
            }
        });
}

#[inline]
fn position_to_vec2(pos: &Position, z: f32) -> Vec3 {
    Vec3::new(BLOCK_WIDTH * pos.x as f32, BLOCK_WIDTH * pos.y as f32, z)
}

#[inline]
fn sprite_bundle(width: f32, color: Color, trans: Vec3) -> SpriteBundle {
    SpriteBundle {
        transform: Transform {
            translation: trans,
            ..default()
        },
        sprite: Sprite {
            color,
            custom_size: Some(Vec2::new(width - BLOCK_INSET, width - BLOCK_INSET)),
            ..default()
        },
        ..default()
    }
}

fn spawn_board(commands: &mut Commands, board: &Board) {
    let board_width = BLOCK_WIDTH * BOARD_VIEW_X as f32;
    let board_height = BLOCK_WIDTH * BOARD_VIEW_Y as f32;

    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(
                (-board_width) / 2. + BLOCK_WIDTH / 2.,
                (-board_height) / 2. + BLOCK_WIDTH / 2.,
                0.0, //zero,which one pixel behind the UI-screen png; cannot be seen in screen
            ),
            ..default()
        })
        .insert(BoardBundle(board.clone()))
        .with_children(|parent| {
            let line_color: Color;
            if ENABLE_SHOWING_BOARD_LINES.load(Ordering::Relaxed) {
                line_color = Color::rgb_u8(32, 31, 30);
            } else {
                // use the color of the default brick, then the lines cannot be seen
                line_color = Color::hex(&BRICK_COLOR_MAP[&BrickType::None]).unwrap();
            }
            // this is the background color view
            parent.spawn(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        (board_width) / 2. - BLOCK_WIDTH / 2.,
                        (board_height) / 2. - BLOCK_WIDTH / 2.,
                        0.0,
                    ),
                    ..default()
                },
                sprite: Sprite {
                    color: line_color,
                    custom_size: Some(Vec2::new(
                        board_width + BLOCK_INSET,
                        board_height + BLOCK_INSET,
                    )),
                    ..default()
                },
                ..default()
            });
            // show all the bricks
            for x in 0..BOARD_VIEW_X {
                for y in 0..BOARD_VIEW_Y {
                    let color = Color::hex(&BRICK_COLOR_MAP[&board.0[x][y]]).unwrap();
                    parent.spawn(sprite_bundle(
                        BLOCK_WIDTH,
                        color,
                        Vec3::new(x as f32 * BLOCK_WIDTH, y as f32 * BLOCK_WIDTH, 0.1),
                    ));
                }
            }
        });
}

fn spawn_game_info(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let board_width = BLOCK_WIDTH * BOARD_X as f32;
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(-(board_width / 2. + 80.), 0., 0.),
                ..default()
            },
            GameInfoBundle,
        ))
        .with_children(|parent| {
            let up_margin: f32 = 60.;
            let top_y: f32 = 180.;
            let x: f32 = -50.;
            parent.spawn(create_text_bundle("SCORE", x, top_y, &asset_server));
            parent
                .spawn(create_text_bundle("0", x, top_y - up_margin, &asset_server))
                .insert(ScoreText(0));
            parent.spawn(create_text_bundle(
                "LEVEL",
                x,
                top_y - 2. * up_margin,
                &asset_server,
            ));
            parent
                .spawn(create_text_bundle(
                    "1",
                    x,
                    top_y - 3. * up_margin,
                    &asset_server,
                ))
                .insert(LevelText(1));
            parent.spawn(create_text_bundle(
                "LINES",
                x,
                top_y - 4. * up_margin,
                &asset_server,
            ));
            parent
                .spawn(create_text_bundle(
                    "0",
                    x,
                    top_y - 5. * up_margin,
                    &asset_server,
                ))
                .insert(LinesText(0));
        });
}

fn spawn_next_brick_title(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let board_width = BLOCK_WIDTH * BOARD_VIEW_X as f32;

    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(board_width / 2. + 100., 180., 0.),
                ..default()
            },
            NextBrickTitleBundle,
        ))
        .with_children(|parent| {
            parent.spawn(create_text_bundle("NEXT", 0., 0., &asset_server));
        });
}

fn spawn_next_brick(commands: &mut Commands, brick: Brick) {
    let board_width = BLOCK_WIDTH * BOARD_X as f32;
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(board_width / 2. + 70., 10., 0.),
            ..default()
        })
        .with_children(|parent| {
            for pos in brick.1 {
                let color = Color::hex(&BRICK_COLOR_MAP[&brick.0]).unwrap();
                parent.spawn(sprite_bundle(
                    BLOCK_WIDTH,
                    color,
                    position_to_vec2(&pos, 0.1),
                ));
            }
        })
        .insert(NextBrickBundle(brick));
}

fn create_text_bundle(msg: &str, x: f32, y: f32, asset_server: &Res<AssetServer>) -> Text2dBundle {
    Text2dBundle {
        text: Text::from_section(
            msg,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 42.0,
                color: GAME_DATA_TEXT_COLOR,
            },
        )
        .with_alignment(TextAlignment::Center),
        transform: Transform {
            translation: Vec3::new(x, y, 0.),
            ..default()
        },
        text_anchor: Anchor::TopCenter,
        ..default()
    }
}

pub fn is_not_pause_state(pause_state: Res<PauseStateRes>) -> bool {
    !pause_state.is_pause_state()
}

pub fn pause_state_changed_event(
    keyboard_input: Res<Input<KeyCode>>,
    mut events: EventReader<WindowFocused>,
    mut pause_state: ResMut<PauseStateRes>,
) {
    for event in events.read() {
        pause_state.lose_focus_pause = !event.focused;
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        pause_state.user_click_pause = !pause_state.user_click_pause;
    }
}

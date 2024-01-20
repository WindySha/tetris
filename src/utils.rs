#![allow(dead_code)]

use bevy::prelude::*;

use crate::constants::{PRESSED_BUTTON, HOVERED_PRESSED_BUTTON, HOVERED_BUTTON, NORMAL_BUTTON};

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_with_component<T: Component>(
    to_despawn: Query<Entity, With<T>>,
    mut commands: Commands,
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn in_two_state<S1: States, S2: States>(
    state1: S1,
    state2: S2,
) -> impl FnMut(Res<State<S1>>, Res<State<S2>>) -> bool + Clone {
    move |current_state1: Res<State<S1>>, current_state2: Res<State<S2>>| {
        *current_state1 == state1 && *current_state2 == state2
    }
}

// Tag component used to mark which setting is currently selected
#[derive(Component)]
pub struct SelectedOption;

// This system handles changing all buttons color based on mouse interaction
pub fn common_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

///tetris scoring  
///use as [Original Nintendo Scoring System]
///https://tetris.fandom.com/wiki/Scoring
#[inline]
pub fn get_score(level: usize, erase_lines: usize) -> usize {
    assert!(0 < erase_lines);
    assert!(erase_lines <= 4);
    vec![40, 100, 300, 1200][(erase_lines - 1) as usize] * level
}

///level  
///increase level every 10 lines.
#[inline]
pub fn get_level(total_lines: usize) -> usize {
    (total_lines / 10 + 1).min(99)
}

///tetris speeding  
///delay = 725 * .85 ^ level + level (ms)
///use formula from dwhacks, http://gist.github.com/dwhacks/8644250
#[inline]
pub fn get_speed(level: usize, default_speed: f32) -> f32 {
    default_speed * (0.85_f32).powi(level as i32) + level as f32 / 1000.0
    // default_speed
}

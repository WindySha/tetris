
use bevy::prelude::Color;
use lazy_static::*;
use std::collections::HashMap;
use crate::{brick::*, position::Position};

pub const WINDOW_WIDTH: f32 = 1100.;
pub const WINDOW_HEIGHT: f32 = 800.;

pub(crate) const BOARD_VIEW_X: usize = 10;
pub(crate) const BOARD_VIEW_Y: usize = 20;

pub(crate) const BOARD_X: usize = 10;
pub(crate) const BOARD_Y: usize = 23; // board is 10x20

lazy_static! {

    pub static ref BRICKS_MAP: HashMap<BrickType, Vec<Brick>> = HashMap::from([
        //O:
        (BrickType::O, vec![Brick(BrickType::O, [Position::new(1, 1), Position::new(1, 2), Position::new(2, 1), Position::new(2, 2)])]),
        //I:
        (BrickType::I, vec![
            Brick(BrickType::I, [Position::new(0, 1), Position::new(1, 1), Position::new(2, 1), Position::new(3, 1)]),
            Brick(BrickType::I, [Position::new(2, 0), Position::new(2, 1), Position::new(2, 2), Position::new(2, 3)])
        ]),
        //J:
        (BrickType::J, vec![
            Brick(BrickType::J, [Position::new(0, 1), Position::new(1, 1), Position::new(2, 1), Position::new(0, 2)]),
            Brick(BrickType::J, [Position::new(1, 0), Position::new(1, 1), Position::new(1, 2), Position::new(2, 2)]),
            Brick(BrickType::J, [Position::new(0, 1), Position::new(1, 1), Position::new(2, 1), Position::new(2, 0)]),
            Brick(BrickType::J, [Position::new(1, 0), Position::new(1, 1), Position::new(1, 2), Position::new(0, 0)]),
        ]),
        //L:
        (BrickType::L, vec![
            Brick(BrickType::L, [Position::new(0, 1), Position::new(1, 1), Position::new(2, 1), Position::new(2, 2)]),
            Brick(BrickType::L, [Position::new(1, 0), Position::new(1, 1), Position::new(1, 2), Position::new(2, 0)]),
            Brick(BrickType::L, [Position::new(0, 1), Position::new(1, 1), Position::new(2, 1), Position::new(0, 0)]),
            Brick(BrickType::L, [Position::new(1, 0), Position::new(1, 1), Position::new(1, 2), Position::new(0, 2)]),
        ]),
        //S:
        (BrickType::S, vec![
            Brick(BrickType::S, [Position::new(0, 0), Position::new(1, 0), Position::new(1, 1), Position::new(2, 1)]),
            Brick(BrickType::S, [Position::new(1, 2), Position::new(1, 1), Position::new(2, 1), Position::new(2, 0)]),
        ]),
        //Z:
        (BrickType::Z, vec![
            Brick(BrickType::Z, [Position::new(0, 1), Position::new(1, 1), Position::new(1, 0), Position::new(2, 0)]),
            Brick(BrickType::Z, [Position::new(2, 2), Position::new(2, 1), Position::new(1, 1), Position::new(1, 0)]),
        ]),
        //T:
        (BrickType::T, vec![
            Brick(BrickType::T,[Position::new(0, 1), Position::new(1, 1), Position::new(2, 1), Position::new(1, 2)]),
            Brick(BrickType::T, [Position::new(1, 0), Position::new(1, 1), Position::new(1, 2), Position::new(2, 1)]),
            Brick(BrickType::T, [Position::new(0, 1), Position::new(1, 1), Position::new(2, 1), Position::new(1, 0)]),
            Brick(BrickType::T,[Position::new(1, 0), Position::new(1, 1), Position::new(1, 2), Position::new(0, 1)]),
        ]),
    ]);
}

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
pub const BACKGROUND: Color = Color::rgb(0.27, 0.43, 0.8);

pub const BACKGROUND_COLOR: Color = Color::rgb(0.61, 0.7, 0.71);

pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const GAME_DATA_TEXT_COLOR: Color = Color::rgb(0., 0.22, 0.76);




use rand::prelude::*;

use crate::constants::BRICKS_MAP;
use crate::position::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BrickType {
    O,
    I,
    J,
    L,
    S,
    Z,
    T,
    None,
}

#[derive(Copy, Clone, Debug)]
pub struct Brick(pub BrickType, pub [Position; 4]);

impl Brick {
    pub fn new() -> Self {
        let type_size = BRICKS_MAP.len();
        // choose a brick type
        let type_index: usize = rand::thread_rng().gen_range(0..type_size);
        let brick_type = match type_index {
            0 => BrickType::O,
            1 => BrickType::I,
            2 => BrickType::J,
            3 => BrickType::L,
            4 => BrickType::S,
            5 => BrickType::Z,
            6 => BrickType::T,
            _ => BrickType::J,
        };

        let brick_kind: &Vec<Brick> = BRICKS_MAP.get(&brick_type).unwrap();
        // choose a rotation type
        // let rotate_index: usize = rand::thread_rng().gen_range(0..brick_kind.len());
        brick_kind[0]
    }

    pub fn rotate_right(&self) -> Self {
        self.rotate(true)
    }

    fn rotate(&self, is_right: bool) -> Self {
        let brick_type = self.0;
        let brick_kind: &Vec<Brick> = BRICKS_MAP.get(&brick_type).unwrap();
        let mut rotate_index = 0;

        let rotate_size = brick_kind.len();
        for ro_index in 0..rotate_size {
            if brick_kind[ro_index].1 == self.1 {
                rotate_index = ro_index;
                break;
            }
        }

        let rotate_len = brick_kind.len();
        if is_right {
            brick_kind[(rotate_index + 1) % rotate_len]
        } else {
            brick_kind[(rotate_index + rotate_len - 1) % rotate_len]
        }
    }
}
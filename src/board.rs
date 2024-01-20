#![allow(dead_code)]

use crate::{
    brick::{Brick, BrickType},
    constants::BOARD_X,
    constants::BOARD_Y,
    position::Position,
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Board(pub [[BrickType; BOARD_Y]; BOARD_X]);

impl Board {
    pub fn new() -> Self {
        Board([[BrickType::None; BOARD_Y]; BOARD_X])
    }

    pub fn occupy_brick(&mut self, brick: Brick, moving_pos: Position) {
        let len: usize = self.0.len();
        let len_y: usize = self.0[0].len();
        for pos in brick.1 {
            let real_pos = pos + moving_pos;
            if (real_pos.x as usize) < len && (real_pos.y as usize) < len_y {
                self.0[real_pos.x as usize][real_pos.y as usize] = brick.0;
            }
        }
    }

    pub fn is_position_occupied(&self, pos: &Position) -> bool {
        self.0[pos.x as usize][pos.y as usize] != BrickType::None
    }

    pub fn is_brick_conflicted(&self, brick: &Brick, moving_pos: &Position) -> bool {
        for pos in brick.1 {
            let real_pos = pos + *moving_pos;
            if self.0[real_pos.x as usize][real_pos.y as usize] != BrickType::None {
                return true;
            }
        }
        return false;
    }

    pub fn is_position_in_board(&self, pos: &Position) -> bool {
        let len = self.0.len();
        let len_y = self.0[0].len();
        0 <= pos.x && (pos.x as usize) < len && (pos.y as usize) < len_y - 3 && pos.y >= 0
    }

    pub fn is_brick_in_board(&self, brick: &Brick, moving_pos: &Position) -> bool {
        for pos in brick.1 {
            let real_pos = pos + *moving_pos;
            if !self.is_position_in_board(&real_pos) {
                return false;
            }
        }
        return true;
    }

    pub fn is_valid_position(&self, pos: &Position) -> bool {
        self.is_position_in_board(pos) && !self.is_position_occupied(pos)
    }

    pub fn is_valid_brick(&self, brick: &Brick, moving_pos: &Position) -> bool {
        self.is_brick_in_board(brick, moving_pos) && !self.is_brick_conflicted(brick, moving_pos)
    }

    pub fn get_bottom_valid_brick_pos(&self, brick: &Brick, moving_pos: &Position) -> Position {
        let mut bottom_pos = moving_pos.clone();
        loop {
            if !self.is_valid_brick(brick, &bottom_pos) {
                if bottom_pos.y < moving_pos.y {
                    bottom_pos.up_assign();
                }
                return bottom_pos;
            } else {
                bottom_pos.down_assign();
            }
        }
    }

    pub fn clear_board(&mut self) {
        for row in self.0.iter_mut() {
            for col in row.iter_mut() {
                *col = BrickType::None;
            }
        }
    }

    pub fn is_line_empty(&self, y: usize) -> bool {
        let len = self.0.len();
        for x in 0..len {
            if self.0[x][y] != BrickType::None {
                return false;
            }
        }
        return true;
    }

    pub fn is_line_full(&self, y: usize) -> bool {
        let len: usize = self.0.len();
        for x in 0..len {
            if self.0[x][y] == BrickType::None {
                return false;
            }
        }
        return true;
    }

    pub fn get_full_lines(&self) -> Vec<usize> {
        let mut vec = Vec::with_capacity(4);
        let len_y = self.0[0].len();
        // must save it from big to small, because we must clear the lines from top to bottom.
        for y in (0..len_y).rev() {
            if self.is_line_full(y) {
                vec.push(y);
            }
        }
        vec
    }

    pub fn clean_one_line(&mut self, y_pos: usize) {
        let len: usize = self.0.len();
        let len_y: usize = self.0[0].len();
        for y in y_pos..len_y {
            if y > y_pos && self.is_line_empty(y) {
                break;
            }
            for x in 0..len {
                if y == len_y - 1 {
                    self.0[x][y_pos] = BrickType::None;
                } else {
                    self.0[x][y] = self.0[x][y + 1];
                }
            }
        }
    }

    pub fn clean_lines(&mut self) -> usize {
        let full_lines = &self.get_full_lines();
        for y in full_lines {
            self.clean_one_line(*y);
        }
        full_lines.len()
    }

    pub fn is_valid_brick_for_rotation(&self, brick: &Brick, moving_pos: &mut Position) -> bool {
        if self.is_valid_brick(brick, moving_pos) {
            return true;
        }
        let origin_moving_pos = *moving_pos;
        let mut change_moving_position = |padding_x: i32, padding_y: i32| -> bool {
            let new_moving_pos = *moving_pos + Position::new(padding_x, padding_y);
            if self.is_valid_brick(brick, &new_moving_pos) {
                *moving_pos = new_moving_pos;
                return true;
            }
            false
        };

        if brick.0 == BrickType::I && brick.1[0] == Position::new(0, 1) {
            // try to move the I shape to left or right by 1 or 2 blocks
            let padding_size = [-1, 1, 2];
            for padding in padding_size {
                if change_moving_position(padding, 0) {
                    return true;
                }
            }
        }
        let is_z_brick = brick.0 == BrickType::Z && brick.1[0] == Position::new(0, 1);
        let is_s_brick = brick.0 == BrickType::S && brick.1[0] == Position::new(0, 0);
        if is_z_brick || is_s_brick {
            // check brick's first block is invalid and others are valid
            for (i, brick_pos) in brick.1.iter().enumerate() {
                let new_pos = *brick_pos + origin_moving_pos;
                if i == 0 && self.is_valid_position(&new_pos) {
                    return false;
                }
                if i > 0 && !self.is_valid_position(&new_pos) {
                    return false;
                }
            }
            let padding = 1;
            if change_moving_position(padding, 0) {
                return true;
            }
        }
        if brick.0 == BrickType::L && brick.1[3] == Position::new(2, 2) {
            let mut is_invalid = false;
            let check_brick_pos = [2, 3];
            for i in check_brick_pos {
                let brick_pos = &brick.1[i];
                let new_pos = *brick_pos + origin_moving_pos;
                if !self.is_valid_position(&new_pos) {
                    is_invalid = true;
                    break;
                }
            }
            if is_invalid {
                let padding = -1;
                if change_moving_position(padding, 0) {
                    return true;
                }
            }
        }
        if brick.0 == BrickType::L && brick.1[3] == Position::new(0, 0) {
            let mut is_invalid = false;
            let check_brick_pos = [0, 1];
            for i in check_brick_pos {
                let brick_pos = &brick.1[i];
                let new_pos = *brick_pos + origin_moving_pos;
                if !self.is_valid_position(&new_pos) {
                    is_invalid = true;
                    break;
                }
            }
            if is_invalid {
                let padding = 1;
                if change_moving_position(padding, 0) {
                    return true;
                }
            }
        }
        if brick.0 == BrickType::J && brick.1[3] == Position::new(2, 0) {
            let new_pos = brick.1[0] + origin_moving_pos;

            if !self.is_valid_position(&new_pos) {
                let padding = 1;
                if change_moving_position(padding, 0) {
                    return true;
                }
            }
        }
        if brick.0 == BrickType::J && brick.1[3] == Position::new(0, 2) {
            let new_pos = brick.1[2] + origin_moving_pos;
            if !self.is_valid_position(&new_pos) {
                let padding = -1;
                if change_moving_position(padding, 0) {
                    return true;
                }
            }
        }
        if brick.0 == BrickType::T && brick.1[3] == Position::new(1, 2) {
            for (i, brick_pos) in brick.1.iter().enumerate() {
                let new_pos = *brick_pos + origin_moving_pos;
                if i == 2 && self.is_valid_position(&new_pos) {
                    return false;
                }
                if i != 2 && !self.is_valid_position(&new_pos) {
                    return false;
                }
            }
            let padding = -1;
            if change_moving_position(padding, 0) {
                return true;
            }
        }
        if brick.0 == BrickType::T && brick.1[3] == Position::new(1, 0) {
            for (i, brick_pos) in brick.1.iter().enumerate() {
                let new_pos = *brick_pos + origin_moving_pos;
                if i == 0 && self.is_valid_position(&new_pos) {
                    return false;
                }
                if i != 0 && !self.is_valid_position(&new_pos) {
                    return false;
                }
            }
            let padding = 1;
            if change_moving_position(padding, 0) {
                return true;
            }
        }

        // the following codes will deal with the case where
        // the brick position is on the top of the board
        if brick.0 == BrickType::I && brick.1[0] == Position::new(2, 0) {
            let padding_size = [-1, -2];
            for padding in padding_size {
                if change_moving_position(0, padding) {
                    return true;
                }
            }
        }

        if (brick.0 == BrickType::Z && brick.1[0] == Position::new(2, 2))
            || (brick.0 == BrickType::S && brick.1[0] == Position::new(1, 2))
        {
            if change_moving_position(0, -1) {
                return true;
            }
        }
        return false;
    }
}

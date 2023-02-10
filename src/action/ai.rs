
use super::*;

use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

pub struct AI{

}

impl AI{

    pub fn get_move(board: &Board, player: Player) -> [Point;2] {
        Action::get_possible_moves(board, player).choose(&mut thread_rng()).unwrap().clone()
    }

    pub fn get_drop(board: &Board, player: Player) -> Point {
        Action::get_possible_drops(board, player).choose(&mut thread_rng()).unwrap().clone()
    }

}
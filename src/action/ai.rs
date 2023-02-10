
use super::*;

use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

pub struct AI{

}

impl AI{

    pub fn get_move(board: &Board, player: Player) -> [Point;2] {
        let mut possible_moves = Vec::<[Point;2]>::new();
        for from in board.get_player_brick_points(player) {
            possible_moves.extend(Action::get_moves(board, from));
        }
        possible_moves.choose(&mut thread_rng()).unwrap().clone()
    }

    pub fn get_drop(board: &Board, player: Player) -> Point {
        let mut possible_drops = Vec::<Point>::new();
        for from in board.get_player_brick_points(player) {
            possible_drops.extend(Action::get_drops(board, from));
        }
        possible_drops.choose(&mut thread_rng()).unwrap().clone()
    }

}
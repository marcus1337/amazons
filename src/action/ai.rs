use super::*;

use rand::seq::SliceRandom;
use rand::thread_rng;
//use rand::Rng;

#[repr(C)]
pub struct AI {}

impl AI {
    pub fn get_action(turn: &Turn) -> Action {
        let points = turn.get_valid_actions()
            .choose(&mut thread_rng())
            .unwrap()
            .clone();
            Action::new(Point::get_from(points), Point::get_to(points))
    }
}

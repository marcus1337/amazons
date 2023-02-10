
extern crate libc;
pub mod board;
pub mod action;

use board::point::Point;

#[repr(C)]
pub struct Amazons {

}

impl Amazons{

    #[no_mangle]
    pub extern "C" fn amazons_make() -> Self {
        Self {
        }
    }
    #[no_mangle]
    pub extern "C" fn amazons_reset(&mut self) {
        *self = Amazons::amazons_make();
    }

    #[no_mangle]
    pub extern "C" fn amazons_is_game_over(&mut self) -> bool {
        false
    }
    #[no_mangle]
    pub extern "C" fn amazons_is_one_winner(&mut self) -> bool {
        false
    }
    #[no_mangle]
    pub extern "C" fn amazons_is_two_winner(&mut self) -> bool {
        false
    }
    #[no_mangle]
    pub extern "C" fn amazons_can_undo(&self) -> bool {
        false
    }
    #[no_mangle]
    pub extern "C" fn amazons_can_redo(&self) -> bool {
        false
    }
    #[no_mangle]
    pub extern "C" fn amazons_undo(&mut self) {
    }
    #[no_mangle]
    pub extern "C" fn amazons_redo(&mut self) {
    }
    #[no_mangle]
    pub extern "C" fn amazons_is_started(&self) -> bool {
        false
    }
    #[no_mangle]
    pub extern "C" fn amazons_is_white_turn(&self) -> bool {
        false
    }

    #[no_mangle]
    pub extern "C" fn amazons_get_last_move_or_drop(&self) -> [Point;2] {
        [Point::new(0,0),Point::new(0,0)]
    }
    #[no_mangle]
    pub extern "C" fn amazons_get_num_stored_actions(&self) -> i32 {
        0
    }
    #[no_mangle]
    pub extern "C" fn amazons_get_stored_move_or_drop(&self, action_number: i32) -> [Point;2] {
        [Point::new(0,0),Point::new(0,0)]
    }
    #[no_mangle]
    pub extern "C" fn amazons_has_brick(&self, point: Point) -> bool {
        false
    }
    #[no_mangle]
    pub extern "C" fn amazons_has_player_brick(&self, point: Point) -> bool {
        false
    }
    #[no_mangle]
    pub extern "C" fn amazons_has_player_one_brick(&self, point: Point) -> bool {
        false
    }
    #[no_mangle]
    pub extern "C" fn amazons_can_move(&self, from: Point, to: Point) -> bool {
        false
    }
    #[no_mangle]
    pub extern "C" fn amazons_can_drop(&self, point: Point) -> bool {
        false
    }
    #[no_mangle]
    pub extern "C" fn amazons_move(&self, from: Point, to: Point) {
    }
    #[no_mangle]
    pub extern "C" fn amazons_drop(&self, point: Point) {
    }
    #[no_mangle]
    pub extern "C" fn amazons_get_ai_move_or_drop(&self) -> [Point; 2] {
        [Point::new(0,0),Point::new(0,0)]
    }

}
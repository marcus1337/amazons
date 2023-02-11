
extern crate libc;
pub mod board;
pub mod action;
pub mod turn;

use board::point::Point;
//use action::history;
use turn::Turn;
use board::tile::Player;
use action::ai::AI;
use action::Action;


#[repr(C)]
pub struct Amazons {
    pub turn: Turn,

}

impl Amazons{

    #[no_mangle]
    pub extern "C" fn amazons_make() -> Self {
        Self {
            turn: Turn::new(),
        }
    }
    #[no_mangle]
    pub extern "C" fn amazons_reset(&mut self) {
        *self = Amazons::amazons_make();
    }

    #[no_mangle]
    pub extern "C" fn amazons_is_game_over(&self) -> bool {
        self.turn.is_game_over()
    }
    #[no_mangle]
    pub extern "C" fn amazons_is_one_winner(&self) -> bool {
        self.amazons_is_game_over() && self.turn.state.get_player() == Player::Two       
    }
    #[no_mangle]
    pub extern "C" fn amazons_is_two_winner(&mut self) -> bool {
        self.amazons_is_game_over() && self.turn.state.get_player() == Player::One
    }
    #[no_mangle]
    pub extern "C" fn amazons_can_undo(&self) -> bool {
        self.turn.history.can_undo()
    }
    #[no_mangle]
    pub extern "C" fn amazons_can_redo(&self) -> bool {
        self.turn.history.can_redo()
    }
    #[no_mangle]
    pub extern "C" fn amazons_undo(&mut self) {
        self.turn.undo()
    }
    #[no_mangle]
    pub extern "C" fn amazons_redo(&mut self) {
        self.turn.redo()
    }

    #[no_mangle]
    pub extern "C" fn amazons_is_player_one_turn(&self) -> bool {
        self.turn.state.get_player() == Player::One
    }
    #[no_mangle]
    pub extern "C" fn amazons_has_brick(&self, point: Point) -> bool {
        self.turn.board.has_brick(point)
    }
    #[no_mangle]
    pub extern "C" fn amazons_has_player_brick(&self, point: Point) -> bool {
        self.turn.board.get_brick(point) != Player::None
    }
    #[no_mangle]
    pub extern "C" fn amazons_has_player_one_brick(&self, point: Point) -> bool {
        self.turn.board.get_brick(point) == Player::One
    }
    #[no_mangle]
    pub extern "C" fn amazons_time_to_move(&self) -> bool {
        self.turn.state.is_move()
    }
    #[no_mangle]
    pub extern "C" fn amazons_time_to_drop(&self) -> bool {
        !self.turn.state.is_move()
    }

    #[no_mangle]
    pub extern "C" fn amazons_get_num_possible_actions(&self) -> i32 {
        self.turn.get_valid_actions().len() as i32
    }

    #[no_mangle]
    pub extern "C" fn amazons_get_action(&self, action_index: i32) -> Action {
        let points = self.turn.get_valid_actions()[action_index as usize];
        Action::new(points[0], points[1])
    }

    #[no_mangle]
    pub extern "C" fn amazons_get_stored_action(&self, action_index: i32) -> Action {
        let points = self.turn.history.get_action_at_index(action_index);
        Action::new(points[0], points[1])
    }

    #[no_mangle]
    pub extern "C" fn amazons_apply_action(&mut self, from: Point, to: Point) {
        self.turn.apply_action([from,to]);
    }

    #[no_mangle]
    pub extern "C" fn amazons_get_ai_action(&self) -> Action {
        AI::get_action(&self.turn)
    }

}
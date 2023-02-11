use super::action::history::History;
use super::action::Action;
use super::board::point::Point;
use super::board::tile::Player;
use super::board::Board;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TurnState {
    Move(Player),
    Drop(Player),
}

impl TurnState {
    pub fn next(&mut self) {
        *self = match self {
            TurnState::Move(player) => TurnState::Drop(*player),
            TurnState::Drop(player) => TurnState::Move(Player::get_opponent(*player)),
        }
    }
    pub fn back(&mut self) {
        *self = match self {
            TurnState::Move(player) => TurnState::Drop(Player::get_opponent(*player)),
            TurnState::Drop(player) => TurnState::Move(*player),
        }
    }

    pub fn get_player(&self) -> Player {
        match self {
            TurnState::Drop(player) => *player,
            TurnState::Move(player) => *player,
        }
    }

    pub fn is_move(&self) -> bool {
        match self {
            TurnState::Drop(_) => false,
            TurnState::Move(_) => true,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Turn {
    pub state: TurnState,
    pub board: Board,
    pub history: History,
}

impl Turn {
    pub fn new() -> Self {
        Self {
            state: TurnState::Move(Player::One),
            board: Board::new(),
            history: History::new(),
        }
    }

    pub fn apply_action(&mut self, action: [Point; 2]) {
        let from = Point::get_from(action);
        let to = Point::get_to(action);
        if Point::is_drop(action) {
            self.board.place_brick(to, Player::None)
        } else {
            let brick = self.board.get_brick(from);
            self.board.remove_brick(from);
            self.board.place_brick(to, brick);
        }
        self.state.next();
    }

    pub fn revert_action(&mut self, action: [Point; 2]) {
        let brick = self.board.get_brick(action[1]);
        if brick != Player::None{
            self.board.place_brick(action[0], brick);
        }
        self.board.remove_brick(action[1]);
        self.state.back();
    }

    pub fn undo(&mut self) {
        let action = self.history.get_action();
        //println!("Undo() {:?}", action);
        self.history.undo();
        self.revert_action(action);
    }

    pub fn redo(&mut self) {
        self.history.redo();
        let action = self.history.get_action();
        //println!("Redo() {:?}", action);
        self.apply_action(action);
    }

    pub fn is_game_over(&self) -> bool {
        match self.state {
            TurnState::Move(player) => Action::get_possible_moves(&self.board, player).is_empty(),
            TurnState::Drop(_) => false,
        }
    }
    pub fn get_valid_actions(&self) -> Vec<[Point; 2]> {
        let last_action = self.history.get_action();
        match self.state {
            TurnState::Drop(_) => Action::get_possible_drops(&self.board, last_action[1]),
            TurnState::Move(player) => Action::get_possible_moves(&self.board, player),
        }
    }
}


use super::point::Point;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct History { //Circular buffer
    pub actions: [[Point;2]; 200],
    action_index: i32,
    num_stored_undo_actions: i32,
    num_stored_redo_actions: i32,
}

impl History{
    pub fn new() -> Self {
        Self {
            actions: [Point::new_null_action(); 200],
            action_index: 0,
            num_stored_undo_actions: 0,
            num_stored_redo_actions: 0,
        }
    }

    pub fn add(&mut self, action: [Point;2]){
        self.num_stored_redo_actions = 0;
        self.num_stored_undo_actions += 1;
        self.actions[self.action_index as usize] = action;
        self.action_index = (self.action_index + 1) % self.actions.len() as i32;
    }

    fn get_previous_index(&self) -> i32 {
        (self.action_index + self.actions.len() as i32 - 1) % self.actions.len() as i32
    }

    pub fn get_action(&self) -> [Point;2] {
        let previous_index = self.get_previous_index();
        self.actions[previous_index as usize].clone()
    }

    pub fn undo(&mut self) {
        self.action_index = self.get_previous_index();
        self.num_stored_undo_actions -= 1;
        self.num_stored_redo_actions += 1;
    }

    pub fn redo(&mut self) {
        self.action_index = (self.action_index + 1) % self.actions.len() as i32;
        self.num_stored_undo_actions += 1;
        self.num_stored_redo_actions -= 1;
    }

    pub fn can_undo(&self) -> bool {
        self.num_stored_undo_actions > 0
    }
    pub fn can_redo(&self) -> bool {
        self.num_stored_redo_actions > 0
    }

    fn get_index_offset(&self) -> i32 {
        (self.action_index - self.num_stored_undo_actions + self.actions.len() as i32) % self.actions.len() as i32
    }

    pub fn get_num_actions(&self) -> i32 {
        self.num_stored_redo_actions + self.num_stored_undo_actions
    }

    pub fn get_action_at_index(&self, action_number: i32) -> [Point;2] {
        let index_of_action_number = (self.get_index_offset() + action_number) % self.actions.len() as i32;
        self.actions[index_of_action_number as usize]
    }

}
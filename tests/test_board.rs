#[path = "../src/lib.rs"]
mod lib;
use lib::Amazons;

#[cfg(test)]
mod board_tests {

    use super::*;

    #[test]
    fn make_amazons() {
        let amazons_state = Amazons::amazons_make();
       // println!("{}", board);
    }

    #[test]
    fn can_undo() {
        let mut amazons_state = Amazons::amazons_make();
        let action1 = amazons_state.amazons_get_action(0);
        amazons_state.amazons_apply_action(action1.from, action1.to);
        assert!(amazons_state.amazons_can_undo());
    }

    #[test]
    fn can_redo() {
        let mut amazons_state = Amazons::amazons_make();
        let action1 = amazons_state.amazons_get_action(0);
        amazons_state.amazons_apply_action(action1.from, action1.to);
        amazons_state.amazons_undo();
        assert!(amazons_state.amazons_can_redo());
    }

    #[test]
    fn undo_redo() {
        let mut amazons_state = Amazons::amazons_make();
        let action1 = amazons_state.amazons_get_action(0);
        amazons_state.amazons_apply_action(action1.from, action1.to);
        amazons_state.amazons_undo();
        amazons_state.amazons_redo();
    }

}


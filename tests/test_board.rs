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

}


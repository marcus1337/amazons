
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    One,
    Two,
    None,
}

impl Player{
    pub fn switch_player(&mut self){
        *self = match self {
            Player::One => Player::Two,
            _ => Player::One,
        };
    }

    pub fn get_opponent(player: Player) -> Player {
        match player{
            Player::One => Player::Two,
            _ => Player::One,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tile {
    Empty,
    Player(Player),
}

impl Tile {
    pub fn to_string(&self) -> String {
        let str = match self {
            Tile::Empty => "[ ]",
            Tile::Player(Player::One) => "[X]",
            Tile::Player(Player::Two) => "[O]",
            Tile::Player(Player::None) => "[-]",
        };
        String::from(str)
    }

    pub fn get_player(& self) -> Player{
        match self {
            Tile::Empty => panic!("Empty is not a player brick"),
            Tile::Player(player) => return *player,
        }
    }
}

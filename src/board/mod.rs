pub mod point;
pub mod tile;

use std::fmt;
use tile::Tile;
use tile::Player;
use point::Point;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Board{
    tiles: [[Tile; 6]; 6],
}

impl Board{
    pub fn new() -> Self {
        let mut tiles = [[Tile::Empty; 6]; 6];

        tiles[2][0] = Tile::Player(tile::Player::One);
        tiles[3][5] = Tile::Player(tile::Player::One);
        tiles[0][2] = Tile::Player(tile::Player::Two);
        tiles[5][3] = Tile::Player(tile::Player::Two);

        Self {
            tiles: tiles
        }
    }

    fn get_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        let grid_size = self.get_grid_size();
        for col in 0..grid_size{
            for row in 0..grid_size{
                points.push(Point::new(col, row));
            }
        }
        points
    }

    pub fn get_player_brick_points(&self, player: Player) -> Vec<Point> {
        self.get_points()
        .into_iter()
        .filter(|&point| self.has_brick(point) && self.get_brick(point) == player)
        .collect()
    }

    pub fn get_tile(&self, point: Point) -> Tile {
        self.tiles[point.col as usize][point.row as usize]
    }

    pub fn has_brick(&self, point: Point) -> bool {
        self.get_tile(point) != Tile::Empty
    }

    pub fn get_brick(&self, point: Point) -> Player {
        match self.get_tile(point){
            Tile::Player(player) => player,
            Tile::Empty => panic!("Should not be empty {:?}", point),
        }
    }

    pub fn remove_brick(&mut self, point: Point) {
        self.tiles[point.col as usize][point.row as usize] = Tile::Empty;
    }
    pub fn place_brick(&mut self, point: Point, player: Player) {
        self.tiles[point.col as usize][point.row as usize] = Tile::Player(player);
    }

    pub fn get_grid_size(&self) -> i32 {
        6
    }


}
pub mod ai;
pub mod history;

use super::board;
use super::board::tile::Player;
use super::turn::Turn;
use board::point;
use board::point::Point;
//use board::tile;
use board::Board;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Action {
    pub from: Point,
    pub to: Point,
}

impl Action {

    pub fn new(from: Point, to: Point) -> Self {
        Self { from: from, to: to }
    }

    fn get_diagonal_lines(grid_size: i32, from: Point) -> Vec<Point> {
        let mut points = Vec::new();
        for i in 0..grid_size {
            let up_right = from + Point::new(i, i);
            let up_left = from + Point::new(-i, i);
            let down_right = from + Point::new(i, -i);
            let down_left = from + Point::new(-i, -i);
            for point in [up_right, up_left, down_left, down_right] {
                if point.in_bounds(grid_size) && point != from {
                    points.push(point);
                }
            }
        }
        points
    }

    fn get_straight_lines(grid_size: i32, from: Point) -> Vec<Point> {
        let mut points = Vec::new();
        for i in 0..grid_size {
            let row_point = Point::new(i, from.row);
            let col_point = Point::new(from.col, i);
            if row_point != from {
                points.push(row_point);
            }
            if col_point != from {
                points.push(col_point);
            }
        }
        points
    }

    fn get_lines(grid_size: i32, from: Point) -> Vec<Point> {
        let mut points = Action::get_diagonal_lines(grid_size, from);
        points.extend(Action::get_straight_lines(grid_size, from));
        points
    }

    fn is_reachable(board: &Board, from: Point, to: Point) -> bool {
        let mut step = from;
        while step.step_towards(to) {
            if board.has_brick(step) {
                return false;
            }
        }
        true
    }

    fn get_reachable_points(board: &Board, from: Point) -> Vec<Point> {
        let grid_size = board.get_grid_size();
        Action::get_lines(grid_size, from)
            .into_iter()
            .filter(|&to| Action::is_reachable(board, from, to))
            .collect()
    }

    fn get_drops(board: &Board, from: Point) -> Vec<Point> {
        Action::get_reachable_points(board, from)
    }

    fn get_moves(board: &Board, from: Point) -> Vec<[Point; 2]> {
        let mut points = Vec::new();
        for point in Action::get_reachable_points(board, from) {
            points.push([from, point]);
        }
        points
    }

    pub fn get_possible_moves(board: &Board, player: Player) -> Vec<[Point; 2]> {
        let mut possible_moves = Vec::<[Point; 2]>::new();
        for from in board.get_player_brick_points(player) {
            possible_moves.extend(Action::get_moves(board, from));
        }
        possible_moves
    }

    pub fn get_possible_drops(board: &Board, from: Point) -> Vec<[Point; 2]> {
        let mut actions = Vec::new();
        for point in Action::get_drops(board, from) {
            actions.push([from, point]);
        }
        actions
    }
}

use std::vec::Vec;

pub struct Cell {
    pub x: i32,
    pub y: i32,
}

pub struct Unit {
    pub members: Vec<Cell>,
    pub pivot: Vec<Cell>,
}

pub enum Command {
    MoveW,
    MoveE,
    MoveSW,
    MoveSE,
    RotateClockwise,
    RotateCounterClockwise,
}

pub struct Input {
    pub id: i32,
    pub units: Vec<Unit>,
    pub width: i32,
    pub height: i32,
    pub filled: Vec<Cell>,
    pub source_length: i32,
    pub source_seeds: Vec<i32>,
}

pub struct Solution {
    pub id: i32,
}

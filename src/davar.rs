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
    pub seed: i32,
    pub tag: Option<String>,
    pub solution: String,
}

pub fn string_to_commands(s: &str) -> Vec<Command> {
    let mut out = Vec::new();
    for c in s.chars() {
        out.push(match c {
            'p' | '\'' | '!' | '.' | '0' | '3' => Command::MoveW,
            'b' | 'c' | 'e' | 'f' | 'y' | '2' => Command::MoveE,
            'a' | 'g' | 'h' | 'i' | 'j' | '4' => Command::MoveSW,
            _ => unreachable!(),
        })
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_to_commands_works() {
    }
}

use std::vec::Vec;

pub mod simulate;

pub mod json;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Unit {
    pub members: Vec<Cell>,
    pub pivot: Vec<Cell>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Command {
    MoveW,
    MoveE,
    MoveSW,
    MoveSE,
    RotateClockwise,
    RotateCounterClockwise,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Input {
    pub id: i32,
    pub units: Vec<Unit>,
    pub width: i32,
    pub height: i32,
    pub filled: Vec<Cell>,
    pub source_length: i32,
    pub source_seeds: Vec<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Solution {
    pub id: i32,
    pub seed: i32,
    pub tag: Option<String>,
    pub solution: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct State {
    pub width: i32,
    pub height: i32,
    pub filled: Vec<Cell>,
    pub unit_sequence: Vec<Unit>, // holds the actual sequence of units
    pub score: i32,
    pub game_over: bool,
}

pub fn input_to_states(i: Input) -> Vec<State> {
    unimplemented!()
}

pub fn string_to_commands(s: &str) -> Vec<Command> {
    let mut out = Vec::new();
    for c in s.chars() {
        match c {
            'p' | '\'' | '!' | '.' | '0' | '3' => out.push(Command::MoveW),
            'b' | 'c' | 'e' | 'f' | 'y' | '2' => out.push(Command::MoveE),
            'a' | 'g' | 'h' | 'i' | 'j' | '4' => out.push(Command::MoveSW),
            'l' | 'm' | 'n' | 'o' | ' ' | '5' => out.push(Command::MoveSE),
            'd' | 'q' | 'r' | 'v' | 'z' | '1' => out.push(Command::RotateClockwise),
            'k' | 's' | 't' | 'u' | 'w' | 'x' => out.push(Command::RotateCounterClockwise),
            '\t' | '\n' | '\r' => (),
            _ => unreachable!(),
        };
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_to_commands_works() {
        assert_eq!(string_to_commands("pack"), vec![Command::MoveW,
                                                    Command::MoveSW,
                                                    Command::MoveE,
                                                    Command::RotateCounterClockwise])
    }
}

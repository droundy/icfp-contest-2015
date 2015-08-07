extern crate rustc_serialize;

use std::vec::Vec;

pub mod simulate;

pub mod in_out;

#[derive(Debug, Eq, PartialEq, Clone, Copy, RustcDecodable, RustcEncodable)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, RustcDecodable, RustcEncodable)]
pub struct Unit {
    pub members: Vec<Cell>,
    pub pivot: Cell,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Direction { W, E, SW, SE }

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Command {
    Move(Direction),
    RotateClockwise,
    RotateCounterClockwise,
}

#[derive(Debug, Eq, PartialEq, Clone, RustcDecodable, RustcEncodable)]
#[allow(non_snake_case)]
pub struct Input {
    pub id: i32,
    pub units: Vec<Unit>,
    pub width: i32,
    pub height: i32,
    pub filled: Vec<Cell>,
    pub sourceLength: i32,
    pub sourceSeeds: Vec<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, RustcEncodable)]
#[allow(non_snake_case)]
pub struct Solution {
    pub problemId: i32,
    pub seed: i32,
    pub tag: Option<String>,
    pub solution: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct State {
    pub width: i32,
    pub height: i32,
    pub filled_array: Vec<bool>,
    pub visited_array: Vec<bool>,
    pub unit_sequence: Vec<Unit>, // holds the actual sequence of units
    pub ls_old: i32,
    pub score: i32,
    pub game_over: bool,
}

fn get_source_order(seed: i32, len: i32) -> Vec<i32> {
    let test: Vec<i32> = vec![2, 3, 4, 5, 4, 3, 2, 2, 3, 4, 54, 5, 4, 3, 3, 3, 2, 1, 2,
                              3, 4, 4, 2, 3, 4, 5, 4, 3, 2, 2, 3];
    test[..len as usize].into()
}

impl State {
    fn new() -> State {
        State {
            width: 10,
            height: 10,
            filled_array: vec![false; 10*10],
            visited_array: vec![false; 10*10],
            unit_sequence: Vec::new(),
            ls_old: 0,
            score: 0,
            game_over: false,
        }
    }

    fn with_size(width: i32, height: i32) -> State {
        State {
            width: width,
            height: height,
            filled_array: vec![false; (width*height) as usize],
            visited_array: vec![false; (width*height) as usize],
            unit_sequence: Vec::new(),
            ls_old: 0,
            score: 0,
            game_over: false,
        }
    }
    fn filled(&mut self, c: Cell) -> &mut bool {
        println!("filled length is {}", self.filled_array.len());
        println!("filled c.x {} c.y {} self.width {}", c.x, c.y, self.width);
        &mut self.filled_array[c.x as usize + (c.y as usize)*(self.width as usize)]
    }
    fn is_filled(&self, c: Cell) -> bool {
        println!("is_filled length is {}", self.filled_array.len());
        if c.x < 0 || c.x >= self.width || c.y < 0 || c.y >= self.height {
            return false;
        }
        self.filled_array[c.x as usize + (c.y as usize)*(self.width as usize)]
    }
    fn visited(&mut self, c: Cell) -> &mut bool {
        println!("visited length is {}", self.visited_array.len());
        println!("visited c.x {} c.y {} self.width {}", c.x, c.y, self.width);
        &mut self.visited_array[c.x as usize + (c.y as usize)*(self.width as usize)]
    }
    fn is_visited(&self, c: Cell) -> bool {
        println!("is_visited length is {}", self.visited_array.len());
        if c.x < 0 || c.x >= self.width || c.y < 0 || c.y >= self.height {
            return false;
        }
        println!("c.x {} c.y {} self.width {}", c.x, c.y, self.width);
        self.visited_array[c.x as usize + (c.y as usize)*(self.width as usize)]
    }
}

impl From<Input> for Vec<State> {
    fn from(input: Input) -> Self {
        input.sourceSeeds.iter().map( |&s| {
            let mut seq: Vec<Unit> = Vec::with_capacity(input.sourceLength as usize);
            for i in get_source_order(s, input.sourceLength) {
                seq.push(input.units[i as usize].clone());
            }
            let mut state = State::with_size(input.width, input.height);
            state.unit_sequence = seq;
            for &cell in input.filled.iter() {
                state.filled(cell);
            }
            state
        }).collect()
    }
}

pub fn input_to_states(i: Input) -> Vec<State> {
    unimplemented!()
}

pub fn string_to_commands(s: &str) -> Vec<Command> {
    use Direction::*;
    let mut out = Vec::new();
    for c in s.chars() {
        match c.to_lowercase().next().unwrap() {
            'p' | '\'' | '!' | '.' | '0' | '3' => out.push(Command::Move(W)),
            'b' | 'c' | 'e' | 'f' | 'y' | '2' => out.push(Command::Move(E)),
            'a' | 'g' | 'h' | 'i' | 'j' | '4' => out.push(Command::Move(SW)),
            'l' | 'm' | 'n' | 'o' | ' ' | '5' => out.push(Command::Move(SE)),
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
        use Direction::*;
        assert_eq!(string_to_commands("pack"), vec![Command::Move(W),
                                                    Command::Move(SW),
                                                    Command::Move(E),
                                                    Command::RotateCounterClockwise]);
        assert_eq!(string_to_commands("PACK"), vec![Command::Move(W),
                                                    Command::Move(SW),
                                                    Command::Move(E),
                                                    Command::RotateCounterClockwise]);
        assert_eq!(string_to_commands("ei! "), vec![Command::Move(E),
                                                    Command::Move(SW),
                                                    Command::Move(W),
                                                    Command::Move(SE)]);
        assert_eq!(string_to_commands("Ei! "), vec![Command::Move(E),
                                                    Command::Move(SW),
                                                    Command::Move(W),
                                                    Command::Move(SE)]);
    }
}

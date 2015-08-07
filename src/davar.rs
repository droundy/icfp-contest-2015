extern crate num;
extern crate rustc_serialize;

use std::vec::Vec;
use rustc_serialize::json;
use std::path::Path;
use std::fs::File;
use std::str;
use std::io::Read;
use num::pow;
use std::num::Wrapping;

use std::convert::AsRef;

pub mod simulate;

//pub mod json;

#[derive(Debug, Eq, PartialEq, Clone, RustcDecodable, RustcEncodable)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, RustcDecodable, RustcEncodable)]
pub struct Unit {
    pub members: Vec<Cell>,
    pub pivot: Cell,
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

impl Input {
    fn from_json<P: AsRef<Path>>(fname: P) -> Input {
        let mut temp = String::new();
        let mut file = match File::open(fname) {
            Ok(r) => r,
            Err(e) => panic!("Failed to open file with error {}", e),
        };
        file.read_to_string(&mut temp).ok().expect("Failed to read file contents.");
        let input: &str = str::from_utf8(temp.as_bytes()).ok().expect("Failed to convert &[u8] to &str???");

        let decoded: Input = match json::decode(input) {
            Ok(r) => r,
            Err(e) => panic!("Failed to decode JSON with error: {}", e),
        };
        decoded
    }
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
impl State {
    fn new() -> State {
        State {
            width: 10,
            height: 10,
            filled: Vec::new(),
            unit_sequence: Vec::new(),
            score: 0,
            game_over: false,
        }
    }
}

pub fn input_to_states(i: Input) -> Vec<State> {
    unimplemented!()
}

pub fn string_to_commands(s: &str) -> Vec<Command> {
    let mut out = Vec::new();
    for c in s.chars() {
        match c.to_lowercase().next().unwrap() {
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

pub fn get_source_order(seed: u64, num: i32) -> Vec<u64> {

    let mut out_vec: Vec<u64> = Vec::with_capacity(num as usize);

    let modulus: u64 = 2_u64.pow(32);
    let multiplier: u64 = 1103515245;
    let increment: u64 = 12345;

    let mut x: u64 = seed;

    out_vec.push((x>>16) & 0x7fff);

    for i in 0..(num as usize) {
      x = (multiplier*x + increment) % modulus;
      out_vec.push((x>>16) & 0x7fff);
    }

    out_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_to_commands_works() {
        assert_eq!(string_to_commands("pack"), vec![Command::MoveW,
                                                    Command::MoveSW,
                                                    Command::MoveE,
                                                    Command::RotateCounterClockwise]);
        assert_eq!(string_to_commands("PACK"), vec![Command::MoveW,
                                                    Command::MoveSW,
                                                    Command::MoveE,
                                                    Command::RotateCounterClockwise]);
        assert_eq!(string_to_commands("ei! "), vec![Command::MoveE,
                                                    Command::MoveSW,
                                                    Command::MoveW,
                                                    Command::MoveSE]);
        assert_eq!(string_to_commands("Ei! "), vec![Command::MoveE,
                                                    Command::MoveSW,
                                                    Command::MoveW,
                                                    Command::MoveSE]);
    }

	  #[test]
    fn test_psuedorandom() {
      let seed = 17;
      let n = 10;
        let sources = get_source_order(seed, n);
        let correct_sources = [0,24107,16552,12125,9427,13152,21440,3383,6873,16117];
        for i in 0..n as usize {
          assert_eq!(sources[i], correct_sources[i]);
        }
    }

    #[test]
    fn decode_p1() {
        let manual = Input{
            id:1,
            units: vec![
                Unit{
                    pivot: Cell{x:0, y:0},
                    members: vec![Cell{x:0, y:0}]
                }],
            width: 5,
            height: 5,
            filled: vec![Cell{x: 2, y: 4}],
            sourceLength: 100,
            sourceSeeds: vec![0],
        };
        let from_file = Input::from_json("problems/test.json");

        assert_eq!(manual, from_file);

    }
}

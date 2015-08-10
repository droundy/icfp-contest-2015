extern crate davar;
extern crate ncurses;

use ncurses::*;
use davar::*;
use davar::Direction::*;
use davar::Command::{Move, Rotate};
use std::io::BufRead;

fn main() {
    print!("Welcome to the game-mode of Davar. The commands are:\n");
    print!("     a or 4: West\n");
    print!("     d or 6: East\n");
    print!("     z or 1: SouthWest\n");
    print!("     c or 3: SouthEast\n");
    print!("     s or 5: Rotate Clockwise\n");
    print!("     x or 2: Rotate Counter-Clockwise\n");
    print!("\nWelcome to Davar. Please select a problem (0-24):\n");

    let stdin = std::io::stdin();

    let line = stdin.lock().lines().next().unwrap().unwrap();

    let prob_num: u8 = match line.parse().ok() {
        Some(n) => if n <= 24 { n } else { println!("Invalid problem number."); return; },
        None => { println!("Invalid problem number."); return; },
    };

    let input = Input::from_json(format!("problems/problem_{}.json", prob_num));
    let states = input_to_states(&input);
    let seeds: Vec<i32> = states.iter().map(|s| s.seed).collect();

    let seed: i32 = if seeds.len() > 1 {

        let printable_seeds = seeds.iter().map(|s| format!("{}", s)).collect::<Vec<String>>().connect(", ");
        println!("\nAvailable seeds are: {}", printable_seeds);

        print!("Please enter desired seed:\n");
        let line = stdin.lock().lines().next().unwrap().unwrap();

        let temp_seed: i32 = match line.parse().ok() {
            Some(n) => if let Some(_) = seeds.iter().find(|&&s| s == n) { n } else { println!("Invalid seed."); return; },
            None => { println!("Invalid seed."); return; },
        };
        temp_seed
    } else {
        println!("Only one seed for this problem. Using it!");
        seeds[0]
    };

    let mut state: State = states.iter().find(|s| s.seed == seed).unwrap().clone();

    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);


    while !state.game_over {
        clear();
        refresh();
        printw(&format!("Problem: {}, Seed: {}\n", prob_num, seed));
        printw(&format!("Remaining pieces: {}\n", state.unit_sequence.len()));
        printw(&state.visualize());
        printw(&format!("\nScore: {}\n", state.score));

        let ch = getch();

        let cmd = match ch as u8 as char {
            'a' | '4' => Some(Move(W)),
            'd' | '6' => Some(Move(E)),
            'z' | '1' => Some(Move(SW)),
            'c' | '3' => Some(Move(SE)),
            's' | '5' => Some(Rotate(Clock::Wise)),
            'x' | '2' => Some(Rotate(Clock::Counter)),
            _ => None,
        };

        if let Some(c) = cmd {
            state = state.apply(c);
        }
    }
    println!("\nGame over!");

    getch();

    endwin();
}

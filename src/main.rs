extern crate davar;
extern crate rustc_serialize;

use davar::*;
// use davar::Direction::*;
// use davar::Command::*;
// use rustc_serialize::json;
// use std::process;
use davar::solver::{Solver};
use std::thread;

#[allow(dead_code)]
fn main() {
    let options = opts::opts();
    let mut totalscore = 0;
    let mut solutions = Vec::new();
    let solver = solver::name_to_solver(&options.solver);
    let mut fnames: Vec<String> = Vec::new();

    if options.files.len() == 0 {
        for i in 0..25 {
            fnames.push(format!("problems/problem_{}.json", i)); // Fix this to work with any file name inputs...
            println!("Element is: {}", fnames[i]);
        }
    }
    else {
        fnames = options.files.clone(); // See above comment
    }
    for e in fnames.iter() {
        let mut problemscore = 0;
        let input = Input::from_json(e);
        let states = input_to_states(&input);
        let num_states = states.len();
        for state in states {
            let (solution, score) = solver.solve(&state, &input, &options);
            if let Some(a) = options.animate {
                solution.animate(a);
            }

            println!("  cmd: {}", solution.solution);
            solutions.push(solution);

            totalscore += score;
            problemscore += score;
        }
        println!("{} score[{}]: {} ({} and {})", solver.name(),
                 e, problemscore as f64 / num_states as f64,
                 problemscore, num_states);
        if let Some(_) = options.animate {
            thread::sleep_ms(1000);
        }
    }
    if options.submit {
        //println!("I am submitting solutions for {}.", i);
        in_out::submit_solutions(&solutions);
    }
    println!("total score: {}", totalscore);

    if !options.submit {
        println!("Not submitting solutions.");
    }
}

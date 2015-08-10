extern crate davar;
extern crate rustc_serialize;

use davar::*;
// use davar::Direction::*;
// use davar::Command::*;
// use rustc_serialize::json;
// use std::process;
use std::thread;
use davar::opts::DavarOptions;

#[allow(dead_code)]
fn main() {
    let options = opts::opts();
    let mut totalscore = 0;
    let solver = solver::name_to_solver(&options.solver);
    let mut fnames: Vec<String> = Vec::new();

    if options.files.len() == 0 {
        for i in 0..25 {
            fnames.push(format!("problems/problem_{}.json", i)); // Fix this to work with any file name inputs...
        }
    }
    else {
        fnames = options.files.clone(); // See above comment
    }
    let mut joinhandles: Vec<thread::JoinHandle<Vec<(Solution, Score)>>> = Vec::new();
    {
        let mut inputlists: Vec<Vec<(State, Input, DavarOptions)>> = Vec::new();
        for _ in 0 .. options.ncores {
            inputlists.push(Vec::new());
        }
        let mut which_core = 0;
        for e in fnames.iter() {
            let input = Input::from_json(e);
            let states = input_to_states(&input);
            for state in states {
                inputlists[which_core].push((state, input.clone(), options.clone()));
                which_core = (which_core + 1) % options.ncores;
                //joinhandles.push(thread::spawn(move || { solver.solve(&state, &input, &options) }));
            }
        }
        for _ in 0 .. options.ncores {
            let myinput = match inputlists.pop() {
                Some(inp) => inp,
                _ => Vec::new(),
            };
            joinhandles.push(thread::spawn(move || { solver.solve_n(&myinput) }));
        }
    }
    let mut solutions: Vec<Solution> = Vec::new();
    let mut solutions_and_scores: Vec<(Solution, Score)> = Vec::new();
    for jh in joinhandles {
        match jh.join() {
            Err(e) => {
                if options.verbose {
                    println!("Error! {:?}", e);
                }
            }
            Ok(more_solutions) => {
                for (s, sc) in more_solutions {
                    solutions_and_scores.push((s.clone(), sc));
                    solutions.push(s);
                    totalscore += sc;
                }
            }
        }
    }
    if options.submit {
        in_out::submit_solutions(&solutions);
    }
    if !options.verbose {
        in_out::print_solutions(&solutions);
    }
    if options.save_solutions {
        in_out::save_solutions(&solutions_and_scores);
    }

    if let Some(a) = options.animate {
        for s in solutions {
            s.animate(a);
        }
    }

    if options.verbose {
        println!("total score: {}", totalscore);
    }

    if !options.submit && options.verbose {
        println!("Not submitting solutions.");
    }
}

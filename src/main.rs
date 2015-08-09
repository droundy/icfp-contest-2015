extern crate davar;
extern crate rustc_serialize;

use davar::*;
// use davar::Direction::*;
// use davar::Command::*;
// use rustc_serialize::json;
// use std::process;
use std::thread;

#[allow(dead_code)]
fn main() {
    let options = opts::opts();
    let mut totalscore = 0;
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
    let mut joinhandles: Vec<thread::JoinHandle<(Solution, Score)>> = Vec::new();
    for e in fnames.iter() {
        let input = Input::from_json(e);
        let states = input_to_states(&input);
        for state in states {
            let options = options.clone();
            let input = input.clone();
            joinhandles.push(thread::spawn(move || { solver.solve(&state, &input, &options) }));
        }
    }
    let mut solutions: Vec<Solution> = Vec::new();
    for jh in joinhandles {
        match jh.join() {
            Err(e) => {
                println!("Error! {:?}", e);
            }
            Ok((solution, score)) => {
                if let Some(a) = options.animate {
                    solution.animate(a);
                }
                // println!("  cmd: {}", solution.solution);

                solutions.push(solution);
                totalscore += score;
            }
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

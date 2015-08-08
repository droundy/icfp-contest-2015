extern crate davar;
extern crate rustc_serialize;

use davar::*;
use davar::Direction::*;
use davar::Command::*;
use rustc_serialize::json;
use std::process;

fn main() {
    let options = opts::opts();

    let mut totalscore = 0;
    for i in 0..24 {
        let mut problemscore = 0;
        let mut solutions = Vec::new();
        let fname = format!("problems/problem_{}.json", i);
        let input = Input::from_json(fname);
        let states = input_to_states(&input);
        let num_states = states.len();
        for state in states {
            let mut cmds: Vec<Command> = Vec::new();
            let mut s = state.clone();
            // println!("Starting position");
            // println!("{}", s.visualize());

            while !s.game_over {
                for &cmd in [Move(SE), Move(SW)].iter() {
                    if !s.game_over {
                        s = s.apply(cmd);
                        cmds.push(cmd);
                        // println!("Score: {}", s.score);
                        // println!("{}", s.visualize());
                        // thread::sleep_ms(100);
                    }
                }
            }
            // println!("Solution[{},{}]: {}", i, s.seed, commands_to_string(cmds.clone()));
            // println!("score[{},{}]: {}", i, s.seed, s.score);
            totalscore += s.score;
            problemscore += s.score;

            solutions.push(Solution {
                problemId: input.id,
                seed: s.seed,
                tag: Some(format!("alldown[{},{}] = {}", i, s.seed, s.score)),
                solution: commands_to_string(cmds.clone()),
            });
        }
        if options.submit {
            println!("I am submitting solutions for {}.", i);
            in_out::submit_solutions(&solutions);
        }
        println!("problem score[{}]: {} ({} and {})", i, problemscore as f64 / num_states as f64,
                 problemscore, num_states);
    }
    println!("total score: {}", totalscore);

    if !options.submit {
        println!("Not submitting solutions.");
    }
}

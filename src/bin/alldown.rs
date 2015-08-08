extern crate davar;
extern crate rustc_serialize;

use davar::*;
use davar::Direction::*;
use davar::Command::*;
use rustc_serialize::json;
use std::process;

fn main() {
    let options = opts::opts();

    println!("all down!");
    let mut solutions = Vec::new();
    for i in 0..24 {
        let fname = format!("problems/problem_{}.json", i);
        println!("all down {}", fname);
        let input = Input::from_json("problems/problem_6.json");
        let states = input_to_states(&input);
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
            println!("Solution[{},{}]: {}", i, s.seed, commands_to_string(cmds.clone()));
            println!("score: {}", s.score);

            solutions.push(Solution {
                problemId: input.id,
                seed: s.seed,
                tag: Some(format!("alldown {}", s.score)),
                solution: commands_to_string(cmds.clone()),
            });
        }
    }
    if options.submit {
        println!("I am submitting solutions.");
        in_out::submit_solutions(&solutions);
    } else {
        println!("Not submitting solutions.");
    }

}

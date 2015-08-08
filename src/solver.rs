use super::*;
use super::Direction::*;
use super::Command::*;

pub trait Solver {
    fn new() -> Self;

    fn solve(&self, &State, &Input) -> (Solution, usize);

    fn name(&self) -> String;
}

pub struct AllDown;

impl Solver for AllDown {
    fn new() -> AllDown { AllDown }
    fn name(&self) -> String { format!("alldown") }
    fn solve(&self, state: &State, input: &Input) -> (Solution, usize) {
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

        (Solution {
            problemId: input.id,
            seed: s.seed,
            tag: Some(format!("alldown[{},{}] = {}", input.id, s.seed, s.score)),
            solution: commands_to_string(cmds.clone()),
        }, s.score as usize)
    }
}

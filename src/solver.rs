use super::*;
use super::Direction::*;
use super::Command::*;

pub type Score = i32;

pub trait Solver {
    fn solve(&self, &State, &Input) -> (Solution, Score);

    fn name(&self) -> String;
}

pub fn name_to_solver(name: &str) -> Box<Solver> {
    let foo: Box<Solver> = Box::new(AllDown::new());
    let solvers: Vec<Box<Solver>> = vec![Box::new(AllDown::new()),
                                         Box::new(SolverSE::new()),
                                         Box::new(MonteCarlo::new()),
                                         ];
    for s in solvers.into_iter() {
        if s.name() == name {
            return s;
        }
    }
    Box::new(AllDown::new())
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct AllDown;

impl AllDown {
    pub fn new() -> AllDown { AllDown }
}

impl Solver for AllDown {
    fn name(&self) -> String { format!("alldown") }
    fn solve(&self, state: &State, input: &Input) -> (Solution, Score) {
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
            tag: Some(format!("{}[{},{}] = {}", self.name(), input.id, s.seed, s.score)),
            solution: commands_to_string(cmds.clone()),
        }, s.score)
    }
}


#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct SolverSE;

impl SolverSE {
    pub fn new() -> SolverSE { SolverSE }
}

impl Solver for SolverSE {
    fn name(&self) -> String { format!("se") }
    fn solve(&self, state: &State, input: &Input) -> (Solution, Score) {
        let mut cmds: Vec<Command> = Vec::new();
        let mut s = state.clone();

        while !s.game_over {
            s = s.apply(Move(SE));
            cmds.push(Move(SE));
        }

        (Solution {
            problemId: input.id,
            seed: s.seed,
            tag: Some(format!("{}[{},{}] = {}", self.name(), input.id, s.seed, s.score)),
            solution: commands_to_string(cmds.clone()),
        }, s.score)
    }
}


#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct MonteCarlo;

impl MonteCarlo {
    pub fn new() -> MonteCarlo { MonteCarlo }
}

use std::num::Wrapping;
struct Random(Wrapping<u32>);

impl Random {
    fn new(seed: u32) -> Random { Random(Wrapping(seed)) }
    fn random(&mut self) -> usize {
        fn unwrap<T>(x: Wrapping<T>) -> T {
            let Wrapping(x) = x;
            x
        }
        let multiplier: Wrapping<u32> = Wrapping(1103515245);
        let increment: Wrapping<u32> = Wrapping(12345);
        self.0 = multiplier*(self.0) + increment;
        unwrap(self.0) as usize
    }
    fn commands(&mut self, s: &State, options: &[String], cmds: &[Vec<Command>])
                -> (String, State) {
        if s.game_over {
            return ("".into(), s.clone())
        }
        let mut handled = vec![false; options.len()];
        let mut i = self.random() % options.len();
        let mut attempts = 0;
        loop {
            handled[i] = true;
            let o = options[self.random() % options.len()].clone();
            let ss = s.apply_sequence(&cmds[i]);
            // println!("attempt {} \"{}\" -> {} <{}>", attempts, o, ss.score, ss.game_over);
            if !ss.game_over || ss.score > 0 {
                return (o, ss)
            }
            // We got a zero-point illegal move, so let us try again!
            let mut allhandled = true;
            for j in 0 .. options.len() {
                allhandled &= handled[j];
            }
            if allhandled {
                return ("".into(), s.clone());
            }
            while handled[i] {
                i = self.random() % options.len();
            }
            attempts += 1
        }
    }
    fn many_commands(&mut self, s: &State, options: &[String], cmds: &[Vec<Command>], max_cmds: usize)
                     -> (String, State) {
        let mut s = s.clone();
        let mut all_cmds: String = "".into();
        for _ in 0 .. max_cmds {
            let (more, snew) = self.commands(&s, options, cmds);
            all_cmds = all_cmds + &more;
            // if snew.score != s.score {
            //     println!("so far: {} -> {}", all_cmds, snew.score);
            // }
            s = snew;
            if s.game_over {
                return (all_cmds, s)
            }
        }
        return (all_cmds, s)
    }
}

impl Solver for MonteCarlo {
    fn name(&self) -> String { format!("mc") }
    fn solve(&self, state: &State, input: &Input) -> (Solution, Score) {
        let mut r = Random::new(5);

        let moves: Vec<String> = vec!["p".into(),
                                      "b".into(),
                                      "a".into(),
                                      "l".into(),
                                      "d".into(),
                                      "k".into()];
        let seqs: Vec<Vec<Command>> = moves.iter().map(|s| { string_to_commands(s) }).collect();

        let mut best_cmds: String = "".into();
        let mut best_state = state.clone();
        for _ in 0..100000 {
            let (cmds, new_s) = r.many_commands(&state, &moves, &seqs, 1000);
            if new_s.score > best_state.score {
                println!("Found better score with {} > {}",
                         new_s.score, best_state.score);
                best_cmds = cmds;
                best_state = new_s;
            }
        }

        (Solution {
            problemId: input.id,
            seed: best_state.seed,
            tag: Some(format!("{}[{},{}] = {}", self.name(), input.id, best_state.seed, best_state.score)),
            solution: best_cmds,
        }, best_state.score)
    }
}

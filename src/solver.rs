use super::*;
use super::Direction::*;
use super::Command::*;
use super::simulate::Lattice;
use super::opts::*;

pub trait Solver {
    fn solve(&self, &State, &Input, &DavarOptions) -> (Solution, Score);

    fn name(&self) -> String;
}

pub fn name_to_solver(name: &str) -> Box<Solver> {
    // let foo: Box<Solver> = Box::new(AllDown::new());
    let solvers: Vec<Box<Solver>> = vec![Box::new(AllDown::new()),
                                         Box::new(SolverSE::new()),
                                         Box::new(MonteCarlo::new()),
                                         Box::new(Supplied::new()),
                                         Box::new(BottomUp::new()),
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
    fn solve(&self, state: &State, input: &Input, _opts: &DavarOptions) -> (Solution, Score) {
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
    fn solve(&self, state: &State, input: &Input, _opts: &DavarOptions) -> (Solution, Score) {
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
pub struct Random(Wrapping<u32>);

impl Random {
    pub fn new(seed: u32) -> Random { Random(Wrapping(seed)) }
    pub fn random(&mut self) -> usize {
        fn unwrap<T>(x: Wrapping<T>) -> T {
            let Wrapping(x) = x;
            x
        }
        let multiplier: Wrapping<u32> = Wrapping(1103515245);
        let increment: Wrapping<u32> = Wrapping(12345);
        self.0 = multiplier*(self.0) + increment;
        unwrap(self.0) as usize
    }
    pub fn commands(&mut self, s: &State, options: &[String], cmds: &[Vec<Command>])
                    -> (String, State) {
        if s.game_over {
            return ("".into(), s.clone())
        }
        let mut handled = vec![false; options.len()];
        let mut i = self.random() % options.len();
        let mut attempts = 0;
        loop {
            handled[i] = true;
            let o = options[i].clone();
            let ss = s.apply_sequence(&cmds[i]);
            if false {
                println!("attempt {} \"{}\" -> {} <{}>", attempts, o, ss.score, ss.game_over);
            }
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
            if snew.score < s.score {
                return (all_cmds, s);
            }
            all_cmds = all_cmds + &more;
            // if snew.score != s.score {
            //     println!("so far: {} -> {}", all_cmds, snew.score);
            // }
            s = snew;
            if s.game_over {
                return (all_cmds, s)
            }
        }
        (all_cmds, s)
    }
}

// pub fn find_path(s: &State, goal: &Unit) -> Option(&[Commands]) {
//     let mut s = s.clone();
//     let mut all_cmds = String::new();
//     for _ in 0 .. max_cmds {
//         let (more, snew) = self.commands(&s, options, cmds);

//     }
// }

impl Solver for MonteCarlo {
    fn name(&self) -> String { format!("mc") }
    fn solve(&self, state: &State, input: &Input, opt: &DavarOptions) -> (Solution, Score) {
        let mut r = Random::new(5);

        let mut moves: Vec<String> = vec!["p".into(),
                                          "b".into(),
                                          "a".into(),
                                          "l".into(),
                                          "d".into(),
                                          "k".into()];
        for i in 0 .. opt.phrases_of_power.len() {
            moves.push(opt.phrases_of_power[i].clone());
        }
        let moves = moves;
        let seqs: Vec<Vec<Command>> = moves.iter().map(|s| { string_to_commands(s) }).collect();

        let mut best_cmds: String = "".into();
        let mut best_state = state.clone();
        let original_time_left = opt.time_left();
        let mut current_time_left;// = original_time_left;
        let mut iters_per_time_check = 100;
        let mut time_per_iter;// = 1.0;
        let time_per_check_goal = 0.25;
        for iters in 1..1000000000 {
            let (cmds, mut new_s) = r.many_commands(&state, &moves, &seqs, 10000);
            let pop_score = simulate::score_pop(&cmds, &opt.phrases_of_power);
            // println!("scores {} and {}", new_s.score, pop_score);
            new_s.score += pop_score;
            let new_s = new_s;
            if iters % iters_per_time_check == 0 {
                current_time_left = opt.time_left();
                if current_time_left < 3.0*time_per_check_goal {
                    return (Solution {
                        problemId: input.id,
                        seed: best_state.seed,
                        tag: Some(format!("{}[{},{}] = {}", self.name(),
                                          input.id, best_state.seed, best_state.score)),
                        solution: best_cmds,
                    }, best_state.score);

                }
                time_per_iter = (original_time_left - current_time_left) / iters as f64;
                iters_per_time_check = (time_per_check_goal / time_per_iter) as usize
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    #[test]
    fn test_random_many_commands() {
        let states = input_to_states(&Input::from_json("problems/problem_0.json"));
        let s = states[0].clone();

        let moves: Vec<String> = vec!["p".into(),
                                      "b".into(),
                                      "a".into(),
                                      "l".into(),
                                      "d".into(),
                                      "k".into()];
        let seqs: Vec<Vec<Command>> = moves.iter().map(|s| { string_to_commands(s) }).collect();

        for i in 0..30 {
            let mut r = Random::new(i);
            let (cmds, snew) = r.many_commands(&s, &moves, &seqs, 100);
            let alt_snew = s.apply_sequence(&string_to_commands(&cmds));
            println!("cmds {}", cmds);
            assert_eq!(snew.score, alt_snew.score);
        }
    }

    #[test]
    fn test_random_commands() {
        let states = input_to_states(&Input::from_json("problems/problem_0.json"));
        let mut s = states[0].clone();
        s.score = 5;
        let s = s;

        let moves: Vec<String> = vec!["p".into(),
                                      "b".into(),
                                      "a".into(),
                                      "l".into(),
                                      "d".into(),
                                      "k".into()];
        let mut seqs: Vec<Vec<Command>> = Vec::new();
        for i in 0 .. moves.len() {
            seqs.push(string_to_commands(&moves[i]));
            println!("hello {} -> {:?}", moves[i], seqs[i]);
        }
        let seqs = seqs;

        for i in 0..30 {
            let mut r = Random::new(i);
            let (cmds, snew) = r.commands(&s, &moves, &seqs);
            let alt_snew = s.apply_sequence(&string_to_commands(&cmds));
            println!("cmds {}", cmds);
            assert_eq!(snew.score, alt_snew.score);
            assert!(snew.score >= s.score);
        }
    }
}

pub struct BottomUp;

impl BottomUp {
    fn new() -> Self { BottomUp }
}

impl Solver for BottomUp {
    fn name(&self) -> String { "bottomup".into() }

    fn solve(&self, state: &State, input: &Input, _opt: &DavarOptions) -> (Solution, Score) {
        unimplemented!()
    }
}

fn d2(a: Cell, b: Cell) -> i32 {
    let v: Lattice = Lattice::from(b) - Lattice::from(a);
    v.x.pow(2) + v.y.pow(2)
}

fn enumerate_resting_positions(state: &State) -> Vec<Unit> {
    let unit = &state.unit_sequence[0];

    let min2 = unit.members.iter().map(|&m| d2(unit.pivot, m)).min().unwrap();
    let min = (min2 as f32).sqrt() as i32;

    let mut valid_positions: Vec<Unit> = Vec::new();

    for x in (-min..state.width + min) {
        for y in (-min..state.height + min) {
            let delta = Lattice::new(x, y);
            let pivot = Cell::from(Lattice::from(unit.pivot) + delta);
            let members = unit.members.iter().map(|&m| Cell::from(Lattice::from(m) + delta));
            let mut unit = Unit{pivot: pivot, members: members.collect()};
            for _ in (0..6) {
                if !state.is_unit_invalid(&unit) {
                    valid_positions.push(unit.clone());
                }
                unit.rotate(Clock::Wise);
            }
        }
    }
    valid_positions
}


pub struct Supplied;

impl Supplied {
    fn new() -> Self { Supplied }
}

impl Solver for Supplied {
    fn name(&self) -> String { "supplied".into() }

    fn solve(&self, state: &State, input: &Input, _opt: &DavarOptions) -> (Solution, Score) {
        let mut s = state.clone();

        let old_solution = _opt.solution.clone().expect("Must enter solution to use \"supplied\" solver");
        let mut cmds: Vec<char> = Vec::new();

        for ch in old_solution.chars() {
            let cmd = string_to_commands(&*ch.to_string())[0];
            s = s.apply(cmd);
            cmds.push(ch);
        }
        (Solution {
            problemId: input.id,
            seed: s.seed,
            tag: Some(format!("{}[{},{}] = {}", self.name(), input.id, s.seed, s.score)),
            solution: cmds.into_iter().collect(),
        }, s.score)
    }
}

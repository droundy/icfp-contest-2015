use super::*;
use super::Direction::*;
use super::Command::*;
use super::simulate::Lattice;
use super::opts::*;

extern crate time;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Solver {
    AllDown,
    SolverSE,
    MonteCarlo,
    Supplied,
    BottomUp,
}

pub fn name_to_solver(name: &str) -> Solver {
    let solvers: Vec<Solver> = vec![Solver::AllDown, Solver::SolverSE,
                                    Solver::MonteCarlo, Solver::Supplied, Solver::BottomUp];
    for s in solvers.into_iter() {
        if s.name() == name {
            return s;
        }
    }
    Solver::MonteCarlo
}

impl Solver {
    pub fn solve(&self, state: &State, input: &Input, opt: &DavarOptions) -> (Solution, Score) {
        match *self {
            Solver::AllDown => {
                let mut cmds: Vec<Command> = Vec::new();
                let mut s = state.clone();
                while !s.game_over {
                    for &cmd in [Move(SE), Move(SW)].iter() {
                        if !s.game_over {
                            s = s.apply(cmd);
                            cmds.push(cmd);
                        }
                    }
                }
                // println!("Solution[{},{}]: {}", i, s.seed, commands_to_string(cmds.clone()));
                // println!("score[{},{}]: {}", i, s.seed, s.score);

                (Solution {
                    problemId: input.id,
                    seed: s.seed,
                    tag: match opt.tag {
                        None => Some(format!("{}[{},{}] = {}", self.name(),
                                             input.id, s.seed, s.score)),
                        ref tag => tag.clone(),
                    },
                    solution: commands_to_string(cmds.clone()),
                }, s.score)
            },
            Solver::SolverSE => {
                let mut cmds: Vec<Command> = Vec::new();
                let mut s = state.clone();

                while !s.game_over {
                    s = s.apply(Move(SE));
                    cmds.push(Move(SE));
                }

                (Solution {
                    problemId: input.id,
                    seed: s.seed,
                    tag: match opt.tag {
                        None => Some(format!("{}[{},{}] = {}", self.name(),
                                             input.id, s.seed, s.score)),
                        ref tag => tag.clone(),
                    },
                    solution: commands_to_string(cmds.clone()),
                }, s.score)
            },
            Solver::MonteCarlo => {
                let mut r = Random::new(5);

                let mut moves: Vec<String> = vec!["p".into(),
                                                  "b".into(),
                                                  "d".into(),
                                                  "k".into(),
                                                  "a".into(),
                                                  "l".into()];
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
                let time_per_check_goal = if original_time_left < 2.0 { original_time_left/20.0 } else { 0.5 };
                for iters in 1..1000000000 {
                    let split_point = if best_cmds.len() > 0 { r.random() % best_cmds.len() } else { 0 };
                    let start: String = best_cmds[0..split_point].into();
                    let mid_state = simulate::score_commands(string_to_commands(&start),
                                                             &state);
                    let (mut cmds, mut new_s) = r.many_commands(&mid_state, &moves, &seqs, 10000);
                    if new_s.score > 0 {
                        cmds = start + &cmds;
                        // Only count pop_score if we have a non-zero other
                        // score, since otherwise we could accidentally count
                        // something as nonzero that actually has zero score
                        // for doing an illegal move.  Maybe this fixes bug?
                        let pop_score = simulate::score_pop(&cmds, &opt.phrases_of_power);
                        // println!("scores {} and {}", new_s.score, pop_score);
                        new_s.score += pop_score;
                    }
                    let new_s = new_s;
                    if new_s.score > best_state.score {
                        // println!("Found better score with {} > {}",
                        //          new_s.score, best_state.score);
                        best_cmds.truncate(split_point);
                        best_cmds = cmds;
                        best_state = new_s;
                    }
                    if iters % iters_per_time_check == 0 {
                        current_time_left = opt.time_left();
                        if current_time_left < 3.0*time_per_check_goal {
                            return (Solution {
                                problemId: input.id,
                                seed: best_state.seed,
                                tag: match opt.tag {
                                    None => Some(format!("{}[{},{}] = {}", self.name(),
                                                         input.id, best_state.seed, best_state.score)),
                                    ref tag => tag.clone(),
                                },
                                solution: best_cmds,
                            }, best_state.score);
                        }
                        time_per_iter = (original_time_left - current_time_left) / iters as f64;
                        iters_per_time_check = (time_per_check_goal / time_per_iter) as usize
                    }
                }

                (Solution {
                    problemId: input.id,
                    seed: best_state.seed,
                    tag: match opt.tag {
                        None => Some(format!("{}[{},{}] = {}", self.name(),
                                             input.id, best_state.seed, best_state.score)),
                        ref tag => tag.clone(),
                    },
                    solution: best_cmds,
                }, best_state.score)
            },
            Solver::Supplied => {
                let mut s = state.clone();

                let old_solution = opt.solution.clone().expect("Must enter solution to use \"supplied\" solver");
                let mut cmds: Vec<char> = Vec::new();

                for ch in old_solution.chars() {
                    let cmd = string_to_commands(&*ch.to_string())[0];
                    s = s.apply(cmd);
                    cmds.push(ch);
                }
                (Solution {
                    problemId: input.id,
                    seed: s.seed,
                    tag: match opt.tag {
                        None => Some(format!("{}[{},{}] = {}", self.name(),
                                             input.id, s.seed, s.score)),
                        ref tag => tag.clone(),
                    },
                    solution: cmds.into_iter().collect(),
                }, s.score)
            },
            Solver::BottomUp => {
                let mut solution = String::new();
                let mut s = state.clone();
                let mut r = Random::new(3);
                let mut moves: Vec<String> = vec!["p".into(),
                                                  "b".into(),
                                                  "d".into(),
                                                  "k".into(),
                                                  "a".into(),
                                                  "l".into()];
                for i in 0 .. opt.phrases_of_power.len() {
                    moves.push(opt.phrases_of_power[i].clone());
                }
                let moves = moves;
                let seqs: Vec<Vec<Command>> = moves.iter().map(|s| { string_to_commands(s) }).collect();

                while !s.game_over {
                    let possible_next_positions = enumerate_resting_positions(&s);
                    for u in possible_next_positions {
                        match r.find_path(&s, &u, &moves, &seqs) {
                            None => (),
                            Some((more_cmds, _score)) => {
                                s = s.apply_sequence(&string_to_commands(&more_cmds));
                                solution = solution + &more_cmds;
                                break;
                            }
                        }
                    }
                }

                (Solution {
                    problemId: input.id,
                    seed: s.seed,
                    tag: match opt.tag {
                        None => Some(format!("{}[{},{}] = {}", self.name(),
                                             input.id, s.seed, s.score)),
                        ref tag => tag.clone(),
                    },
                    solution: solution,
                }, s.score)
            },
        }
    }

    pub fn solve_n(&self, args: &[(State, Input, DavarOptions)]) -> Vec<(Solution, Score)> {
        let nargs = args.len() as f64;
        let mut solutions = Vec::new();
        for i in 0 .. args.len() {
            let fraction_of_time = (i as f64 + 1.0)/nargs;
            let mut opts = args[i].2.clone();
            opts.time_limit = fraction_of_time*opts.time_limit;
            // println!("{}/{} seconds left",
            //          opts.time_limit + opts.starting_time - time::precise_time_s(),
            //          opts.time_limit);
            let (sol, sc) = self.solve(&args[i].0, &args[i].1, &opts);
            println!("finished {}[{}, {}] = {}", self.name(), sol.problemId, sol.seed, sc);
            solutions.push((sol, sc));
        }
        solutions
    }

    pub fn name(&self) -> String {
        match *self {
            Solver::AllDown => "alldown".into(),
            Solver::SolverSE => "se".into(),
            Solver::MonteCarlo => "mc".into(),
            Solver::Supplied => "supplied".into(),
            Solver::BottomUp => "bottomup".into(),
        }
    }
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
        let mut all_cmds = String::new();
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
    fn find_level(&mut self, s: &State, target: i32, options: &[String], cmds: &[Vec<Command>])
                  -> Option<(String, State)> {
        let mut s = s.clone();
        let mut all_cmds = String::new();
        let num_units = s.unit_sequence.len();
        loop {
            let (more, snew) = self.commands(&s, options, cmds);
            if snew.unit_sequence.len() != num_units || snew.game_over {
                return None;
            }
            all_cmds = all_cmds + &more;
            s = snew;
            if s.unit_sequence[0].pivot.y >= target {
                return Some((all_cmds, s));
            }
        }
    }
    fn find_path(&mut self, input_s: &State, target: &Unit, options: &[String], cmds: &[Vec<Command>])
                 -> Option<(String, State)> {
        let mut s = input_s.clone();
        let mut all_cmds = String::new();
        let mut attempts = 0;
        let num_units = s.unit_sequence.len();

        let mut level = s.unit_sequence[0].pivot.y + 1;
        // println!("starting at level {} with target level {}",
        //          level, target.pivot.y);
        while level <= target.pivot.y {
            attempts += 1;
            match self.find_level(&s, level, options, cmds) {
                None => (),
                Some((cmds,news)) => {
                    all_cmds = all_cmds + &cmds;
                    s = news;
                    level = s.unit_sequence[0].pivot.y + 1;
                    attempts = 0;
                }
            }
            if attempts > 10*s.width {
                println!("NO PATH to level {} for {},{}!",
                         level, target.pivot.x, target.pivot.y);
                return None;
            }
        }
        if s.unit_sequence[0].pivot.y == target.pivot.y {
            for _ in 0..30*s.width {
                let (more, snew) = self.commands(&s, &options[0..4], &cmds[0..4]);
                if snew.unit_sequence.len() != num_units {
                    continue;
                }
                all_cmds = all_cmds + &more;
                s = snew;
                if s.unit_sequence[0] == *target {
                    println!("Found a path to target at {}, {}!",
                             target.pivot.x, target.pivot.y);
                    for _ in 0..6 {
                        let (more, snew) = self.commands(&s, &options[4..6], &cmds[4..6]);
                        if snew.unit_sequence.len() != num_units {
                            println!("Found the finisher");
                            return Some((all_cmds + &more, snew));
                        }
                    }
                }
            }
        } else {
            println!("NO PATH to target: got wrong level {} for {}, {}!",
                     s.unit_sequence[0].pivot.y,
                     target.pivot.x, target.pivot.y);
        }
        println!("NO PATH to target at {}, {}!",
                 target.pivot.x, target.pivot.y);
        None
    }
}

/// Taxicab-like distance formula for our lattice

// 1 SE = 1
// 1 E = 1
// 1 SE - 1 E = 1 SW = 1
// - 1 SE = 1
// -1 SE + 1 E = -1 SW = 1
fn distance(a: Cell, b: Cell) -> i32 {
    let v: Lattice = Lattice::from(b) - Lattice::from(a);
    if v.x.is_positive() == v.y.is_positive() {
        v.x.abs() + v.y.abs()
    } else {
        ::std::cmp::max(v.x.abs(), v.y.abs())
    }
}

#[test]
fn test_distance() {
    // tuples in form (a.x, a.y, b.x, b.y, distance)
    let tests = &[(1, 2, 0, 5, 3),
                  (1, 7, 1, 4, 3),
                  (2, 5, 3, 6, 1),
                  (2, 5, 2, 6, 1),
                  (3, 7, 3, 7, 0),
                  (2, 5, 2, 4, 1),
                  (2, 5, 3, 4, 1),
                  (1, 2, 0, 4, 2),
                  (3, 2, 4, 4, 2),
                  ];
    for &(ax, ay, bx, by, d) in tests {
        println!("a: ({}, {}), b: ({}, {}), d: {}", ax, ay, bx, by, d);
        println!("Ensuring symmetry.");
        assert_eq!(distance(Cell{x:ax, y:ay}, Cell{x:bx, y:by}), distance(Cell{x:bx, y:by}, Cell{x:ax, y:ay}));
        println!("Ensuring correctness.");
        assert_eq!(distance(Cell{x:ax, y:ay}, Cell{x:bx, y:by}), d);
    }
}

fn enumerate_resting_positions(state: &State) -> Vec<Unit> {
    let unit = &state.unit_sequence[0];

    let min = unit.members.iter().map(|&m| distance(unit.pivot, m)).min().unwrap();

    let mut valid_positions: Vec<Unit> = Vec::new();

    for x in (-min..state.width + min).rev() {
        for y in (-min..state.height + min).rev() {
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
    // We should have all valid positions. Now let's trim them; to start, we only want
    // ones that have either filled cells or floor below
    let mut i: usize = 0;
    loop {
        // done this way because length may change
        if i >= valid_positions.len() {
            break
        }
        #[inline]
        fn has_lower_neighbor(state: &State, c: Cell) -> bool {
            state.is_filled(Cell{x: c.x, y: c.y + 1}) ||
                if c.y % 2 == 0 {
                    state.is_filled(Cell{x: c.x - 1, y: c.y + 1})
                } else {
                    state.is_filled(Cell{x: c.x + 1, y: c.y + 1})
                }
        }
        let keep: bool = valid_positions[i].members.iter().any(|&c| {
            // fixme: make sure this isn't an off by one error
            c.y == state.height - 1 || has_lower_neighbor(state, c)
        });
        if !keep {
            valid_positions.remove(i);
            // don't increment i here because we're removing
        }
        else {
            i += 1;
        }
    }
    valid_positions
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
                                      "d".into(),
                                      "k".into(),
                                      "a".into(),
                                      "l".into()];
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
                                      "d".into(),
                                      "k".into(),
                                      "a".into(),
                                      "l".into()];
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

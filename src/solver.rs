extern crate num;

use super::*;
use super::Direction::*;
use super::Command::*;
use super::simulate::Lattice;
use super::opts::*;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Solver {
    AllDown,
    SolverSE,
    MonteCarlo,
    Supplied,
    BottomUp,
    BottomUpDfs,
    LookAhead,
}

pub fn name_to_solver(name: &str) -> Solver {
    let solvers: Vec<Solver> = vec![Solver::AllDown, Solver::SolverSE,
                                    Solver::MonteCarlo, Solver::Supplied, Solver::BottomUp,
                                    Solver::BottomUpDfs, Solver::LookAhead];
    for s in solvers.into_iter() {
        if s.name() == name {
            return s;
        }
    }
    Solver::BottomUpDfs
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
                let mut iters_per_time_check = 100;
                let mut time_per_iter;// = 1.0;
                let time_per_check_goal = if original_time_left < 2.0 { original_time_left/20.0 } else { 0.5 };
                for iters in 1..1000000000 {
                    let split_point = if best_cmds.len() > 0 { r.random() % best_cmds.len() } else { 0 };
                    let start: String = best_cmds[0..split_point].into();
                    let mid_state = simulate::score_commands(&string_to_commands(&start),
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
                        let current_time_left = opt.time_left();
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
                    // for i in 0 .. possible_next_positions.len() {
                    //     println!("could go to {},{}",
                    //              possible_next_positions[i].pivot.x,
                    //              possible_next_positions[i].pivot.y);
                    // }
                    if possible_next_positions.len() == 0 {
                        break;
                    }
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
            Solver::BottomUpDfs => {
                let extra_time = 1.0;

                let mut solution = String::new();
                let mut s = state.clone();

                let mut pop_sorted = opt.phrases_of_power.clone();
                pop_sorted.sort_by(|a, b| b.len().cmp(&a.len()));
                // let seqs: Vec<Vec<Command>> = moves.iter().map(|s| { string_to_commands(s) }).collect();

                let mut find_path_opt = opt.clone();

                let time_per_piece = opt.time_left() / (s.unit_sequence.len()+2) as f64;

                'bu_dfs_main_loop: while !s.game_over {
                    let possible_next_positions = enumerate_resting_positions(&s);
                    // for i in 0 .. possible_next_positions.len() {
                    //     println!("could go to {},{}",
                    //              possible_next_positions[i].pivot.x,
                    //              possible_next_positions[i].pivot.y);
                    // }
                    if possible_next_positions.len() == 0 {
                        break;
                    }
                    for u in possible_next_positions {
                        if opt.time_left() < 0. {
                            // aaack, we are late!!!
                            break;
                        }
                        let pieces_left = s.unit_sequence.len() as f64;
                        find_path_opt.time_limit = opt.time_limit - (pieces_left+0.3)*time_per_piece;
                        match find_path_dfs(&s, &u, &[], &find_path_opt) {
                            None => (),
                            Some(_) => {
                                // we want extra time for when we're using pops
                                find_path_opt.time_limit = opt.time_limit - pieces_left*time_per_piece;
                                match find_path_dfs(&s, &u, &opt.phrases_of_power, &find_path_opt) {
                                    Some((mut more_cmds, _)) => {
                                        more_cmds = more_cmds + "l";
                                        s = s.apply_sequence(&string_to_commands(&more_cmds));
                                        solution = solution + &more_cmds;

                                        if opt.verbose {
                                            println!("Got {} to get to {},{}", more_cmds,
                                                     u.pivot.x, u.pivot.y);
                                            println!("{}", s.visualize());
                                        }
                                        break;
                                    }
                                    None => (),
                                }
                            }
                        }
                        if opt.time_left() < extra_time { break 'bu_dfs_main_loop; }
                    }
                }

                // fixme: Ideally we should be tracking this as we go so we can use it.
                let pop_score = simulate::score_pop(&solution, &opt.phrases_of_power);
                s.score += pop_score;


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
            Solver::LookAhead => {
                unimplemented!()
                // let depth: u8 = 0;

                // let extra_time = Duration::seconds(1);
                // let start = PreciseTime::now();
                // let time_limit = Duration::seconds(opt.time_limit as i64);

                // let mut solution = String::new();
                // let mut s = state.clone();

                // let mut pop_sorted = opt.phrases_of_power.clone();
                // pop_sorted.sort_by(|a, b| b.len().cmp(&a.len()));

                // let mut now;

                // 'lu_main_loop: while !s.game_over {
                //     let nlooks = s.unit_sequence.len() as i32 / (depth + 1) as i32;
                //     now = PreciseTime::now();
                //     let time_to_look = (time_limit - start.to(now) - extra_time) / nlooks;
                //     let (new_sol, new_state) = look_ahead_dfs(&s, &solution, depth, &pop_sorted, time_to_look);
                //     solution = new_sol;
                //     s = new_state;
                // }

                // (Solution {
                //     problemId: input.id,
                //     seed: s.seed,
                //     tag: match opt.tag {
                //         None => Some(format!("{}[{},{}] = {}", self.name(),
                //                              input.id, s.seed, s.score)),
                //         ref tag => tag.clone(),
                //     },
                //     solution: solution,
                // }, s.score)
            },
        }
    }

    pub fn solve_n(&self, args: &[(State, Input, DavarOptions)]) -> Vec<(Solution, Score)> {
        let nargs = args.len() as f64;
        let mut solutions = Vec::new();
        let buffer_time = 0.5; // this is maybe the time needed to do
                               // the printing and all.
        for i in 0 .. args.len() {
            let fraction_of_time = (i as f64 + 1.0)/nargs;
            let mut opts = args[i].2.clone();
            opts.time_limit = fraction_of_time*(opts.time_limit - buffer_time);
            // println!("{}/{} seconds left",
            //          opts.time_limit + opts.starting_time - time::precise_time_s(),
            //          opts.time_limit);
            let (sol, sc) = self.solve(&args[i].0, &args[i].1, &opts);
            if opts.verbose {
                println!("finished {}[{}, {}] = {}", self.name(), sol.problemId, sol.seed, sc);
            }
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
            Solver::BottomUpDfs => "bottomupdfs".into(),
            Solver::LookAhead => "lookahead".into(),
        }
    }
}

// fn look_ahead_dfs(state: &State, route_so_far: &String, remaining_depth: u8, pops: &[String], time_allowed: Duration)
//                       -> (String, State)
// {

//     let time_allowed = Duration::seconds(10);
//     let possible_positions = enumerate_resting_positions(&state);
//     let mut routes_and_states = possible_positions.iter()
//         .filter_map(|u| find_path_dfs(&state, &u, pops, time_allowed));

//     if remaining_depth == 0 || state.game_over {
//         if let Some((mut route, mut final_state)) = routes_and_states.next() {
//             for (r, s) in routes_and_states {
//                 if s.score > state.score {
//                     route = r;
//                     final_state = s;
//                 }
//             }
//             final_state = final_state.apply_sequence(&string_to_commands(route_so_far));
//             final_state.visualize();
//             return (format!("{}{}", route_so_far, route), final_state);
//         } else {
//             // no valid states found? This shouldn't happen.
//             return ("".into(), state.clone());
//         }
//     } else {
//         unreachable!();
//         let mut better_routes_and_states = routes_and_states.map(|(r, s)| {
//             let route_to_use = format!("{}{}", route_so_far, r);
//             // fixme: calculate
//             let time_for_inner = Duration::seconds(0);
//             look_ahead_dfs(&s, &route_to_use, remaining_depth - 1, pops, time_for_inner)
//         });
//         if let Some((mut route, mut final_state)) = better_routes_and_states.next() {
//             for (r, s) in better_routes_and_states {
//                 if s.score > state.score {
//                     route = r;
//                     final_state = s;
//                 }
//             }
//             return (format!("{}{}", route_so_far, route), final_state);
//         } else {
//             // no valid states found? This shouldn't happen.
//             return ("".into(), state.clone());
//         }
//     }
// }

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
        for _ in 0 .. 40 {
            match self.find_path_once(input_s, target, options, cmds) {
                None => (),
                x => {
                    return x;
                },
            }
        }
        None
    }

    fn find_path_once(&mut self, input_s: &State, target: &Unit, options: &[String], cmds: &[Vec<Command>])
                      -> Option<(String, State)> {
        let mut s = input_s.clone();
        let mut all_cmds = String::new();
        let mut attempts = 0;
        let num_units = s.unit_sequence.len();

        let debug_path = false;
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
            if attempts > 4*s.width {
                if debug_path {
                    println!("NO PATH to level {} for {},{}!",
                             level, target.pivot.x, target.pivot.y);
                }
                return None;
            }
        }
        if s.unit_sequence[0].pivot.y == target.pivot.y {
            for _ in 0..4*s.width {
                if s.unit_sequence[0] == *target {
                    println!("Found a path to target at {}, {} from {},{}! ({} left)",
                             target.pivot.x, target.pivot.y,
                             s.unit_sequence[0].pivot.x,
                             s.unit_sequence[0].pivot.y,
                             s.unit_sequence.len());
                    for _ in 0..6 {
                        let (more, snew) = self.commands(&s, &options[4..6], &cmds[4..6]);
                        if snew.unit_sequence.len() != num_units {
                            // println!("Found the finisher");
                            println!("{}", snew.visualize());
                            return Some((all_cmds + &more, snew));
                        }
                    }
                }
                let (more, snew) = self.commands(&s, &options[0..4], &cmds[0..4]);
                if snew.unit_sequence.len() != num_units {
                    continue;
                }
                all_cmds = all_cmds + &more;
                s = snew;
            }
        } else {
            if debug_path {
                println!("NO PATH to target: got wrong level {} for {}, {}!",
                         s.unit_sequence[0].pivot.y,
                         target.pivot.x, target.pivot.y);
                }
        }
        if debug_path {
            println!("NO PATH to target at {}, {}!",
                     target.pivot.x, target.pivot.y);
        }
        None
                      }
    // fn look_ahead_mc(&self, state: &State, route_so_far: &String, remaining_depth: u8,
    //                  options: &[String], cmds: &[Vec<Command>]) -> (String, State) {

    //     let possible_positions = enumerate_resting_positions(&state);
    //     let mut routes_and_states = possible_positions.iter()
    //         .filter_map(|u| self.find_path(&state, u, options, cmds));

    //     if remaining_depth == 0 || state.game_over {
    //         if let Some((mut route, mut final_state)) = routes_and_states.next() {
    //             for (r, s) in routes_and_states {
    //                 if s.score > state.score {
    //                     route = r;
    //                     final_state = s;
    //                 }
    //             }
    //             return (format!("{}{}", route_so_far, route), final_state);
    //         } else {
    //             // no valid states found? This shouldn't happen.
    //             return (("".into(), state.clone()));
    //         }
    //     } else {
    //         let mut better_routes_and_states = routes_and_states.map(|(r, s)| {
    //             let route_to_use = format!("{}{}", route_so_far, r);
    //             self.look_ahead_mc(&s, &route_to_use, remaining_depth-1, options, cmds)
    //         });
    //         if let Some((mut route, mut final_state)) = better_routes_and_states.next() {
    //             for (r, s) in better_routes_and_states {
    //                 if s.score > state.score {
    //                     route = r;
    //                     final_state = s;
    //                 }
    //             }
    //             return (format!("{}{}", route_so_far, route), final_state);
    //         } else {
    //             // no valid states found? This shouldn't happen.
    //             return (("".into(), state.clone()));
    //         }
    //     }
    // }
}

fn get_score(s: &State, goal: &Unit, move_string: &String) -> i32 {         // Return how much closer the move gets you
    use std::i32;
    //println!("finding distance for {}", move_string);
    let mut s0 = s.clone();
    let start_dist = distance(s0.unit_sequence[0].pivot, goal.pivot);
    //println!("Before move looks like: \n{}", s0.visualize());
    let num_units = s.unit_sequence.len();

    for cmd in string_to_commands(&move_string[..]) {
        s0 = s0.apply(cmd);
        //println!("After a move: \n{}", s0.visualize());
        if s0.game_over || s0.unit_sequence.len() != num_units {
            //println!("Blah. That move caused an invalid state.");
            return i32::MIN         // Game-ending move. Return lowest weight possible.
        }
    }

    //println!("for {} found score to be: {}", move_string, start_dist - distance(s0.unit_sequence[0].pivot, goal.pivot));
    return start_dist - distance(s0.unit_sequence[0].pivot, goal.pivot)
}

fn get_move_ranking_dfs(s: &State, goal: &Unit, pop: &[String], moves: &[String]) -> Option<Vec<String>> {
    let mut recommended_moves: Vec<String> = Vec::new();        // Order moves powerwords first by distance-minimizing

    let mut pop_cpy: Vec<String> = pop.clone().into();  // Order phrases by distance-minimizing
    pop_cpy.sort_by( |a, b| get_score(s, goal, b).cmp(&get_score(s, goal, a)) );

    // pop_cpy.filter(|&phrase| get_score(s, goal, &phrase) >= 0)
    for phrase in pop_cpy {
        if get_score(s, goal, &phrase) >= -1 {           // If it gets you closer, add it to moves
            recommended_moves.push(phrase.clone());
        }
    }

    let mut moves_cpy: Vec<String> = moves.clone().into();  // Then order moves by distance-minimizing
    moves_cpy.sort_by( |a, b| get_score(s, goal, b).cmp(&get_score(s, goal, a)) );
    for mov_str in moves_cpy {
        if get_score(s, goal, &mov_str) >= -1 {
            recommended_moves.push(mov_str.clone());
        }
    }
    //println!("recommended moves: {:?}", recommended_moves);
    if recommended_moves.len() > 0 {
        Some(recommended_moves)
    } else {
        None
    }
}

pub fn find_path_dfs(s: &State, goal_unit: &Unit, pop: &[String],
                     opt: &DavarOptions) -> Option<(String, State)> {

    let mut state = s.clone();
    let mut out_cmd_stack: Vec<String> = Vec::new();

    let moves: Vec<String> = vec!["p".into(), "b".into(), "a".into(), "l".into(), "d".into(), "k".into()];

    let mut dfs_stack: Vec<(State, usize)> = Vec::new();
    let mut cur_move_idx: usize = 0;

    //println!("Finding Nemo!");

    let mut units_moved_down_to = Vec::new();
    loop {
        //println!("entered first loop. len: {}", dfs_stack.len());
        while let Some(next_moves) = get_move_ranking_dfs(&state, &goal_unit, pop, &moves[..]) {
            //println!("entered second loop");

            if cur_move_idx >= next_moves.len() {
                break;
            }

            dfs_stack.push( (state.clone(), cur_move_idx) );
            out_cmd_stack.push(next_moves[cur_move_idx].clone());

            let commands = string_to_commands(&next_moves[cur_move_idx]);
            state = state.apply_sequence(&commands);
            match commands[commands.len()-1] {
                Move(SW) | Move(SE) | Rotate(_) => {
                    if units_moved_down_to.contains(&state.unit_sequence[0]) {
                        // println!("We saved some time at {},{} ({} explored) {}",
                        //          state.unit_sequence[0].pivot.x,
                        //          state.unit_sequence[0].pivot.y,
                        //          units_moved_down_to.len(),
                        //          out_cmd_stack.connect(""));
                        break;
                    }
                    // println!("We found a new thing at level {}",
                    //          state.unit_sequence[0].pivot.y);
                    units_moved_down_to.push(state.unit_sequence[0].clone());
                },
                _ => (),
            }

            cur_move_idx = 0;

            // win!
            // fixme: This will succeed even if we don't have correct rotation. CHECK ROTATION.
            if state.unit_sequence[0] == *goal_unit {
                // println!("Got {},{} using {} ({} left)", goal_unit.pivot.x,
                //          goal_unit.pivot.y, out_cmd_stack.connect(""),
                //          state.unit_sequence.len());
                return Some((out_cmd_stack.connect(""), state));
            }
        }
        //println!("Exited first LOOPOPOPPPPPO*******************. len: {}", dfs_stack.len());
        // We've tried all paths and nothing works or we're out of time
        if dfs_stack.len() == 0 || opt.time_left() < 0.0 {
            //panic!();
            return None;
        }
        //println!("Backtracking.");
        let (old_state, old_move_idx) = dfs_stack.pop().unwrap();
        state = old_state;
        cur_move_idx = old_move_idx + 1;
        out_cmd_stack.pop();
    }
}

// pub fn find_path(s: &State, goal: &Unit) -> Option(&[Commands]) {
//     let mut s = s.clone();
//     let mut all_cmds = String::new();
//     for _ in 0 .. max_cmds {
//         let (more, snew) = self.commands(&s, options, cmds);

//     }
// }

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
    if state.unit_sequence.len() == 0 {
        return Vec::new();
    }
    let unit = &state.unit_sequence[0];

    let min = unit.members.iter().map(|&m| distance(unit.pivot, m)).min().unwrap();

    let mut valid_positions: Vec<Unit> = Vec::new();

    let mut orientations: Vec<Unit> = Vec::new();
    {
        let mut u = unit.clone();
        for _ in 0..6 {
            if !orientations.contains(&u) {
                orientations.push(u.clone());
            } else {
                break;
            }
            u.rotate(Clock::Wise);
        }
    }
    for y in (-min..state.height + min).rev() {
        for x in (-min..state.width + min) {
            let final_pivot = Cell::new(x, y);
            let delta = Lattice::from(final_pivot) - Lattice::from(unit.pivot);
            for u in orientations.iter() {
                let final_members = u.members.iter().map(|&m| Cell::from(Lattice::from(m) + delta));
                let unit = Unit{pivot: final_pivot, members: final_members.collect()};
                if !state.is_unit_invalid(&unit) {
                    valid_positions.push(unit);
                }
            }
        }
    }
    // We should have all valid positions. Now let's trim them; to start, we only want
    // ones that have either filled cells or floor below
    let mut real_positions = Vec::with_capacity(valid_positions.len());
    for u in valid_positions {
        #[inline]
        fn has_lower_neighbor(state: &State, c: Cell) -> bool {
            state.is_filled(Cell{x: c.x, y: c.y + 1}) ||
                if c.y % 2 == 0 {
                    state.is_filled(Cell{x: c.x - 1, y: c.y + 1})
                } else {
                    state.is_filled(Cell{x: c.x + 1, y: c.y + 1})
                }
        }
        if u.members.iter().any(|&c| {
            c.y == state.height - 1 || has_lower_neighbor(state, c)
        }) {
            real_positions.push(u);
        }
    }

    // let's sort them by center of mass!
    #[inline]
    fn center_of_mass(unit: &Unit) -> f32 {
        let y = unit.members.iter().fold(0, |curr_y, &cell| curr_y + cell.y);
        y as f32 / unit.members.len() as f32
    }
    real_positions.sort_by(|a, b| center_of_mass(b).partial_cmp(&center_of_mass(a)).unwrap_or(::std::cmp::Ordering::Equal));
    real_positions
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

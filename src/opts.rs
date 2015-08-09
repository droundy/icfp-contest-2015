extern crate getopts;
extern crate time;

use std::env;
use std::process;
use std::vec::Vec;

#[derive(Debug, PartialEq, Clone)]
pub struct DavarOptions {
    pub ncores: usize,
    pub submit: bool,
    pub files: Vec<String>,
    pub time_limit: f64,
    pub memory_limit: Option<usize>,
    pub phrases_of_power: Vec<String>,
    pub solver: String,
    pub animate: Option<u32>,
    pub starting_time: f64,
    pub seed: Option<i32>,
    pub solution: Option<String>,
}


impl DavarOptions {
    pub fn time_left(&self) -> f64 {
        self.time_limit as f64 - time::precise_time_s() + self.starting_time
    }
}

pub fn opts() -> DavarOptions {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optflag("", "submit", "submit to server");
    opts.optopt("", "solver", "name of solver algorithm", "ALGORITHM");
    opts.optopt("c", "", "number of cores", "NCORE");
    opts.optflag("h", "help", "print this help menu");
    opts.optmulti("f", "", "input filename", "FILENAME");
    opts.optopt("t", "", "time limit", "SECONDS");
    opts.optopt("m", "", "memory limit", "MEGABYTES");
    opts.optmulti("p", "", "phrase of power", "PHRASE");
    opts.optopt("", "animate", "MILISECONDS", "display animation of solution");
    opts.optopt("", "seed", "INT", "specify if you only want to run for a single seed");
    opts.optopt("", "solution", "STRING", "Only used with \"supplied\" solver. Instead of running an algorithm, will just solve with this solution.");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        let brief = format!("Usage: {} [options]", program);
        print!("{}", opts.usage(&brief));
        process::exit(0);
    }
    let mut davar_options = DavarOptions {
        ncores: 1,
        submit: matches.opt_present("submit"),
        files: matches.opt_strs("f"),
        time_limit: 60.0*60.0*24.0, // one day time limit!
        memory_limit: None,
        phrases_of_power: matches.opt_strs("p"),
        solver: "alldone".into(),
        animate: None,
        starting_time: time::precise_time_s(),
        seed: None,
        solution: None,
    };
    if let Some(nc) = matches.opt_str("c") {
        davar_options.ncores = nc.parse().unwrap();
    }
    if let Some(t) = matches.opt_str("t") {
        davar_options.time_limit = t.parse().unwrap();
    }
    if let Some(m) = matches.opt_str("m") {
        davar_options.memory_limit = Some(m.parse().unwrap());
    }
    if let Some(alg) = matches.opt_str("solver") {
        davar_options.solver = alg;
    }
    if let Some(a) = matches.opt_str("animate") {
        davar_options.animate = Some(a.parse().unwrap());
    }
    if let Some(s) = matches.opt_str("seed") {
        davar_options.seed = Some(s.parse().unwrap());
    }

    davar_options.solution = matches.opt_str("solution");

    davar_options
}

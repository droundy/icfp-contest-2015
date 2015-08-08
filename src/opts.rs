extern crate getopts;

use std::env;
use std::process;
use std::vec::Vec;

pub struct DavarOptions {
    pub ncores: usize,
    pub submit: bool,
    pub files: Vec<String>,
    pub time_limit: Option<usize>,
    pub memory_limit: Option<usize>,
    pub phrases_of_power: Vec<String>,
    pub solver: String,
}

pub fn opts() -> DavarOptions {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optflag("", "submit", "submit to server");
    opts.optopt("", "solver", "name of solver algorithm", "ALGORITHM");
    opts.optopt("c", "", "number of cores", "NCORE");
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("f", "", "input filename", "FILENAME");
    opts.optopt("t", "", "time limit", "SECONDS");
    opts.optopt("m", "", "memory limit", "MEGABYTES");
    opts.optmulti("p", "", "phrase of power", "PHRASE");
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
        time_limit: None,
        memory_limit: None,
        phrases_of_power: matches.opt_strs("p"),
        solver: "alldone".into(),
    };
    if let Some(nc) = matches.opt_str("c") {
        davar_options.ncores = nc.parse().unwrap();
    }
    if let Some(t) = matches.opt_str("t") {
        davar_options.time_limit = Some(t.parse().unwrap());
    }
    if let Some(m) = matches.opt_str("m") {
        davar_options.memory_limit = Some(m.parse().unwrap());
    }
    if let Some(alg) = matches.opt_str("solver") {
        davar_options.solver = alg;
    }
    davar_options
}

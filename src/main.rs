mod puzzles;

use crate::puzzles::day01;
use getopts::Options;
use std::env;

const AOC_VERSION: &str = "0.1.0";

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] PUZZLE INPUTFILE", program);
    print_version(&program);
    print!("{}", opts.usage(&brief));
}

fn print_version(program: &str) {
    println!("{} {} -- solve Advent of Code 2020 puzzles\n", program, AOC_VERSION);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = "adventofcode-2020";

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help and exit");
    opts.optflag("v", "version", "print version information and exit");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(f) => { panic!("{}", f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    if matches.opt_present("v") {
        print_version(&program);
        return;
    }
    if matches.free.len() < 2 {
        print_usage(&program, opts);
        return;
    }
    let puzzle = matches.free[0].clone();
    let infile = matches.free[1].clone();

    match puzzle.as_str() {
        "day01" => day01::solve(&infile),
        &_ => panic!("Unknown puzzle: {}", puzzle.as_str())
    }
}

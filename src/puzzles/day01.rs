use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut numbers = Vec::<usize>::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let number: usize = line.parse().unwrap();
        numbers.push(number);
    }

    println!("{}", solve_for(numbers.clone(), 2));
    println!("{}", solve_for(numbers, 3));
}

fn solve_for(numbers: Vec<usize>, combinations: usize) -> usize {
    match numbers.into_iter().combinations(combinations).find(|x| x.iter().sum::<usize>() == 2020) {
        Some(v) => v.iter().product::<usize>(),
        None => panic!("No result found")
    }
}

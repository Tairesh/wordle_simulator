mod solver;
mod wordle;

use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let words = include_str!("../words.txt")
        .split('\n')
        .map(String::from)
        .collect::<Vec<String>>();

    let target = match std::env::args().nth(1) {
        None => words
            .choose(&mut thread_rng())
            .unwrap() // Unreachable
            .clone(),
        Some(target) => target,
    };

    let (solution, attempts) = solver::solve(&target, &words);
    println!("{solution}\n");
    for (word, matches) in attempts {
        println!("{} ||{}||", matches, word);
    }
}

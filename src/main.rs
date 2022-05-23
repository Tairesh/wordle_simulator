mod solver;
mod wordle;

fn main() {
    let words = include_str!("../dictionary.txt")
        .split('\n')
        .collect::<Vec<&str>>();

    let targets = include_str!("../targets.txt")
        .split('\n')
        .collect::<Vec<&str>>();

    for target in std::env::args().skip(1) {
        let (solution, attempts) = solver::solve(
            &target,
            &words,
            solver::MAX_ATTEMPTS,
            if target.chars().count() == 5 {
                Some("тоска")
            } else {
                None
            },
            &targets,
        );
        println!("{solution}\n");
        for (word, matches) in attempts {
            println!("{matches} ||{word}||");
        }
    }
}

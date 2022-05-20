mod wordle;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let target = args
        .get(1)
        .unwrap_or_else(|| {
            println!("No word provided");
            exit(1);
        })
        .clone();
    if target.is_empty() {
        println!("Invalid argument");
        exit(1);
    }

    let mut words: Vec<String> = include_str!("../words.txt")
        .split('\n')
        .filter(|w| w.len() == target.len())
        .map(|s| s.trim().to_lowercase().replace('ё', "е"))
        .collect();

    let mut results = Vec::new();
    let mut rng = thread_rng();

    while !words.is_empty() {
        let current_word =
            // TODO: smarter choose
            words.choose(&mut rng).unwrap().clone();

        let result = wordle::diff(current_word.as_str(), target.as_str());
        results.push((current_word.clone(), result.clone()));
        if result.success() {
            break;
        }
        if results.len() == 6 {
            break;
        }

        words = words
            .into_iter()
            .filter(|w| wordle::filter_word(w.as_str(), &results))
            .collect();
    }

    if results.last().unwrap().1.success() {
        println!("Wordle {}/6", results.len());
    } else {
        println!("Wordle x/6");
    }
    println!();
    for (word, result) in results {
        println!("{} ||{}||", result, word);
    }
}

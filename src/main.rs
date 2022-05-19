mod wordle;

extern crate serde;
extern crate serde_json;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::process::exit;

fn main() {
    let words: Vec<String> = serde_json::from_str(include_str!("../words.json")).unwrap();
    let mut words: Vec<String> = words
        .into_iter()
        .filter(|w| w.len() == wordle::WORD_LEN * 2)
        .collect();

    let args: Vec<String> = std::env::args().collect();
    let target = args
        .get(1)
        .unwrap_or_else(|| {
            println!("No word provided");
            exit(1);
        })
        .clone();
    if target.is_empty() || target.len() != wordle::WORD_LEN * 2 {
        println!("Invalid argument");
        exit(1);
    }
    let mut results = Vec::new();
    let mut rng = thread_rng();

    while !words.is_empty() {
        let current_word = if results.is_empty() {
            String::from(
                *[
                    "сдоба",
                    "белка",
                    "нитка",
                    "полка",
                    "пакет",
                    "багет",
                    "лимон",
                ]
                .choose(&mut rng)
                .unwrap(),
            )
        } else {
            // TODO: smarter choose
            words.choose(&mut rng).unwrap().clone()
        };

        let result = wordle::check_word(current_word.as_str(), target.as_str());
        results.push((current_word.clone(), result.clone()));
        if result.success() {
            break;
        }
        if results.len() == 6 {
            break;
        }

        words = words
            .into_iter()
            .filter(|w| wordle::filter_word(w, &results))
            .collect();
    }

    if results.last().unwrap().1.success() {
        println!("Wordle {}/6", results.len());
    } else {
        println!("Wordle x/6");
    }
    for (word, result) in results {
        println!("{} ({})", result.as_string(), word);
    }
}
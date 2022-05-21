use crate::wordle::{self, Matches};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt::{Display, Formatter};

const MAX_ATTEMPTS: usize = 6;

#[derive(Debug)]
pub enum Solution {
    Success(usize),
    Failure,
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Solution::Success(tries) => {
                f.write_str(format!("Wordle {tries}/{MAX_ATTEMPTS}").as_str())
            }
            Solution::Failure => f.write_str(format!("Wordle x/{MAX_ATTEMPTS}").as_str()),
        }
    }
}

pub type Attempt = (String, Matches);
pub type Attempts = Vec<Attempt>;

pub fn solve(target: &str, words: &[String]) -> (Solution, Attempts) {
    let mut attempts = Vec::new();
    let mut rng = thread_rng();
    let mut words: Vec<String> = words
        .iter()
        .filter(|w| w.len() == target.len())
        .cloned()
        .collect();

    while !words.is_empty() {
        // TODO: smarter choose
        let current_word = words
            .choose(&mut rng)
            .unwrap() // Unreachable
            .clone();

        let diff = wordle::diff(current_word.as_str(), target);
        attempts.push((current_word, diff.clone()));

        if diff.success() {
            return (Solution::Success(attempts.len()), attempts);
        }
        if attempts.len() > MAX_ATTEMPTS {
            return (Solution::Failure, attempts);
        }

        words = words
            .into_iter()
            .filter(|w| wordle::filter_word(w, &attempts))
            .collect();
    }

    (Solution::Failure, attempts)
}

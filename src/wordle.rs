use std::fmt::Display;
use std::ops::Index;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Match {
    Green,
    Yellow,
    Gray,
}

impl Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Match::Green => write!(f, "🟩"),
            Match::Yellow => write!(f, "🟨"),
            Match::Gray => write!(f, "⬛"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Matches(pub Vec<Match>);

impl Matches {
    pub fn success(&self) -> bool {
        self.0.iter().all(|r| *r == Match::Green)
    }
}

impl Display for Matches {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for m in &self.0 {
            write!(f, "{}", m)?;
        }
        write!(f, "")
    }
}

impl Index<usize> for Matches {
    type Output = Match;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

pub fn diff(word: &str, target: &str) -> Matches {
    let word = str_to_chars(word);
    let mut target = str_to_chars(target);

    let mut diff = vec![Match::Gray; target.len()];
    let diff_slice = diff.as_mut_slice();

    word.iter().enumerate().for_each(|(i, c)| {
        if target[i] == *c {
            diff_slice[i] = Match::Green;
            target[i] = ' '; // letters only match once
        }
    });

    word.iter().enumerate().for_each(|(i, &b)| {
        if diff_slice[i] == Match::Gray {
            if let Some(j) = target.iter().position(|&x| x == b) {
                target[j] = ' '; // letters only match once
                diff_slice[i] = Match::Yellow;
            }
        }
    });

    Matches(diff)
}

pub fn filter_word(word: &str, results: &Vec<(String, Matches)>) -> bool {
    let word = str_to_chars(word);
    let word = word.as_slice();
    for (current, result) in results {
        let current = str_to_chars(current.as_str());
        let current = current.as_slice();
        for (i, letter) in result.0.iter().enumerate() {
            let c = current[i];
            let count = current.iter().filter(|p| **p == c).count();
            match letter {
                Match::Green => {
                    if word[i] != current[i] {
                        return false;
                    }
                }
                Match::Yellow => {
                    if !word.contains(&current[i]) || word[i] == current[i] {
                        return false;
                    }
                }
                Match::Gray => {
                    if word[i] == current[i] {
                        return false;
                    }
                    if count == 1 && word.contains(&c) {
                        return false;
                    }
                    // TODO: if count > 1 ???
                }
            }
        }
    }

    true
}

fn str_to_chars(word: &str) -> Vec<char> {
    word.chars().into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::{diff, Match};

    // ТКАНЬ tested here: https://wordle.belousov.one/?word_id=XgT7TH8clN1

    #[test]
    fn check_word_test() {
        let result = diff("сдоба", "ткань");
        assert_eq!(
            vec![
                Match::Gray,
                Match::Gray,
                Match::Gray,
                Match::Gray,
                Match::Yellow,
            ],
            result.0
        );
        assert_eq!(false, result.success());
    }

    #[test]
    fn check_second_occurrence_is_gray() {
        let result = diff("канал", "ткань");
        assert_eq!(
            vec![
                Match::Yellow,
                Match::Yellow,
                Match::Yellow,
                Match::Gray,
                Match::Gray,
            ],
            result.0,
        );
        assert_eq!(false, result.success());
        assert_eq!(Match::Yellow, result[0]);
    }

    #[test]
    fn check_second_occurrence_is_gray2() {
        let result = diff("коала", "ткань");
        assert_eq!(
            vec![
                Match::Yellow,
                Match::Gray,
                Match::Green,
                Match::Gray,
                Match::Gray,
            ],
            result.0,
        );
        assert_eq!(false, result.success());
    }

    #[test]
    fn check_second_occurrence_is_gray3() {
        let result = diff("пиала", "пизда");
        assert_eq!(
            vec![
                Match::Green,
                Match::Green,
                Match::Gray,
                Match::Gray,
                Match::Green,
            ],
            result.0,
        );
    }

    #[test]
    fn check_second_occurrence_is_yellow() {
        let result = diff("коала", "панда");
        assert_eq!(
            vec![
                Match::Gray,
                Match::Gray,
                Match::Yellow,
                Match::Gray,
                Match::Green,
            ],
            result.0,
        );
    }

    #[test]
    fn check_second_occurrence_is_yellow2() {
        let result = diff("шимми", "визит");
        assert_eq!(
            vec![
                Match::Gray,
                Match::Green,
                Match::Gray,
                Match::Gray,
                Match::Yellow,
            ],
            result.0,
        );
    }

    #[test]
    fn check_success() {
        let result = diff("ткань", "ткань");
        assert_eq!(
            vec![
                Match::Green,
                Match::Green,
                Match::Green,
                Match::Green,
                Match::Green,
            ],
            result.0,
        );
        assert_eq!(true, result.success());
    }
}

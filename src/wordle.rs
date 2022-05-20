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

// TODO: tests
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
    use super::{
        diff,
        Match::{self, *},
    };

    use test_case::test_case;

    #[test_case("сдоба", "ткань", vec![Gray, Gray, Gray, Gray, Yellow]; "one yellow letter")]
    #[test_case("канал", "ткань", vec![Yellow, Yellow, Yellow, Gray, Gray]; "first occurrence is yellow, second is gray")]
    #[test_case("коала", "ткань", vec![Yellow, Gray, Green, Gray, Gray]; "first occurrence is green, second is gray")]
    #[test_case("пиала", "пизда", vec![Green, Green, Gray, Gray, Green]; "first occurrence is gray, second is green")]
    #[test_case("коала", "панда", vec![Gray, Gray, Yellow, Gray, Green]; "first occurrence is yellow, second is green")]
    #[test_case("шимми", "визит", vec![Gray, Green, Gray, Gray, Yellow]; "first occurrence is green, second is yellow")]
    #[test_case("ткань", "ткань", vec![Green, Green, Green, Green, Green]; "all greens")]
    fn diff_test(word: &str, target: &str, matches: Vec<Match>) {
        assert_eq!(diff(word, target).0, matches);
    }
}

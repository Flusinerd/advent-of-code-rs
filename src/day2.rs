use std::fs;

#[derive(PartialEq, Eq, Clone)]
enum Shapes {
    Rock,
    Paper,
    Scissor,
}

#[derive(PartialEq, Eq)]
enum Outcomes {
    Win,
    Loose,
    Draw,
}

pub fn day2() {
    let path = "src/day_2_input.txt";
    let mut sum_score = 0;
    for line in fs::read_to_string(path).expect("Expected File").lines() {
        let opponent_pick =
            char_to_shape(line.chars().nth(0).expect("Expected char")).expect("Invalid Char");
        let outcome_pick =
            char_to_outcome(line.chars().nth(2).expect("Expected char")).expect("Invalid Char");
        let my_pick = pick_my_shape(&opponent_pick, &outcome_pick);
        sum_score += calculate_score(my_pick, &opponent_pick);
    }
    println!("Score: {}", sum_score);
}

fn char_to_shape(character: char) -> Option<Shapes> {
    match character {
        'A' => Some(Shapes::Rock),
        'B' => Some(Shapes::Paper),
        'C' => Some(Shapes::Scissor),
        _ => None,
    }
}

fn char_to_outcome(character: char) -> Option<Outcomes> {
    match character {
        'X' => Some(Outcomes::Loose),
        'Y' => Some(Outcomes::Draw),
        'Z' => Some(Outcomes::Win),
        _ => None,
    }
}

// We need to specify the lifetime here, because the opponent_shape reference will be returned
// if the desired outcome is a draw
// The rust compiler needs to know, to keep the reference opponent_shape alive as long as the
// return value is alive
// More info: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
fn pick_my_shape<'a>(opponent_shape: &'a Shapes, outcome: &Outcomes) -> &'a Shapes {
    if outcome == &Outcomes::Draw {
        return opponent_shape;
    }

    match opponent_shape {
        Shapes::Rock => {
            if outcome == &Outcomes::Win {
                &Shapes::Paper
            } else {
                &Shapes::Scissor
            }
        }
        Shapes::Paper => {
            if outcome == &Outcomes::Win {
                &Shapes::Scissor
            } else {
                &Shapes::Rock
            }
        }
        Shapes::Scissor => {
            if outcome == &Outcomes::Win {
                &Shapes::Rock
            } else {
                &Shapes::Paper
            }
        }
    }
}

fn calculate_score(my_shape: &Shapes, opponent_shape: &Shapes) -> i32 {
    let mut score = 0;
    score += match my_shape {
        Shapes::Rock => 1,
        Shapes::Paper => 2,
        Shapes::Scissor => 3,
    };

    if my_shape == opponent_shape {
        return score + 3;
    }

    score += match my_shape {
        Shapes::Rock => {
            if opponent_shape == &Shapes::Paper {
                0
            } else {
                6
            }
        }
        Shapes::Paper => {
            if opponent_shape == &Shapes::Scissor {
                0
            } else {
                6
            }
        }
        Shapes::Scissor => {
            if opponent_shape == &Shapes::Rock {
                0
            } else {
                6
            }
        }
    };

    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(
        my_shape,
        opponent_shape,
        outcome,
        case(Shapes::Rock, Shapes::Rock, 4),
        case(Shapes::Paper, Shapes::Paper, 5),
        case(Shapes::Scissor, Shapes::Scissor, 6),
        case(Shapes::Rock, Shapes::Paper, 1),
        case(Shapes::Rock, Shapes::Scissor, 7),
        case(Shapes::Paper, Shapes::Rock, 8),
        case(Shapes::Paper, Shapes::Scissor, 2),
        case(Shapes::Scissor, Shapes::Rock, 3),
        case(Shapes::Scissor, Shapes::Paper, 9)
    )]
    fn test_calculate_score(my_shape: Shapes, opponent_shape: Shapes, outcome: i32) {
        let result = calculate_score(&my_shape, &opponent_shape);
        assert_eq!(result, outcome);
    }
}

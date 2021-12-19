use std::{collections::HashSet, num::ParseIntError, str::Lines};

use itertools::process_results;

use crate::error::{ErrorKind, MyError};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        let output = part_one(input.lines());
        assert!(output.is_ok());
        assert_eq!("4512".to_string(), output.unwrap());
    }

    #[test]
    fn part_two_example() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        let output = part_two(input.lines());
        assert!(output.is_ok());
        assert_eq!("1924".to_string(), output.unwrap());
    }
}

type Board<'a> = Vec<Vec<&'a str>>;

struct BoardWins<'a> {
    lines: Vec<Vec<&'a str>>,
    row_count: usize,
}
impl BoardWins<'_> {
    pub fn board_size(&self) -> usize {
        self.lines[0].len()
    }

    pub fn wins(&self, draws: &HashSet<&str>) -> bool {
        self.lines
            .iter()
            .any(|row_or_col| row_or_col.iter().all(|s| draws.contains(s)))
    }

    pub fn score(&self, draws: &HashSet<&str>) -> Result<i32, ParseIntError> {
        process_results(
            self.lines
                .iter()
                .take(self.row_count)
                .flatten()
                .filter(|&s| !draws.contains(s))
                .map(|s| s.parse::<i32>()),
            |nums| nums.sum(),
        )
    }
}

fn win_options(board: Board) -> BoardWins {
    let mut wins = vec![];

    // Collect each row
    wins.extend(board.iter().map(|row| row.clone().into_iter().collect()));

    // Then collect each col
    for idx in 0..board[0].len() {
        wins.push(board.iter().map(|line| line[idx]).collect());
    }

    BoardWins {
        lines: wins,
        row_count: board.len(),
    }
}

fn collect_boards(lines: Lines) -> Result<Vec<BoardWins>, MyError> {
    let mut boards: Vec<Board> = vec![];
    let mut board: Board = vec![];
    for line in lines {
        if line.trim().is_empty() {
            boards.push(board);
            board = vec![];
        } else {
            board.push(line.split_whitespace().collect());
        };
    }

    if !board.is_empty() {
        boards.push(board);
    }

    Ok(boards.into_iter().map(|b| win_options(b)).collect())
}

fn final_score(
    winner: &BoardWins,
    draw_set: &HashSet<&str>,
    last_draw: &str,
) -> Result<String, MyError> {
    let score = winner
        .score(draw_set)
        .map_err(|e| MyError::wrap_parse_int(e))?;

    let last_called = last_draw
        .parse::<i32>()
        .map_err(|e| MyError::wrap_parse_int(e))?;

    Ok(format!("{}", score * last_called))
}

pub fn part_one(mut lines: Lines) -> Result<String, MyError> {
    let draws: Vec<&str> = lines
        .next()
        .ok_or(MyError::new(ErrorKind::BadInput))?
        .split(',')
        .collect();
    lines.next(); // Advance past first blank line

    let board_win_states = collect_boards(lines)?;
    let board_len = board_win_states[0].board_size();

    let mut draw_set = HashSet::new();
    for draw in draws {
        draw_set.insert(draw);
        if draw_set.len() < board_len {
            continue;
        };

        let maybe_winner = board_win_states.iter().find(|board| board.wins(&draw_set));
        if let Some(winner) = maybe_winner {
            return final_score(winner, &draw_set, draw);
        }
    }

    Err(MyError::new(ErrorKind::Other("Nobody won!".to_string())))
}

pub fn part_two(mut lines: Lines) -> Result<String, MyError> {
    let draws: Vec<&str> = lines
        .next()
        .ok_or(MyError::new(ErrorKind::BadInput))?
        .split(',')
        .collect();
    lines.next(); // Advance past first blank line

    let mut board_win_states: Vec<BoardWins> = collect_boards(lines)?;
    let board_len = board_win_states[0].board_size();

    let mut draw_set = HashSet::new();
    for draw in draws {
        draw_set.insert(draw);
        if draw_set.len() < board_len {
            continue;
        };

        if board_win_states.iter().all(|board| board.wins(&draw_set)) {
            // If we're down to only winners left, then we've found the last board(s) to win!
            return final_score(&board_win_states[0], &draw_set, draw);
        } else {
            // Else we need to remove this round's winners (if any) and keep looping
            board_win_states.retain(|board| !board.wins(&draw_set));
        };
    }

    Err(MyError::new(ErrorKind::Other(
        "Some board(s) never won!".to_string(),
    )))
}

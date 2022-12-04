use std::fs::read_to_string;

struct Game {
    values: Vec<usize>,
    boards: Vec<Board>,
}

enum Objective {
    Win,
    Lose,
}

#[derive(Default, Debug, Clone)]
struct Board {
    values: Vec<usize>,
    completed: Vec<usize>,
    size: usize,
}

impl Board {
    fn is_completed(&self) -> bool {
        self.has_completed_row() || self.has_completed_column()
    }

    fn has_completed_row(&self) -> bool {
        (0..self.size)
            .any(|row| (0..self.size).all(|col| self.completed.contains(&(row * self.size + col))))
    }

    fn has_completed_column(&self) -> bool {
        (0..self.size)
            .any(|col| (0..self.size).all(|row| self.completed.contains(&(row * self.size + col))))
    }

    fn get_score(&mut self) -> usize {
        let score = self
            .values
            .iter()
            .enumerate()
            .filter_map(|(index, value)| (!self.completed.contains(&index)).then_some(value))
            .sum::<usize>();

        let winning_index = self
            .completed
            .pop()
            .expect("No completed indices on winning board");

        score * self.values[winning_index]
    }
}

fn main() {
    let data = parse_data("data/04.txt");

    let answer = part_one(&data);
    println!("Part 1: {answer}");

    let answer = part_two(&data);
    println!("Part 2: {answer}");
}

fn part_one(data: &str) -> usize {
    let game = parse_input(data).expect("Could not create game with data");
    let mut winning_board =
        play_game(game, Objective::Win).expect("Could not find winning board for game");

    winning_board.get_score()
}

fn part_two(data: &str) -> usize {
    let game = parse_input(data).expect("Could not create game with data");
    let mut losing_board =
        play_game(game, Objective::Lose).expect("Could not find losing board for game");

    losing_board.get_score()
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

fn parse_input(data: &str) -> Result<Game, Box<dyn std::error::Error>> {
    let mut lines = data.lines();

    let values = lines
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let _ = lines.next().unwrap();

    let mut size = 0;
    let mut boards = Vec::new();
    let mut board = Board::default();

    for line in lines {
        if line.is_empty() {
            board.size = size;
            boards.push(board);
            board = Board::default();
            continue;
        }

        let numbers = line.split_whitespace();
        size = numbers.clone().count();

        board.values.append(
            numbers
                .map(|number| number.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
                .as_mut(),
        );
    }
    board.size = size;
    boards.push(board);

    Ok(Game { values, boards })
}

fn play_game(mut game: Game, objective: Objective) -> Option<Board> {
    let mut winning_boards = Vec::new();
    let num_boards = game.boards.len();
    for value in game.values.iter() {
        for (board_index, board) in game.boards.iter_mut().enumerate() {
	    if winning_boards.contains(&board_index) {
		continue;
	    }
            for (value_index, board_value) in board.values.iter().enumerate() {
                if board_value == value {
                    board.completed.push(value_index);
                }
            }
            if board.is_completed() {
                match objective {
                    Objective::Win => {
                        return Some(board.clone());
                    }
                    Objective::Lose => {
                        winning_boards.push(board_index);
                        if winning_boards.len() == num_boards {
                            return Some(game.boards[board_index].clone());
                        }
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = parse_data("data/04-example.txt");
        assert_eq!(4512, part_one(&data));
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/04-example.txt");
        assert_eq!(1924, part_two(&data));
    }
}

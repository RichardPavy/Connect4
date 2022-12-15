use crate::shared::board::board_all_points::BoardAllPoints;
use crate::shared::board::board_get_set::BoardGet;
use crate::shared::board::board_size::BoardSize;
use crate::shared::coord::point::Point;

use super::symbol::Symbol;

pub fn play_connect4(difficulty: i32, winning_tokens: i32) -> Result<(), Error> {
    println!("Welcome to Connect4!");

    let me = Symbol::Red;
    println!("You are playing '{:?}'", me);
    println!("Computer is playing '{:?}'", me.other());

    let mut board = super::board::Connect4::<7, 6>::new(winning_tokens);

    println!("Board is now:");
    println!("{}", board.view_as_xo());

    println!("Player '{:?}' choose a column", me);

    let lines = std::io::stdin().lines();
    for line in lines {
        let line = line?.to_ascii_uppercase();
        let column = {
            match line.parse::<i32>() {
                Ok(column) => column - 1,
                Err(_err) => {
                    println!("Invalid number: {line}, try again");
                    continue;
                }
            }
        };
        if !board.is_valid(&Point::new(column, 0)) {
            println!(
                "Out ouf range, please choose a number between [1..{}]",
                board.width()
            );
            continue;
        }
        if let Some(my_move) = board.play_column(me, column) {
            println!("{:?} Playing {:?}", me, my_move.position.x + 1);

            println!("Board is now:");
            println!("{}", board.view_as_xo());

            if my_move.end_of_game {
                println!("{:?} won!!", me);
                break;
            }
        } else {
            println!("Please choose a different column");
            continue;
        }

        if let Some(opponent_move) = board.next_move(me.other(), difficulty) {
            println!("{:?} Playing {:?}", me.other(), opponent_move.column + 1);

            println!("Board is now:");
            println!("{}", board.view_as_xo());

            if opponent_move.end_of_game {
                println!("{:?} won!!", me.other());
                break;
            }
        }

        if board
            .all_points()
            .map(|point| board.get(&point))
            .all(|cell| cell.symbol != Symbol::Empty)
        {
            return Err(Error::EOG);
        }
    }

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    IoError(#[from] std::io::Error),

    #[error("End of the game")]
    EOG,
}

use std::char::ParseCharError;
use std::num::ParseIntError;

use lazy_static::lazy_static;

use regex::Regex;

use crate::shared::board::board_all_points::BoardAllPoints;
use crate::shared::board::board_generate::BoardGenerate;
use crate::shared::board::board_get_set::BoardGet;
use crate::shared::board::board_get_set::BoardSet;
use crate::shared::board::board_size::BoardSize;
use crate::shared::coord::point::Point;
use crate::tictactoe::board::Board;
use crate::tictactoe::tictactoe::Symbol;
use crate::tictactoe::tictactoe::TicTacToeBoard;

pub fn play_tictactoe() -> Result<(), Error> {
    println!("START");

    let me = Symbol::Circle;
    let mut board = Board::generate(|_| Symbol::None);
    board.set(&Point::new(0, 0), me.other());

    println!("Board is now:");
    println!("{}", board);

    let lines = std::io::stdin().lines();
    for line in lines {
        let run_round = || -> Result<(), Error> {
            let line = line?.to_ascii_uppercase();

            lazy_static! {
                static ref MOVE_REGEX: Regex = Regex::new("([1-3])[ \t]*([a-cA-C1A])").unwrap();
            }

            let point = {
                let parse = MOVE_REGEX
                    .captures(&line)
                    .ok_or(Error::NextMoveParseError { line: line.clone() })?;

                let x = {
                    let x = parse
                        .get(1)
                        .ok_or_else(|| Error::NextMoveParseError { line: line.clone() })?;
                    let x: Result<i32, _> = x.as_str().parse();
                    let x = x.map_err(|err| Error::ParseRowError(err))?;
                    x - 1
                };

                let y = {
                    let y = parse
                        .get(2)
                        .ok_or_else(|| Error::NextMoveParseError { line: line.clone() })?;
                    let y: Result<char, _> = y.as_str().parse();
                    let y = y.map_err(|err| Error::ParseColumnError(err))?;
                    y as i32 - 'A' as i32
                };

                Point::new(x, y)
            };
            println!("{:?} Plays {:?}", me, point);
            if !board.is_valid(&point) {
                return Err(Error::InvalidPoint { point });
            }

            *board.get_mut(&point) = me;
            println!("Board is now:");
            println!("{}", board);
            println!("");
            if board.is_winning_move(point, me) {
                println!("{:?} WINS !!!", me);
                return Err(Error::EOG);
            }

            let next = board.next_move(me.other()).ok_or(Error::EOG)?;
            *board.get_mut(&next.point()) = me.other();

            println!(
                "{:?} Plays {:?} with score {}",
                me.other(),
                next.point(),
                next.score()
            );
            println!("Board is now:");
            println!("{}", board);
            println!("");

            if board.is_winning_move(next.point(), me.other()) {
                println!("{:?} WINS !!!", me.other());
                return Err(Error::EOG);
            }

            if board
                .all_points()
                .map(|point| *board.get(&point))
                .all(|v| v != Symbol::None)
            {
                return Err(Error::EOG);
            }

            Ok(())
        };

        match run_round() {
            Ok(_) => (),
            Err(Error::EOG) => {
                println!("End of game");
                return Ok(());
            }
            Err(err) => println!("Failed with {err}"),
        }
    }
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    IoError(#[from] std::io::Error),

    #[error("Unable to parse the next move. Expected 'row:[1..3] column:[A-C]'. Example: '1 B'. Got '{}'", .line)]
    NextMoveParseError { line: String },

    #[error("Unable to parse the row number")]
    ParseRowError(ParseIntError),

    #[error("Unable to parse the column number")]
    ParseColumnError(ParseCharError),

    #[error("End of the game")]
    EOG,

    #[error("Point is out of the board: {:?}", .point)]
    InvalidPoint { point: Point },
}

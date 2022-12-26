use connect4::play::play_connect4;
use tictactoe::play::play_tictactoe;

mod connect4;
mod puzzlesolver;
mod rubicscube;
mod shared;
mod tictactoe;

fn main() {
    let difficulty = 1;
    let winning_tokens = 4;
    loop {
        println!("Type 't' for TicTacToe or 'c' for Connect4");
        let line = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().to_owned()
        };
        if line == "t" {
            play_tictactoe().unwrap();
            continue;
        } else if line == "c" {
            play_connect4(difficulty, winning_tokens).unwrap();
            continue;
        } else if line == "q" {
            println!("Goodbye");
            return;
        } else {
            println!("Unexpected input '{}'", line);
        }
    }
}

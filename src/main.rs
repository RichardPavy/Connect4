mod connect4;
mod shared;

fn main() {
    let difficulty = 1;
    let winning_tokens = 4;
    connect4::play::play_connect4(difficulty, winning_tokens).unwrap()
}

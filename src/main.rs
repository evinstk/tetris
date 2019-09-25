use ggez::GameResult;

mod tetris;

pub fn main() -> GameResult {
    tetris::run_game()
}

use std::io::stdin;
use termion::{event::Key, input::TermRead};

pub mod game;

use game::*;

fn main() {
    let mut game = Game::new();

    let stdin = stdin();

    game.render();
    for c in stdin.keys() {
        use Key::*;
        match c.unwrap() {
            Esc | Char('q') | Char('Q') | Ctrl('c') => break,
            Char(' ') | Char('\n') => game.update(Input::Continue),
            Up => game.update(Input::Up),
            Down => game.update(Input::Down),
            _ => continue,
        }
        game.render();
    }
}

pub mod card;

use card::*;
use rand::seq::SliceRandom;
use std::{
    fmt::Display,
    io::{stdout, Stdout, Write},
};
use termion::{
    color::*,
    cursor,
    raw::{IntoRawMode, RawTerminal},
    screen, style,
};

pub struct Terminal {
    terminal: cursor::HideCursor<RawTerminal<Stdout>>,
}

impl Terminal {
    pub fn println<T: Display>(&mut self, s: T) {
        write!(self.terminal, "{}\r\n", s).unwrap();
    }

    pub fn clear(&mut self) {
        write!(self.terminal, "{}", termion::clear::All).unwrap();
        write!(self.terminal, "{}", cursor::Goto(1, 1)).unwrap();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Input {
    Continue,
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Choice {
    Hit,
    Stand,
    // DoubleDown,
    Surrender,
}

impl Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Choice::*;
        match self {
            Hit => write!(f, "Hit"),
            Stand => write!(f, "Stand"),
            Surrender => write!(f, "Surrender"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameResult {
    Win,
    Lose,
    Tie,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stage {
    First,
    Second,
    Third,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Starting(Stage),
    Presenting,
    Selecting(Choice),
    Standing,
    GameOver(GameResult),
}

pub struct Game {
    terminal: Terminal,
    deck: Vec<Card>,
    state: State,
    player: Hand,
    dealer: Hand,
}

impl Game {
    pub fn new() -> Game {
        use Stage::*;
        use State::*;
        let mut terminal = cursor::HideCursor::from(stdout().into_raw_mode().unwrap());
        write!(terminal, "{}", screen::ToAlternateScreen).unwrap();
        Game {
            terminal: Terminal { terminal },
            deck: pack(),
            state: Starting(First),
            player: Hand::new(),
            dealer: Hand::new(),
        }
    }

    pub fn update(&mut self, input: Input) {
        use Choice::*;
        use GameResult::*;
        use Input::*;
        use Stage::*;
        use State::*;
        self.state = {
            match &self.state {
                Starting(stage) if input == Continue => match stage {
                    First => {
                        self.shuffle();
                        self.deal_player();
                        Starting(Second)
                    }
                    Second => {
                        self.deal_dealer();
                        Starting(Third)
                    }
                    Third => {
                        self.deal_player();
                        Selecting(Hit)
                    }
                },
                Selecting(choice) => match (input, choice) {
                    (Continue, Hit) => {
                        self.deal_player();
                        if self.player.points() > 21 {
                            GameOver(Lose)
                        } else {
                            Selecting(Hit)
                        }
                    }
                    (Continue, Stand) => {
                        self.deal_dealer();
                        if self.dealer.points() > 21 {
                            GameOver(Win)
                        } else if self.dealer.points() < 17 {
                            Standing
                        } else if self.dealer.points() > self.player.points() {
                            GameOver(Lose)
                        } else if self.dealer.points() < self.player.points() {
                            GameOver(Win)
                        } else {
                            GameOver(Tie)
                        }
                    }
                    (Continue, Surrender) => GameOver(Lose),

                    (Up, Hit) => Selecting(Surrender),
                    (Up, Stand) => Selecting(Hit),
                    (Up, Surrender) => Selecting(Stand),

                    (Down, Hit) => Selecting(Stand),
                    (Down, Stand) => Selecting(Surrender),
                    (Down, Surrender) => Selecting(Hit),
                },
                Presenting => Selecting(Hit),
                Standing if input == Continue => {
                    self.deal_dealer();
                    if self.dealer.points() > 21 {
                        GameOver(Win)
                    } else if self.dealer.points() < 17 {
                        Standing
                    } else if self.dealer.points() > self.player.points() {
                        GameOver(Lose)
                    } else if self.dealer.points() < self.player.points() {
                        GameOver(Win)
                    } else {
                        GameOver(Tie)
                    }
                }
                GameOver(_) if input == Continue => {
                    self.deck.append(&mut self.dealer.cards);
                    self.deck.append(&mut self.player.cards);
                    Starting(First)
                }
                _ => self.state.clone(),
            }
        }
    }

    pub fn render(&mut self) {
        use Choice::*;
        use GameResult::*;
        use Stage::*;
        use State::*;
        self.terminal.clear();

        self.terminal.println(format!(
            "{}{}BLACKJACK v0.1.0{}{}",
            Fg(White),
            style::Bold,
            style::Reset,
            Fg(Reset)
        ));
        self.terminal.println("");

        self.dealer.render(&mut self.terminal, "Dealer");
        self.player.render(&mut self.terminal, "You");

        match &self.state {
            Selecting(c) => {
                let select = |it: Choice| {
                    format!(
                        "{} {}{}",
                        if it == *c {
                            String::from(LightBlue.fg_str()) + ">"
                        } else {
                            "-".to_string()
                        },
                        it,
                        Fg(Reset)
                    )
                };
                self.terminal.println(select(Hit));
                self.terminal.println(select(Stand));
                self.terminal.println(select(Surrender));
            }
            Starting(First) => {
                self.terminal.println("Welcome to Blackjack!");
                self.terminal.println("[SPACE / ENTER] Start");
            }
            Presenting | Standing | Starting(_) => {
                self.terminal.println("[SPACE / ENTER] Continue");
            }
            GameOver(result) => {
                match result {
                    Win => self
                        .terminal
                        .println(format!("{}You win!{}", Fg(Green), Fg(Reset))),
                    Lose => self
                        .terminal
                        .println(format!("{}You lose!{}", Fg(Red), Fg(Reset))),
                    Tie => self
                        .terminal
                        .println(format!("{}It's a tie!{}", Fg(Yellow), Fg(Reset))),
                };
                self.terminal.println("[SPACE / ENTER] Play again");
                self.terminal.println("[ESC / Q] Quit");
            }
        };
        self.terminal.terminal.flush().unwrap();
    }

    fn shuffle(&mut self) {
        self.deck.shuffle(&mut rand::thread_rng());
    }

    fn deal_player(&mut self) {
        self.player.add_card(self.deck.pop().unwrap());
    }

    fn deal_dealer(&mut self) {
        self.dealer.add_card(self.deck.pop().unwrap());
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        self.terminal.clear();
        write!(self.terminal.terminal, "{}", screen::ToMainScreen).unwrap();
        self.terminal.terminal.flush().unwrap();
    }
}

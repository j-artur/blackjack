use std::{
    fmt::Display,
    io::{stdout, Stdout, Write},
};

use rand::seq::SliceRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use termion::{
    clear,
    color::{Blue, Fg, Green, LightBlue, Red, Reset, White, Yellow},
    cursor::{self, HideCursor},
    raw::{IntoRawMode, RawTerminal},
    style::{self, Bold},
};

pub struct Terminal {
    terminal: HideCursor<RawTerminal<Stdout>>,
}

impl Terminal {
    pub fn println<T: Display>(&mut self, s: T) {
        write!(self.terminal, "{}\r\n", s).unwrap();
    }

    pub fn clear(&mut self) {
        write!(self.terminal, "{}", clear::All).unwrap();
        write!(self.terminal, "{}", cursor::Goto(1, 1)).unwrap();
    }
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq)]
pub enum CardNumber {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}
use CardNumber::*;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Spades,
    Hearts,
}

use Suit::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card(pub CardNumber, pub Suit);

impl Display for CardNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ace => write!(f, " A"),
            Two => write!(f, " 2"),
            Three => write!(f, " 3"),
            Four => write!(f, " 4"),
            Five => write!(f, " 5"),
            Six => write!(f, " 6"),
            Seven => write!(f, " 7"),
            Eight => write!(f, " 8"),
            Nine => write!(f, " 9"),
            Ten => write!(f, "10"),
            Jack => write!(f, " J"),
            Queen => write!(f, " Q"),
            King => write!(f, " K"),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Diamonds => write!(f, "♦"),
            Clubs => write!(f, "♣"),
            Hearts => write!(f, "♥"),
            Spades => write!(f, "♠"),
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{} {}{}",
            match self.1 {
                Diamonds => format!("{}", Fg(Red)),
                Clubs => format!("{}", Fg(White)),
                Hearts => format!("{}", Fg(Red)),
                Spades => format!("{}", Fg(White)),
            },
            self.0,
            self.1,
            Fg(Reset),
        )
    }
}

pub struct Hand {
    pub cards: Vec<Card>,
    pub points: u16,
}

impl Hand {
    pub fn new() -> Hand {
        Hand {
            cards: Vec::with_capacity(11),
            points: 0,
        }
    }

    pub fn add_card(&mut self, card: Card) {
        use CardNumber::*;
        self.points += match card.0 {
            Ace => {
                if self.points + 11 > 21 {
                    1
                } else {
                    11
                }
            }
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            Ten | Jack | Queen | King => 10,
        };
        self.cards.push(card);
    }

    pub fn render(&self, terminal: &mut Terminal, name: &str) {
        terminal.println(format!("{}{}:{}", Fg(LightBlue), name, Fg(Reset)));
        terminal.println(format!("Cards: {}", self));
        terminal.println(format!("Points: {}{}{}", Fg(Blue), self.points, Fg(Reset)));
        terminal.println("");
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.cards
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(" "),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Input {
    Continue,
    Up,
    Down,
}
use Input::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Choice {
    Hit,
    Stand,
    // DoubleDown,
    Surrender,
}
use Choice::*;

impl Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
use GameResult::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stage {
    First,
    Second,
    Third,
}
use Stage::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Starting(Stage),
    Presenting,
    Selecting(Choice),
    Standing,
    GameOver(GameResult),
}
use State::*;

pub fn pack() -> Vec<Card> {
    Suit::iter()
        .flat_map(|suit| CardNumber::iter().map(move |number| Card(number, suit)))
        .collect()
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
        Game {
            terminal: Terminal {
                terminal: HideCursor::from(stdout().into_raw_mode().unwrap()),
            },
            deck: pack(),
            state: Starting(First),
            player: Hand::new(),
            dealer: Hand::new(),
        }
    }

    pub fn update(&mut self, input: Input) {
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
                        if self.player.points > 21 {
                            GameOver(Lose)
                        } else {
                            Selecting(Hit)
                        }
                    }
                    (Continue, Stand) => {
                        self.deal_dealer();
                        if self.dealer.points > 21 {
                            GameOver(Win)
                        } else if self.dealer.points < 17 {
                            Standing
                        } else if self.dealer.points > self.player.points {
                            GameOver(Lose)
                        } else if self.dealer.points < self.player.points {
                            GameOver(Win)
                        } else {
                            GameOver(Tie)
                        }
                    }
                    (Continue, Surrender) => GameOver(Lose),

                    (Up, Hit) => Selecting(Stand),
                    (Up, Stand) => Selecting(Surrender),
                    (Up, Surrender) => Selecting(Hit),

                    (Down, Hit) => Selecting(Stand),
                    (Down, Stand) => Selecting(Surrender),
                    (Down, Surrender) => Selecting(Hit),
                },
                Presenting => Selecting(Hit),
                Standing if input == Continue => {
                    self.deal_dealer();
                    if self.dealer.points > 21 {
                        GameOver(Win)
                    } else if self.dealer.points < 17 {
                        Standing
                    } else if self.dealer.points > self.player.points {
                        GameOver(Lose)
                    } else if self.dealer.points < self.player.points {
                        GameOver(Win)
                    } else {
                        GameOver(Tie)
                    }
                }
                GameOver(_) if input == Continue => {
                    self.deck.append(&mut self.dealer.cards);
                    self.dealer.points = 0;
                    self.deck.append(&mut self.player.cards);
                    self.player.points = 0;
                    Starting(Stage::First)
                }
                _ => self.state.clone(),
            }
        }
    }

    pub fn render(&mut self) {
        self.terminal.clear();

        self.terminal.println(format!(
            "{}{}BLACKJACK v0.1.0{}{}",
            Fg(White),
            Bold,
            style::Reset,
            Fg(Reset)
        ));
        self.terminal.println("");

        self.dealer.render(&mut self.terminal, "Dealer");
        self.player.render(&mut self.terminal, "You");

        match &self.state {
            Selecting(c) => {
                let select = |it: Choice| format!("{} {}", if it == *c { '>' } else { ' ' }, it);
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
        self.terminal.terminal.flush().unwrap();
    }
}

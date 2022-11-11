use std::fmt::Display;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use termion::color::*;

use super::Terminal;

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

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Spades,
    Hearts,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card(pub CardNumber, pub Suit);

impl Display for CardNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CardNumber::*;
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
        use Suit::*;
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
        use Suit::*;
        let Card(card_number, suit) = self;
        write!(
            f,
            "{}{} {}{}",
            match suit {
                Diamonds => format!("{}", Fg(Red)),
                Clubs => format!("{}", Fg(White)),
                Hearts => format!("{}", Fg(Red)),
                Spades => format!("{}", Fg(White)),
            },
            card_number,
            suit,
            Fg(Reset),
        )
    }
}

pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand {
            cards: Vec::with_capacity(11),
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn points(&self) -> u8 {
        use CardNumber::*;
        self.cards.iter().fold(0, |it, Card(n, _)| match n {
            Ace => {
                if it + 11 > 21 {
                    it + 1
                } else {
                    it + 11
                }
            }
            Two => it + 2,
            Three => it + 3,
            Four => it + 4,
            Five => it + 5,
            Six => it + 6,
            Seven => it + 7,
            Eight => it + 8,
            Nine => it + 9,
            Ten | Jack | Queen | King => it + 10,
        })
    }

    pub fn render(&self, terminal: &mut Terminal, name: &str) {
        terminal.println(format!("{}{}:{}", Fg(LightBlue), name, Fg(Reset)));
        terminal.println(format!("Cards: {}", self));
        terminal.println(format!(
            "Points: {}{}{}",
            Fg(Blue),
            self.points(),
            Fg(Reset)
        ));
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

pub fn pack() -> Vec<Card> {
    Suit::iter()
        .flat_map(|suit| CardNumber::iter().map(move |number| Card(number, suit)))
        .collect()
}

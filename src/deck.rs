use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Community {
    pub cards: [Card; 5],
}

impl Community {
    pub fn new() -> Community {
        Community {
            cards: [Card {
                suit: Suit::None,
                rank: Rank::None,
            }; 5],
        }
    }

    pub fn reset(&mut self) {
        self.cards = [Card {
            suit: Suit::None,
            rank: Rank::None,
        }; 5];
    }

    pub fn flop(&mut self, deck: &mut Deck) -> [Card; 3] {
        let cards = [deck.deal(), deck.deal(), deck.deal()];
        self.cards[0] = cards[0];
        self.cards[1] = cards[1];
        self.cards[2] = cards[2];

        cards
    }

    pub fn turn(&mut self, deck: &mut Deck) -> Card {
        self.cards[3] = deck.deal();

        self.cards[3]
    }

    pub fn river(&mut self, deck: &mut Deck) -> Card {
        self.cards[4] = deck.deal();

        self.cards[4]
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum Suit {
    None,
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum Rank {
    None,
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
    Ace,
}

#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub struct Deck {
    cards: [Card; 52],
    pub dealt: usize,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = [Card {
            suit: Suit::Clubs,
            rank: Rank::Ace,
        }; 52];
        let mut i = 0;
        for suit in [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades].iter() {
            for rank in [
                Rank::Ace,
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Ten,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
            ]
            .iter()
            {
                cards[i] = Card {
                    suit: *suit,
                    rank: *rank,
                };
                i += 1;
            }
        }
        Deck { 
            cards,
            dealt: 0
        }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
        self.dealt = 0;
    }

    pub fn deal(&mut self) -> Card {
        let card = self.cards[self.dealt];
        self.dealt += 1;
        card
    }

    pub fn discard(&mut self) {
        self.dealt += 1;
    }
}

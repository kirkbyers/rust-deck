use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy)]
pub struct Hand {
    cards: [Card; 2],
    pub value: (u8, u8),
}

impl Hand {
    pub fn new() -> Hand {
        Hand {
            cards: [Card {
                suit: Suit::None,
                rank: Rank::None,
            }; 2],
            value: (0, 0),
        }
    }

    pub fn fill(&mut self, deck: &mut Deck) {
        self.cards[0] = deck.deal();
        self.cards[1] = deck.deal();
    }

    pub fn hand_value(&mut self, community: &Community) -> (u8, u8) {
        self.value = self.determine_value(community);
        self.value
    }

    pub fn determine_value(&mut self, community: &Community) -> (u8, u8) {
        let mut cards_ranked = [self.cards[0], self.cards[1], community.cards[0], community.cards[1], community.cards[2], community.cards[3], community.cards[4]];
        cards_ranked.sort_by(|a, b| b.rank.cmp(&a.rank));

        let mut cards_suited = [self.cards[0], self.cards[1], community.cards[0], community.cards[1], community.cards[2], community.cards[3], community.cards[4]];
        cards_suited.sort_by(|a, b| a.suit.cmp(&b.suit));

        // Royal Flush
        if cards_ranked[0].rank == Rank::Ace && cards_ranked[1].rank == Rank::King && cards_ranked[2].rank == Rank::Queen && cards_ranked[3].rank == Rank::Jack && cards_ranked[4].rank == Rank::Ten {
            return (10, Rank::Ace as u8);
        }

        // Straight Flush
        for i in 0..cards_ranked.len()-5 {
            if cards_ranked[i].rank as u8 + 1 == cards_ranked[i + 1].rank as u8 && cards_ranked[i + 1].rank as u8 + 1 == cards_ranked[i + 2].rank as u8 && cards_ranked[i + 2].rank as u8 + 1 == cards_ranked[i + 3].rank as u8 && cards_ranked[i + 3].rank as u8 + 1 == cards_ranked[i + 4].rank as u8 {
                if cards_ranked[i].suit == cards_ranked[i + 1].suit && cards_ranked[i + 1].suit == cards_ranked[i + 2].suit && cards_ranked[i + 2].suit == cards_ranked[i + 3].suit && cards_ranked[i + 3].suit == cards_ranked[i + 4].suit {
                    return (9, cards_ranked[i].rank as u8);
                }
            }
        }

        // Four of a Kind
        for i in 0..cards_ranked.len()-4 {
            if cards_ranked[i].rank == cards_ranked[i + 1].rank && cards_ranked[i + 1].rank == cards_ranked[i + 2].rank && cards_ranked[i + 2].rank == cards_ranked[i + 3].rank {
                return (8, cards_ranked[i].rank as u8);
            }
        }

        // Full House
        let mut pair: u8 = 0;
        let mut three_of_a_kind: u8 = 0;
        for i in 0..cards_ranked.len()-2 {
            if i + 3 < cards_ranked.len() && cards_ranked[i].rank == cards_ranked[i + 1].rank && cards_ranked[i + 1].rank == cards_ranked[i + 2].rank  {
                three_of_a_kind = cards_ranked[i].rank as u8;
            } else if cards_ranked[i].rank == cards_ranked[i + 1].rank {
                pair = cards_ranked[i].rank as u8;
            }

            if pair != 0 && three_of_a_kind != 0 {
                return (7, if pair > three_of_a_kind { pair } else { three_of_a_kind });
            }
        }

        // Flush
        for i in 0..cards_suited.len()-5 {
            if cards_suited[i].suit == cards_suited[i + 1].suit && cards_suited[i + 1].suit == cards_suited[i + 2].suit && cards_suited[i + 2].suit == cards_suited[i + 3].suit && cards_suited[i + 3].suit == cards_suited[i + 4].suit {
                return (6, cards_ranked[i].rank as u8);
            }
        }

        // Straight
        for i in 0..cards_ranked.len()-5 {
            if cards_ranked[i].rank as u8 + 1 == cards_ranked[i + 1].rank as u8 && cards_ranked[i + 1].rank as u8 + 1 == cards_ranked[i + 2].rank as u8 && cards_ranked[i + 2].rank as u8 + 1 == cards_ranked[i + 3].rank as u8 && cards_ranked[i + 3].rank as u8 + 1 == cards_ranked[i + 4].rank as u8 {
                return (5, cards_ranked[i].rank as u8);
            }
        }

        // Three of a Kind
        for i in 0..cards_ranked.len()-2 {
            if cards_ranked[i].rank == cards_ranked[i + 1].rank && cards_ranked[i + 1].rank == cards_ranked[i + 2].rank {
                return (4, cards_ranked[i].rank as u8);
            }
        }

        // One Pair
        let mut pair_rank: u8 = 0;
        let mut pair_count: u8 = 0;
        for i in 0..cards_ranked.len()-2 {
            if cards_ranked[i].rank == cards_ranked[i + 1].rank {
                pair_count += 1;
                if cards_ranked[i].rank as u8 > pair_rank {
                    pair_rank = cards_ranked[i].rank as u8;
                }
                if pair_count == 2 {
                    return (3, pair_rank);
                }
            }
        }
        if pair_count == 1 {
            return (2, pair_rank);
        }

        // High Card
        (0, cards_ranked[0].rank as u8)
    }

}

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

#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
    None,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum Rank {
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
    None,
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
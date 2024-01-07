use std::collections::HashMap;
use crate::deck::{Card, Suit, Rank, Deck, Community};

#[derive(Debug, Clone, Copy)]
pub struct Hand {
    pub cards: [Card; 2],
    pub value: (ScoringHands, u8),
}

impl Hand {
    pub fn new() -> Hand {
        Hand {
            cards: [Card {
                suit: Suit::None,
                rank: Rank::None,
            }; 2],
            value: (ScoringHands::None, 0),
        }
    }

    pub fn reset(&mut self) {
        self.cards = [Card {
            suit: Suit::None,
            rank: Rank::None,
        }; 2];
        self.value = (ScoringHands::None, 0);
    }

    pub fn fill(&mut self, deck: &mut Deck) {
        self.cards[0] = deck.deal();
        self.cards[1] = deck.deal();
    }

    pub fn hand_value(&mut self, community: &Community) -> (ScoringHands, u8) {
        self.value = self.determine_value(community);
        self.value
    }

    pub fn determine_value(&mut self, community: &Community) -> (ScoringHands, u8) {
        let mut cards_by_rank: HashMap<Rank, Vec<Card>> = HashMap::new();
        let mut cards_by_suit: HashMap<Suit, Vec<Card>> = HashMap::new();

        let all_cards = [self.cards[0], self.cards[1], community.cards[0], community.cards[1], community.cards[2], community.cards[3], community.cards[4]].to_vec();

        for card in all_cards.iter() {
            let rank = card.rank;
            let suit = card.suit;

            if cards_by_rank.contains_key(&rank) {
                cards_by_rank.get_mut(&rank).unwrap().push(*card);
            } else {
                cards_by_rank.insert(rank, vec![*card]);
            }

            if cards_by_suit.contains_key(&suit) {
                cards_by_suit.get_mut(&suit).unwrap().push(*card);
            } else {
                cards_by_suit.insert(suit, vec![*card]);
            }
        }

        // Check for Royal Flush, Straight Flush, and Straight
        for (_, cards) in cards_by_suit.iter() {
            if cards.len() >= 5 {
                let mut cards_ranked = cards.clone();
                cards_ranked.sort_by(|a, b| b.rank.cmp(&a.rank));
                cards_ranked.dedup_by(|a, b| a.rank == b.rank);
                if cards_ranked.len() < 5 {
                    continue;
                }

                // Check if cards_ranked contains a card with rank ten, jack, queen, king, and ace in it
                let royal_flush_ranks = [Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace];
                let card_ranks = cards_ranked.iter().map(|card| card.rank).collect::<Vec<Rank>>();
                if royal_flush_ranks.iter().all(|rank| card_ranks.contains(&rank)) {
                    return (ScoringHands::RoyalFlush, cards_ranked[0].rank as u8);
                }

                // Straight Flush
                for i in 0..cards_ranked.len()-5 {
                    if cards_ranked[i].rank as u8 + 1 == cards_ranked[i + 1].rank as u8 && cards_ranked[i + 1].rank as u8 + 1 == cards_ranked[i + 2].rank as u8 && cards_ranked[i + 2].rank as u8 + 1 == cards_ranked[i + 3].rank as u8 && cards_ranked[i + 3].rank as u8 + 1 == cards_ranked[i + 4].rank as u8 {
                        return (ScoringHands::StraightFlush, cards_ranked[i].rank as u8);
                    }
                }
            }
        }

        // Four of a Kind
        for (rank, cards) in cards_by_rank.iter() {
            if cards.len() == 4 {
                return (ScoringHands::FourOfAKind, *rank as u8);
            }
        }

        // Full House
        let mut pair: u8 = 0;
        let mut three_of_a_kind: u8 = 0;
        for (rank, cards) in cards_by_rank.iter() {
            if cards.len() == 3 && *rank as u8 > three_of_a_kind{
                three_of_a_kind = *rank as u8;
            }
            if cards.len() == 2 && *rank as u8 > pair {
                pair = *rank as u8;
            }

            if three_of_a_kind > 0 && pair > 0 {
                return (ScoringHands::FullHouse, three_of_a_kind);
            }
        }

        // Flush
        for (_, cards) in cards_by_suit.iter() {
            if cards.len() >= 5 {
                let mut cards_ranked = cards.clone();
                cards_ranked.sort_by(|a, b| b.rank.cmp(&a.rank));
                return (ScoringHands::Flush, cards_ranked[0].rank as u8);
            }
        }

        // Straight
        let mut cards_ranked = all_cards.clone();
        cards_ranked.sort_by(|a, b| b.rank.cmp(&a.rank));
        cards_ranked.dedup_by(|a, b| a.rank == b.rank);
        if cards_ranked.len() >= 5 {
            for i in 0..cards_ranked.len()-5 {
                if cards_ranked[i].rank as u8 + 1 == cards_ranked[i + 1].rank as u8 && cards_ranked[i + 1].rank as u8 + 1 == cards_ranked[i + 2].rank as u8 && cards_ranked[i + 2].rank as u8 + 1 == cards_ranked[i + 3].rank as u8 && cards_ranked[i + 3].rank as u8 + 1 == cards_ranked[i + 4].rank as u8 {
                    return (ScoringHands::Straight, cards_ranked[i].rank as u8);
                }
            }
        }

        // Three of a Kind
        let mut three_of_a_kind: u8 = 0;
        for (rank, cards) in cards_by_rank.iter() {
            if cards.len() == 3 && *rank as u8 > three_of_a_kind{
                three_of_a_kind = *rank as u8;
            }
        }
        if three_of_a_kind > 0 {
            return (ScoringHands::ThreeOfAKind, three_of_a_kind);
        }

        // One Pair
        let mut pair_rank: u8 = 0;
        let mut pair_count: u8 = 0;
        for (rank, cards) in cards_by_rank.iter() {
            if cards.len() == 2 {
                pair_count += 1;
                if *rank as u8 > pair_rank {
                    pair_rank = *rank as u8;
                }
            }
        }
        if pair_count == 1 {
            return (ScoringHands::OnePair, pair_rank);
        } else if pair_count >= 2 {
            // Two Pair
            return (ScoringHands::TwoPair, pair_rank);
        }

        // High Card
        (ScoringHands::HighCard, cards_ranked[0].rank as u8)
    }

}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ScoringHands {
    None,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

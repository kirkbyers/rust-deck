use rust_deck::deck::Hand;
use rust_deck::deck::Community;
use rust_deck::deck::Card;
use rust_deck::deck::Suit;
use rust_deck::deck::Rank;
use rust_deck::deck::ScoringHands;

#[test]
fn test_hand_value() {
    let mut hand_1 = Hand {
        cards: [Card {
            suit: Suit::Clubs,
            rank: Rank::Ace,
        }, Card {
            suit: Suit::Clubs,
            rank: Rank::King,
        }],
        value: (ScoringHands::None, 0),
    };
    let mut hand_2 = Hand {
        cards: [Card {
            suit: Suit::Diamonds,
            rank: Rank::Two,
        }, Card {
            suit: Suit::Spades,
            rank: Rank::Seven,
        }],
        value: (ScoringHands::None, 0),
    };

    let mut community = Community{
        cards: [Card {
            suit: Suit::Spades,
            rank: Rank::Four,
        }, Card {
            suit: Suit::Spades,
            rank: Rank::Five,
        }, Card {
            suit: Suit::Clubs,
            rank: Rank::Queen,
        }, Card {
            suit: Suit::Clubs,
            rank: Rank::Jack,
        }, Card {
            suit: Suit::Clubs,
            rank: Rank::Ten,
        }],
    };

    let hand_value = hand_1.hand_value(&community);
    assert_eq!(hand_value, (ScoringHands::RoyalFlush, Rank::Ace as u8));

    let hand_value2 = hand_2.hand_value(&community);
    assert_eq!(hand_value2, (ScoringHands::HighCard, Rank::Queen as u8));
}

use rust_deck::player::Player;
use rust_deck::deck;

#[test]
fn test_player_prompt() {
    let mut player = Player::new(0, String::from("Player 0"), None);
    let mut deck = deck::Deck::new();
    deck.shuffle();

    player.hand.fill(&mut deck);
    
    let mut output = Vec::new();
    let input = b"fold";

    let action = player.prompt_action(&input[..], &mut output, None);
    assert_eq!(action, rust_deck::player::PlayerAction::Fold);
}
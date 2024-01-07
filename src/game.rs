use crate::deck::{Deck, Community, Rank};
use crate::hand::ScoringHands;
use crate::player::{Player, PlayerAction};
use rand::seq::SliceRandom;
use rand::thread_rng;

struct Game {
    players: Vec<Player>,
    deck: Deck,
    community: Community,
    turn: usize,
    state: GameState,
    pot: f32,
    blind: f32,
}

enum GameState {
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
    Closed,
}

impl Game {
    pub fn new(player_count: u8) -> Game {
        // Initialize players
        let mut players: Vec<Player> = Vec::new();
        for i in 0..player_count {
            players.push(Player::new(i, format!("Player {}", i), None));
        }
        players.shuffle(&mut thread_rng());

        // Initialize deck
        let mut deck = Deck::new();
        deck.shuffle();

        // Initialize community
        let community = Community::new();

        Game { 
            players,
            deck,
            turn: 0,
            state: GameState::PreFlop,
            pot: 0.0,
            blind: 0.25,
            community,
        }
    }

    pub fn should_advance_state(&self) -> bool {
        let mut active_players = 0;
        for player in self.players.iter() {
            if player.is_active() {
                active_players += 1;
            }
        }

        if active_players <= 1 {
            return true;
        }

        return false;
    }

    fn get_active_players(&self) -> Vec<&Player> {
        let result = self.players.iter().filter(|player| player.is_active()).collect();
        result
    }

    pub fn payout(&mut self, player_idx: usize) {
        self.players[player_idx].bank += self.pot;
        self.pot = 0.0;
    }

    pub fn advance_state(&mut self) {
        match self.state {
            GameState::PreFlop => {
                self.state = GameState::Flop;
                self.deal_community(3);

                // TODO: Collect Small and Big blinds

                // Set the turn to the player after the Big Blind (UTG), or the first player
                // if there is only 2 players active
                self.turn = 2;

                 // find active players
                let active_players = self.get_active_players();
                let active_player_count = active_players.len();

                if active_player_count == 1 {
                    // TODO: Game over - Declare table winner
                }

                if active_player_count == 2 {
                    self.turn = 0;
                }
            },
            GameState::Flop => {
                self.state = GameState::Turn;
                self.deal_community(1);
                self.turn = 2;

                // find active players
                let active_players = self.get_active_players();
                let active_player_count = active_players.len();
                if active_player_count == 1 {
                    // TODO: Game over - Declare table winner
                }

                if active_player_count == 2 {
                    self.turn = 0;
                }
            },
            GameState::Turn => {
                self.state = GameState::River;
                self.deal_community(1);
                self.turn = 2;

                 // find active players
                let active_players = self.get_active_players();
                let active_player_count = active_players.len();
                if active_player_count == 1 {
                    // TODO: Game over - Declare table winner
                }

                if active_player_count == 2 {
                    self.turn = 0;
                }
            },
            GameState::River => {
                self.state = GameState::Showdown;
                // Determine winner
                let active_players = self.get_active_players();
                let mut active_player_hand_values: Vec<(ScoringHands, u8, usize)> = Vec::new();
                for (idx, player) in active_players.iter().enumerate() {
                    let mut hand = player.hand;
                    let value = hand.hand_value(&self.community);
                    active_player_hand_values.push((value.0, value.1, idx));
                }
                // TODO: fix the sorting
                active_player_hand_values.sort_by(|a, b| b.partial_cmp(a).unwrap());
                let winner_idx = active_player_hand_values[0].2;
                self.payout(winner_idx);
            },
            GameState::Showdown => {
                self.state = GameState::PreFlop;
                self.deck.shuffle();
                self.community.reset();
                for player in self.players.iter_mut() {
                    player.hand.reset();
                    player.last_action = PlayerAction::None;
                    player.hand.fill(&mut self.deck);
                }
            },
            GameState::Closed => {
                println!("Thanks for playing!");
                std::process::exit(0);
            },
        }
    }

    pub fn deal_community(&mut self, count: u8) {
        // Find the first card that hasn't been dealt
        let mut index = 0;
        for (i, card) in self.community.cards.iter().enumerate() {
            if card.rank == Rank::None {
                index = i;
                break;
            }
        }
        // Deal the cards
        for _ in 0..count {
            let card = self.deck.deal();
            self.community.cards[index] = card;
            index += 1;
        }
    }

    pub fn loop_turn(&mut self) {
        match self.state {
            GameState::PreFlop => {},
            GameState::Flop => {},
            GameState::Turn => {},
            GameState::River => {},
            GameState::Showdown => {},
            GameState::Closed => {},
        }
        self.advance_state();
    }
}

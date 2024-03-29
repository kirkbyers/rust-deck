use crate::deck::{Deck, Community, Rank};
use crate::hand::ScoringHands;
use crate::player::{Player, PlayerAction, self};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Game {
    players: Vec<Player>,
    deck: Deck,
    community: Community,
    turn: usize,
    state: GameState,
    pot: f32,
    current_bid: f32,
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
            current_bid: 0.0,
            community,
        }
    }

    pub fn start(&mut self) {
        self.state = GameState::Showdown;
        self.advance_state();
        self.loop_turns();
    }

    fn get_active_players(&self) -> Vec<&Player> {
        let result = self.players.iter().filter(|player| player.is_active()).collect();
        result
    }

    fn payout_player_idx(&mut self, player_idx: usize) {
        self.players[player_idx].bank += self.pot;
        self.pot = 0.0;
    }

    fn advance_state(&mut self) {
        match self.state {
            GameState::PreFlop => {
                println!("\n\n\nStarting Flop");
                self.state = GameState::Flop;
                self.deal_community(3);
                self.community.print(std::io::stdout());

                 // find active players
                let active_players = self.get_active_players();
                let active_player_count = active_players.len();

                for player in self.players.iter_mut() {
                    if player.is_active() {
                        player.last_action = PlayerAction::None;
                    }
                }

                if active_player_count == 1 {
                    // Game over - Declare table winner
                    self.state = GameState::River;
                    return self.advance_state();
                }

                self.turn = 0;
            },
            GameState::Flop => {
                println!("\n\n\nStarting Turn");
                self.state = GameState::Turn;
                self.deal_community(1);
                self.turn = 0;

                // find active players
                let active_players = self.get_active_players();
                let active_player_count = active_players.len();
                if active_player_count == 1 {
                    // Game over - Declare table winner
                    self.state = GameState::River;
                    return self.advance_state();
                }

                for player in self.players.iter_mut() {
                    if player.is_active() {
                        player.last_action = PlayerAction::None;
                    }
                }
            },
            GameState::Turn => {
                println!("\n\n\nStarting River");
                self.state = GameState::River;
                self.deal_community(1);

                 // find active players
                let active_players = self.get_active_players();
                let active_player_count = active_players.len();
                if active_player_count == 1 {
                    // Game over - Declare table winner
                    self.state = GameState::River;
                    return self.advance_state();
                }

                for player in self.players.iter_mut() {
                    if player.is_active() {
                        player.last_action = PlayerAction::None;
                    }
                }

                self.turn = 0;
            },
            GameState::River => {
                println!("\n\n\nStarting Showdown");
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
                println!("Winner: {}", self.players[winner_idx].name);
                println!("Hand: {:?}", self.players[winner_idx].hand);
                self.payout_player_idx(winner_idx);
            },
            GameState::Showdown => {
                println!("\n\n\nStarting PreFlop");
                self.state = GameState::PreFlop;
                let active_players = self.get_active_players();
                let active_player_count = active_players.len();
                if active_player_count == 1 {
                    // Game over - Declare table winner
                    self.state = GameState::Closed;
                    return self.advance_state();
                }
                self.deck.shuffle();
                self.community.reset();
                self.community.print(std::io::stdout());
                self.current_bid = 0.0;
                self.pot = 0.0;
                // Shift the player order by 1
                let first_player = self.players.remove(0);
                self.players.push(first_player);
                for player in self.players.iter_mut() {
                    if player.is_active() {
                        player.reset();
                        player.hand.fill(&mut self.deck);
                    }
                    if self.pot == 0.0 {
                        // Collect small blind
                        self.pot += player.blind(self.blind);
                    } else if self.pot < self.blind * 3.0 {
                        // Collect big blind
                        self.pot += player.blind(self.blind * 2.0);
                    }
                }
                self.turn = 2;
                if active_player_count == 2 {
                    self.turn = 0;
                }
            },
            GameState::Closed => {
                let active_players = self.get_active_players();
                println!("\n\n\nTable Winner: {}", active_players[0].name);
                println!("Thanks for playing!");
                std::process::exit(0);
            },
        }
        self.current_bid = 0.0;
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

        // TODO: make argument for printing injected into this function
        self.community.print(std::io::stdout());
    }

    pub fn loop_turns(&mut self) {
        let player_last_actions = self.get_active_players().iter().map(|player| player.last_action).collect::<Vec<PlayerAction>>();
        // If all players have checked, folded, called, or gone all in, then advance the state
        if player_last_actions.iter().all(|&action| action == PlayerAction::Check || action == PlayerAction::Fold || action == PlayerAction::Call || action == PlayerAction::AllIn) {
            self.advance_state();
        }

        let player = &mut self.players[self.turn];
        if player.is_active() {
            let (action, pot_contribution) = player.prompt_action(std::io::stdin().lock(), std::io::stdout(), Some(self.current_bid));
            self.pot += pot_contribution;
            if action == PlayerAction::Raise {
                let raised_bid = player.bid - self.current_bid;
                self.current_bid += raised_bid;
            } else if action == PlayerAction::AllIn {
                let raised_bid = player.bid - self.current_bid;
                if raised_bid > self.current_bid {
                    self.current_bid += raised_bid;
                }
            }
        }
        self.turn += 1;
        if self.turn >= self.players.len() {
            self.turn = 0;
        }
        return self.loop_turns();
    }
}

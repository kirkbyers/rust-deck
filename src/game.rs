use crate::deck::Hand;
use crate::deck::Deck;
use rand::seq::SliceRandom;
use rand::thread_rng;

struct Player {
    id: u8,
    name: String,
    bank: f32,
    hand: Hand,
    last_action: PlayerAction,
}

impl Player {
    pub fn new(id: u8, name: String, bank: Option<f32>) -> Player {
        Player {
            id,
            name,
            bank: bank.unwrap_or(100.0),
            hand: Hand::new(),
            last_action: PlayerAction::Check,
        }
    }

    pub fn raise(&mut self, amount: f32) -> f32 {
        if amount > self.bank {
            // If return 0, then player cannot raise
            return 0.0;
        }
        self.bank -= amount;

        self.last_action = PlayerAction::Raise;

        amount
    }

    pub fn call(&mut self, amount: f32) -> f32{
        if amount > self.bank {
            // If return 0, then player cannot call
            return 0.0
        }
        self.bank -= amount;

        self.last_action = PlayerAction::Call;

        amount
    }

    pub fn check(&mut self) {
        self.last_action = PlayerAction::Check;
    }

    pub fn fold(&mut self) {
        self.last_action = PlayerAction::Fold;
    }

    pub fn all_in(&mut self) -> f32 {
        let amount = self.bank;
        self.bank = 0.0;
        self.last_action = PlayerAction::Raise;

        amount
    }

    pub fn is_active(&self) -> bool {
        self.bank > 0.0 && self.last_action != PlayerAction::Fold
    }

    pub fn take_action(&mut self, action: PlayerAction, amount: Option<f32>) {
        match action {
            PlayerAction::Fold => {
                self.fold();
            },
            PlayerAction::Check => {
                self.check();
            },
            PlayerAction::Call => {
                self.call(amount.unwrap());
            },
            PlayerAction::Raise => {
                self.raise(amount.unwrap());
            },
        }
    }
}

#[derive(PartialEq)]
enum PlayerAction {
    Fold,
    Check,
    Call,
    Raise,
}

struct Game {
    players: Vec<Player>,
    deck: Deck,
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

        Game { 
            players,
            deck,
            turn: 0,
            state: GameState::PreFlop,
            pot: 0.0,
            blind: 0.25,
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

    pub fn advance_state(&mut self) {
        match self.state {
            GameState::PreFlop => {
                self.state = GameState::Flop;
                // self.deal_community(3);
            },
            GameState::Flop => {
                self.state = GameState::Turn;
                // self.deal_community(1);
            },
            GameState::Turn => {
                self.state = GameState::River;
                // self.deal_community(1);
            },
            GameState::River => {
                self.state = GameState::Showdown;
            },
            GameState::Showdown => {
                // TODO: Determine winner
                // TODO: Award pot to winner
                // TODO: Reset game if more than one player has money to play
                self.state = GameState::PreFlop;
            },
        }
    }
}

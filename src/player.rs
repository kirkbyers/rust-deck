use crate::hand::Hand;

pub struct Player {
    id: u8,
    name: String,
    pub bank: f32,
    pub hand: Hand,
    pub last_action: PlayerAction,
    pub bid: f32,
}

impl Player {
    pub fn new(id: u8, name: String, bank: Option<f32>) -> Player {
        Player {
            id,
            name,
            bank: bank.unwrap_or(100.0),
            hand: Hand::new(),
            last_action: PlayerAction::None,
            bid: 0.0,
        }
    }

    pub fn raise(&mut self, amount: f32) -> f32 {
        if amount > self.bank {
            // If return 0, then player cannot raise
            return 0.0;
        }
        self.bank -= amount;
        self.bid += amount;

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
        self.bid += amount;
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
            PlayerAction::AllIn => {
                self.all_in();
            },
            PlayerAction::None => {
                // Do nothing
            },
        }
    }
}

#[derive(PartialEq)]
pub enum PlayerAction {
    Fold,
    Check,
    Call,
    Raise,
    AllIn,
    None,
}

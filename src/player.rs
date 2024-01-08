use crate::hand::Hand;
use std::io::{stdout, stdin, BufRead, Write};

#[derive(Debug)]
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

    // Reader + Writer injection from
    // https://stackoverflow.com/questions/28370126/how-can-i-test-stdin-and-stdout
    pub fn prompt_action<R, W>(&mut self, mut reader: R, mut write: W, amount: Option<f32>) -> PlayerAction 
    where
        R: BufRead,
        W: Write,
    {
        let _ = stdout().flush();
        let mut action = String::new();
        writeln!(&mut write, "Player {}'s turn", self.id).expect("Unable to write");
        writeln!(&mut write, "Player {}'s bank: {}", self.id, self.bank).expect("Unable to write");
        writeln!(&mut write, "Current bid: {}", amount.unwrap_or(0.0)).expect("Unable to write");
        writeln!(&mut write, "Player {}'s hand: {:?}", self.id, self.hand).expect("Unable to write");
        writeln!(&mut write, "Enter action: [fold|check|call|all-in|raise] [amount]").expect("Unable to write");
        match reader.read_line(&mut action) {
            Ok(_) => {
                let action = action.trim().to_lowercase();
                let action: Vec<&str> = action.split(" ").collect();
                match action[0] {
                    "fold" => {
                        self.fold();
                        PlayerAction::Fold
                    },
                    "check" => {
                        self.check();
                        PlayerAction::Check
                    },
                    "call" => {
                        self.call(amount.unwrap());
                        PlayerAction::Call
                    },
                    "all-in" => {
                        self.all_in();
                        PlayerAction::AllIn
                    },
                    "raise" => {
                        self.raise(amount.unwrap());
                        PlayerAction::Raise
                    },
                    _ => {
                        writeln!(&mut write, "Invalid action").expect("Unable to write");
                        self.prompt_action(reader, write, amount)
                    },
                }
            },
            Err(_) => {
                println!("Invalid action");
                self.prompt_action(reader, write, amount)
            },
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum PlayerAction {
    Fold,
    Check,
    Call,
    Raise,
    AllIn,
    None,
}

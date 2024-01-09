use crate::hand::Hand;
use std::io::{stdout, stdin, BufRead, Write};

#[derive(Debug)]
pub struct Player {
    id: u8,
    pub name: String,
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

    pub fn reset(&mut self) {
        self.hand.reset();
        self.last_action = PlayerAction::None;
        self.bid = 0.0;
    }

    pub fn raise(&mut self, total_bid: f32) -> f32 {
        let bid_diff = total_bid - self.bid;
        if bid_diff > self.bank {
            // If return 0, then player cannot raise
            return 0.0;
        }
        self.bank -= bid_diff;
        self.bid += bid_diff;

        self.last_action = PlayerAction::Raise;

        total_bid
    }

    pub fn call(&mut self, current_bid: f32) -> f32{
        let bid_diff = current_bid - self.bid;
        if bid_diff > self.bank {
            // If return 0, then player cannot call
            return 0.0
        }
        self.bank -= bid_diff;
        self.bid = current_bid;

        self.last_action = PlayerAction::Call;

        bid_diff
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
        self.last_action = PlayerAction::AllIn;

        amount
    }

    pub fn is_active(&self) -> bool {
        self.bank > 0.0 && [PlayerAction::Fold, PlayerAction::AllIn].contains(&self.last_action) == false
    }

    // Reader + Writer injection from
    // https://stackoverflow.com/questions/28370126/how-can-i-test-stdin-and-stdout
    pub fn prompt_action<R, W>(&mut self, mut reader: R, mut write: W, current_bid: Option<f32>) -> (PlayerAction, f32) 
    where
        R: BufRead,
        W: Write,
    {
        if [PlayerAction::Fold, PlayerAction::AllIn].contains(&self.last_action) {
            return (self.last_action, 0.0);
        }
        let _ = stdout().flush();
        let mut action = String::new();
        writeln!(&mut write, "Player {}'s turn", self.id).expect("Unable to write");
        writeln!(&mut write, "Player {}'s bank: {}", self.id, self.bank).expect("Unable to write");
        writeln!(&mut write, "Current bid: {}", current_bid.unwrap_or(0.0)).expect("Unable to write");
        writeln!(&mut write, "Player {}'s hand: {:?}", self.id, self.hand).expect("Unable to write");
        writeln!(&mut write, "Enter action: [fold|check|call|all-in|raise] [amount]").expect("Unable to write");
        match reader.read_line(&mut action) {
            Ok(_) => {
                let action = action.trim().to_lowercase();
                let action: Vec<&str> = action.split(" ").collect();
                match action[0] {
                    "fold" => {
                        self.fold();
                        (PlayerAction::Fold, 0.0)
                    },
                    "check" => {
                        if current_bid.unwrap() > 0.0 {
                            writeln!(&mut write, "Cannot check. Must at least call. Current bid {}", current_bid.unwrap_or(0.0)).expect("Unable to write");
                            return self.prompt_action(reader, write, current_bid);
                        } else {
                            self.check();
                            (PlayerAction::Check, 0.0)
                        }
                    },
                    "call" => {
                        let player_bid_diff = self.call(current_bid.unwrap_or(0.0));
                        if player_bid_diff == 0.0 {
                            writeln!(
                                &mut write, 
                                "Insufficient funds to call. Must all-in or fold.\nCurrent bid is {}.\nCurrent bank is {}", current_bid.unwrap_or(0.0), self.bank
                            ).expect("Unable to write");
                            return self.prompt_action(reader, write, current_bid);
                        }
                        (PlayerAction::Call, player_bid_diff)
                    },
                    "all-in" => {
                        let player_remainder = self.all_in();
                        (PlayerAction::AllIn, player_remainder)
                    },
                    "raise" => {
                        match action.len() {
                            2 => {
                                let raise = action[1].parse::<f32>();
                                match raise {
                                    Ok(raise) => {
                                        let raised_ammount = self.raise(current_bid.unwrap_or(0.0) + raise);
                                        if raised_ammount == 0.0 {
                                            writeln!(
                                                &mut write, 
                                                "Insufficient funds to raise.\nCurrent table bid is {}.\nCurrent bank is {}\nYour current bid is {}", current_bid.unwrap_or(0.0), self.bank, self.bid
                                            ).expect("Unable to write");
                                        }
                                        (PlayerAction::Raise, raised_ammount)
                                    },
                                    Err(_) => {
                                        writeln!(&mut write, "Invalid amount").expect("Unable to write");
                                        self.prompt_action(reader, write, current_bid)
                                    },
                                }
                            },
                            _ => {
                                writeln!(&mut write, "Invalid amount").expect("Unable to write");
                                self.prompt_action(reader, write, current_bid)
                            },
                        }
                    },
                    _ => {
                        writeln!(&mut write, "Invalid action").expect("Unable to write");
                        self.prompt_action(reader, write, current_bid)
                    },
                }
            },
            Err(_) => {
                writeln!(&mut write, "Invalid action").expect("Unable to write");
                self.prompt_action(reader, write, current_bid)
            },
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum PlayerAction {
    Fold,
    Check,
    Call,
    Raise,
    AllIn,
    None,
}

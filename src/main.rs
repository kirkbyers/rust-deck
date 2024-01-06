use rust_deck::deck;

fn main() {
    let mut players: [deck::Hand; 4] = [deck::Hand::new(); 4];
    let mut community = deck::Community::new();

    let mut deck = deck::Deck::new();
 
    deck.shuffle();
    println!("deck: {:?}", deck);

    for player in players.iter_mut() {
        player.fill(&mut deck);
        println!("player: {:?}", player)
    }

    let flop = community.flop(&mut deck);
    println!("flop: {:?}", flop);

    let turn = community.turn(&mut deck);
    println!("turn: {:?}", turn);

    let river = community.river(&mut deck);
    println!("river: {:?}", river);

    println!("community: {:?}", community);
    println!("deck::dealt: {:?}", deck.dealt)
}

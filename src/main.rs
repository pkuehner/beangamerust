mod deck;
use deck::Deck;
fn main() {
    let mut deck = Deck::init();
    deck.shuffle();
    let len = deck.remaining_cards() as i32;
    for _ in 0..len{
        let card = deck.draw().unwrap();
        println!("{:#?}", card); 
        deck.trash_card(card);
    }

    deck.reshuffle();
    for _ in 0..len{
        let card = deck.draw().unwrap();
        println!("{:#?}", card); 
    }
}   

use std::slice::Iter;
use rand::Rng;
use serde::de::value;

#[derive(Debug, Clone)]
pub struct Card {
    card_type: CardType,
    bean: Bean,
}

#[derive(Debug, Clone, Copy)]
pub struct BeanValue {
    num_beans: i64,
    value: i64,
}

#[derive(Debug, Clone)]
pub struct Bean {
    values: Vec<BeanValue>,
    amount_in_deck: i64,
}
#[derive(Debug, Clone, Copy)]
pub enum CardType {
    BEAN,
    ACTION,
}
#[derive(Debug, Clone, Copy)]

pub enum BeanType {
    RED,
    GREEN,
}

impl BeanType {
    fn value(&self) -> Bean {
        match &self {
            BeanType::RED => Bean {
                amount_in_deck: 5,
                values: vec![
                    BeanValue {
                        num_beans: 1,
                        value: 1,
                    },
                    BeanValue {
                        num_beans: 2,
                        value: 3,
                    },
                ],
            },
            BeanType::GREEN => Bean {
                amount_in_deck: 3,
                values: vec![
                    BeanValue {
                        num_beans: 1,
                        value: 1,
                    },
                    BeanValue {
                        num_beans: 3,
                        value: 5,
                    },
                ],
            },
        }
    }

    pub fn iterator() -> Iter<'static, BeanType> {
        static BEAN_TYPE: [BeanType; 2] = [BeanType::RED, BeanType::GREEN];
        BEAN_TYPE.iter()
    }
}

pub struct Deck {
    cards_in_deck: Vec<Card>,
    cards_used: Vec<Card>,
}

impl Deck {
    pub fn init() -> Deck {
        let mut cards: Vec<Card> = vec![];
        for bean_type in BeanType::iterator() {
            
            for _ in 0..bean_type.value().amount_in_deck {
                let card = Card {
                    card_type: CardType::BEAN,
                    bean: bean_type.value(),
                };
                cards.push(card)
            }
        }
        return Deck {
            cards_in_deck: cards.to_vec(),
            cards_used: vec![],
        };
    }

    pub fn draw(&mut self) -> Option<Card> {
        return self.cards_in_deck.pop();
    }

    pub fn trash_card(&mut self, card: Card) -> () {
        return self.cards_used.push(card)
    }

    pub fn reshuffle(&mut self) -> () {
        self.cards_in_deck.extend(self.cards_used.split_off(0));
        self.cards_used.retain(|_| false);
        self.shuffle();
    }

    pub fn shuffle(&mut self) -> () {
        let mut len = self.cards_in_deck.len();
        while len > 0{
            let mut rng = rand::thread_rng();
            let value = rng.gen_range(0..len);
            let card = self.cards_in_deck.remove(value);
            self.cards_in_deck.push(card);
            len -= 1;
        }
    }

    pub fn remaining_cards(&mut self) -> usize {
        return self.cards_in_deck.len();
    }
}

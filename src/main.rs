use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let mut cards = vec![];

        let suits = ["Clubs", "Diamonds", "Hearts", "Spades"];
        let values = [
            "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace",
        ];

        for suit in suits {
            for value in values {
                cards.push(format!("{} of {}", value, suit));
            }
        }

        return Deck { cards };
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        if (num_cards > self.cards.len()) {
            println!("{num_cards} not available on the deck.");
            return vec![];
        }

        return self.cards.split_off(self.cards.len() - num_cards);
    }
}

fn main() {
    let mut deck = Deck::new();

    deck.shuffle();

    println!("{:#?}", deck.deal(3));
}

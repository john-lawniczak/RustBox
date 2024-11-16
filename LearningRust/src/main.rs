use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>, 
}

// inherent implementation
impl Deck {
    fn new() -> Self { //associated function tied to struct
        // List of suits
        let suits = vec!["hearts", "spades", "diamonds", "clubs"];
        
        // List of card values
        let values = [
            "Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King",
        ];

        // Create a vector to store the cards
        let mut cards = vec![];

        // Iterate over all suits and values to generate the cards
        for suit in suits {
            for value in values.iter() {  // use .iter() to iterate over the array
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        // Return the deck with the generated cards
        Deck { cards }
    }
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    // Create a new deck
    let mut deck = Deck::new();

    deck.shuffle();
    let cards = deck.deal(3);

    // Print the deck with a nice format
    println!("My deck: {:?}", deck); // Use {:#?} for pretty-printing the debug output
    println!();
    println!("Here's you hand: {:#?}", cards);
}

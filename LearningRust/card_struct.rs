
#[derive(Debug)]
struct Deck {
    cards: Vec<String>, 
}

fn main () {
    // list suits
    // list values
    // generate all types of cards

    // array is faster because it is not dynamic in size, since fixed it can be an array
    let suits = vec!["hearts", "spades", "diamonds", "clubs"];
    let values = [
        "Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King",
    ];

    let mut cards = vec![];

    // iterate over all
    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }

    let deck = Deck { cards }; // deck is variable, {struct literal}
    
    // # here formatter print nicely
    println! ("My deck: {:#?}", deck); // or  printlin! ("My deck: {deck}") 
}
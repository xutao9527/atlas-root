use rs_poker::core::{Card, CardBitSet, FlatHand, Rankable, Suit, Value};

fn main() {

    let mut card_bit_set = CardBitSet::new();

    for suit in 0..4 {
        for value in 0..13 {
            let card = Card::new(Value::from(value), Suit::from(suit));
            print!("{:?}-{:>2} ", card ,u8::from(card));
            card_bit_set.insert(card);
        }
        println!()
    }
    let h1 = FlatHand::new_from_str("AdKdQdJdTd9d").unwrap();
    let h2 = FlatHand::new_from_str("Ad2d3d4d5d").unwrap();
    let h3 = FlatHand::new_from_str("2d3d4d5d6d").unwrap();


    println!("{:?}{:?}", h1.rank(),h2.rank());
    println!("{:?}{:?}{:?}", h1.rank(),h2.rank(),h3.rank());

}
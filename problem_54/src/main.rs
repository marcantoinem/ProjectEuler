use std::fs::read_to_string;

use poker::{cards, Card, Evaluator};

fn main() {
    let eval = Evaluator::new();
    let mut sum = 0;
    let file = read_to_string("src/p054_poker.txt").unwrap();
    for line in file.lines() {
        let hand_a: Vec<Card> = cards!(line[0..14]).try_collect().unwrap();
        let hand_a = eval.evaluate(hand_a).unwrap();
        let hand_b: Vec<Card> = cards!(line[15..29]).try_collect().unwrap();
        let hand_b = eval.evaluate(hand_b).unwrap();
        if hand_a.is_better_than(hand_b) {
            sum += 1;
        }
    }
    println!("{}", sum);
}
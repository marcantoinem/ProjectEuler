// https://rosettacode.org/wiki/Count_the_coins#Rust
// Really clever ways to find all the combination with maximum performance
fn make_change(coins: &[usize], cents: usize) -> usize {
    let size = cents + 1;
    let mut ways = vec![0; size];
    ways[0] = 1;
    for &coin in coins {
        for amount in coin..size {
            ways[amount] += ways[amount - coin];
        }
    }
    ways[cents]
}

fn main() {
    println!("{}", make_change(&[1,2,5,10,20,50,100,200], 200));
}
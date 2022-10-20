use primal::Sieve;

// Same as 31;
const NB: usize = 100;

fn main() {
    let prime = Sieve::new(100_009);
    let mut ways = [0; NB + 1];
    ways[0] = 1;
    for i in prime.primes_from(2) {
        for j in i..=NB {
            ways[j] += ways[j - i];
        }
    }
    for (index, way) in ways.iter().enumerate() {
        println!("{}: {}", index, way);
    } 
}
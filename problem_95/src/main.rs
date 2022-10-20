use std::time::Instant;

use primal::Sieve;
const LIMIT: usize = 1_000_000;

fn main() {
    let instant = Instant::now();
    let mut aliquot = [0;LIMIT as usize];
    aliquot[0] = 1;
    let sieve = Sieve::new(LIMIT);
    for nb in 1..LIMIT {
        let factor = sieve.factor(nb).unwrap();
        let mut product = 1;
        for (p, a) in factor {
            product *= (p.pow((a+1) as u32) - 1) / (p - 1);
        }
        product -= nb;
        aliquot[nb] = product;
    }
    let mut longest_chain = vec![];
    for nb in 1..LIMIT {
        let mut chain = vec![nb];
        loop {
            let len = chain.len() - 1;
            let last_element = chain[len];
            let next_element = aliquot[last_element];
            if next_element == nb {
                if len > longest_chain.len() {
                    longest_chain = chain;
                }
                break
            } else if next_element < nb {
                break
            } else if next_element > LIMIT {
                break
            } else if chain.contains(&next_element) {
                break
            }
            chain.push(next_element);
        }
    }
    println!("The solution is {} computed in {}s", longest_chain[0], instant.elapsed().as_secs_f64());
}

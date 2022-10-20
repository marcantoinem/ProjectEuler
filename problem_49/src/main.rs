use digits_iterator::DigitsExtension;
use primal::Sieve;

// fn permutation_prime(sieve: &Sieve, n: usize) -> bool {
//     let mut vec: Vec<usize> = n.digits().permutations(4).map(|x| to_usize(x)).collect();
//     vec.sort();
//     let len = vec.len();
//     let mut frequency = HashMap::new();
//     for i in 0..len-1 {
//         for j in i..len {
//             let diff = vec[j] - vec[i];
//             *frequency.entry(diff).or_insert(0_usize) += 1;
//         }
//     }
//     for (diff, freq) in frequency {
//         if freq > 3 {
//             println!("{}, {}", freq, diff);
//         }
//     }
//     true
// }

fn permutation_prime(vec: Vec<usize>) {
    let len = vec.len();
    for i in 0..len-2 {
        for j in i+1..len-1 {
            for k in j+1..len {
                let diff = vec[j] - vec[i];
                if diff == vec[k] - vec[j] {
                    let mut vec_i: Vec<u8> = vec[i].digits().collect();
                    vec_i.sort();
                    let mut vec_j: Vec<u8> = vec[j].digits().collect();
                    vec_j.sort();
                    let mut vec_k: Vec<u8> = vec[k].digits().collect();
                    vec_k.sort();
                    if vec_i == vec_j && vec_i == vec_k {
                        println!("{} {} {} with diff {}", vec[i], vec[j], vec[k], diff);
                    }
                }
            }
        }
    }
}

fn main() {
    let sieve = Sieve::new(1_000_000);
    let vec: Vec<usize> = sieve.primes_from(1_000).take_while(|x| x < &10_000).collect();
    permutation_prime(vec);
}

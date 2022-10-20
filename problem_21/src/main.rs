use primal::Sieve;

// Brute force method
// fn d(n: usize) -> usize {
//     let mut sum = 1;
//     let bound = (n as f64).sqrt().ceil() as usize;
//     for i in 2..bound {
//         if n%i == 0 {
//             if i == n/i {
//                 sum += i;
//             } else {
//                 sum += i + n/i;
//             }
//         }
//     }
//     sum
// }

fn d(sieve: &Sieve, n: usize) -> usize {
    let factors = sieve.factor(n);
    let mut product = 1;
    // p: prime, a: exponent
    for (p, a) in factors.unwrap() {
        if a == 1 {
            product *= p + 1;
        } else {
            product *= (p.pow(a as u32 + 1) - 1) / (p - 1)
        }
    }
    product - n
}

fn main() {
    let mut sum = 0;
    let sieve = Sieve::new(10_000);
    for n in 2..10_000 {
        // println!("{n}");
        let a = d(&sieve, n);
        let b = d(&sieve, a);
        if a != b && n == b{
            // a == d(d(a)) && a != d(a)
            println!("d({n}) = {a}, d({a}) = {b}");
            sum += n;
        }   
    }
    println!("{sum}");
}

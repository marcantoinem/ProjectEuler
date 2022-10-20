fn is_square(n: usize) -> bool {
    let sqrt = (n as f64).sqrt();
    sqrt.ceil() == sqrt
}

fn main() {
    let mut nb_sol = 0;
    let mut a: usize = 2;
    while nb_sol <= 1_000_000 {
        a += 1;
        for bc in 3..=2*a {
            let s: usize = a.pow(2) + bc.pow(2);
            if is_square(s) {
                nb_sol += if bc <= a {bc/2} else {1 + (a - (bc+1)/2)};
            } 
        }
    }
    println!("a {}, number of solutions {}", a, nb_sol);
}

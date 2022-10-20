use math_arsenal::factor::Factors;
use primal::Sieve;

fn compare(vec1: &Factors, vec2: &Factors, vec3: &Factors, vec4: &Factors) -> bool {
    if vec1.0.len() != 4 || vec2.0.len() != 4 || vec3.0.len() != 4 || vec4.0.len() != 4 {
        return false
    }
    for factors in vec1.0.iter() {
        if vec2.0.contains(&factors) || vec3.0.contains(&factors) || vec4.0.contains(&factors) {
            return false
        }
    }
    true
}

fn main() {
    let sieve = Sieve::new(1_000_000);
    let mut vec = vec![];
    let len = 1_000_000;
    for n in 2..len {
        let factors = Factors::from(&sieve, n);
        vec.push(factors);
    }
    for n in 2..(len-5) {
        if compare(&vec[n], &vec[n+1], &vec[n+2], &vec[n+3]) {
            println!("{}", vec[n].expand());
        }
    }
}
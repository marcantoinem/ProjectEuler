use rug::{ops::Pow, Complete, Integer, Rational};

fn is_square(n: usize) -> bool {
    let isqrt = (n as f64).sqrt() as usize;
    n == isqrt.pow(2)
}

// From reikna crate
fn square_root(x: usize, depth: usize) -> Vec<usize> {
    let a0: f64 = (x as f64).sqrt();

    let mut m: usize = 0;
    let mut d: usize = 1;
    let mut a: usize = a0.floor() as usize;
    let mut vec = vec![a];
    let mut expansion = vec![a];

    let end: usize = a * 2;
    while a != end {
        m = d * a - m;
        d = (x - m * m) / d;
        a = ((a0 + m as f64) / d as f64).floor() as usize;
        expansion.push(a);
    }
    
    let len = expansion.len() - 1;
    if depth == 1 {
        return vec;
    } else if depth <= len {
        for num in &expansion[1..depth] {
            vec.push(*num);
        }
        return vec;
    } else {
        // depth 10,11
        // len 5
        for _ in 0..(depth/len) {
            for num in &expansion[1..] {
                vec.push(*num);
            }
        }
    
        let rest = depth - (depth / len) * (len);
        if rest == 0 {
            vec.pop();
            return vec
        } else {
            for num in &expansion[1..rest] {
                vec.push(*num);
            }
            return vec
        }
    }   
}

fn develop(vec: &Vec<usize>) -> Rational {
    let i = 1;
    let n = vec.len();
    Rational::from(vec[0]) + develop_r(vec, i, n)
}

fn develop_r(vec: &Vec<usize>, i: usize, n: usize) -> Rational {
    if i < n {
        return Rational::from(1) / (Rational::from(vec[i]) + develop_r(vec, i + 1, n));
    } else {
        return Rational::from(0_usize);
    }
}

fn minimal_solution(d: usize) -> Integer {
    if is_square(d as usize) {
        return Integer::from(0_usize);
    }
    let mut depth = 1;
    loop {
        let vec = square_root(d, depth);
        let fraction = develop(&vec);
        if fraction.numer().pow(2).complete()
            == Integer::from(1_usize) + d * fraction.denom().pow(2).complete()
        {
            return fraction.numer().clone();
        }
        depth += 1;
    }
}

fn main() {
    let mut max = Integer::from(0_usize);
    let mut best_d = 0;
    let max_d = 1_000;
    for d in 1..=max_d {
        let isqrt = (d as f64).sqrt() as usize;
        if isqrt.pow(2) != d {
            let x = minimal_solution(d);
            if x > max {
                max = x;
                best_d = d;
            }
        }
    }
    println!("{}", best_d);
}

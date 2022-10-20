use rug::{Rational, Float, Complete, ops::CompleteRound};

const PRECISION: u32 = 342;

fn sqrt_newton(x: usize, iteration: usize) -> Float {
    // x_0
    let mut root = Rational::from((x as f64).sqrt() as usize);
    for _ in 0..iteration {
        root = (&root + (x / &root).complete())/2;
    }
    Float::with_val(PRECISION, root)
}

fn is_square(x: usize) -> bool {
    (x as f64).sqrt().floor().powf(2.0) as usize == x
} 

fn sum_digit(f: Float, n: usize) -> u32 {
    let digit = &f.floor_ref().complete(PRECISION);
    let mut f = (&f - digit).complete(PRECISION);
    let mut sum = digit.to_u32_saturating().unwrap();
    for _ in 1..n {
        f *= 10;
        let digit = f.floor_ref().complete(PRECISION);
        sum += digit.to_u32_saturating().unwrap();
        f -= digit;
    }
    sum
}

fn main() {
    let mut sum = 0;
    for x in 1..=100 {
        if !is_square(x) {
            let sqrt = sqrt_newton(x, 12);
            sum += sum_digit(sqrt, 100);        
        }
    }
    println!("{}", sum);
}

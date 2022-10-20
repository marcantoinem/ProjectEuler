use rug::{Integer, ops::Pow, Complete, Float};

fn main() {
    let mut a = Integer::from(1_usize);
    let mut b = Integer::from(1_usize);
    let mut n = 2;
    while a <= Integer::from(10_usize).pow(999) && b <= Integer::from(10_usize).pow(999) {
        a = (&a + &b).complete();
        n += 1;
        b = (&a + &b).complete();
        n += 1;
    }
    println!("{}, {}", Float::with_val(1024, &a).log10().floor(), n);
    println!("{}, {}", Float::with_val(1024, &b).log10().floor(), n);
    // println!("The sum of the even below 4 000 000 is {}", sum);
}

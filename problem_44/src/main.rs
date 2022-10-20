trait Pentagonal {
    fn pentagonal(self) -> Self;
    fn is_pentagonal(self) -> bool;
}

impl Pentagonal for usize {
    fn pentagonal(self) -> usize {
        self * (3 * self - 1) / 2
    }
    fn is_pentagonal(self) -> bool {
        let determinant = 24 * self + 1;
        let isqrt = ((determinant) as f64).sqrt() as Self;
        (isqrt) % 6 == 5 && isqrt.pow(2) == determinant
    }
}

fn main() {
    let max = 5_000;
    for i in 1..(max - 1) {
        for j in (i + 1)..max {
            let a = i.pentagonal();
            let b = j.pentagonal();
            if (b + a).is_pentagonal() && (b - a).is_pentagonal() {
                println!("{}", b - a);
            }
        }
    }
}

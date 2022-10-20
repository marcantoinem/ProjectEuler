trait Polygonal {
    fn hexagonal(self) -> Self;
    // fn is_triangular(self) -> bool;
    fn is_pentagonal(self) -> bool;
    // fn is_hexagonal(self) -> bool;
}

// if a number is hexagonal it is triangular
// but it doesn't mean that a triangular number have to be hexagonal
impl Polygonal for usize {
    fn hexagonal(self) -> usize {
        self * (2 * self - 1)
    }
    // fn is_triangular(self) -> bool {
    //     let determinant = 8 * self + 1;
    //     let isqrt = ((determinant) as f64).sqrt() as Self;
    //     isqrt.pow(2) == determinant
    // }
    fn is_pentagonal(self) -> bool {
        let determinant = 24 * self + 1;
        let isqrt = ((determinant) as f64).sqrt() as Self;
        isqrt % 6 == 5 && isqrt.pow(2) == determinant
    }
    // fn is_hexagonal(self)  -> bool {
    //     let determinant = 8 * self + 1;
    //     let isqrt = ((determinant) as f64).sqrt() as Self;
    //     isqrt % 4 == 3 && isqrt.pow(2) == determinant
    // }
}

fn main() {
    for n in 5..100_000_000 {
        let t = n.hexagonal();
        if t.is_pentagonal() {
            println!("{}", t);
        }
    }
}

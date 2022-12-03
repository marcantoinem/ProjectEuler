// Number of the form 1_2_3_4_5_6_7_8_9
fn is_ok(n: usize) -> bool {
    n % 10 == 9
        && n % 1000 / 100 == 8
        && n % 100000 / 10000 == 7
        && n % 10000000 / 1000000 == 6
        && n % 1000000000 / 100000000 == 5
        && n % 100000000000 / 10000000000 == 4
        && n % 10000000000000 / 1000000000000 == 3
        && n % 1000000000000000 / 100000000000000 == 2
        && n % 100000000000000000 / 10000000000000000 == 1
}

// End with zero so must be a square of 100
fn main() {
    let maximum: usize = (19293949596979899usize as f64).sqrt() as usize;
    let minimum: usize = (10203040506070809usize as f64).sqrt() as usize;
    for n in minimum..maximum {
        let square = n * n;
        if is_ok(square) {
            println!("{}", n * 10);
        }
    }
}

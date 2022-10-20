fn main() {
    for n in 1..100_isize {
        let lower_bound = n / 2 as isize;
        let upper_bound = ((1_f64+2_f64.sqrt())/2_f64 * n as f64).floor() as isize;
        let mut count = 0;
        for x in lower_bound..=upper_bound {
            let determinant = n.pow(2) - 4*x.pow(2)+4*n*x;
            let sqrt = (determinant as f64).sqrt();
            if sqrt == sqrt.ceil() && sqrt as isize % 2 == 0 {
                count += 4;
            }
        }
        print!("{}, ", count);
    }
    println!("End");
}

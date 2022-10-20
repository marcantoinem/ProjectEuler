fn main() {
    let mut sum = 0;
    for x in 1..22 {
        let born_inf = 10_f64.powf((x-1) as f64 / x as f64).ceil() as usize;
        sum += 10 - born_inf;
    }
    println!("{}", sum);
}

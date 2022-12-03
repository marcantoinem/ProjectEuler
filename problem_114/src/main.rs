use malachite::integer::Integer;
use std::time::Instant;

fn main() {
    let instant = Instant::now();
    let end = 50;
    let mut number_of_way = vec![Integer::from(1); end + 1];
    for size in 3..=end {
        for offset in 0..=(size - 3) {
            for first_block in 3..=(size - offset) {
                // println!("{}: {} {}", size, offset, first_block + offset);
                number_of_way[size] += Integer::from(1);
                let rest = size as isize - 1 - offset as isize - first_block as isize;
                if rest >= 3 {
                    let number_of_way_rest =
                        number_of_way[rest as usize].clone() - Integer::from(1);
                    number_of_way[size] += number_of_way_rest;
                }
            }
        }
    }
    println!(
        "{:?}, {:.8}s",
        number_of_way,
        instant.elapsed().as_secs_f64()
    );
    println!("{}", number_of_way[50]);
}

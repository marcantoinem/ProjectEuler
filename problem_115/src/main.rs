use malachite::integer::Integer;

fn find_size(m: usize, n: usize) -> Integer {
    let mut number_of_way = vec![Integer::from(1); n + 1];
    for size in m..=n {
        for offset in 0..=(size - m) {
            for first_block in m..=(size - offset) {
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
    return number_of_way[n].clone();
}

fn main() {
    let mut n = 0;
    let mut result = Integer::from(0);
    let m = 50;
    while result < 1_000_000 {
        n += 1;
        result = find_size(m, n);
    }
    println!("{} {}", n, find_size(m, n));
}

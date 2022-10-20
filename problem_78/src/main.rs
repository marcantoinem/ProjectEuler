// fn p(previous: &mut [usize; 2_000], n: isize) -> usize {
//     if n < 1 {
//         return 0;
//     } else if n == 1 {
//         return 1;
//     } else if previous[n as usize - 2] > 0 {
//         return previous[n as usize - 2];
//     }
//     let mut sum = 0_isize;
//     for k in 1..=n {
//         let first_term = n - k*(3*k - 1)/2;
//         let second_term = n - k*(3*k + 1)/2;
//         sum += (-1_isize).pow(k as u32 + 1) * (p(previous,first_term) + p(previous, second_term)) as isize;
//     }
//     let sum = sum as usize % 1_000_000;
//     previous[n as usize - 2] = sum;
//     sum
// }

fn main() {
    let mut previous = [0;69_000];
    previous[0] = 1;
    let mut n = 1;
    loop {
        let mut i: isize = 0;
        let mut penta: isize = 1;
        while penta <= n {
            let sign = if i % 4 > 1 {-1} else {1};
            previous[n as usize] += sign * previous[(n - penta) as usize];
            previous[n as usize] %= 1_000_000;
            i += 1;
            let j = if i % 2 == 0 {i/2+1} else {-(i / 2 + 1)};
            penta = j * (3*j - 1) / 2;
        }
        if previous[n as usize] == 0 {
            println!("{}", n);
            break
        }
        n += 1;
    }
    // println!("{:?}", previous);
}

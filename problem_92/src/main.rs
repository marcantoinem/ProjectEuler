use std::time::Instant;
const TOTAL: usize = 10_000_000;
const SMALLEST: usize = 568;

fn square_digit_sum(n: usize) -> usize {
        let mut num = n;
        let mut square_digit_sum = 0;
        while num != 0 {
            square_digit_sum += (num % 10).pow(2);
            num /= 10;
        }
        square_digit_sum
}

fn main() {
    let instant = Instant::now();
    let mut destination_89: [Option<bool>; SMALLEST] = [None; SMALLEST];
    let mut sum = 0;
    for nb in 2..SMALLEST {
        let mut chain = square_digit_sum(nb);
        loop {
            if let Some(value) = destination_89[chain] {
                if value == true {
                    destination_89[nb] = Some(true);
                    sum += 1;
                } else {
                    destination_89[nb] = Some(false);
                }
                break
            }
            if chain == 89 {
                destination_89[nb] = Some(true);
                sum += 1;
                break
            } else if chain == 1 {
                destination_89[nb] = Some(false);
                break
            }
            chain = square_digit_sum(chain);
        }
    }
    for nb in SMALLEST..TOTAL {
        let chain = square_digit_sum(nb);
        if destination_89[chain] == Some(true) {
            sum += 1;
        }
    }
    println!("{} computed in {}ms", sum, instant.elapsed().as_millis());
}

use std::time::Instant;

use itertools::Itertools;

fn main() {
    let instant = Instant::now();
    let game_size = 15;
    let n: usize = (2..game_size+2).product();
    let mut d = 1;
    let upper_bound = (game_size + 1)/ 2;
    for i in 1..upper_bound {
        d += (1..=game_size).combinations(i).map(|x| x.iter().product::<usize>()).sum::<usize>();
    }
    let prize = (n - d) / d + 1;
    println!("The maximum prize is {} computed in {}s", prize, instant.elapsed().as_secs_f64());
}

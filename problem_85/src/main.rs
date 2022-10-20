use std::time::Instant;



fn main() {
    let instant = Instant::now();
    let mut x = 2000;
    let mut y = 1;
    let to_approach = 2_000_000;
    // The number of solutions is equal to C(x+1, 2) * C(y+1, 2)
    // Which can be simplified to x(x+1)*y(y+1)/4
    let mut best = 2_000_000;
    while y < x {
        let nb_sol = x*(x+1)*y*(y+1) / 4; 
        let diff = (nb_sol as isize - to_approach).abs();
        if diff < best {
            best = diff;
            println!("{}x{} area: {} number of solutions: {}", x, y, x*y, nb_sol);
        }
        if nb_sol > to_approach {
            x -= 1;
        } else {
            y += 1;
        }
    }
    println!("It took {}µs to compute.", instant.elapsed().as_micros());
}

use std::time::Instant;

const LIMIT: isize = 1_000_000_000;

fn main() {
    let instant = Instant::now();
    let mut nb_sol = 0;
    let mut x = 2;
    let mut y = 1;
    loop {
        let a_time_3 = 2 * x - 1; 
        let area_time_3 = y * (x-2);
        if a_time_3 > LIMIT {break};
        if a_time_3 > 0 && area_time_3 > 0 && a_time_3 % 3 == 0 && area_time_3 % 3 == 0 {
            let a = a_time_3 / 3;
            nb_sol += 3 * a + 1;
        }

        let a_time_3 = 2 * x + 1;
        let area_time_3 = y * (x + 2);
        if a_time_3 > 0 && area_time_3 > 0 && a_time_3 % 3 == 0 && area_time_3 % 3 == 0 {
            let a = a_time_3 / 3;
            nb_sol += 3 * a - 1;
        }

        let next_x = 2*x + 3*y;
        let next_y = x + 2*y;
        x = next_x;
        y = next_y;
    }
    println!("The sum of all almost equilateral triangles is {} computed in {}s.", nb_sol, instant.elapsed().as_secs_f64());
}

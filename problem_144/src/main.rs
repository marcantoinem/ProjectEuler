use std::time::Instant;

// By some miracle it works.
// See my notes for the formula.
fn main() {
    let instant = Instant::now();
    let mut count = 0;
    let mut last_point = (0.0, 10.1_f64);
    let mut a: f64 = -197.0/14.0;
    let mut b: f64 = 10.1;
    loop {
        let sq_det = (400.0 * a * a - 16.0 * b * b + 1600.0).sqrt();
        let potential_x = (-2.0*a*b + sq_det) / (8.0 + 2.0*a*a);
        let x = if potential_x <= last_point.0 + 0.00001 && potential_x >= last_point.0 -0.00001 {
            (-2.0*a*b - sq_det) / (8.0 + 2.0*a*a)
        } else {
            potential_x
        };
        let y = a * x + b;
        if y > 9.9 && (x >= -0.01 && x <= 0.01) {
            break
        }
        let m = -4.0*x/y;
        let c = (2.0*m + m*m * a - a) / (1.0 - m*m + 2.0*a*m);
        let d = y - c*x;
        count += 1;
        if d < 0.0 {
            println!("y={}x{}", c, d);
        } else {
            println!("y={}x+{}", c, d);
        }
        last_point = (x,y);
        a = c;
        b = d;
    }
    println!("{} reflections computed in {}s", count, instant.elapsed().as_secs_f64());
}

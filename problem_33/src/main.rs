fn main() {
    for n in 10..100 {
        for d in (n+1)..100 {
            let n_10 = n / 10 % 10;
            let n_1 = n % 10;
            let d_10 = d / 10 % 10;
            let d_1 = d % 10;
            if n_10 == d_1 {
                let a = n as f64/ d as f64;
                let b = n_1 as f64 / d_10 as f64;
                if a == b {
                    println!("{n} / {d} == {n_1} / {d_10}");
                }
            } else if n_1 == d_10 {
                let a = n as f64/ d as f64;
                let b = n_10 as f64 / d_1 as f64;
                if a == b {
                    println!("{n} / {d} = {n_10} / {d_1}");
                }
            }
        }
    }
}

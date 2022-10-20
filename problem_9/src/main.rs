use std::time::Instant;
// 1. Find coprime number
// 2. Find pythagorician triplet
// 3. ???
// 4. Profit


fn find_evenodd_coprime() -> Vec<(usize,usize)>{
    let mut vec = vec![];
    branch(2, 1, &mut vec);
    vec
}

fn branch(m: usize, n: usize, vec: &mut Vec<(usize,usize)>) {
    vec.push((m,n));
    if m < 100 {
        if 2*n >= m {
            branch(2 * m - n, m, vec);
        }
        branch(2 * m + n, m, vec);
        branch(m + 2 * n, n, vec);
    }
}

// fn find_pythagorean_triples() 

fn main() {
    let time = Instant::now();
    let coprimes = find_evenodd_coprime();
    let interesting_factor = [8, 10, 20, 25, 40, 50, 100, 125, 500, 1000];
    for (m, n) in coprimes {
        let a = m*m - n*n;
        let b = 2*m*n;
        let c = m*m + n*n;
        let sum = a + b + c;
        if interesting_factor.contains(&sum) {
            println!("{}+{}+{}={}", a, b, c, sum);
            println!("{}²+{}²={}² computed in {}s", a, b, c, time.elapsed().as_secs_f64());
        }
    }
}
// Result is a=15, b=8, c=17 to get a sum of 40.
// If we multiply each by 25, we get a sum of 1000.
// And our result is abc=31875000

// Do not go beyond 16
fn find_evenodd_coprime(depth: usize) -> Vec<(usize,usize)>{
    let mut vec = vec![];
    branch(2, 1, depth, 0, &mut vec);
    vec
}

fn branch(m: usize, n: usize, max_depth: usize, depth: usize, vec: &mut Vec<(usize,usize)>) {
    vec.push((m,n));
    if depth < max_depth {
        let depth = depth + 1;
        branch(2 * m - n, m, max_depth, depth, vec);
        branch(2 * m + n, m, max_depth, depth, vec);
        branch(m + 2 * n, n, max_depth, depth, vec);
    }
}

fn num_sol(coprimes: &Vec<(usize,usize)>, n: usize) -> usize {
    let factors = divisors::get_divisors(n);
    let mut sum = 0;
    for (m, n) in coprimes {
        let a = m*m - n*n;
        let b = 2*m*n;
        let c = m*m + n*n;
        let p = a + b + c;
        if factors.contains(&p) {
            sum += 1;
        }
    }
    sum
}

// The 
fn main() {
    let coprimes = find_evenodd_coprime(9);
    let num_sol = num_sol(&coprimes,840);
    println!("{}", num_sol)
}

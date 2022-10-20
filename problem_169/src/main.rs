use std::collections::HashMap;

fn f(n: u128, hashmap: &mut HashMap<u128, u128>) -> u128 {
    if let Some(value) = hashmap.get(&n) {
        *value
    } else if n % 2 == 0 {
        let value = f(n/2, hashmap);
        hashmap.insert(n, value);
        value
    } else {
        let value = f((n-1)/2, hashmap) + f((n+1)/2, hashmap);
        hashmap.insert(n, value);
        value
    }
}


fn main() {
    let mut hashmap = HashMap::new();
    hashmap.insert(0, 0);
    hashmap.insert(1, 1);
    println!("{}", f(10_u128.pow(25) + 1, &mut hashmap));
}

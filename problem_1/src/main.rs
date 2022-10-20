fn main() {
    let num = 1000;
    // Obtained with sum properties
    // Σk with maximum n and starting at k=1 = n(n+1)/2
    // Σcf(x)=cΣf(x) where c is a constant
    
    // Dividing integer in rust give the result floor
    // which is the number of the full division
    // which is also the maximum of the sum
    let n_three = (num - 1) / 3;
    let n_five = (num - 1) / 5;
    let n_fifteen = (num - 1) / 15;
    let sum = 3 * n_three * (n_three + 1) / 2 + 5 * n_five*(n_five + 1) / 2 - 15 * n_fifteen* ( n_fifteen + 1) / 2;
    println!("{}", sum);
}

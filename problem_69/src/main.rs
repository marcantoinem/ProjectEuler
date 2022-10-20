fn main() {
    let prime = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
    for len in 0..10 {
        let num: usize = (0..len).into_iter().map(|x| prime[x]).product();
        println!("{}", num);
   }
}

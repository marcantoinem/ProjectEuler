fn main() {
    let digits = [1,2,3,4,5,6,7,8,9];
    let mut products = vec![];
    for product in 1..3 {
        for equality in (product+1)..6 {
            for a in 0..9 {
                for b in 0..8 {
                    for c in 0..7 {
                        for d in 0..6 {
                            for e in 0..5 {
                                for f in 0..4 {
                                    for g in 0..3 {
                                        for h in 0..2 {
                                            let mut digits = digits.to_vec();
                                            let combination = [a,b,c,d,e,f,g,h,0];
                                            let mut vec = vec![];
                                            for element in combination {
                                                vec.push(digits[element]);
                                                digits.remove(element);
                                            }
                                            let mut first = 0;
                                            for i in 0..product {
                                                first += vec[i] * 10_usize.pow((product - i - 1) as u32);
                                            }
                                            let mut second = 0;
                                            for i in product..equality {
                                                second += vec[i] * 10_usize.pow((equality - i - 1) as u32);
                                            }
                                            let mut third = 0;
                                            for i in equality..9 {
                                                third += vec[i] * 10_usize.pow((9 - i - 1) as u32);
                                            }
                                            if first * second == third {
                                                products.push(third);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    products.sort();
    products.dedup();
    let sum: usize = products.iter().sum();
    println!("{}", sum);
}

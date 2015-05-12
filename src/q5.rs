fn check(input:usize) {
    println!("Checking {}",input);
    for i in 2..21 {
        if input % i != 0 {
            println!("Not by: {}",i);
        }
    }
}
pub fn soln() -> usize {
    let mut num:usize = 1;
    let primes = [5,7,9,11,13,16,17,19];
    for i in primes.iter() {
        num *= *i;
    } 
    //check(num);
    num
}

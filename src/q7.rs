use utils::primes::PrimeIterator;

pub fn soln(input:usize) -> usize {
    let test = PrimeIterator::new();
    test.skip(input-1).next().unwrap_or(0)
}
#[cfg(test)]
#[test]
pub fn q7() {assert!(soln(10001) == 104743);}

use utils::primes::PrimeIterator;

pub fn soln () -> usize {
    let test = PrimeIterator::new();
    test.take_while(|&i| i < 2000000).sum()
}
#[cfg(test)]
#[test]
pub fn q10() {assert!(soln() == 142913828922);}

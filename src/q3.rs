use std::iter::{iterate};

pub fn largest_prime_factor(input:usize) -> usize {
    let mut largestfactor = 1;
    let mut  p = 3;
    let mut div = input;

    //remove any factors of two
    while div % 2 == 0 {
        largestfactor = 2;
        div = div/2
    }

    //remove odd factors
    while div != 1 {
        while  div % p == 0  {
            largestfactor = p;
            div = div/p;
        }
        p += 2;
    }

    largestfactor
}



pub fn largest_prime_factor2(input:usize) -> usize {
    iterate((input,2),|(div,p)|
            match (div,p) {
                (div,p) if div % p == 0 => (div/p,p),
                (div,2) => (div,3),
                (div,p) => (div,p+2),
            })
            .find(|&(div,_)| div == 1).unwrap().1
}

pub fn soln(num:usize) -> usize {
    let a = largest_prime_factor(num);
    let b = largest_prime_factor2(num);
    assert!(a == b);
    a
}

#[cfg(test)]
#[test]
pub fn q3() {assert!(soln(600851475143) == 6857)}

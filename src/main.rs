#![feature(core)]
#![feature(test)]

extern crate test;

pub mod q1;
pub mod q2;
pub mod q3;

#[cfg(not(test))]
fn main() {
    println!("Q1: {:?}" ,q1::soln());
    println!("Q2: {:?}" ,q2::soln());
    println!("Q3: {:?}" ,q3::soln(600851475143));
}

#[cfg(test)]
use test::Bencher;
#[test]
pub fn q1() {assert!(q1::soln() == 233168)}
#[test]
pub fn q2() {assert!(q2::soln() == 4613732)}
#[test]
pub fn q3() {assert!(q3::soln(600851475143) == (6857,6857))}

#[bench]
fn iter_primes(b: &mut Bencher) {
    b.iter(|| {
        test::black_box(
            for _ in 0..100 {
                q3::largest_prime_factor(600851475143);
            }
        )
    })
}
#[bench]
fn fold_primes(b: &mut Bencher) {
    b.iter(|| {
        test::black_box(
            for _ in 0..100 {
                q3::largest_prime_factor2(600851475143);
            }
        )
    })
}

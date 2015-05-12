#![feature(core)]
#![feature(test)]
#![feature(convert)]

extern crate test;

pub mod q1;
pub mod q2;
pub mod q3;
pub mod q4;

#[cfg(not(test))]
fn main() {
    println!("Q1: {:?}" ,q1::soln());
    println!("Q2: {:?}" ,q2::soln());
    println!("Q3: {:?}" ,q3::soln(600851475143));
    println!("Q4: {:?}" ,q4::soln());
}

#[cfg(test)]
use test::Bencher;
#[test]
pub fn q1() {assert!(q1::soln() == 233168)}
#[test]
pub fn q2() {assert!(q2::soln() == 4613732)}
#[test]
pub fn q3() {assert!(q3::soln(600851475143) == (6857,6857))}
#[test]
pub fn q4() {assert!(q4::soln() == (906609,906609))}

#[bench]
fn loop_q3(b: &mut Bencher) {
    b.iter(|| {
        test::black_box(
            for _ in 0..5 {
                q3::largest_prime_factor(600851475143);
            }
        )
    })
}
#[bench]
fn iter_q3(b: &mut Bencher) {
    b.iter(|| {
        test::black_box(
            for _ in 0..5 {
                q3::largest_prime_factor2(600851475143);
            }
        )
    })
}
#[bench]
fn loop_q4(b: &mut Bencher) {
    b.iter(|| {
        test::black_box(
            for _ in 0..2 {
                q4::loop_soln();
            }
        )
    })
}
#[bench]
fn iter_q4(b: &mut Bencher) {
    b.iter(|| {
        test::black_box(
            for _ in 0..2 {
                q4::iter_soln();
            }
        )
    })
}

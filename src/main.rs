#![feature(core)]
#![feature(test)]
#![feature(convert)]
#![feature(collections)]
#![feature(step_by)]
#![feature(scoped)]

extern crate test;
extern crate primal;

mod utils;

mod q1;
mod q2;
mod q3;
mod q4;
mod q5;
mod q6;
mod q7;
mod q8;
mod q9;
mod q10;
mod q11;
mod q12;
mod q13;
mod q14;


#[cfg(not(test))]
use std::env;

#[cfg(not(test))]
fn main() {
    let arg = env::args().nth(1);
    let target = match arg {
        Some(x) => vec!(x.parse::<usize>().ok().expect("Please enter a number!")),
        None => (1..15).collect(),
    };
    
    for x in target {
        print!("Q{} => {}\n",x,run(x))
    }

}

#[cfg(not(test))]
fn run(input:usize) -> usize{
    match input {
        1 => q1::soln(),
        2 => q2::soln(),
        3 => q3::soln(600851475143),
        4 => q4::soln(),
        5 => q5::soln(),
        6 => q6::soln(),
        7 => q7::soln(10001),
        8 => q8::soln(),
        9 => q9::soln(),
        10 => q10::soln(),
        11 => q11::soln(),
        12 => q12::soln(),
        13 => q13::soln(),
        14 => q14::soln(),
        _ => 0,
    }
}


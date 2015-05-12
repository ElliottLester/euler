#![feature(convert)]

pub fn is_palindrome(input:&str) -> bool {
    let forward = input.chars().take(input.len()/2);
    let reverse = input.chars().rev().take(input.len()/2);
    let mut both    = forward.zip(reverse);
    both.all( |(a,b)| {a == b})
}

pub fn main() {
    let mut high = 0;
    let mut test;
    for x in 0..999 {
        for y in 0..999 {
            test = x*y;
            if test > high && is_palindrome(format!("{}",test).as_str()) {
                high = test;
            }
        }
    }
    println!("test {:?}",high);

}

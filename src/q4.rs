use std;

fn is_palindrome(input:&str) -> bool {
    let forward = input.chars().take(input.len()/2);
    let reverse = input.chars().rev().take(input.len()/2);
    let mut both    = forward.zip(reverse);
    both.all( |(a,b)| {a == b})
}

pub fn iter_soln() -> usize {
        (0..999)
        .rev()
        .flat_map(|x|
                  std::iter::repeat(x)
                  .zip((1..x+1).rev())
                 )
        .map(|(x,y)| x * y)
        .filter(|x|
                is_palindrome(format!("{}",x).as_str()))
        .max()
        .unwrap_or(0)
}


pub fn loop_soln() -> usize {
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
    high
}

pub fn soln() -> usize {
    let a = iter_soln();
    let b = loop_soln();
    assert!(a == b);
    a
}

#[cfg(test)]
#[test]
pub fn q4() {assert!(soln() == 906609);}

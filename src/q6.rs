pub fn soln() -> usize{
    let mut sosq:u64 = 0;
    let mut sqos:u64 = 0;
    for i in 0..101 {
        sosq += (i as u64).pow(2);
        sqos += i as u64;
    }
    sqos = sqos.pow(2);
    //println!("Sum of squares: {}",sosq);
    //println!("Square of sum: {}",sqos);
    //println!("diffrence {}",sqos - sosq);
    (sqos - sosq) as usize
}
#[cfg(test)]
#[test]
pub fn q6() {assert!(soln() == 25164150);}

pub fn soln() -> usize{
    let output = 0;
    for a in 0..1000 {
        for b in 0..1000 {
            for c in 0..1000 {
                if a + b + c == 1000{
                    if a < b && b < c{
                        if a*a + b*b == c*c {
                            return a*b*c
                        }
                    }
                }
            }
        }
    } 
    output
}
#[cfg(test)]
#[test]
pub fn q9() {assert!(soln() == 31875000);}

struct Primes {
    state:Vec<usize>,
}
   
impl Iterator for Primes {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let last = match self.state.last() {
            Some(x) => *x,
            None => 2,
        };

        for i in last.. {
            if self.state.iter().all(|x| i % x != 0) {
                self.state.push(i);
                break;
            }
            
        }
        Some(*self.state.last().unwrap())
    } 
}

pub fn soln(input:usize) -> usize {
    let test: Primes = Primes{state:vec!()};
    test.skip(input-1).next().unwrap_or(0)
}

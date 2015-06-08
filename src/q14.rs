use std::thread;
struct Collatz {
    state: usize,
}

impl Collatz {
    pub fn new(a:usize) -> Collatz{
        Collatz{state:a}
    }
}

impl Iterator for Collatz {
    type Item = usize;
    
    fn next(&mut self) -> Option<usize> {
        let next =  
            match self.state {
                0 => None,
                1 => None,
                x if x%2 == 0 => {
                    self.state = x/2;
                    Some(x/2)},
                x => {
                    self.state = 3*x+1; 
                    Some((3*x)+1)},
            };
        next
    }
}

static NTHREADS: usize = 4;

pub fn soln() -> usize {
    let target:usize = 1_000_000;
    let load:usize = target/NTHREADS;
    let mut threads = Vec::with_capacity(NTHREADS as usize);

    for i in 0..NTHREADS {
        threads.push(thread::scoped(move || {
            let range = (i*load)+1..(((i+1)*load));
            range
            .map(|x| 
                 (x,
                  Collatz::new(x as usize)
                  .collect::<Vec<usize>>()
                  .len()
                 )
                )
            .max_by(|x|x.1)
            .unwrap()
        }));
    }

    let output = 
        threads
        .drain(..)
        .map(|x| x.join() )
        .max_by(|x| x.1)
        .unwrap()
        .0;

    output
}
#[cfg(test)]
#[test]
pub fn q14() {assert!(soln() == 837799);}

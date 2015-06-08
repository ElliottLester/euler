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
            let mut num = 0;
            let mut max = 0;
            for x in range {
                let seq = Collatz::new(x as usize).collect::<Vec<usize>>().len();
                if seq > max {
                    max = seq;
                    num = x;
                }
            }
            (num,max)
        }));
    }
    let mut num = 0;
    let mut max = 0;

    for x in threads {
        let seq = x.join();
        if seq.1 > max {
                    max = seq.1;
                    num = seq.0;
                }
    }
    num
}
#[cfg(test)]
#[test]
pub fn q14() {assert!(soln() == 837799);}

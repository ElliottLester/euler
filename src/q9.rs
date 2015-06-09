use std::thread;
/*pub fn soln2() -> usize{
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
}*/

static NTHREADS: usize = 4;

pub fn soln() -> usize {
    let max:usize = 1_000_000_000;
    let load = max/NTHREADS;
    let mut threads = Vec::with_capacity(NTHREADS);

    for i in 0..NTHREADS {
        threads.push(thread::scoped(move || {
            let range = (i*load)+1..(((i+1)*load));
            range
            .map(|x| (x/1_000_000,x/1000%1000,x%1000))
            .filter(|x| x.0 < x.1 && x.1 < x.2)
            .filter(|x| (x.0 + x.1 + x.2) == 1000)
            .filter(|x| x.0*x.0 + x.1*x.1 == x.2*x.2)
            .take(1)
            .collect::<Vec<_>>()
        }));
    }

    let result =
    threads
    .drain(..)
    .map(|x| x.join() )
    .flat_map(|x|x);

    let output = result
        .map(|x| x.0*x.1*x.2)
        .max()
        .unwrap_or(0);
    output
}

#[cfg(test)]
#[test]
pub fn q9() {assert!(soln() == 31875000);}

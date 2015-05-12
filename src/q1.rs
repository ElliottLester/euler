pub fn soln() -> usize{
    (0..1000)
        .filter(|i| i % 3 == 0 || i % 5 == 0 )
        .fold(0,|acc, i| acc + i)
}

pub fn soln() -> usize {
    let mut i = 0;
    let mut j = 2;
    let mut v = vec![1,2];
    while i < 4000000 {
        i = v[0] + v[1];
        v.push(i);
        if i % 2 == 0 {
            j+=i;
        }
        v.remove(0);
    }
    j
}

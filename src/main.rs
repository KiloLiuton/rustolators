fn main() {
    const N: usize = 4;
    const K: usize = 1;

    let mut neighbors: [usize; 2 * N * K] = [0; 2 * N * K];
    //let mut neighbors: [usize; 2 * N * K] = [
    //    3, 1,
    //    0, 2,
    //    1, 3,
    //    2, 4,
    //];
    for i in 0 .. N {
        for j in 0 .. 2*K {
            println!("{}{}", i, j);
            neighbors[i * 2 * K + j] = 666;
        }
    }
    for n in neighbors {
        println!("{}", n);
    }
}

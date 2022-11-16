const N: u32 = 8;
const K: u32 = 1;

fn get_right_neighbor(s: u32, n: u32) -> u32 {
    if (s + n) >= N {
        return (s + n) - N;
    }
    s + n
}

fn get_left_neighbor(s: u32, n: u32) -> u32 {
    if s < n {
        return (s + N) - n;
    }
    s - n
}

fn main() {
    let mut neighbors: [u32; (2 * N * K) as usize] = [0; (2 * N * K) as usize];
    let mut indexes: [u32; N as usize] = [0; N as usize];
    // let mut states: [u8; N as usize] = [0; N as usize];

    for i in 0 .. N {
        let n = i * 2 * K;
        indexes[i as usize] = n;
        for j in 0 .. K {
            neighbors[(n + j) as usize] = get_left_neighbor(i, j+1);
            neighbors[(n + j + 1) as usize] = get_right_neighbor(i, j+1);
        }
    }
    for n in neighbors {
        println!("{}", n);
    }
}

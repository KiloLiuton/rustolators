use rand::Rng;

const N: u32 = 8;
const K: u32 = 1;
const TWO_NK: u32 = 2 * N * K;
const ITERS: u32 = 20;

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

fn step(sts: & mut [u8; N as usize], &_nbrs: &[u32; TWO_NK as usize]) {
    let mut rng = rand::thread_rng();
    let n: usize = rng.gen_range(0..N as usize);
    sts[n] = (sts[n] + 1) % 3;
}

fn main() {
    let mut neighbors: [u32; TWO_NK as usize] = [0; TWO_NK as usize];
    let mut indexes: [u32; N as usize] = [0; N as usize];
    let mut states: [u8; N as usize] = [0; N as usize];
    // let mut rates: [f32; N as usize] = [0.0; N as usize];

    for i in 0 .. N {
        let n = i * 2 * K;
        for j in 0 .. K {
            neighbors[(n + j) as usize] = get_left_neighbor(i, j+1);
            neighbors[(n + j + 1) as usize] = get_right_neighbor(i, j+1);
        }
        indexes[i as usize] = n;
    }

    for i in 1 ..= ITERS {
        step(&mut states, &neighbors);

        println!("iter: {}", i);
        for j in 0 .. N {
            print!("{}", states[j as usize]);
        }
        println!("");
    }
}

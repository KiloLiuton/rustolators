use rand::Rng;

const N: u32 = 8;
const K: u32 = 1;
const TWO_NK: u32 = 2 * N * K;
const ITERS: u32 = 20;

struct State {
    neighbors: [u32; TWO_NK as usize],
    indexes: [u32; N as usize],
    states: [u8; N as usize],
    rates: [f32; N as usize],
    rates_sum: f32,
}

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

fn step(s: &mut State) {
    let mut rng = rand::thread_rng();
    let n: usize = rng.gen_range(0..N as usize);
    s.states[n] = (s.states[n] + 1) % 3;
}

fn initialize_state() -> State {
    let mut state = State {
        neighbors: [0; TWO_NK as usize],
        indexes: [0; N as usize],
        states: [0; N as usize],
        rates: [0.0; N as usize],
        rates_sum: 0.0,
    };

    for i in 0..N {
        let n = i * 2 * K;
        for j in 0..K {
            state.neighbors[(n + j) as usize] = get_left_neighbor(i, j + 1);
            state.neighbors[(n + j + 1) as usize] = get_right_neighbor(i, j + 1);
        }
        state.indexes[i as usize] = n;
    }

    for i in 0..N as usize {
        state.rates[i] = 1.0;
        state.rates_sum = state.rates_sum + 1.0;
    }
    state
}

fn main() {
    let mut state: State = initialize_state();

    for i in 1..=ITERS {
        step(&mut state);

        print!("iter {:2}: ", i);
        for j in 0..N as usize {
            print!("{}", state.states[j]);
        }
        println!("");
    }
}

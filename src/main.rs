use rand::Rng;
use std::env;
use std::process;

const SIZE: usize = 12;
const RANGE: usize = 1;
const TWO_NK: usize = 2 * SIZE * RANGE;

#[derive(Default, Debug)]
pub struct Lattice {
    neighbors: [usize; TWO_NK],
    num_neighbors: [usize; SIZE],
    indexes: [usize; SIZE],
}

impl Lattice {
    fn get_right_neighbor(s: usize, n: usize) -> usize {
        (s + n) % SIZE
    }

    fn get_left_neighbor(s: usize, n: usize) -> usize {
        ((s + SIZE) - n) % SIZE
    }

    fn create_regular_ring() -> Lattice {
        let mut lattice: Lattice = Default::default();
        for i in 0..SIZE {
            let n = i * 2 * RANGE;
            for j in 0..RANGE {
                lattice.neighbors[n + j] = Lattice::get_left_neighbor(i, j + 1);
                lattice.neighbors[n + j + 1] = Lattice::get_right_neighbor(i, j + 1);
            }
            lattice.indexes[i] = n;
            lattice.num_neighbors[i] = 2 * RANGE;
        }
        lattice
    }
}

#[derive(Default, Debug)]
pub struct System {
    states: [u8; SIZE],
    rates: [f32; SIZE],
    rates_sum: f32,
    coupling: f32,
}

impl System {
    pub fn initialize(coupling: f32, lattice: &Lattice) -> System {
        let mut system: System = Default::default();

        system.coupling = coupling;

        let mut rng = rand::thread_rng();
        for i in 0..SIZE {
            system.states[i] = rng.gen_range(0..=2);
        }
        system.calc_rates(lattice);
        system
    }

    pub fn step(&mut self) {
        let mut rng = rand::thread_rng();
        let n: usize = rng.gen_range(0..SIZE);
        self.states[n] = (self.states[n] + 1) % 3;
    }

    fn calc_rates(&mut self, lattice: &Lattice) {
        for (i, &idx) in lattice.indexes.iter().enumerate() {
            let cur_state = self.states[i];
            let next_state = (cur_state + 1) % 3;
            let prev_state = (cur_state + 2) % 3;

            let mut num_prev: usize = 0;
            let mut num_next: usize = 0;

            let num_nbrs = lattice.num_neighbors[i];

            for &nbr_index in &lattice.neighbors[idx..idx + num_nbrs] {
                let nbr_state = self.states[nbr_index];
                if nbr_state == prev_state {
                    num_prev += 1;
                } else if nbr_state == next_state {
                    num_next += 1;
                }
            }
            let delta: f32 = num_next as f32 - num_prev as f32;
            let rate: f32 = f32::exp(self.coupling * delta / num_nbrs as f32);
            self.rates[i] = rate;
            self.rates_sum = self.rates_sum + rate;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let iters: u32 = if args.len() > 1 {
        args[1].parse().unwrap_or_else(|err| {
            println!("Trouble parsing iters due to error:\n{err}");
            process::exit(1);
        })
    } else {
        10
    };
    let coupling: f32 = if args.len() > 2 {
        args[2].parse().unwrap_or_else(|err| {
            println!("Trouble parsing coupling due to error:\n{err}");
            process::exit(1);
        })
    } else {
        2.0
    };

    let lattice = Lattice::create_regular_ring();

    let mut system = System::initialize(coupling, &lattice);

    println!("System: {:#?}", system);

    for i in 1..=iters {
        system.step();

        print!("iter {:2}: ", i);
        for j in 0..SIZE {
            print!("{}", system.states[j]);
        }
        println!("");
    }
}

use rand::rngs::ThreadRng;
use rand::Rng;
use std::env;
use std::fs::OpenOptions;
use std::process;
use std::io::prelude::*;

const SIZE: usize = 100;
const RANGE: usize = 8;
const TWO_NK: usize = 2 * SIZE * RANGE;
const NUM_STATES: u8 = 3;

pub struct Lattice {
    neighbors: [usize; TWO_NK],
    num_neighbors: [usize; SIZE],
    indexes: [usize; SIZE],
}

impl Lattice {
    fn create_regular_ring() -> Lattice {
        let mut lattice: Lattice = Lattice {
            neighbors: [0; TWO_NK],
            num_neighbors: [0; SIZE],
            indexes: [0; SIZE],
        };
        for i in 0..SIZE {
            let n = i * 2 * RANGE;
            for j in 0..RANGE {
                lattice.neighbors[n + j] = ((i + SIZE) - j - 1) % SIZE;
                lattice.neighbors[n + j + 1] = (i + j + 1) % SIZE;
            }
            lattice.indexes[i] = n;
            lattice.num_neighbors[i] = 2 * RANGE;
        }
        lattice
    }
}

pub struct System {
    states: [u8; SIZE],
    deltas: [f64; SIZE],
    rates: [f64; SIZE],
    rates_sum: f64,
    coupling: f64,
    rng: ThreadRng,
    lattice: Lattice,
}

impl System {
    pub fn initialize_random(coupling: f64) -> System {
        let mut system: System = System {
            states: [0; SIZE],
            deltas: [0.0; SIZE],
            rates: [0.0; SIZE],
            rates_sum: 0.0,
            coupling,
            rng: rand::thread_rng(),
            lattice: Lattice::create_regular_ring(),
        };

        for i in 0..SIZE {
            system.states[i] = system.rng.gen_range(0..=2);
        }
        system.calc_rates();
        system
    }

    pub fn initialize_uniform(coupling: f64) -> System {
        let mut system: System = System {
            states: [0; SIZE],
            deltas: [0.0; SIZE],
            rates: [0.0; SIZE],
            rates_sum: 0.0,
            coupling,
            rng: rand::thread_rng(),
            lattice: Lattice::create_regular_ring(),
        };

        system.coupling = coupling;
        system.rng = rand::thread_rng();
        system.lattice = Lattice::create_regular_ring();

        for i in 0..SIZE {
            system.states[i] = 0;
        }
        system.calc_rates();
        system
    }

    pub fn initialize_wave(coupling: f64, periods: usize) -> System {
        let mut system: System = System {
            states: [0; SIZE],
            deltas: [0.0; SIZE],
            rates: [0.0; SIZE],
            rates_sum: 0.0,
            coupling,
            rng: rand::thread_rng(),
            lattice: Lattice::create_regular_ring(),
        };

        system.coupling = coupling;
        system.rng = rand::thread_rng();
        system.lattice = Lattice::create_regular_ring();

        let wave_len = SIZE / (periods * NUM_STATES as usize);
        let mut counter: usize = 0;
        let mut state: u8 = 0;
        for i in 0..SIZE {
            system.states[i] = state;
            counter += 1;
            if counter > wave_len {
                state = (state + 1) % NUM_STATES;
                counter = 0;
            }
        }
        system.calc_rates();
        system
    }

    pub fn step(&mut self) {
        let r = self.rng.gen_range(0.0..self.rates_sum);
        let mut partial: f64 = 0.0;

        for i in 0..SIZE {
            partial += self.rates[i];
            if partial > r {
                self.update_rates(i);
                break;
            }
        }
    }

    // nbr       prev   new
    // nbr=prev: 0/0 -> 0/1 = -1 -> +1 = +2
    // nbr=new:  0/2 -> 0/0 =  0 -> -1 = -1
    // else:     0/1 -> 0/2 = +1 ->  0 = -1
    // site      prev   new
    // nbr=prev: 0/0 -> 0/1 = -1 ->  0 = +1
    // nbr=new:  0/2 -> 0/0 = +1 -> -1 = -2
    // else:     0/1 -> 0/2 =  0 -> +1 = +1
    fn update_rates(&mut self, site: usize) {
        let prev_state = self.states[site];
        let new_state = (prev_state + 1) % NUM_STATES;
        self.states[site] = new_state;

        let idx = self.lattice.indexes[site];
        let num_nbrs = self.lattice.num_neighbors[site];
        for &nbr in &self.lattice.neighbors[idx..idx + num_nbrs] {
            let nbr_state = self.states[nbr];
            if nbr_state == prev_state {
                self.deltas[nbr] += 2.0;
                self.deltas[site] += 1.0;
            } else if nbr_state == new_state {
                self.deltas[nbr] -= 1.0;
                self.deltas[site] -= 2.0;
            } else {
                self.deltas[nbr] -= 1.0;
                self.deltas[site] += 1.0;
            }
            let new_rate =
                f64::exp(self.coupling * self.deltas[nbr] / self.lattice.num_neighbors[nbr] as f64);
            self.rates_sum += new_rate - self.rates[nbr];
            self.rates[nbr] = new_rate;
        }
        let new_rate = f64::exp(self.coupling * self.deltas[site] / num_nbrs as f64);
        self.rates_sum += new_rate - self.rates[site];
        self.rates[site] = new_rate;
    }

    fn calc_rates(&mut self) {
        for i in 0..SIZE {
            let curr_state = self.states[i];
            let next_state = (curr_state + 1) % NUM_STATES;
            let mut num_curr: f64 = 0.0;
            let mut num_next: f64 = 0.0;

            let num_nbrs = self.lattice.num_neighbors[i];
            let idx = self.lattice.indexes[i];

            for &nbr_index in &self.lattice.neighbors[idx..idx + num_nbrs] {
                let nbr_state = self.states[nbr_index];
                if nbr_state == next_state {
                    num_next += 1.0;
                } else if nbr_state == curr_state {
                    num_curr += 1.0;
                }
            }
            let delta = num_next - num_curr;
            let rate: f64 = f64::exp(self.coupling * delta / num_nbrs as f64);
            self.deltas[i] = delta;
            self.rates[i] = rate;
            self.rates_sum = self.rates_sum + rate;
        }
    }
}

fn print_iter(system: &System, iter: u64) {
    if SIZE > 100 {
        return;
    }
    print!("iter {:4}: ", iter);
    for j in 0..SIZE {
        print!("{}", system.states[j]);
    }
    println!("");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let iters: u64 = if args.len() > 1 {
        args[1].parse().unwrap_or_else(|err| {
            println!("Trouble parsing iters due to error:\n{err}");
            process::exit(1);
        })
    } else {
        10
    };
    let coupling: f64 = if args.len() > 2 {
        args[2].parse().unwrap_or_else(|err| {
            println!("Trouble parsing coupling due to error:\n{err}");
            process::exit(1);
        })
    } else {
        2.0
    };

    // let mut system = System::initialize_random(coupling);
    // let mut system = System::initialize_uniform(coupling);
    let mut system = System::initialize_wave(coupling, 1);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("out.txt")
        .unwrap();

    if let Err(e) = writeln!(file, system.states) {
        eprintln!("Couldn't write to file: {}", e);
    }

    print_iter(&system, 0);
    for i in 1..=iters {
        system.step();
        print_iter(&system, i);
    }
}

use std::thread::Thread;

use half::f16;
use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Qubit {
    id: u64,
    states: usize,
    observed: bool,
    wave_function: Vec<f16>,
    entangled_qubits: Vec<u64>,
}

impl Qubit {
    pub fn new(id: u64, states: usize) -> Self {
        Self {
            id,
            states,
            observed: false,
            wave_function: vec![f16::from_f64(1.0 / states as f64); states],
            entangled_qubits: Vec::new(),
        }
    }

    pub fn observe(&self) -> () {
        let mut rng = rand::thread_rng();

        let rand_f64: f64 = rng.gen();
        let rand_const: f16 = f16::from_f64_const(rand_f64);

        let mut total_probability: f16 = f16::from_f32(0.0);
        for state in &self.wave_function {
            total_probability += state;
            if (*state < rand_const) {}
        }
    }

    pub fn entangle_qubit(&mut self, other_qubit: &mut Qubit) {
        self.entangled_qubits.push(other_qubit.id);

        let new_states: usize = self.states * other_qubit.states;

        self.states = new_states;
        other_qubit.states = new_states;

        let mut new_wave_function: Vec<f16> = Vec::new();
        for state_self in &self.wave_function {
            for state_other in &other_qubit.wave_function {
                new_wave_function.push(*state_self * *state_other);
            }
        }

        let mut other_wave_function: Vec<f16> = Vec::new();
        for state_other in &other_qubit.wave_function {
            for state_self in &self.wave_function {
                other_wave_function.push(*state_self * *state_other);
            }
        }

        self.wave_function = new_wave_function;
        other_qubit.wave_function = other_wave_function;
    }
}

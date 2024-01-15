mod custom_binary_num;

use custom_binary_num::BinaryNum;
use half::f16;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spin {
    Up,
    Down,
    Undetermined,
}

#[derive(Clone)]
pub struct Qubit {
    id: u64,
    spin: Spin,
    states: usize,
    observed: bool,
    wave_function: Vec<f16>,
    entangled_qubits: Vec<u64>,
    //TODO: add a vec of type BinaryNum that represents the state that the wave functions value coresponeds to
}

impl Qubit {
    pub fn new(id: u64, states: usize) -> Self {
        Self {
            id,
            spin: Spin::Undetermined,
            states,
            observed: false,
            wave_function: vec![f16::from_f64(1.0 / states as f64); states],
            entangled_qubits: Vec::new(),
        }
    }

    pub fn observe<'a>(&self, entangled_qubits: Vec<&'a Qubit>) -> usize {
        let mut rng = thread_rng();

        let rand_f64: f64 = rng.gen();
        let rand_const: f16 = f16::from_f64_const(rand_f64);

        let mut total_probability: f16 = f16::from_f32(0.0);
        let mut result_state: usize = 0;
        for state in 0..self.wave_function.len() {
            total_probability += &self.wave_function[state];
            if self.wave_function[state] <= rand_const {
                result_state = state;
                break;
            }
        }

        result_state
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

    fn get_entangled_qubits<'a>(&self, all_qubits: &'a [Qubit]) -> Vec<&'a Qubit> {
        let mut entangled_qubits_refs = Vec::new();
        for &entangled_id in &self.entangled_qubits {
            if let Some(qubit) = all_qubits.iter().find(|&q| q.id == entangled_id) {
                entangled_qubits_refs.push(qubit);
            }
        }

        entangled_qubits_refs
    }
}

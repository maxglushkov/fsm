use super::Model;
use ndarray::Array2;
use std::mem::MaybeUninit;

#[inline]
pub fn empty_output(model: Model, n_inputs: usize) -> Array2<usize> {
	Array2::default((0, if model == Model::Mealy { n_inputs } else { 1 }))
}

#[inline]
pub fn empty_state(n_inputs: usize) -> Array2<usize> {
	Array2::default((0, n_inputs))
}

#[inline]
pub fn uninit_output(model: Model, n_states: usize, n_inputs: usize) -> Array2<MaybeUninit<usize>> {
	Array2::uninit((n_states, if model == Model::Mealy { n_inputs } else { 1 }))
}

#[inline]
pub fn uninit_state(n_states: usize, n_inputs: usize) -> Array2<MaybeUninit<usize>> {
	Array2::uninit((n_states, n_inputs))
}

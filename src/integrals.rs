//! This module contains functions to compute the core Hamiltonian matrix.

use ndarray::Array2;

/// Combine kinetic and nuclear attraction integrals to form the core Hamiltonian
pub fn build_core_hamiltonian(kinetic: &Array2<f64>, nuclear_attraction: &Array2<f64>) -> Array2<f64> {
    kinetic + nuclear_attraction
}
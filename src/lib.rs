//! use PyO3’s macros to expose Rust functions and structs to Python

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use ndarray::Array2;

// Import modules
mod utils;
mod integrals;
mod orthogonalization;
mod scf;
mod helpers;

use scf::scf_procedure;

/// A Python module implemented in Rust.
#[pymodule]
fn hartree_fock(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_scf, m)?)?;
    Ok(())
}

/// Runs the SCF procedure.
///
/// Parameters:
/// - `enuc_path`: Path to enuc.dat
/// - `s_path`: Path to overlap integrals (s.dat)
/// - `t_path`: Path to kinetic integrals (t.dat)
/// - `v_path`: Path to nuclear attraction integrals (v.dat)
/// - `eri_path`: Path to electron repulsion integrals (eri.dat)
/// - `max_iter`: Maximum number of SCF iterations
/// - `conv_energy`: Convergence threshold for energy
/// - `conv_density`: Convergence threshold for density
///
/// Returns:
/// - `total_energy`: Total SCF energy
/// - `density_matrix`: Density matrix as a 2D list
#[pyfunction]
fn run_scf(
    enuc_path: String,
    s_path: String,
    t_path: String,
    v_path: String,
    eri_path: String,
    max_iter: usize,
    conv_energy: f64,
    conv_density: f64,
) -> PyResult<(f64, Vec<Vec<f64>>)> {
    // Load data
    let enuc = data_loading::read_nuclear_repulsion(&enuc_path);
    let basis_size = 3; // Example basis size; replace with actual value or determine dynamically
    let s = data_loading::read_matrix(&s_path, basis_size);
    let kinetic = data_loading::read_matrix(&t_path, basis_size);
    let nuclear_attraction = data_loading::read_matrix(&v_path, basis_size);
    let h_core = integrals::build_core_hamiltonian(&kinetic, &nuclear_attraction);

    // Read ERI (this should be implemented properly)
    let eri = vec![]; // Placeholder

    // Run SCF
    let result = scf_procedure(
        enuc,
        s,
        h_core,
        eri,
        max_iter,
        conv_energy,
        conv_density,
    );

    // Convert density matrix to Vec<Vec<f64>> for Python
    let density_matrix_py: Vec<Vec<f64>> = result
        .density_matrix
        .outer_iter()
        .map(|row| row.to_vec())
        .collect();

    Ok((result.total_energy, density_matrix_py))
}
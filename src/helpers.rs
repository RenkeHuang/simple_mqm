//! Helper functions for the project

// Mapping four indices to a single compound index based on permutational symmetry
pub fn compound_index(mu: usize, nu: usize, lambda_: usize, sigma: usize, basis_size: usize) -> usize {
    // Implement the compound index calculation based on the symmetry rules
    // This is a placeholder example
    mu * basis_size * basis_size * basis_size + nu * basis_size * basis_size + lambda_ * basis_size + sigma
}
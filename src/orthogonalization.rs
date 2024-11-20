use ndarray::{Array2, Array1};
use ndarray_linalg::Eigh;

/// Perform symmetric orthogonalization on the overlap matrix S.
pub fn symmetric_orthogonalization(s: &Array2<f64>) -> Array2<f64> {
    // Diagonalize the overlap matrix S
    let (eigvals, eigvecs) = s.clone().eigh(UPLO::Lower).expect("Diagonalization failed");

    // Compute S^(-1/2)
    let s_inv_sqrt = eigvecs.dot(&Array2::from_diag(&eigvals.mapv(|x| 1.0 / x.sqrt())))).dot(&eigvecs.t());

    s_inv_sqrt
}
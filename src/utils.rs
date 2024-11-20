use ndarray::Array2;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Read the scalar term from a file.
/// Usage: let enuc = read_nuclear_repulsion("./input/h2o/STO-3G/enuc.dat")
pub fn read_nuclear_repulsion(file_path: &str) -> f64 {
    let file = File::open(file_path).expect("Unable to open enuc.dat");
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap().unwrap();
    line.parse::<f64>().expect("Unable to parse nuclear repulsion energy")
}

/// Read the matrix from a file.
// let overlap = read_matrix("./input/h2o/STO-3G/s.dat", basis_size);
// let kinetic = read_matrix("./input/h2o/STO-3G/t.dat", basis_size);
// let nuclear_attraction = read_matrix("./input/h2o/STO-3G/v.dat", basis_size);
pub fn read_matrix(file_path: &str, size: usize) -> Array2<f64> {
    let file = File::open(file_path).expect(&format!("Unable to open {}", file_path));
    let reader = BufReader::new(file);
    let mut data = Vec::new();
    for line in reader.lines() {
        let nums: Vec<f64> = line
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        data.extend(nums);
    }
    Array2::from_shape_vec((size, size), data).expect("Error constructing matrix")
}
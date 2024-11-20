# simple_mqm

**simple_mqm** will be a high-performance Rust library designed for molecular quantum mechanics (QM) computations. It will provide implementations of various quantum chemistry methods, starting with the Hartree-Fock Self-Consistent Field (SCF) procedure.

Python bindings are available, enabling easy integration with Python-based workflows and tools.

### Why Rust?

- **Memory Management:** Take advantage of Rust’s ownership model to manage memory efficiently and prevent leaks.
- **High Performance:** Leverages Rust's speed and safety for efficient computations.
e.g. use Rust’s concurrency features to parallelize the computation of the Fock matrix.
- **Python Integration:** Accessible via Python bindings using PyO3

## Installation

### Prerequisites

- **Rust:** Ensure you have Rust and Cargo installed. [Install Rust](https://www.rust-lang.org/tools/install)
- **Python:** Python 3.9 or higher is required.
- **Maturin:** Build and publish Rust-based Python packages.
```bash
python -m pip install maturin
```
Also consider setuptools-rust for building the Python module.

### Build the Python module
From the root directory, build the Rust code and install the resulting Python module into current Python environment:
```bash
maturin develop
```
To build a wheel for distribution:
```
maturin build
pip install target/wheels/simple_mqm-0.1.0-py3-none-any.whl
```

### Ideal usage
```python
import simple_mqm

# Define file paths to input data
enuc_path = "./input/h2o/STO-3G/enuc.dat"
s_path = "./input/h2o/STO-3G/s.dat"
t_path = "./input/h2o/STO-3G/t.dat"
v_path = "./input/h2o/STO-3G/v.dat"
eri_path = "./input/h2o/STO-3G/eri.dat"

# SCF parameters
max_iter = 100           # Maximum number of SCF iterations
conv_energy = 1e-6       # Energy convergence threshold
conv_density = 1e-6      # Density matrix convergence threshold

# Run the SCF procedure
total_energy, density_matrix = simple_mqm.run_scf(
    enuc_path=enuc_path,
    s_path=s_path,
    t_path=t_path,
    v_path=v_path,
    eri_path=eri_path,
    max_iter=max_iter,
    conv_energy=conv_energy,
    conv_density=conv_density
)

# Display the results
print(f"Total SCF Energy: {total_energy}")
print("Density Matrix:")
for row in density_matrix:
    print(row)
```

### Project Structure in plan
```
simple_mqm/
├── src/
│   ├── lib.rs
│   ├── helpers.rs
│   ├── integrals.rs
│   ├── orthogonalization.rs
│   ├── scf.rs
│   └── utils.rs
├── tests/
│   └── test_scf.py
├── input/
│   └── h2o/
│       └── STO-3G/
│           ├── enuc.dat
│           ├── s.dat
│           ├── t.dat
│           ├── v.dat
│           └── eri.dat
├── Cargo.toml
├── README.md
└── LICENSE
```


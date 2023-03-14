mod error;
mod sum_json;

use pyo3::prelude::*;

use error::JSONSumError;
use sum_json::sum_json;

/// Sum the numeric values of the given key in a JSON file
///
/// # Arguments
///
/// * `path` - Path to the JSON file
/// * `key` - The key that will have its values summed
///
/// # Returns
///
/// The sum of numeric values for the `key`
#[pyfunction]
fn sum(path: String, key: String) -> Result<f64, JSONSumError> {
    // Open file and read bytes into buffer
    let mut buffer = std::fs::read(path)?;

    // Parse JSON
    let value = simd_json::to_borrowed_value(&mut buffer)?;

    // Recursively sum numeric values of the specified search key
    Ok(sum_json(&key, value))
}

/// A Python module implemented in Rust.
#[pymodule]
fn json_sum(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum, m)?)?;
    Ok(())
}

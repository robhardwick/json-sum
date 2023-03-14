use std::io;

use pyo3::exceptions::{PyIOError, PyValueError};
use pyo3::prelude::*;

/// An error enum for the `sum_json` package
#[derive(Debug)]
pub enum JSONSumError {
    IO(io::Error),
    Parse(simd_json::Error),
}

/// Convert from std IO errors
impl From<io::Error> for JSONSumError {
    fn from(error: io::Error) -> Self {
        Self::IO(error)
    }
}

/// Convert from simd_json parsing errors
impl From<simd_json::Error> for JSONSumError {
    fn from(error: simd_json::Error) -> Self {
        Self::Parse(error)
    }
}

/// Convert to a PyO3 error
impl From<JSONSumError> for PyErr {
    fn from(error: JSONSumError) -> PyErr {
        match error {
            // IO errors are Python `IOError` exceptions
            JSONSumError::IO(error) => PyIOError::new_err(error.to_string()),
            // smid_json parsing errors are `ValueError` exceptions
            JSONSumError::Parse(error) => PyValueError::new_err(error.to_string()),
        }
    }
}

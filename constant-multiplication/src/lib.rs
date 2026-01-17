// src/lib.rs
// Cargo.toml should have:
// [dependencies]
// pyo3 = { version = "0.23", features = ["extension-module"] }
//
// [lib]
// name = "compact_vector"
// crate-type = ["cdylib"]
//
// [build-dependencies]
// (none needed if using simple approach below)

use pyo3::prelude::*;
use pyo3::exceptions::{PyIndexError, PyValueError};

// Include the compile-time generated data
include!(concat!(env!("OUT_DIR"), "/embedded_data.rs"));

/// Get value at odd index from the embedded packed data
/// If an even index is passed, it's right-shifted until odd
#[pyfunction]
fn adder_cost(mut idx: usize) -> PyResult<u8> {
    // Right-shift even indices until odd
    while idx % 2 == 0 && idx > 0 {
        idx >>= 1;
    }

    let value_position = idx / 2;
    if value_position >= DATA_COUNT {
        return Err(PyIndexError::new_err("Index out of range"));
    }

    let bit_offset = value_position * 3;
    let byte_offset = bit_offset / 8;
    let bit_in_byte = bit_offset % 8;

    if byte_offset >= DATA_BYTES.len() {
        return Err(PyValueError::new_err("Data corruption"));
    }

    let mut val = (DATA_BYTES[byte_offset] >> bit_in_byte) & 0b111;

    // Handle values that span two bytes
    if bit_in_byte > 5 && byte_offset + 1 < DATA_BYTES.len() {
        let bits_from_next = 3 - (8 - bit_in_byte);
        val |= (DATA_BYTES[byte_offset + 1] & ((1 << bits_from_next) - 1))
               << (8 - bit_in_byte);
    }

    Ok(val & 0b111)
}

/// Get info about the embedded data
#[pyfunction]
fn info() -> String {
    format!(
        "Embedded vector: {} elements, {} bytes packed",
        DATA_COUNT * 2,
        DATA_BYTES.len()
    )
}

#[pymodule]
fn constant_multiplication(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(adder_cost))?;
    m.add_wrapped(wrap_pyfunction!(info))?;
    Ok(())
}
// src/lib.rs
use pyo3::exceptions::{PyIndexError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyList;
use serde::{Deserialize, Serialize};

// Include the compile-time generated data
include!(concat!(env!("OUT_DIR"), "/embedded_data.rs"));
include!(concat!(env!("OUT_DIR"), "/embedded_graph_types.rs"));

// GraphType enum definition (must match prepare_data.rs)
#[derive(Debug, Clone, Serialize, Deserialize)]
enum GraphTypeInternal {
    A(usize, usize),
    S(usize, usize),
    C(usize, usize),
    L1(usize, usize, usize, usize, usize),
    L2(usize, usize, usize, usize, usize),
    L3(usize, usize, usize, usize, usize),
    L4(usize, usize, usize, usize, usize),
}

// GraphType as a Python class
#[pyclass]
#[derive(Clone)]
struct GraphType {
    #[pyo3(get)]
    variant: String,
    #[pyo3(get)]
    params: Vec<usize>,
}

#[pymethods]
impl GraphType {
    fn __repr__(&self) -> String {
        match self.params.len() {
            2 => format!("{}({}, {})", self.variant, self.params[0], self.params[1]),
            5 => format!(
                "{}({}, {}, {}, {}, {})",
                self.variant,
                self.params[0],
                self.params[1],
                self.params[2],
                self.params[3],
                self.params[4]
            ),
            _ => format!("{}({:?})", self.variant, self.params),
        }
    }
}

impl From<GraphTypeInternal> for GraphType {
    fn from(gt: GraphTypeInternal) -> Self {
        match gt {
            GraphTypeInternal::A(a, b) => GraphType {
                variant: "Adder".to_string(),
                params: vec![a, b],
            },
            GraphTypeInternal::S(a, b) => GraphType {
                variant: "Subtractor".to_string(),
                params: vec![a, b],
            },
            GraphTypeInternal::C(a, b) => GraphType {
                variant: "Cascade".to_string(),
                params: vec![a, b],
            },
            GraphTypeInternal::L1(a, b, c, d, e) => GraphType {
                variant: "Leapfrog1".to_string(),
                params: vec![a, b, c, d, e],
            },
            GraphTypeInternal::L2(a, b, c, d, e) => GraphType {
                variant: "Leapfrog2".to_string(),
                params: vec![a, b, c, d, e],
            },
            GraphTypeInternal::L3(a, b, c, d, e) => GraphType {
                variant: "Leapfrog3".to_string(),
                params: vec![a, b, c, d, e],
            },
            GraphTypeInternal::L4(a, b, c, d, e) => GraphType {
                variant: "Leapfrog4".to_string(),
                params: vec![a, b, c, d, e],
            },
        }
    }
}

// Deserialize graph types once and cache
fn get_graph_types_data() -> PyResult<Vec<Vec<GraphType>>> {
    // Decompress the LZ4 data
    let decompressed = lz4_flex::decompress_size_prepended(GRAPH_TYPES_BYTES)
        .map_err(|e| PyValueError::new_err(format!("Failed to decompress: {}", e)))?;

    // Deserialize with bincode
    let internal: Vec<Vec<GraphTypeInternal>> = bincode::deserialize(&decompressed)
        .map_err(|e| PyValueError::new_err(format!("Failed to deserialize: {}", e)))?;

    Ok(internal
        .into_iter()
        .map(|vec| vec.into_iter().map(GraphType::from).collect())
        .collect())
}

/// Get adder cost at index (right-shifts even indices until odd)
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
        val |= (DATA_BYTES[byte_offset + 1] & ((1 << bits_from_next) - 1)) << (8 - bit_in_byte);
    }

    Ok(val & 0b111)
}

/// Get the total length of the original vector
#[pyfunction]
fn len() -> usize {
    DATA_COUNT * 2
}

/// Get info about the embedded data
#[pyfunction]
fn info() -> String {
    format!(
        "Embedded data: {} elements, {} bytes packed, graph types: {} bytes compressed",
        DATA_COUNT * 2,
        DATA_BYTES.len(),
        GRAPH_TYPES_BYTES.len()
    )
}

/// Get graph types at index (right-shifts even indices until odd)
#[pyfunction]
fn get_graph_types(py: Python, mut idx: usize) -> PyResult<Py<PyAny>> {
    // Right-shift even indices until odd
    while idx % 2 == 0 && idx > 0 {
        idx >>= 1;
    }

    let all_types = get_graph_types_data()?;

    // Convert odd index to position in the compact array
    // index 1 -> position 0, index 3 -> position 1, index 5 -> position 2, etc.
    let position = idx / 2;

    if position >= all_types.len() {
        return Err(PyIndexError::new_err("Index out of range"));
    }

    let types = &all_types[position];
    let list = PyList::empty(py);
    for gt in types {
        list.append(gt.clone())?;
    }
    Ok(list.into())
}

/// Get total number of graph type entries
#[pyfunction]
fn graph_types_len() -> PyResult<usize> {
    let all_types = get_graph_types_data()?;
    Ok(all_types.len())
}

/// Get all graph types as a list
#[pyfunction]
fn get_all_graph_types(py: Python) -> PyResult<Py<PyAny>> {
    let all_types = get_graph_types_data()?;
    let result = PyList::empty(py);

    for types in all_types {
        let inner_list = PyList::empty(py);
        for gt in types {
            inner_list.append(gt)?;
        }
        result.append(inner_list)?;
    }

    Ok(result.into())
}

#[pymodule]
fn constant_multiplication(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<GraphType>()?;
    m.add_function(wrap_pyfunction!(adder_cost, m)?)?;
    m.add_function(wrap_pyfunction!(len, m)?)?;
    m.add_function(wrap_pyfunction!(info, m)?)?;
    m.add_function(wrap_pyfunction!(get_graph_types, m)?)?;
    m.add_function(wrap_pyfunction!(graph_types_len, m)?)?;
    m.add_function(wrap_pyfunction!(get_all_graph_types, m)?)?;
    Ok(())
}

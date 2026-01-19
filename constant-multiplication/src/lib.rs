// src/lib.rs
use pyo3::exceptions::{PyIndexError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyList;
use serde::{Deserialize, Serialize};
use unsigned_varint::decode as varint_decode;

// Include the compile-time generated data
include!(concat!(env!("OUT_DIR"), "/embedded_adder_cost.rs"));
include!(concat!(env!("OUT_DIR"), "/embedded_graph_types.rs"));

// GraphType enum definition (must match prepare_data.rs)
#[derive(Debug, Clone, Serialize, Deserialize)]
enum GraphTypeInternal {
    Adder(usize, usize),
    Subtractor(usize, usize),
    Cascade(usize, usize),
    Leapfrog4_1(usize, usize, usize, usize),
    Leapfrog4_2(usize, usize, usize, usize),
    Leapfrog4_3(usize, usize, usize, usize),
    Leapfrog4_4(usize, usize, usize, usize),
    Leapfrog5_1(usize, usize, usize, usize, usize),
    Leapfrog5_2(usize, usize, usize, usize, usize),
    Leapfrog5_3(usize, usize, usize, usize, usize),
    Leapfrog5_4(usize, usize, usize, usize, usize),
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

fn extract_shift(value: usize) -> String {
    let shift = value.trailing_zeros();
    let current = value >> shift;
    if shift == 0 {
        current.to_string()
    } else {
        format!("{current} << {shift}")
    }
}

#[pymethods]
impl GraphType {
    fn __repr__(&self) -> String {
        match self.params.len() {
            2 => format!(
                "{}({}, {})",
                self.variant,
                extract_shift(self.params[0]),
                extract_shift(self.params[1])
            ),
            4 => format!(
                "{}({}, {}, {}, {})",
                self.variant,
                extract_shift(self.params[0]),
                extract_shift(self.params[1]),
                extract_shift(self.params[2]),
                extract_shift(self.params[3]),
            ),
            5 => format!(
                "{}({}, {}, {}, {}, {})",
                self.variant,
                extract_shift(self.params[0]),
                extract_shift(self.params[1]),
                extract_shift(self.params[2]),
                extract_shift(self.params[3]),
                extract_shift(self.params[4])
            ),
            _ => format!(
                "{}({:?})",
                self.variant,
                self.params
                    .iter()
                    .map(|&v| extract_shift(v))
                    .collect::<Vec<_>>()
            ),
        }
    }
}

impl From<GraphTypeInternal> for GraphType {
    fn from(gt: GraphTypeInternal) -> Self {
        match gt {
            GraphTypeInternal::Adder(a, b) => GraphType {
                variant: "Adder".to_string(),
                params: vec![a, b],
            },
            GraphTypeInternal::Subtractor(a, b) => GraphType {
                variant: "Subtractor".to_string(),
                params: vec![a, b],
            },
            GraphTypeInternal::Cascade(a, b) => GraphType {
                variant: "Cascade".to_string(),
                params: vec![a, b],
            },
            GraphTypeInternal::Leapfrog4_1(a, b, c, d) => GraphType {
                variant: "Leapfrog4_1".to_string(),
                params: vec![a, b, c, d],
            },
            GraphTypeInternal::Leapfrog4_2(a, b, c, d) => GraphType {
                variant: "Leapfrog4_2".to_string(),
                params: vec![a, b, c, d],
            },
            GraphTypeInternal::Leapfrog4_3(a, b, c, d) => GraphType {
                variant: "Leapfrog4_3".to_string(),
                params: vec![a, b, c, d],
            },
            GraphTypeInternal::Leapfrog4_4(a, b, c, d) => GraphType {
                variant: "Leapfrog4_4".to_string(),
                params: vec![a, b, c, d],
            },
            GraphTypeInternal::Leapfrog5_1(a, b, c, d, e) => GraphType {
                variant: "Leapfrog5_1".to_string(),
                params: vec![a, b, c, d, e],
            },
            GraphTypeInternal::Leapfrog5_2(a, b, c, d, e) => GraphType {
                variant: "Leapfrog5_2".to_string(),
                params: vec![a, b, c, d, e],
            },
            GraphTypeInternal::Leapfrog5_3(a, b, c, d, e) => GraphType {
                variant: "Leapfrog5_3".to_string(),
                params: vec![a, b, c, d, e],
            },
            GraphTypeInternal::Leapfrog5_4(a, b, c, d, e) => GraphType {
                variant: "Leapfrog5_4".to_string(),
                params: vec![a, b, c, d, e],
            },
        }
    }
}

/// Deserialization with varint decoding
fn deserialize_graph_types(data: &[u8]) -> Result<Vec<Vec<GraphType>>, String> {
    let mut remaining = data;

    // Read number of entries
    let (count, rest) =
        varint_decode::usize(remaining).map_err(|e| format!("Failed to decode count: {}", e))?;
    remaining = rest;

    let mut result = Vec::with_capacity(count);

    for _ in 0..count {
        // Read length of this Vec
        let (vec_len, rest) = varint_decode::usize(remaining)
            .map_err(|e| format!("Failed to decode vec length: {}", e))?;
        remaining = rest;

        let mut type_vec = Vec::with_capacity(vec_len);

        for _ in 0..vec_len {
            if remaining.is_empty() {
                return Err("Unexpected end of data".to_string());
            }

            let variant_tag = remaining[0];
            remaining = &remaining[1..];

            let graph_type = match variant_tag {
                0 => {
                    let (a, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (b, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    GraphType {
                        variant: "Adder".to_string(),
                        params: vec![a, b],
                    }
                }
                1 => {
                    let (a, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (b, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    GraphType {
                        variant: "Subtractor".to_string(),
                        params: vec![a, b],
                    }
                }
                2 => {
                    let (a, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (b, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    GraphType {
                        variant: "Cascade".to_string(),
                        params: vec![a, b],
                    }
                }
                3 => {
                    let (a, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (b, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (c, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (d, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    GraphType {
                        variant: "Leapfrog4_1".to_string(),
                        params: vec![a, b, c, d],
                    }
                }
                4 => {
                    let (a, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (b, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (c, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (d, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    GraphType {
                        variant: "Leapfrog4_2".to_string(),
                        params: vec![a, b, c, d],
                    }
                }
                5 => {
                    let (a, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (b, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (c, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (d, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    GraphType {
                        variant: "Leapfrog4_3".to_string(),
                        params: vec![a, b, c, d],
                    }
                }
                6 => {
                    let (a, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (b, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (c, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (d, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    GraphType {
                        variant: "Leapfrog4_4".to_string(),
                        params: vec![a, b, c, d],
                    }
                }
                7 => {
                    let (a, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (b, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (c, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (d, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (e, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    GraphType {
                        variant: "Leapfrog5_1".to_string(),
                        params: vec![a, b, c, d, e],
                    }
                }
                8 => {
                    let (a, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (b, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (c, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (d, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (e, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    GraphType {
                        variant: "Leapfrog5_2".to_string(),
                        params: vec![a, b, c, d, e],
                    }
                }
                9 => {
                    let (a, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (b, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (c, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (d, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (e, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    GraphType {
                        variant: "Leapfrog5_3".to_string(),
                        params: vec![a, b, c, d, e],
                    }
                }
                10 => {
                    let (a, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (b, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (c, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (d, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    let (e, rest) = varint_decode::usize(remaining)
                        .map_err(|e| format!("Failed to decode usize: {}", e))?;
                    remaining = rest;
                    GraphType {
                        variant: "Leapfrog5_4".to_string(),
                        params: vec![a, b, c, d, e],
                    }
                }
                _ => return Err(format!("Unknown variant tag: {}", variant_tag)),
            };

            type_vec.push(graph_type);
        }

        result.push(type_vec);
    }

    Ok(result)
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

fn get_graph_types_data() -> PyResult<Vec<Vec<GraphType>>> {
    // Decompress the LZ4 data
    let decompressed = lz4_flex::decompress_size_prepended(GRAPH_TYPES_BYTES)
        .map_err(|e| PyValueError::new_err(format!("Failed to decompress: {}", e)))?;

    // Deserialize with varint decoding
    deserialize_graph_types(&decompressed)
        .map_err(|e| PyValueError::new_err(format!("Failed to deserialize: {}", e)))
}

#[pymodule]
fn constant_multiplication(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<GraphType>()?;
    m.add_function(wrap_pyfunction!(adder_cost, m)?)?;
    m.add_function(wrap_pyfunction!(info, m)?)?;
    m.add_function(wrap_pyfunction!(get_graph_types, m)?)?;
    m.add_function(wrap_pyfunction!(get_all_graph_types, m)?)?;
    Ok(())
}

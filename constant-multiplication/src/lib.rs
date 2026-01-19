// src/lib.rs
use pyo3::exceptions::{PyIndexError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyList;
use serde::{Deserialize, Serialize};
use unsigned_varint::decode as varint_decode;

// Include the data files directly at compile time
const DATA_FILE: &[u8] = include_bytes!("../adder_cost.bin");
const GRAPH_TYPES_FILE: &[u8] = include_bytes!("../graph_types.bin");

// Parse the adder cost data at compile time
const fn parse_data_header() -> usize {
    // First 8 bytes are the count
    let count = u64::from_le_bytes([
        DATA_FILE[0],
        DATA_FILE[1],
        DATA_FILE[2],
        DATA_FILE[3],
        DATA_FILE[4],
        DATA_FILE[5],
        DATA_FILE[6],
        DATA_FILE[7],
    ]) as usize;
    count // count and offset where data starts
}

const DATA_COUNT: usize = parse_data_header();
const DATA_OFFSET: usize = 8;
const GRAPH_TYPES_BYTES: &[u8] = GRAPH_TYPES_FILE;

// Compile-time validation of DATA_FILE length
const _: () = assert!(DATA_FILE.len() >= DATA_OFFSET, "DATA_FILE is too small");

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
    variant: &'static str,
    #[pyo3(get)]
    params: Vec<usize>,
}

fn extract_shift(value: usize) -> String {
    let shift = value.trailing_zeros();
    if shift == 0 {
        value.to_string()
    } else {
        let shifted = value >> shift;
        format!("{shifted} << {shift}")
    }
}

#[pymethods]
impl GraphType {
    fn __repr__(&self) -> String {
        let shifted_params: Vec<String> = self.params.iter().map(|&v| extract_shift(v)).collect();
        format!("{}({})", self.variant, shifted_params.join(", "))
    }
}

impl From<GraphTypeInternal> for GraphType {
    fn from(gt: GraphTypeInternal) -> Self {
        match gt {
            GraphTypeInternal::Adder(a, b) => GraphType {
                variant: "Adder",
                params: vec![a, b],
            },
            GraphTypeInternal::Subtractor(a, b) => GraphType {
                variant: "Subtractor",
                params: vec![a, b],
            },
            GraphTypeInternal::Cascade(a, b) => GraphType {
                variant: "Cascade",
                params: vec![a, b],
            },
            GraphTypeInternal::Leapfrog4_1(a, b, c, d) => GraphType {
                variant: "Leapfrog4_1",
                params: vec![a, b, c, d],
            },
            GraphTypeInternal::Leapfrog4_2(a, b, c, d) => GraphType {
                variant: "Leapfrog4_2",
                params: vec![a, b, c, d],
            },
            GraphTypeInternal::Leapfrog4_3(a, b, c, d) => GraphType {
                variant: "Leapfrog4_3",
                params: vec![a, b, c, d],
            },
            GraphTypeInternal::Leapfrog4_4(a, b, c, d) => GraphType {
                variant: "Leapfrog4_4",
                params: vec![a, b, c, d],
            },
            GraphTypeInternal::Leapfrog5_1(a, b, c, d, e) => GraphType {
                variant: "Leapfrog5_1",
                params: vec![a, b, c, d, e],
            },
            GraphTypeInternal::Leapfrog5_2(a, b, c, d, e) => GraphType {
                variant: "Leapfrog5_2",
                params: vec![a, b, c, d, e],
            },
            GraphTypeInternal::Leapfrog5_3(a, b, c, d, e) => GraphType {
                variant: "Leapfrog5_3",
                params: vec![a, b, c, d, e],
            },
            GraphTypeInternal::Leapfrog5_4(a, b, c, d, e) => GraphType {
                variant: "Leapfrog5_4",
                params: vec![a, b, c, d, e],
            },
        }
    }
}

/// Helper function to decode multiple usize parameters from varint-encoded data
fn decode_params(remaining: &mut &[u8], count: usize) -> Result<Vec<usize>, String> {
    let mut params = Vec::with_capacity(count);
    for _ in 0..count {
        let (param, rest) = varint_decode::usize(*remaining)
            .map_err(|e| format!("Failed to decode usize: {}", e))?;
        *remaining = rest;
        params.push(param);
    }
    Ok(params)
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
                    let params = decode_params(&mut remaining, 2)?;
                    GraphType {
                        variant: "Adder",
                        params,
                    }
                }
                1 => {
                    let params = decode_params(&mut remaining, 2)?;
                    GraphType {
                        variant: "Subtractor",
                        params,
                    }
                }
                2 => {
                    let params = decode_params(&mut remaining, 2)?;
                    GraphType {
                        variant: "Cascade",
                        params,
                    }
                }
                3 => {
                    let params = decode_params(&mut remaining, 4)?;
                    GraphType {
                        variant: "Leapfrog4_1",
                        params,
                    }
                }
                4 => {
                    let params = decode_params(&mut remaining, 4)?;
                    GraphType {
                        variant: "Leapfrog4_2",
                        params,
                    }
                }
                5 => {
                    let params = decode_params(&mut remaining, 4)?;
                    GraphType {
                        variant: "Leapfrog4_3",
                        params,
                    }
                }
                6 => {
                    let params = decode_params(&mut remaining, 4)?;
                    GraphType {
                        variant: "Leapfrog4_4",
                        params,
                    }
                }
                7 => {
                    let params = decode_params(&mut remaining, 5)?;
                    GraphType {
                        variant: "Leapfrog5_1",
                        params,
                    }
                }
                8 => {
                    let params = decode_params(&mut remaining, 5)?;
                    GraphType {
                        variant: "Leapfrog5_2",
                        params,
                    }
                }
                9 => {
                    let params = decode_params(&mut remaining, 5)?;
                    GraphType {
                        variant: "Leapfrog5_3",
                        params,
                    }
                }
                10 => {
                    let params = decode_params(&mut remaining, 5)?;
                    GraphType {
                        variant: "Leapfrog5_4",
                        params,
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

    if byte_offset >= DATA_FILE.len() - DATA_OFFSET {
        return Err(PyValueError::new_err("Data corruption"));
    }

    let mut val = (DATA_FILE[byte_offset + DATA_OFFSET] >> bit_in_byte) & 0b111;

    // Handle values that span two bytes
    if bit_in_byte > 5 && byte_offset + 1 < DATA_FILE.len() - DATA_OFFSET {
        let bits_from_next = 3 - (8 - bit_in_byte);
        val |= (DATA_FILE[byte_offset + 1 + DATA_OFFSET] & ((1 << bits_from_next) - 1))
            << (8 - bit_in_byte);
    }

    Ok(val & 0b111)
}

/// Get info about the embedded data
#[pyfunction]
fn info() -> String {
    format!(
        "Embedded data: {} elements, {} bytes packed, graph types: {} bytes compressed",
        DATA_COUNT * 2,
        DATA_FILE.len() - DATA_OFFSET,
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

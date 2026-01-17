// build.rs - Place this in the root of your project
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn pack_data(input_path: &str, output_path: &str) {
    // Read the original vector from file
    let mut f = File::open(input_path).expect("Failed to open input file");
    let mut count_bytes = [0u8; 8];
    f.read_exact(&mut count_bytes)
        .expect("Failed to read count");
    let count = u64::from_le_bytes(count_bytes) as usize;

    let mut data = Vec::new();
    f.read_to_end(&mut data).expect("Failed to read data");

    // Generate Rust code with the data embedded
    let mut out = File::create(output_path).expect("Failed to create output file");

    writeln!(out, "// Auto-generated - do not edit").unwrap();
    writeln!(out, "pub const DATA_COUNT: usize = {};", count).unwrap();
    writeln!(out, "pub const DATA_BYTES: &[u8] = &[").unwrap();

    for (i, byte) in data.iter().enumerate() {
        if i % 16 == 0 {
            write!(out, "    ").unwrap();
        }
        write!(out, "0x{:02x},", byte).unwrap();
        if i % 16 == 15 {
            writeln!(out).unwrap();
        } else {
            write!(out, " ").unwrap();
        }
    }

    writeln!(out, "\n];").unwrap();
}

fn pack_graph_types(input_path: &str, output_path: &str) {
    // Read the serialized graph types from file
    let data = std::fs::read(input_path).expect("Failed to read graph types file");

    let mut out = File::create(output_path).expect("Failed to create output file");

    writeln!(out, "// Auto-generated - do not edit").unwrap();
    writeln!(out, "pub const GRAPH_TYPES_BYTES: &[u8] = &[").unwrap();

    for (i, byte) in data.iter().enumerate() {
        if i % 16 == 0 {
            write!(out, "    ").unwrap();
        }
        write!(out, "0x{:02x},", byte).unwrap();
        if i % 16 == 15 {
            writeln!(out).unwrap();
        } else {
            write!(out, " ").unwrap();
        }
    }

    writeln!(out, "\n];").unwrap();
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("embedded_data.rs");

    // Pack the data file into Rust code
    pack_data("adder_cost.bin", dest_path.to_str().unwrap());

    // Pack the graph types data
    let graph_dest_path = Path::new(&out_dir).join("embedded_graph_types.rs");
    pack_graph_types("graph_types.bin", graph_dest_path.to_str().unwrap());

    // Rebuild if either data file changes
    println!("cargo:rerun-if-changed=adder_cost.bin");
    println!("cargo:rerun-if-changed=graph_types.bin");
}

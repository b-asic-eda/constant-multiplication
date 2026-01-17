use derive_more::Display;
use std::{iter::zip, ops::Shr};
use tracing::{Level, debug, info, span};

fn main() -> Result<()> {
    // Initialize tracing subscriber
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let _span = span!(Level::DEBUG, "main").entered();
    info!("Starting constant multiplication optimization");

    let max_bits: usize = 12;
    let max_extra_bits: usize = 2;
    let total_bits: usize = max_bits + max_extra_bits;
    let max_value: usize = (1 << (total_bits)) - 1;

    info!(
        max_bits,
        max_extra_bits, total_bits, max_value, "Configuration initialized"
    );

    let mut adder_count: Vec<u8> = vec![10; max_value as usize + 1];
    let mut adder_structures: Vec<Option<Vec<GraphType>>> = vec![None; max_value as usize + 1];
    let cost0: Vec<usize> = vec![1];
    let mut cost0_shifted: Vec<usize> = Vec::new();
    for i in 0..total_bits {
        cost0_shifted.push(1 << i);
    }
    for i in cost0_shifted.iter() {
        adder_count[*i] = 0;
    }
    adder_count[1] = 0; // Cost 0 for constant 1
    // Cost 1 combinations
    debug!("Processing cost 1 combinations");
    addsub_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost0,
        &cost0_shifted,
        1,
        max_value,
    );
    let mut cost1: Vec<usize> = Vec::new();
    for i in adder_count.iter().enumerate() {
        if *i.1 == 1 {
            cost1.push(i.0);
        }
    }
    debug!(cost1_count = cost1.len(), "Cost 1 values found");
    let mut cost1_shifted: Vec<usize> = Vec::new();
    for i in cost1.iter() {
        let mut shift = 0;
        while (i << shift) <= max_value {
            cost1_shifted.push(i << shift);
            shift += 1;
        }
    }
    debug!("Processing cost 2 combinations");
    addsub_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost1,
        &cost0_shifted,
        2,
        max_value,
    );
    addsub_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost0,
        &cost1_shifted,
        2,
        max_value,
    );
    cascade_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost1,
        &cost1,
        2,
        max_value,
    );
    let mut cost2: Vec<usize> = Vec::new();
    for i in adder_count.iter().enumerate() {
        if *i.1 == 2 {
            cost2.push(i.0);
        }
    }
    debug!(cost2_count = cost2.len(), "Cost 2 values found");

    let mut cost2_shifted: Vec<usize> = Vec::new();
    for i in cost2.iter() {
        let mut shift = 0;
        while (i << shift) <= max_value {
            cost2_shifted.push(i << shift);
            shift += 1;
        }
    }
    debug!("Processing cost 3 combinations");
    addsub_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost2,
        &cost0_shifted,
        3,
        max_value,
    );
    addsub_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost1,
        &cost1_shifted,
        3,
        max_value,
    );
    addsub_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost0,
        &cost2_shifted,
        3,
        max_value,
    );
    cascade_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost1,
        &cost2,
        3,
        max_value,
    );

    let mut cost3: Vec<usize> = Vec::new();
    for i in adder_count.iter().enumerate() {
        if *i.1 == 3 {
            cost3.push(i.0);
        }
    }
    debug!(cost3_count = cost3.len(), "Cost 3 values found");
    let mut cost3_shifted: Vec<usize> = Vec::new();
    for i in cost3.iter() {
        let mut shift = 0;
        while (i << shift) <= max_value {
            cost3_shifted.push(i << shift);
            shift += 1;
        }
    }
    debug!("Processing cost 4 combinations");
    addsub_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost3,
        &cost0_shifted,
        4,
        max_value,
    );
    addsub_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost2,
        &cost1_shifted,
        4,
        max_value,
    );
    addsub_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost1,
        &cost2_shifted,
        4,
        max_value,
    );
    addsub_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost0,
        &cost3_shifted,
        4,
        max_value,
    );
    cascade_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost1,
        &cost3,
        4,
        max_value,
    );
    cascade_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost2,
        &cost2,
        4,
        max_value,
    );
    leapfrog_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost1_shifted,
        &cost0_shifted,
        &cost0_shifted,
        &cost0_shifted,
        &cost1_shifted,
        4,
        max_value,
    );

    let mut cost4: Vec<usize> = Vec::new();
    for i in adder_count.iter().enumerate() {
        if *i.1 == 4 {
            cost4.push(i.0);
        }
    }
    debug!(cost4_count = cost4.len(), "Cost 4 values found");

    let mut cost4_shifted: Vec<usize> = Vec::new();
    for i in cost4.iter() {
        let mut shift = 0;
        while (i << shift) <= max_value {
            cost4_shifted.push(i << shift);
            shift += 1;
        }
    }
    if total_bits > 12 {
        debug!("Processing cost 5 combinations");
        addsub_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost4,
            &cost0_shifted,
            5,
            max_value,
        );
        addsub_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost3,
            &cost1_shifted,
            5,
            max_value,
        );
        addsub_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost2,
            &cost2_shifted,
            5,
            max_value,
        );
        addsub_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost1,
            &cost3_shifted,
            5,
            max_value,
        );
        addsub_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost0,
            &cost4_shifted,
            5,
            max_value,
        );
        cascade_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost1,
            &cost4,
            5,
            max_value,
        );
        cascade_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost2,
            &cost3,
            5,
            max_value,
        );
        leapfrog_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost1_shifted,
            &cost0_shifted,
            &cost1_shifted,
            &cost0_shifted,
            &cost1_shifted,
            5,
            max_value,
        );

        leapfrog_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost2_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost1_shifted,
            5,
            max_value,
        );

        leapfrog_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost1_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost2_shifted,
            5,
            max_value,
        );
    }
    if total_bits > 19 {
        let mut cost5: Vec<usize> = Vec::new();
        for i in adder_count.iter().enumerate() {
            if *i.1 == 5 {
                cost5.push(i.0);
            }
        }
        debug!(cost5_count = cost5.len(), "Cost 5 values found");

        let mut cost5_shifted: Vec<usize> = Vec::new();
        for i in cost5.iter() {
            let mut shift = 0;
            while (i << shift) <= max_value {
                cost5_shifted.push(i << shift);
                shift += 1;
            }
        }
        debug!("Processing cost 6 combinations");
        addsub_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost5,
            &cost0_shifted,
            6,
            max_value,
        );
        addsub_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost4,
            &cost1_shifted,
            6,
            max_value,
        );
        addsub_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost3,
            &cost2_shifted,
            6,
            max_value,
        );
        addsub_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost2,
            &cost3_shifted,
            6,
            max_value,
        );
        addsub_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost1,
            &cost4_shifted,
            6,
            max_value,
        );
        addsub_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost0,
            &cost5_shifted,
            6,
            max_value,
        );
        cascade_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost1,
            &cost5,
            6,
            max_value,
        );
        cascade_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost2,
            &cost4,
            6,
            max_value,
        );
        cascade_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost3,
            &cost3,
            6,
            max_value,
        );
        leapfrog_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost2_shifted,
            &cost0_shifted,
            &cost1_shifted,
            &cost0_shifted,
            &cost1_shifted,
            6,
            max_value,
        );

        leapfrog_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost3_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost1_shifted,
            6,
            max_value,
        );

        leapfrog_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost2_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost2_shifted,
            6,
            max_value,
        );

        leapfrog_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost1_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost3_shifted,
            6,
            max_value,
        );

        leapfrog_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost1_shifted,
            &cost0_shifted,
            &cost2_shifted,
            &cost0_shifted,
            &cost1_shifted,
            6,
            max_value,
        );

        leapfrog_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost1_shifted,
            &cost0_shifted,
            &cost1_shifted,
            &cost0_shifted,
            &cost2_shifted,
            6,
            max_value,
        );
    }

    info!("Packing and saving data");
    let (packed, count) = pack_sparse_vector(&adder_count);
    save_packed_data("data.bin", &packed, count)?;

    info!("Packed {} odd values into {} bytes", count, packed.len());
    info!("Original size: {} bytes", adder_count.len());
    info!(
        "Compression ratio: {:.1}%",
        (packed.len() as f64 / adder_count.len() as f64) * 100.0
    );

    debug!("Generating final results");
    // Print results
    let mut result_count = 0;
    let mut missing_count = 0;
    for c in zip(adder_count.iter(), adder_structures.iter()).enumerate() {
        if let Some(structures) = c.1.1 {
            result_count += 1;
            /* println!(
                "Value: {}, Cost: {}, Structures: {:?}",
                c.0, c.1.0, structures
            ); */
        }
    }

    for c in adder_structures.iter().enumerate() {
        if c.1.is_none() && c.0 % 2 == 1 && c.0 != 1 {
            missing_count += 1;
            println!("Missing: {}", c.0);
        }
    }

    info!(result_count, missing_count, "Computation complete");
    Ok(())
}

#[derive(Debug, Display, Clone)]
enum GraphType {
    #[display("Adder({_0}, {_1})")]
    Adder(usize, usize),
    #[display("Subtractor({_0}, {_1})")]
    Subtractor(usize, usize),
    #[display("Cascade({_0}, {_1})")]
    Cascade(usize, usize),
    #[display("Leapfrog1({_0}, {_1}, {_2}, {_3}, {_4})")]
    Leapfrog1(usize, usize, usize, usize, usize),
    #[display("Leapfrog2({_0}, {_1}, {_2}, {_3}, {_4})")]
    Leapfrog2(usize, usize, usize, usize, usize),
    #[display("Leapfrog3({_0}, {_1}, {_2}, {_3}, {_4})")]
    Leapfrog3(usize, usize, usize, usize, usize),
    #[display("Leapfrog4({_0}, {_1}, {_2}, {_3}, {_4})")]
    Leapfrog4(usize, usize, usize, usize, usize),
}

fn addsub_combinations(
    adder_count: &mut Vec<u8>,
    adder_structures: &mut Vec<Option<Vec<GraphType>>>,
    terms: &Vec<usize>,
    terms_shifted: &Vec<usize>,
    adder_cost: u8,
    max_value: usize,
) {
    debug!(
        terms1_count = terms.len(),
        terms2_count = terms_shifted.len(),
        cost = adder_cost,
        "addsub_combinations: starting"
    );
    for &term1 in terms.iter() {
        for &term2 in terms_shifted.iter() {
            let sum = findodd(term1 + term2);
            if sum <= max_value && adder_count[sum] >= adder_cost {
                adder_count[sum] = adder_cost;
                add_graph_type(adder_structures, sum, GraphType::Adder(term1, term2));
            }
            let diff = findodd(term1.abs_diff(term2));

            if diff <= max_value && diff != 0 && adder_count[diff] > adder_cost {
                adder_count[diff] = adder_cost;
                add_graph_type(adder_structures, diff, GraphType::Subtractor(term1, term2));
            }
        }
    }
}

fn cascade_combinations(
    adder_count: &mut Vec<u8>,
    adder_structures: &mut Vec<Option<Vec<GraphType>>>,
    terms1: &Vec<usize>,
    terms2: &Vec<usize>,
    adder_cost: u8,
    max_value: usize,
) {
    debug!(
        terms1_count = terms1.len(),
        terms2_count = terms2.len(),
        cost = adder_cost,
        "cascade_combinations: starting"
    );
    for &term1 in terms1.iter() {
        for &term2 in terms2.iter() {
            let cascade = term1 * term2;
            if cascade <= max_value && adder_count[cascade] >= adder_cost {
                adder_count[cascade] = adder_cost;
                add_graph_type(adder_structures, cascade, GraphType::Cascade(term1, term2));
            }
        }
    }
}

fn leapfrog_combinations(
    adder_count: &mut Vec<u8>,
    adder_structures: &mut Vec<Option<Vec<GraphType>>>,
    terms1: &Vec<usize>,
    terms2: &Vec<usize>,
    terms3: &Vec<usize>,
    terms4: &Vec<usize>,
    terms5: &Vec<usize>,
    adder_cost: u8,
    max_value: usize,
) {
    debug!(
        terms1_count = terms1.len(),
        terms2_count = terms2.len(),
        terms3_count = terms3.len(),
        terms4_count = terms4.len(),
        terms5_count = terms5.len(),
        cost = adder_cost,
        "leapfrog_combinations: starting"
    );
    let max_value_u128 = max_value as u128;

    for &term1 in terms1.iter() {
        for &term2 in terms2.iter() {
            if term1.is_multiple_of(2) && term2.is_multiple_of(2) {
                continue;
            }
            for &term3 in terms3.iter() {
                for &term4 in terms4.iter() {
                    if term2.is_multiple_of(2) && term3.is_multiple_of(2) && term4.is_multiple_of(2)
                    {
                        continue;
                    }
                    for &term5 in terms5.iter() {
                        if term4.is_multiple_of(2) && term5.is_multiple_of(2) {
                            continue;
                        }

                        let t1 = term1 as u128;
                        let t2 = term2 as u128;
                        let t3 = term3 as u128;
                        let t4 = term4 as u128;
                        let t5 = term5 as u128;

                        let leapfrog = findodd_u128(t5 * (t1 * t3 + t2) + t1 * t4);
                        if leapfrog != 0
                            && leapfrog <= max_value_u128
                            && adder_count[leapfrog as usize] >= adder_cost
                        {
                            adder_count[leapfrog as usize] = adder_cost;
                            add_graph_type(
                                adder_structures,
                                leapfrog as usize,
                                GraphType::Leapfrog1(term1, term2, term3, term4, term5),
                            );
                        }

                        let leapfrog = findodd_u128((t5 * (t1 * t3 + t2)).abs_diff(t1 * t4));
                        if leapfrog != 0
                            && leapfrog <= max_value_u128
                            && adder_count[leapfrog as usize] >= adder_cost
                        {
                            adder_count[leapfrog as usize] = adder_cost;
                            add_graph_type(
                                adder_structures,
                                leapfrog as usize,
                                GraphType::Leapfrog2(term1, term2, term3, term4, term5),
                            );
                        }

                        let leapfrog = findodd_u128(t5 * ((t1 * t3).abs_diff(t2)) + t1 * t4);
                        if leapfrog != 0
                            && leapfrog <= max_value_u128
                            && adder_count[leapfrog as usize] >= adder_cost
                        {
                            adder_count[leapfrog as usize] = adder_cost;
                            add_graph_type(
                                adder_structures,
                                leapfrog as usize,
                                GraphType::Leapfrog3(term1, term2, term3, term4, term5),
                            );
                        }

                        let leapfrog =
                            findodd_u128((t5 * (t1 * t3).abs_diff(t2)).abs_diff(t1 * t4));
                        if leapfrog != 0
                            && leapfrog <= max_value_u128
                            && adder_count[leapfrog as usize] >= adder_cost
                        {
                            adder_count[leapfrog as usize] = adder_cost;
                            add_graph_type(
                                adder_structures,
                                leapfrog as usize,
                                GraphType::Leapfrog4(term1, term2, term3, term4, term5),
                            );
                        }
                    }
                }
            }
        }
    }
}

fn add_graph_type(
    adder_structures: &mut Vec<Option<Vec<GraphType>>>,
    result: usize,
    graph_type: GraphType,
) {
    if let Some(structure) = &mut adder_structures[result] {
        structure.push(graph_type);
    } else {
        adder_structures[result] = Some(vec![graph_type]);
    }
}

#[inline]
fn findodd(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    let shift = n.trailing_zeros();
    n.shr(shift)
}

#[inline]
fn findodd_u128(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    let shift = n.trailing_zeros();
    n >> shift
}

// prepare_data.rs - Run this once to create data.bin
use std::fs::File;
use std::io::{Result, Write};

fn pack_sparse_vector(sparse_vec: &[u8]) -> (Vec<u8>, usize) {
    let odd_count = (sparse_vec.len() + 1) / 2;
    let mut packed = Vec::new();
    let mut bit_buffer = 0u32;
    let mut bits_in_buffer = 0;

    // Extract and pack values at odd indices
    for i in (1..sparse_vec.len()).step_by(2) {
        let val = sparse_vec[i] & 0b111; // Ensure only 3 bits
        bit_buffer |= (val as u32) << bits_in_buffer;
        bits_in_buffer += 3;

        // Flush complete bytes
        while bits_in_buffer >= 8 {
            packed.push(bit_buffer as u8);
            bit_buffer >>= 8;
            bits_in_buffer -= 8;
        }
    }

    // Flush remaining bits
    if bits_in_buffer > 0 {
        packed.push(bit_buffer as u8);
    }

    (packed, odd_count)
}

fn save_packed_data(path: &str, packed: &[u8], count: usize) -> Result<()> {
    let mut f = File::create(path)?;
    // Write count as u64 little-endian
    f.write_all(&(count as u64).to_le_bytes())?;
    // Write packed data
    f.write_all(packed)?;
    Ok(())
}

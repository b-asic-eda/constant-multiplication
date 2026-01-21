use std::{iter::zip, ops::Shr};
use tracing::{Level, debug, info, warn};
use unsigned_varint::encode as varint_encode;

type AdderStructures = [Option<Vec<GraphType>>];

fn main() -> Result<()> {
    // Initialize tracing subscriber
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    info!("Starting constant multiplication optimization");

    let max_bits: usize = 19;
    let max_extra_bits: usize = 2;
    let table_max: usize = 1 << max_bits;
    let max_value: usize = 1 << (max_bits + max_extra_bits);

    let print_structures = false;
    let print_missing = true;

    info!(
        max_bits,
        max_extra_bits, table_max, max_value, "Configuration initialized"
    );

    let mut adder_count: Vec<u8> = vec![7; table_max + 1];
    let mut adder_structures: Vec<Option<Vec<GraphType>>> = vec![None; table_max + 1];
    let cost0: Vec<usize> = vec![1];
    let cost0_shifted = create_shifted_variants(&cost0, max_value);
    adder_count[1] = 0; // Cost 0 for constant 1
    // Cost 1 combinations
    debug!("Processing cost 1 combinations");
    addsub_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost0,
        &cost0_shifted,
        1,
        table_max,
    );
    let cost1 = extract_cost_values(&adder_count, 1);
    debug!(cost1_count = cost1.len(), "Cost 1 values found");
    let cost1_shifted = create_shifted_variants(&cost1, max_value);
    debug!("Processing cost 2 combinations");
    addsub_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost1,
        &cost0_shifted,
        2,
        table_max,
    );
    addsub_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost0,
        &cost1_shifted,
        2,
        table_max,
    );
    cascade_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost1,
        &cost1,
        2,
        table_max,
        true,
    );
    let cost2 = extract_cost_values(&adder_count, 2);
    debug!(cost2_count = cost2.len(), "Cost 2 values found");
    let cost2_shifted = create_shifted_variants(&cost2, max_value);
    debug!("Processing cost 3 combinations");
    addsub_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost2,
        &cost0_shifted,
        3,
        table_max,
    );
    addsub_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost1,
        &cost1_shifted,
        3,
        table_max,
    );
    addsub_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost0,
        &cost2_shifted,
        3,
        table_max,
    );
    cascade_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost1,
        &cost2,
        3,
        table_max,
        false,
    );

    let cost3 = extract_cost_values(&adder_count, 3);
    debug!(cost3_count = cost3.len(), "Cost 3 values found");
    let cost3_shifted = create_shifted_variants(&cost3, max_value);
    debug!("Processing cost 4 combinations");
    addsub_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost3,
        &cost0_shifted,
        4,
        table_max,
    );
    addsub_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost2,
        &cost1_shifted,
        4,
        table_max,
    );
    addsub_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost1,
        &cost2_shifted,
        4,
        table_max,
    );
    addsub_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost0,
        &cost3_shifted,
        4,
        table_max,
    );
    cascade_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost1,
        &cost3,
        4,
        table_max,
        false,
    );
    cascade_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost2,
        &cost2,
        4,
        table_max,
        true,
    );
    leapfrog4_combinations(
        &mut adder_count,
        &mut adder_structures,
        &cost1_shifted,
        &cost0_shifted,
        &cost0_shifted,
        &cost1_shifted,
        4,
        table_max,
    );

    let cost4 = extract_cost_values(&adder_count, 4);
    debug!(cost4_count = cost4.len(), "Cost 4 values found");
    let cost4_shifted = create_shifted_variants(&cost4, max_value);
    if max_bits > 12 {
        debug!("Processing cost 5 combinations");
        addsub_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost4,
            &cost0_shifted,
            5,
            table_max,
        );
        addsub_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost3,
            &cost1_shifted,
            5,
            table_max,
        );
        addsub_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost2,
            &cost2_shifted,
            5,
            table_max,
        );
        addsub_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost1,
            &cost3_shifted,
            5,
            table_max,
        );
        addsub_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost0,
            &cost4_shifted,
            5,
            table_max,
        );
        cascade_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost1,
            &cost4,
            5,
            table_max,
            false,
        );
        cascade_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost2,
            &cost3,
            5,
            table_max,
            false,
        );
        leapfrog5_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost1_shifted,
            &cost0_shifted,
            &cost1,
            &cost0_shifted,
            &cost1_shifted,
            5,
            table_max,
        );

        leapfrog4_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost2_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost1_shifted,
            5,
            table_max,
        );

        leapfrog4_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost1_shifted,
            &cost1_shifted,
            &cost0_shifted,
            &cost1_shifted,
            5,
            table_max,
        );

        leapfrog4_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost1_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost2_shifted,
            5,
            table_max,
        );
        leapfrog7_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost1_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost1_shifted,
            5,
            table_max,
        );
    }
    if max_bits > 19 {
        let cost5 = extract_cost_values(&adder_count, 5);
        debug!(cost5_count = cost5.len(), "Cost 5 values found");
        let cost5_shifted = create_shifted_variants(&cost5, max_value);
        debug!("Processing cost 6 combinations");
        addsub_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost5,
            &cost0_shifted,
            6,
            table_max,
        );
        addsub_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost4,
            &cost1_shifted,
            6,
            table_max,
        );
        addsub_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost3,
            &cost2_shifted,
            6,
            table_max,
        );
        addsub_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost2,
            &cost3_shifted,
            6,
            table_max,
        );
        addsub_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost1,
            &cost4_shifted,
            6,
            table_max,
        );
        addsub_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost0,
            &cost5_shifted,
            6,
            table_max,
        );
        cascade_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost1,
            &cost5,
            6,
            table_max,
            false,
        );
        cascade_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost2,
            &cost4,
            6,
            table_max,
            false,
        );
        cascade_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost3,
            &cost3,
            6,
            table_max,
            true,
        );
        leapfrog5_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost2_shifted,
            &cost0_shifted,
            &cost1,
            &cost0_shifted,
            &cost1_shifted,
            6,
            table_max,
        );

        leapfrog4_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost3_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost1_shifted,
            6,
            table_max,
        );

        leapfrog4_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost2_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost2_shifted,
            6,
            table_max,
        );

        leapfrog4_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost1_shifted,
            &cost1_shifted,
            &cost0_shifted,
            &cost2_shifted,
            6,
            table_max,
        );

        leapfrog4_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost1_shifted,
            &cost1_shifted,
            &cost1_shifted,
            &cost1_shifted,
            6,
            table_max,
        );

        leapfrog4_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost2_shifted,
            &cost1_shifted,
            &cost0_shifted,
            &cost1_shifted,
            6,
            table_max,
        );

        leapfrog5_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost1_shifted,
            &cost0_shifted,
            &cost2,
            &cost0_shifted,
            &cost1_shifted,
            6,
            table_max,
        );

        /* leapfrog5_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost1_shifted,
            &cost0_shifted,
            &cost1_shifted,
            &cost0_shifted,
            &cost2_shifted,
            6,
            table_max,
        ); */
        leapfrog7_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost2_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost1_shifted,
            6,
            table_max,
        );
        leapfrog7_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost1_shifted,
            &cost1_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost1_shifted,
            6,
            table_max,
        );
        leapfrog7_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost1_shifted,
            &cost0_shifted,
            &cost1_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost1_shifted,
            6,
            table_max,
        );
        leapfrog7_combinations(
            &mut adder_count,
            &mut adder_structures,
            &cost1_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost1_shifted,
            &cost0_shifted,
            &cost0_shifted,
            &cost1_shifted,
            6,
            table_max,
        );
    }

    info!("Packing and saving data");
    let (packed, count) = pack_sparse_vector(&adder_count);
    save_packed_data("constant-multiplication/adder_cost.bin", &packed, count)?;

    info!("Packed {} odd values into {} bytes", count, packed.len());
    info!("Original size: {} bytes", adder_count.len());
    info!(
        "Compression ratio: {:.1}%",
        (packed.len() as f64 / adder_count.len() as f64) * 100.0
    );

    info!("Saving graph types");
    let mut graph_types: Vec<Vec<GraphType>> = Vec::new();
    for (i, structure) in adder_structures.iter().enumerate() {
        if i % 2 == 1 && i <= table_max {
            if let Some(types) = structure {
                graph_types.push(types.clone());
            } else {
                graph_types.push(vec![]);
                warn!("No graph types found for value {}", i);
            }
        }
    }

    // Serialize with varint encoding
    let serialized = serialize_graph_types(&graph_types);

    info!("Graph types serialization:");
    info!("  Varint-encoded: {} bytes", serialized.len());

    // Compress with lz4
    let compressed = lz4_flex::compress_prepend_size(&serialized);

    info!("  Compressed (LZ4): {} bytes", compressed.len());
    info!(
        "  Compression ratio: {:.2}%",
        (compressed.len() as f64 / serialized.len() as f64) * 100.0
    );

    std::fs::write("constant-multiplication/graph_types.bin", &compressed)?;

    debug!("Generating final results");
    // Print results
    let mut result_count = 0;
    let mut missing_count = 0;
    for c in zip(adder_count.iter(), adder_structures.iter()).enumerate() {
        if let Some(structures) = c.1.1 {
            result_count += 1;
            if print_structures {
                println!(
                    "Value: {}, Cost: {}, Structures: {:?}",
                    c.0, c.1.0, structures
                );
            }
        }
    }

    for c in adder_structures.iter().enumerate() {
        if c.1.is_none() && c.0 % 2 == 1 && c.0 != 1 && c.0 <= table_max {
            missing_count += 1;
            if print_missing {
                println!("Missing: {}", c.0);
            }
        }
    }

    info!(result_count, missing_count, "Computation complete");
    Ok(())
}

#[derive(Debug, Clone)]
enum GraphType {
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
    Leapfrog7_1(usize, usize, usize, usize, usize, usize, usize),
    Leapfrog7_2(usize, usize, usize, usize, usize, usize, usize),
    Leapfrog7_3(usize, usize, usize, usize, usize, usize, usize),
    Leapfrog7_4(usize, usize, usize, usize, usize, usize, usize),
    Leapfrog7_5(usize, usize, usize, usize, usize, usize, usize),
    Leapfrog7_6(usize, usize, usize, usize, usize, usize, usize),
    Leapfrog7_7(usize, usize, usize, usize, usize, usize, usize),
    Leapfrog7_8(usize, usize, usize, usize, usize, usize, usize),
}

fn addsub_combinations(
    adder_count: &mut [u8],
    adder_structures: &mut AdderStructures,
    terms: &[usize],
    terms_shifted: &[usize],
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
                if term1 <= term2 {
                    add_graph_type(adder_structures, sum, GraphType::Adder(term1, term2));
                } else {
                    add_graph_type(adder_structures, sum, GraphType::Adder(term2, term1));
                }
            }
            let diff = findodd(term1.abs_diff(term2));

            if diff <= max_value && diff != 0 && adder_count[diff] > adder_cost {
                adder_count[diff] = adder_cost;
                if term1 >= term2 {
                    add_graph_type(adder_structures, diff, GraphType::Subtractor(term1, term2));
                } else {
                    add_graph_type(adder_structures, diff, GraphType::Subtractor(term2, term1));
                }
            }
        }
    }
}

fn cascade_combinations(
    adder_count: &mut [u8],
    adder_structures: &mut AdderStructures,
    terms1: &[usize],
    terms2: &[usize],
    adder_cost: u8,
    max_value: usize,
    same_terms: bool,
) {
    debug!(
        terms1_count = terms1.len(),
        terms2_count = terms2.len(),
        cost = adder_cost,
        "cascade_combinations: starting"
    );
    for &term1 in terms1.iter() {
        for &term2 in terms2.iter() {
            if same_terms && term2 < term1 {
                continue;
            }
            let cascade = term1 * term2;
            if cascade <= max_value && adder_count[cascade] >= adder_cost {
                adder_count[cascade] = adder_cost;
                if term1 <= term2 {
                    add_graph_type(adder_structures, cascade, GraphType::Cascade(term1, term2));
                } else {
                    add_graph_type(adder_structures, cascade, GraphType::Cascade(term2, term1));
                }
            }
        }
    }
}

fn leapfrog5_combinations(
    adder_count: &mut [u8],
    adder_structures: &mut AdderStructures,
    terms1: &[usize],
    terms2: &[usize],
    terms3: &[usize],
    terms4: &[usize],
    terms5: &[usize],
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
        "leapfrog5_combinations: starting"
    );
    let max_value_u128 = max_value as u128;

    for &term1 in terms1.iter() {
        let t1 = term1 as u128;
        let t1_odd = findodd_u128(t1);
        for &term2 in terms2.iter() {
            if term1.is_multiple_of(2) && term2.is_multiple_of(2) {
                continue;
            }
            let t2 = term2 as u128;
            for &term3 in terms3.iter() {
                if findodd(term3) == 1
                    || ((term1.is_multiple_of(2) || term2.is_multiple_of(2))
                        && term3.is_multiple_of(2))
                {
                    continue;
                }
                let t3 = term3 as u128;
                for &term4 in terms4.iter() {
                    if (term1.is_multiple_of(2) || term3.is_multiple_of(2))
                        && term4.is_multiple_of(2)
                    {
                        continue;
                    }
                    let t4 = term4 as u128;
                    for &term5 in terms5.iter() {
                        if (term3.is_multiple_of(2) || term4.is_multiple_of(2))
                            && term5.is_multiple_of(2)
                        {
                            continue;
                        }

                        let t5 = term5 as u128;
                        let t5_odd = findodd_u128(t5);
                        let leapfrog = findodd_u128(t5 * (t1 * t3 + t2) + t1 * t4);
                        if leapfrog != 0
                            && leapfrog <= max_value_u128
                            && adder_count[leapfrog as usize] >= adder_cost
                        {
                            // Symmetric case when t2 = t4
                            // (t5 * (t1 * t3 + t2) + t1 * t4 == (t1 * (t5 * t3 + t4) + t5 * t2
                            if t2 != t4 {
                                adder_count[leapfrog as usize] = adder_cost;
                                add_graph_type(
                                    adder_structures,
                                    leapfrog as usize,
                                    GraphType::Leapfrog5_1(term1, term2, term3, term4, term5),
                                );
                            }
                        }

                        let leapfrog = findodd_u128((t5 * (t1 * t3 + t2)).abs_diff(t1 * t4));
                        if leapfrog != 0
                            && leapfrog <= max_value_u128
                            && adder_count[leapfrog as usize] >= adder_cost
                        {
                            // Symmetric case with 5_3 or 5_4 when t1 and t5 are odd
                            if !(t1 == t1_odd && t5 == t5_odd && t1 >= t5) {
                                adder_count[leapfrog as usize] = adder_cost;
                                add_graph_type(
                                    adder_structures,
                                    leapfrog as usize,
                                    GraphType::Leapfrog5_2(term1, term2, term3, term4, term5),
                                );
                            }
                        }

                        let leapfrog = findodd_u128(t5 * ((t1 * t3).abs_diff(t2)) + t1 * t4);
                        if leapfrog != 0
                            && leapfrog <= max_value_u128
                            && adder_count[leapfrog as usize] >= adder_cost
                        {
                            if !(t2 == 1 && t4 == 1)
                                && !(t2 == t4 && t4 > t3 * t5 && t1 >= t5)
                                && !(t1 == t1_odd && t5 == t5_odd && t1 >= t5)
                            {
                                // Symmetric case (with 5_2)
                                // t5 * ((t1 * t3 - 1)) + t1 * 1 = (t1 * (t5 * t3 + 1)) - t5 * 1)
                                // Symmetric case when t2 = t4 > t3 * t5
                                // t5 * ((t2 - t1 * t3)) + t1 * t4 = t1 * (t4 - t5 * t3) - t1 * t2
                                // Symmetric case with 5_2 when t1 and t5 are odd
                                adder_count[leapfrog as usize] = adder_cost;
                                add_graph_type(
                                    adder_structures,
                                    leapfrog as usize,
                                    GraphType::Leapfrog5_3(term1, term2, term3, term4, term5),
                                );
                            }
                        }

                        let leapfrog =
                            findodd_u128((t5 * (t1 * t3).abs_diff(t2)).abs_diff(t1 * t4));
                        if leapfrog != 0
                            && leapfrog <= max_value_u128
                            && adder_count[leapfrog as usize] >= adder_cost
                        {
                            // Symmetric case with 5_2 when t1 and t5 are odd
                            if !(t1 == t1_odd && t5 == t5_odd && t1 >= t5)
                                && !(term1 >= term5
                                    && term2 == 1
                                    && term4 == 1
                                    && term1.is_multiple_of(2)
                                    && term5.is_multiple_of(2))
                            {
                                adder_count[leapfrog as usize] = adder_cost;
                                add_graph_type(
                                    adder_structures,
                                    leapfrog as usize,
                                    GraphType::Leapfrog5_4(term1, term2, term3, term4, term5),
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

fn leapfrog4_combinations(
    adder_count: &mut [u8],
    adder_structures: &mut AdderStructures,
    terms1: &[usize],
    terms2: &[usize],
    terms4: &[usize],
    terms5: &[usize],
    adder_cost: u8,
    max_value: usize,
) {
    debug!(
        terms1_count = terms1.len(),
        terms2_count = terms2.len(),
        terms4_count = terms4.len(),
        terms5_count = terms5.len(),
        cost = adder_cost,
        "leapfrog4_combinations: starting"
    );
    let max_value_u128 = max_value as u128;

    for &term1 in terms1.iter() {
        let t1 = term1 as u128;
        for &term2 in terms2.iter() {
            if term1.is_multiple_of(2) && term2.is_multiple_of(2) {
                continue;
            }
            let t2 = term2 as u128;
            for &term4 in terms4.iter() {
                if term1.is_multiple_of(2) && term4.is_multiple_of(2) {
                    continue;
                }
                let t4 = term4 as u128;

                for &term5 in terms5.iter() {
                    if term4.is_multiple_of(2) && term5.is_multiple_of(2) {
                        continue;
                    }

                    let t5 = term5 as u128;

                    let leapfrog = findodd_u128(t5 * (t1 + t2) + t1 * t4);
                    if leapfrog != 0
                        && leapfrog <= max_value_u128
                        && adder_count[leapfrog as usize] >= adder_cost
                    {
                        // Symmetric case
                        // t5 * (t1 + t2) + t1 * t4 == t1 * (t5 + t4) + t5 * t2
                        // Symmetric case when t2 == t4
                        // t5 * (t1 + 1) + t1 * 1 == t1 * (t5 + 1) + t5 * 1
                        if !(t1 == findodd_u128(t1) && t5 == findodd_u128(t5) && t1 >= t5)
                            && !(t2 == t4 && t1 >= t5)
                        {
                            adder_count[leapfrog as usize] = adder_cost;
                            add_graph_type(
                                adder_structures,
                                leapfrog as usize,
                                GraphType::Leapfrog4_1(term1, term2, term4, term5),
                            );
                        }
                    }

                    let leapfrog = findodd_u128((t5 * (t1 + t2)).abs_diff(t1 * t4));
                    if leapfrog != 0
                        && leapfrog <= max_value_u128
                        && adder_count[leapfrog as usize] >= adder_cost
                    {
                        // Symmetric case (with 4_3)
                        // t5 * (t1 - t2) + t1 * t4 == t1 * (t5 + t4) - t5 * t2
                        if !(t1 == findodd_u128(t1) && t5 == findodd_u128(t5) && t1 >= t5) {
                            adder_count[leapfrog as usize] = adder_cost;
                            add_graph_type(
                                adder_structures,
                                leapfrog as usize,
                                GraphType::Leapfrog4_2(term1, term2, term4, term5),
                            );
                        }
                    }

                    let leapfrog = findodd_u128(t5 * (t1.abs_diff(t2)) + t1 * t4);
                    if leapfrog != 0
                        && leapfrog <= max_value_u128
                        && adder_count[leapfrog as usize] >= adder_cost
                    {
                        // Symmetric case (with 4_2)
                        // t5 * (t1 - t2) + t1 * t4 == t1 * (t5 + t4) - t5 * t2
                        if !(t1 == findodd_u128(t1) && t5 == findodd_u128(t5) && t1 >= t5)
                            && !(term2 == 1
                                && term4 == 1
                                && term1.is_multiple_of(2)
                                && term5.is_multiple_of(2))
                        {
                            adder_count[leapfrog as usize] = adder_cost;
                            add_graph_type(
                                adder_structures,
                                leapfrog as usize,
                                GraphType::Leapfrog4_3(term1, term2, term4, term5),
                            );
                        }
                    }

                    let leapfrog = findodd_u128((t5 * (t1.abs_diff(t2))).abs_diff(t1 * t4));
                    if leapfrog != 0
                        && leapfrog <= max_value_u128
                        && adder_count[leapfrog as usize] >= adder_cost
                    {
                        if !(t1 == findodd_u128(t1) && t5 == findodd_u128(t5) && t1 >= t5)
                            && !(term1 >= term5 && term2 == 1 && term4 == 1)
                        {
                            adder_count[leapfrog as usize] = adder_cost;
                            add_graph_type(
                                adder_structures,
                                leapfrog as usize,
                                GraphType::Leapfrog4_4(term1, term2, term4, term5),
                            );
                        }
                    }
                }
            }
        }
    }
}

fn leapfrog7_combinations(
    adder_count: &mut [u8],
    adder_structures: &mut AdderStructures,
    terms1: &[usize],
    terms2: &[usize],
    terms3: &[usize],
    terms4: &[usize],
    terms5: &[usize],
    terms6: &[usize],
    terms7: &[usize],
    adder_cost: u8,
    max_value: usize,
) {
    debug!(
        terms1_count = terms1.len(),
        terms2_count = terms2.len(),
        terms3_count = terms3.len(),
        terms4_count = terms4.len(),
        terms5_count = terms5.len(),
        terms6_count = terms6.len(),
        terms7_count = terms7.len(),
        cost = adder_cost,
        "leapfrog7_combinations: starting"
    );
    let max_value_u128 = max_value as u128;

    for &term1 in terms1.iter() {
        let t1 = term1 as u128;
        let t1_odd = findodd_u128(t1);
        for &term2 in terms2.iter() {
            if term1.is_multiple_of(2) && term2.is_multiple_of(2) {
                continue;
            }
            let t2 = term2 as u128;
            for &term3 in terms3.iter() {
                if term2.is_multiple_of(2) && term3.is_multiple_of(2) {
                    continue;
                }
                let t3 = term3 as u128;
                for &term4 in terms4.iter() {
                    if (term2.is_multiple_of(2) || term3.is_multiple_of(2))
                        && term4.is_multiple_of(2)
                    {
                        continue;
                    }
                    let t4 = term4 as u128;
                    for &term5 in terms5.iter() {
                        if term4.is_multiple_of(2) && term5.is_multiple_of(2) {
                            continue;
                        }
                        let t5 = term5 as u128;
                        let t5_odd = findodd_u128(t5);

                        for &term6 in terms6.iter() {
                            if (term4.is_multiple_of(2) || term5.is_multiple_of(2))
                                && term6.is_multiple_of(2)
                            {
                                continue;
                            }
                            let t6 = term6 as u128;
                            let t6_odd = findodd_u128(t6);

                            for &term7 in terms7.iter() {
                                if term6.is_multiple_of(2) && term7.is_multiple_of(2) {
                                    continue;
                                }
                                let t7 = term7 as u128;
                                let t7_odd = findodd_u128(t7);

                                let leapfrog = findodd_u128(
                                    (t7 * (t5 * (t1 * t3 + t2) + t1 * t4)) + t6 * (t1 * t3 + t2),
                                );
                                if leapfrog != 0
                                    && leapfrog <= max_value_u128
                                    && adder_count[leapfrog as usize] >= adder_cost
                                {
                                    adder_count[leapfrog as usize] = adder_cost;
                                    add_graph_type(
                                        adder_structures,
                                        leapfrog as usize,
                                        GraphType::Leapfrog7_1(
                                            term1, term2, term3, term4, term5, term6, term7,
                                        ),
                                    );
                                }

                                let leapfrog = findodd_u128(
                                    (t7 * (t5 * ((t1 * t3).abs_diff(t2)) + t1 * t4))
                                        + t6 * ((t1 * t3).abs_diff(t2)),
                                );
                                if leapfrog != 0
                                    && leapfrog <= max_value_u128
                                    && adder_count[leapfrog as usize] >= adder_cost
                                {
                                    adder_count[leapfrog as usize] = adder_cost;
                                    add_graph_type(
                                        adder_structures,
                                        leapfrog as usize,
                                        GraphType::Leapfrog7_2(
                                            term1, term2, term3, term4, term5, term6, term7,
                                        ),
                                    );
                                }
                                let leapfrog = findodd_u128(
                                    t7 * ((t5 * (t1 * t3 + t2)).abs_diff(t1 * t4))
                                        + t6 * (t1 * t3 + t2),
                                );
                                if leapfrog != 0
                                    && leapfrog <= max_value_u128
                                    && adder_count[leapfrog as usize] >= adder_cost
                                {
                                    adder_count[leapfrog as usize] = adder_cost;
                                    add_graph_type(
                                        adder_structures,
                                        leapfrog as usize,
                                        GraphType::Leapfrog7_3(
                                            term1, term2, term3, term4, term5, term6, term7,
                                        ),
                                    );
                                }
                                let leapfrog = findodd_u128(
                                    (t7 * (t5 * (t1 * t3 + t2) + t1 * t4))
                                        .abs_diff(t6 * (t1 * t3 + t2)),
                                );
                                if leapfrog != 0
                                    && leapfrog <= max_value_u128
                                    && adder_count[leapfrog as usize] >= adder_cost
                                {
                                    adder_count[leapfrog as usize] = adder_cost;
                                    add_graph_type(
                                        adder_structures,
                                        leapfrog as usize,
                                        GraphType::Leapfrog7_4(
                                            term1, term2, term3, term4, term5, term6, term7,
                                        ),
                                    );
                                }
                                let leapfrog = findodd_u128(
                                    t7 * ((t5 * ((t1 * t3).abs_diff(t2))).abs_diff(t1 * t4))
                                        + t6 * ((t1 * t3).abs_diff(t2)),
                                );
                                if leapfrog != 0
                                    && leapfrog <= max_value_u128
                                    && adder_count[leapfrog as usize] >= adder_cost
                                {
                                    adder_count[leapfrog as usize] = adder_cost;
                                    add_graph_type(
                                        adder_structures,
                                        leapfrog as usize,
                                        GraphType::Leapfrog7_5(
                                            term1, term2, term3, term4, term5, term6, term7,
                                        ),
                                    );
                                }
                                let leapfrog = findodd_u128(
                                    (t7 * (t5 * ((t1 * t3).abs_diff(t2)) + t1 * t4))
                                        .abs_diff(t6 * ((t1 * t3).abs_diff(t2))),
                                );
                                if leapfrog != 0
                                    && leapfrog <= max_value_u128
                                    && adder_count[leapfrog as usize] >= adder_cost
                                {
                                    adder_count[leapfrog as usize] = adder_cost;
                                    add_graph_type(
                                        adder_structures,
                                        leapfrog as usize,
                                        GraphType::Leapfrog7_6(
                                            term1, term2, term3, term4, term5, term6, term7,
                                        ),
                                    );
                                }
                                let leapfrog = findodd_u128(
                                    (t7 * ((t5 * (t1 * t3 + t2)).abs_diff(t1 * t4)))
                                        .abs_diff(t6 * (t1 * t3 + t2)),
                                );
                                if leapfrog != 0
                                    && leapfrog <= max_value_u128
                                    && adder_count[leapfrog as usize] >= adder_cost
                                {
                                    adder_count[leapfrog as usize] = adder_cost;
                                    add_graph_type(
                                        adder_structures,
                                        leapfrog as usize,
                                        GraphType::Leapfrog7_7(
                                            term1, term2, term3, term4, term5, term6, term7,
                                        ),
                                    );
                                }
                                let leapfrog = findodd_u128(
                                    (t7 * (t5 * ((t1 * t3).abs_diff(t2))).abs_diff(t1 * t4))
                                        .abs_diff(t6 * ((t1 * t3).abs_diff(t2))),
                                );
                                if leapfrog != 0
                                    && leapfrog <= max_value_u128
                                    && adder_count[leapfrog as usize] >= adder_cost
                                {
                                    adder_count[leapfrog as usize] = adder_cost;
                                    add_graph_type(
                                        adder_structures,
                                        leapfrog as usize,
                                        GraphType::Leapfrog7_8(
                                            term1, term2, term3, term4, term5, term6, term7,
                                        ),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn add_graph_type(adder_structures: &mut AdderStructures, result: usize, graph_type: GraphType) {
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

// prepare_data.rs - Run this once to create adder_cost.bin
use std::fs::File;
use std::io::{Result, Write};

fn pack_sparse_vector(sparse_vec: &[u8]) -> (Vec<u8>, usize) {
    let odd_count = sparse_vec.len().div_ceil(2);
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

// Custom serialization with varint encoding
fn serialize_graph_types(types: &[Vec<GraphType>]) -> Vec<u8> {
    let mut buf = Vec::new();

    // Write number of entries (varint)
    let mut count_buf = varint_encode::usize_buffer();
    let count_bytes = varint_encode::usize(types.len(), &mut count_buf);
    buf.extend_from_slice(count_bytes);

    for type_vec in types {
        // Write length of this Vec (varint)
        let mut len_buf = varint_encode::usize_buffer();
        let len_bytes = varint_encode::usize(type_vec.len(), &mut len_buf);
        buf.extend_from_slice(len_bytes);

        for gt in type_vec {
            match gt {
                GraphType::Adder(a, b) => {
                    buf.push(0); // variant tag
                    let mut a_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*a, &mut a_buf));
                    let mut b_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*b, &mut b_buf));
                }
                GraphType::Subtractor(a, b) => {
                    buf.push(1);
                    let mut a_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*a, &mut a_buf));
                    let mut b_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*b, &mut b_buf));
                }
                GraphType::Cascade(a, b) => {
                    buf.push(2);
                    let mut a_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*a, &mut a_buf));
                    let mut b_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*b, &mut b_buf));
                }
                GraphType::Leapfrog4_1(a, b, c, d) => {
                    buf.push(3);
                    let mut a_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*a, &mut a_buf));
                    let mut b_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*b, &mut b_buf));
                    let mut c_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*c, &mut c_buf));
                    let mut d_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*d, &mut d_buf));
                }
                GraphType::Leapfrog4_2(a, b, c, d) => {
                    buf.push(4);
                    let mut a_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*a, &mut a_buf));
                    let mut b_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*b, &mut b_buf));
                    let mut c_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*c, &mut c_buf));
                    let mut d_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*d, &mut d_buf));
                }
                GraphType::Leapfrog4_3(a, b, c, d) => {
                    buf.push(5);
                    let mut a_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*a, &mut a_buf));
                    let mut b_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*b, &mut b_buf));
                    let mut c_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*c, &mut c_buf));
                    let mut d_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*d, &mut d_buf));
                }
                GraphType::Leapfrog4_4(a, b, c, d) => {
                    buf.push(6);
                    let mut a_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*a, &mut a_buf));
                    let mut b_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*b, &mut b_buf));
                    let mut c_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*c, &mut c_buf));
                    let mut d_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*d, &mut d_buf));
                }
                GraphType::Leapfrog5_1(a, b, c, d, e) => {
                    buf.push(7);
                    let mut a_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*a, &mut a_buf));
                    let mut b_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*b, &mut b_buf));
                    let mut c_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*c, &mut c_buf));
                    let mut d_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*d, &mut d_buf));
                    let mut e_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*e, &mut e_buf));
                }
                GraphType::Leapfrog5_2(a, b, c, d, e) => {
                    buf.push(8);
                    let mut a_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*a, &mut a_buf));
                    let mut b_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*b, &mut b_buf));
                    let mut c_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*c, &mut c_buf));
                    let mut d_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*d, &mut d_buf));
                    let mut e_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*e, &mut e_buf));
                }
                GraphType::Leapfrog5_3(a, b, c, d, e) => {
                    buf.push(9);
                    let mut a_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*a, &mut a_buf));
                    let mut b_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*b, &mut b_buf));
                    let mut c_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*c, &mut c_buf));
                    let mut d_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*d, &mut d_buf));
                    let mut e_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*e, &mut e_buf));
                }
                GraphType::Leapfrog5_4(a, b, c, d, e) => {
                    buf.push(10);
                    let mut a_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*a, &mut a_buf));
                    let mut b_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*b, &mut b_buf));
                    let mut c_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*c, &mut c_buf));
                    let mut d_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*d, &mut d_buf));
                    let mut e_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*e, &mut e_buf));
                }
                GraphType::Leapfrog7_1(a, b, c, d, e, f, g) => {
                    buf.push(11);
                    let mut a_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*a, &mut a_buf));
                    let mut b_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*b, &mut b_buf));
                    let mut c_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*c, &mut c_buf));
                    let mut d_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*d, &mut d_buf));
                    let mut e_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*e, &mut e_buf));
                    let mut f_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*f, &mut f_buf));
                    let mut g_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*g, &mut g_buf));
                }
                GraphType::Leapfrog7_2(a, b, c, d, e, f, g) => {
                    buf.push(12);
                    let mut a_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*a, &mut a_buf));
                    let mut b_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*b, &mut b_buf));
                    let mut c_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*c, &mut c_buf));
                    let mut d_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*d, &mut d_buf));
                    let mut e_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*e, &mut e_buf));
                    let mut f_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*f, &mut f_buf));
                    let mut g_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*g, &mut g_buf));
                }
                GraphType::Leapfrog7_3(a, b, c, d, e, f, g) => {
                    buf.push(13);
                    let mut a_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*a, &mut a_buf));
                    let mut b_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*b, &mut b_buf));
                    let mut c_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*c, &mut c_buf));
                    let mut d_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*d, &mut d_buf));
                    let mut e_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*e, &mut e_buf));
                    let mut f_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*f, &mut f_buf));
                    let mut g_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*g, &mut g_buf));
                }
                GraphType::Leapfrog7_4(a, b, c, d, e, f, g) => {
                    buf.push(14);
                    let mut a_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*a, &mut a_buf));
                    let mut b_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*b, &mut b_buf));
                    let mut c_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*c, &mut c_buf));
                    let mut d_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*d, &mut d_buf));
                    let mut e_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*e, &mut e_buf));
                    let mut f_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*f, &mut f_buf));
                    let mut g_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*g, &mut g_buf));
                }
                GraphType::Leapfrog7_5(a, b, c, d, e, f, g) => {
                    buf.push(15);
                    let mut a_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*a, &mut a_buf));
                    let mut b_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*b, &mut b_buf));
                    let mut c_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*c, &mut c_buf));
                    let mut d_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*d, &mut d_buf));
                    let mut e_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*e, &mut e_buf));
                    let mut f_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*f, &mut f_buf));
                    let mut g_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*g, &mut g_buf));
                }
                GraphType::Leapfrog7_6(a, b, c, d, e, f, g) => {
                    buf.push(16);
                    let mut a_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*a, &mut a_buf));
                    let mut b_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*b, &mut b_buf));
                    let mut c_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*c, &mut c_buf));
                    let mut d_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*d, &mut d_buf));
                    let mut e_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*e, &mut e_buf));
                    let mut f_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*f, &mut f_buf));
                    let mut g_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*g, &mut g_buf));
                }
                GraphType::Leapfrog7_7(a, b, c, d, e, f, g) => {
                    buf.push(10);
                    let mut a_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*a, &mut a_buf));
                    let mut b_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*b, &mut b_buf));
                    let mut c_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*c, &mut c_buf));
                    let mut d_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*d, &mut d_buf));
                    let mut e_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*e, &mut e_buf));
                    let mut f_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*f, &mut f_buf));
                    let mut g_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*g, &mut g_buf));
                }
                GraphType::Leapfrog7_8(a, b, c, d, e, f, g) => {
                    buf.push(10);
                    let mut a_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*a, &mut a_buf));
                    let mut b_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*b, &mut b_buf));
                    let mut c_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*c, &mut c_buf));
                    let mut d_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*d, &mut d_buf));
                    let mut e_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*e, &mut e_buf));
                    let mut f_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*f, &mut f_buf));
                    let mut g_buf = varint_encode::usize_buffer();
                    buf.extend_from_slice(varint_encode::usize(*g, &mut g_buf));
                }
            }
        }
    }

    buf
}

/// Extract all indices with a specific adder cost
fn extract_cost_values(adder_count: &[u8], cost: u8) -> Vec<usize> {
    adder_count
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == cost)
        .map(|(idx, _)| idx)
        .collect()
}

/// Create shifted variants of values up to max_value
fn create_shifted_variants(values: &[usize], max_value: usize) -> Vec<usize> {
    let mut shifted = Vec::new();
    for &v in values {
        let mut shift = 0;
        while (v << shift) <= max_value {
            shifted.push(v << shift);
            shift += 1;
        }
    }
    shifted
}

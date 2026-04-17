use std::path::Path;

use itertools::Itertools;

use crate::{
    error::GenerationError,
    scenario::{Scenario, ScenarioConfig},
};

fn update_recursive(
    total_clients: u32,
    weights: &[u32],
    total_weight: u32,
    algo_distributions: &[Vec<String>],
) -> eyre::Result<Vec<Vec<(String, u32)>>> {
    let Some(available_algorithms) = algo_distributions.first() else {
        return Ok(vec![Vec::new()]);
    };

    if weights.len() != algo_distributions.len() {
        Err(GenerationError::InvalidWeights(
            weights.to_vec().into_iter().join("-"),
            weights.len(),
            algo_distributions.len(),
        ))?;
    }

    assert!(!available_algorithms.is_empty());

    let weight = weights.first().unwrap();
    let count = (((total_clients as f64) * (*weight as f64)) / (total_weight as f64)) as u32;
    if count == 0 {
        return update_recursive(
            total_clients,
            &weights[1..],
            total_weight,
            &algo_distributions[1..],
        );
    }

    let mut scenarii = Vec::new();

    let subconfigs = update_recursive(
        total_clients,
        &weights[1..],
        total_weight,
        &algo_distributions[1..],
    )?;

    for algo_name in available_algorithms {
        for subconfig in &subconfigs {
            let mut configuration = vec![(algo_name.to_owned(), count)];
            configuration.extend_from_slice(&subconfig);
            scenarii.push(configuration);
        }
    }

    Ok(scenarii)
}

pub fn generate(
    total_client_count: u32,
    bandwidths: Vec<u32>,
    algo_distributions: Vec<Vec<String>>,
    scenarii_weights: Vec<Vec<u32>>,
    folder: &Path,
) -> eyre::Result<()> {
    let partial_scenarii = scenarii_weights
        .iter()
        .map(|weights| {
            update_recursive(
                total_client_count,
                weights,
                weights.iter().sum(),
                &algo_distributions,
            )
        })
        .collect::<eyre::Result<Vec<Vec<Vec<(String, u32)>>>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<Vec<(String, u32)>>>();

    for partial_scenario in &partial_scenarii {
        if partial_scenario.iter().map(|&(_, n)| n).sum::<u32>() != total_client_count {
            eprintln!(
                "Warning: total number of clients does not match target {total_client_count}: {partial_scenario:?}"
            )
        }

        for bandwidth in &bandwidths {
            let max_bandwidth = bandwidth.to_string() + "mbit";
            let delay_config = "10ms 1ms".into();

            let mut scenario = Scenario::new(max_bandwidth, delay_config);

            for (name, count) in partial_scenario {
                let config = ScenarioConfig::from(name);
                scenario.add_instances(name, config, *count);
            }

            scenario.finalize();

            let stringified = serde_json::to_string_pretty(&scenario)?;

            // println!("{:#?}", stringified);

            std::fs::write(folder.join(scenario.get_filename()), stringified)?;
        }
    }

    if bandwidths.is_empty() {
        eprintln!("Warning: no scenario generated (no bandwidths)");
    } else if partial_scenarii.is_empty() {
        eprintln!("Warning: no scenario generated (no clients)");
    } else {
        println!(
            "Generated {} scenarii",
            bandwidths.len() * partial_scenarii.len()
        );
    }

    Ok(())
}

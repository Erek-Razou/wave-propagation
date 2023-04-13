use crate::find_max_distance::{calc_field_strength_for_line_at_km, find_max_distance_for_line};
use crate::terrain::defaults::Terrain;
use crate::terrain::{Line, LineSegment};
use anyhow::{bail, Context, Result};
use rayon::prelude::*;
use std::path::Path;
use std::result;
use textplots::{Chart, Plot, Shape};
mod cli;
mod file;
mod find_max_distance;
mod lfmf;
mod plot;
mod terrain;

fn main() -> Result<()> {
    let cli = cli::parse();

    println!("Hello, world!");

    let min_e = cli.min_field_strength;
    match cli.input_file {
        Some(path) => {
            find_distances_for_input_file(min_e, &path)?;
        }
        None => {
            find_distance_for_hardcoded_line(min_e)?;
        }
    }

    Ok(())
}

fn find_distances_for_input_file(min_e: f64, input_file: &Path) -> Result<()> {
    let lines = file::read(input_file)?;
    let results: Vec<_> = lines
        .par_iter()
        .map(|line| find_max_distance_for_line(min_e, line))
        .collect();

    // Print all errors, if any exist.
    let errors = results
        .iter()
        .enumerate()
        .filter_map(|(i, result)| match result {
            Err(error) => Some(format!("Error for angle {}: {error:?}", lines[i].angle())),
            Ok(_) => None,
        })
        .collect::<Vec<_>>();
    if !errors.is_empty() {
        bail!("{}", errors.join("\n\n"));
    }

    for (i, result) in results.into_iter().enumerate() {
        let max_distance = result.expect(
            "Already checked that there are no Errors in results so everything should be Ok.",
        );
        println!("Angle: {}", lines[i].angle());
        println!("Maximum distance: {max_distance} km");
        let field_strength_at_max_distance =
            calc_field_strength_for_line_at_km(&lines[i], max_distance)
                .expect("It was already calculated, should compute again.");
        println!("Field strength: {field_strength_at_max_distance} dB(uV)/m");
        let plot_result = plot::line_with_divider(
            &lines[i],
            max_distance as f32,
            field_strength_at_max_distance as f32,
            0.5,
        );
        if let Err(error) = plot_result {
            println!("Error plotting graph: {error:#}");
        }
    }

    Ok(())
}

fn find_distance_for_hardcoded_line(min_e: f64) -> Result<()> {
    let segments = [
        LineSegment::with_length(Terrain::Ground.parameters(), 20.0),
        LineSegment::with_length(Terrain::Sea.parameters(), 100.0),
        LineSegment::with_length(Terrain::Ground.parameters(), 280.0),
    ];
    let line = Line::with_segments(0.0, segments);

    let max_distance = find_max_distance_for_line(min_e, &line)?;
    println!("Maximum distance: {max_distance} km");
    let field_strength_at_max_distance = calc_field_strength_for_line_at_km(&line, max_distance)
        .expect("It was already calculated, should compute again.");
    println!("Field strength: {field_strength_at_max_distance} dB(uV)/m");
    let plot_result = plot::line_with_divider(
        &line,
        max_distance as f32,
        field_strength_at_max_distance as f32,
        1.0,
    );
    if let Err(error) = plot_result {
        println!("Error making chart: {error:#}")
    }
    Ok(())
}

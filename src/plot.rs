use crate::find_max_distance::calc_field_strength_for_line_at_km;
use crate::terrain::Line;
use anyhow::{ensure, Result};
use rayon::prelude::*;
use textplots::{Chart, Plot, Shape};

fn calc_points(line: &Line, step_km: f64) -> Result<Vec<(f32, f32)>> {
    ensure!(0.0 < step_km, "`step_km` must be positive.");
    let line_max_distance = line.max_distance();
    ensure!(
        0.0 < line_max_distance,
        "`line` must span a positive distance."
    );
    ensure!(
        line_max_distance <= 10000.0,
        "`line` must not be longer than 10000 km, the longest distance allowed by LFMF."
    );

    let iterations = (line_max_distance / step_km).floor() as usize;
    let points: Vec<_> = (1..iterations)
        .into_par_iter()
        .map(|iteration| {
            let distance = step_km * iteration as f64;
            let field_strength = calc_field_strength_for_line_at_km(line, distance)
                .expect("The assurances at the start should be enough.");
            (distance as f32, field_strength as f32)
        })
        .collect();

    Ok(points)
}

pub fn line(line: &Line, step_km: f64) -> Result<()> {
    let points = calc_points(line, step_km)?;
    Chart::new(300, 100, 0.0, line.max_distance() as f32)
        .lineplot(&Shape::Points(&points))
        .nice();
    Ok(())
}

pub fn line_with_divider(
    line: &Line,
    divider_x: f32,
    divider_y_centre: f32,
    step_km: f64,
) -> Result<()> {
    let points = calc_points(line, step_km)?;
    Chart::new(300, 100, 0.0, line.max_distance() as f32)
        .lineplot(&Shape::Points(&points))
        .lineplot(&Shape::Lines(&[
            (divider_x, divider_y_centre - 20.0),
            (divider_x, divider_y_centre + 20.0),
        ]))
        .nice();
    Ok(())
}

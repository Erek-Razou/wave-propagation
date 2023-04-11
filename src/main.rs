use crate::find_max_distance::{calc_field_strength_for_line_at_km, find_max_distance_for_line};
use crate::terrain::defaults::Terrain;
use crate::terrain::{Line, LineSegment};
use rayon::prelude::*;
use textplots::{Chart, Plot, Shape};
mod cli;
mod find_max_distance;
mod lfmf;
mod terrain;

fn main() {
    let cli = cli::parse();

    println!("Hello, world!");

    let min_e = cli.min_field_strength;

    let segments = [
        LineSegment::with_length(Terrain::Ground.parameters(), 20.0),
        LineSegment::with_length(Terrain::Sea.parameters(), 100.0),
        LineSegment::with_length(Terrain::Ground.parameters(), 280.0),
    ];
    let line = Line::with_segments(0.0, segments);

    let result = find_max_distance_for_line(min_e, &line);
    match result {
        Err(error) => println!("Error: {error}"),
        Ok(max_distance) => {
            println!("Maximum distance: {max_distance}");
            let field_strength_at_max_distance =
                calc_field_strength_for_line_at_km(&line, max_distance)
                    .expect("It was already calculated, should compute again.");
            println!("Field strength: {field_strength_at_max_distance}");
            let max_x = line.max_distance().floor() as usize;

            let points: Vec<_> = (1..max_x)
                .into_par_iter()
                .map(|x| {
                    (
                        x as f32,
                        calc_field_strength_for_line_at_km(&line, x as f64)
                            .expect("The line should be valid, it has been analyzed after all.")
                            as f32,
                    )
                })
                .collect();
            Chart::new(300, 100, 0.0, max_x as f32)
                .lineplot(&Shape::Points(&points))
                .lineplot(&Shape::Lines(&[
                    (
                        max_distance as f32,
                        field_strength_at_max_distance as f32 + 25.0,
                    ),
                    (
                        max_distance as f32,
                        field_strength_at_max_distance as f32 - 25.0,
                    ),
                ]))
                .nice();
        }
    }
}

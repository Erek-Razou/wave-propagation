use crate::terrain::defaults::Terrain;
use crate::terrain::{Line, LineSegment};
use anyhow::{ensure, Context, Result};
use std::fmt::format;
use std::path::Path;

pub fn read(path: &Path) -> Result<Vec<Line>> {
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("Could not read file `{}`", path.display()))?;
    let mut lines = Vec::with_capacity(360 / 5);
    // Skip the headers in the first line, read all the rest as lines.
    for (i, file_line) in contents.lines().enumerate().skip(1) {
        let line =
            parse_to_line(file_line).with_context(|| format!("Failed to parse line #{}", i + 1))?;
        lines.push(line);
    }
    Ok(lines)
}

fn parse_to_line(file_line: &str) -> Result<Line> {
    let mut columns = file_line.split(',').map(str::trim);
    let angle: f64 = columns
        .next()
        .context("There was no 1st column (angle).")?
        .parse()
        .context("Could not parse 1st column (angle) to float.")?;

    // We'll actually use the pixel distances.
    let border_column = columns
        .next()
        .context("There was no 2nd column (end distance/border).")?;
    let end_km_distance = match border_column.is_empty() {
        true => None,
        false => {
            let end_px_distance = border_column
                .parse()
                .context("Could not parse 2nd column (end distance/border) to float.")?;
            Some(px_to_km(end_px_distance))
        }
    };

    // The start from the 4th column and measure all the segment distances.
    let mut segments = Vec::with_capacity(8);
    let mut current_km_distance = 0.0;
    let mut current_terrain = Terrain::Ground;
    for column in columns.skip(1).step_by(2) {
        if column == "border" {
            let end_km_distance =
                end_km_distance.context("`border` tag was used but not defined.")?;
            let last_km_length = end_km_distance - current_km_distance;
            ensure!(last_km_length > 0.0, "Length must be positive but it was {end_km_distance} - {current_km_distance} = {last_km_length} for line with angle {angle} degrees.");
            segments.push(LineSegment::with_length(
                current_terrain.parameters(),
                last_km_length,
            ));
            break;
        } else if column.is_empty() {
            break;
        }

        let px_distance: f64 = column.parse().with_context(|| {
            format!("Could not parse `{column}` to a float as a pixel distance.")
        })?;
        let km_distance = px_distance * 200.0 / 254.0;
        let km_length = km_distance - current_km_distance;
        ensure!(km_length > 0.0, "Length must be positive but it was {km_distance} - {current_km_distance} = {km_length} for line with angle {angle} degrees.");

        segments.push(LineSegment::with_length(
            current_terrain.parameters(),
            km_length,
        ));

        current_km_distance += km_length;
        current_terrain = match current_terrain {
            Terrain::Ground => Terrain::Sea,
            Terrain::Sea => Terrain::Ground,
        }
    }

    Ok(Line::with_segments(angle, segments))
}

fn px_to_km(px: f64) -> f64 {
    px * 254.0 / 200.0
}

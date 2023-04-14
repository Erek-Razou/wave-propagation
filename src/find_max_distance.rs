use crate::lfmf::calc_LFMF;
use crate::terrain::{Line, LineSegment};
use anyhow::{bail, Context, Result};

fn calc_one_way_field_strength_for_segments<'a, I: Iterator<Item = &'a LineSegment>>(
    segments: I,
) -> Result<f64> {
    let mut field_strength = 0.0;
    let mut old_distance = 0.0;
    for segment in segments {
        let new_distance = old_distance + segment.length_km();
        let mut parameters = segment.lfmf_parameters();
        if old_distance != 0.0 {
            parameters.d__km = old_distance;
            field_strength -= calc_LFMF(parameters)
                .with_context(|| {
                    format!("Failed to calculate field_strength for parameters {parameters:?}.")
                })?
                .E_dBuVm;
        }
        parameters.d__km = new_distance;
        field_strength += calc_LFMF(parameters)
            .with_context(|| {
                format!("Failed to calculate field_strength for parameters {parameters:?}.")
            })?
            .E_dBuVm;
        old_distance = new_distance;
    }
    Ok(field_strength)
}

pub fn calc_field_strength_for_line_at_km(line: &Line, distance: f64) -> Result<f64> {
    let segments = line.segments_until(distance).with_context(|| {
        format!("Could not get segments for distance of {distance} km in line {line:?}")
    })?;
    match segments.len() {
        0 => bail!("Got 0 segments for distance of {distance} km in line {line:?}"),
        1 => Ok(calc_LFMF(segments[0].lfmf_parameters())
            .with_context(|| {
                format!(
                    "Could not calculate field strength for segment {:?}",
                    segments[0]
                )
            })?
            .E_dBuVm),
        _ => {
            let field_strength1 = calc_one_way_field_strength_for_segments(segments.iter())
                .with_context(|| {
                    format!(
                        "Could not calculate forwards way field strength for segments {segments:?}."
                    )
                })?;
            let field_strength2 = calc_one_way_field_strength_for_segments(segments.iter().rev())
                .with_context(|| {
                format!("Could not calculate reverse way field strength for segments {segments:?}.")
            })?;
            let field_strength_final = (field_strength1 + field_strength2) * 0.5;
            Ok(field_strength_final)
        }
    }
}

/// Find the maximum distance between the transmitter and the receiver for a given minimum usable field strength in dB(uV)/m.
///
/// # Arguments
/// * `min_usable_field_strength` - Minimum usable field strength in dB(uV)/m.
/// * `line` - The line in which to search.
///
/// # Returns
/// A result of either the maximum distance in km where the field strength doesn't fall below the minimum or an error.
pub fn find_max_distance_for_line(min_usable_field_strength: f64, line: &Line) -> Result<f64> {
    const FIELD_STRENGTH_DB_TOLERANCE: f64 = 0.0001;
    const MINIMUM_STEP: f64 = 0.001; // An accuracy of 1 m should be enough given how approximated the results of LFMF are and is inline with it's minimum distance.
    let upper_bound = min_usable_field_strength + FIELD_STRENGTH_DB_TOLERANCE;
    let lower_bound = min_usable_field_strength;

    // First we'll try to find the first segment that contains the value we're searching for to isolate it.
    let mut min_distance = 0.0;
    let mut max_distance = 0.0;
    let mut current_distance = 0.0;
    for segment in line.segments() {
        current_distance += segment.length_km();
        let field_strength = calc_field_strength_for_line_at_km(line, current_distance).with_context(||format!("While searching for min and max distance, could not calculate field strength at {current_distance} km for line {line:?}."))?;
        if field_strength > upper_bound {
            min_distance = current_distance;
        } else if field_strength <= upper_bound {
            if lower_bound <= field_strength {
                // Lucky find!
                return Ok(current_distance);
            }
            // Else we found a bound for the distance, the field strength must be in this segment.
            max_distance = current_distance;
            break;
        }
    }
    if max_distance == 0.0 {
        bail!("The distance where the minimum field strength value is at is greater than the length of {min_distance} km of line {line:?}.");
    }

    // Ideally we could use the bisection method/binary search here, but Millington's method can produce unpredictable results.
    // For example, a sea path after a ground path will have the predicted field strength go upwards as you go further before it goes back down.
    // So we'll just do a slow but fault tolerant linear search.
    let mut current_distance = min_distance;
    let mut step = 5.0; // The choice of 5 km is a bit arbitrary but should be an ok compromise between speed and resistance to abnormalities.
    loop {
        let new_distance = current_distance + step;
        let field_strength = if new_distance < max_distance {
            calc_field_strength_for_line_at_km(line, new_distance).with_context(|| format!("While linearly searching, could not calculate field strength for distance {new_distance} km in line {line:?}"))?
        } else {
            f64::NEG_INFINITY
        };
        if field_strength < lower_bound {
            max_distance = new_distance;
            step *= 0.125; // Division by 8. This means that it will take 8 iterations to get to the new maximum distance.
            if step < MINIMUM_STEP {
                return Ok(current_distance); // As a last resort, while it may be out of the set bounds, return the last valid value.
            }
        } else if field_strength <= upper_bound && lower_bound <= field_strength {
            return Ok(new_distance);
        } else {
            current_distance = new_distance;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terrain::defaults::Terrain;
    use std::iter::once;

    const DISTANCE_TOLERANCE: f64 = 0.01; // 10 m

    #[test]
    fn calc_field_strength_for_line_at_km_with_no_segments() {
        let line = Line::new(0.0);
        let result = calc_field_strength_for_line_at_km(&line, 1000.0);
        assert!(result.is_err());
    }

    #[test]
    fn calc_field_strength_for_line_at_km_with_one_ground_segment_whole() {
        let distance = 142.092;
        let segment = LineSegment::with_length(Terrain::Ground.parameters(), distance);
        let line = Line::with_segments(0.0, once(segment));
        let result = calc_field_strength_for_line_at_km(&line, distance).unwrap();
        let expected = calc_LFMF(segment.lfmf_parameters()).unwrap().E_dBuVm;
        assert_eq!(result, expected);
    }

    #[test]
    fn calc_field_strength_for_line_at_km_with_one_ground_segment_cut_short() {
        let distance = 203.539;
        let shorter_distance = 95.28;
        let segment = LineSegment::with_length(Terrain::Ground.parameters(), distance);
        let line = Line::with_segments(0.0, once(segment));
        let result = calc_field_strength_for_line_at_km(&line, shorter_distance).unwrap();
        let mut parameters = segment.lfmf_parameters();
        parameters.d__km = shorter_distance;
        let expected = calc_LFMF(parameters).unwrap().E_dBuVm;
        assert_eq!(result, expected);
    }

    #[test]
    fn calc_field_strength_for_line_at_km_with_one_sea_segment_whole() {
        let distance = 402.23;
        let segment = LineSegment::with_length(Terrain::Sea.parameters(), distance);
        let line = Line::with_segments(0.0, once(segment));
        let result = calc_field_strength_for_line_at_km(&line, distance).unwrap();
        let expected = calc_LFMF(segment.lfmf_parameters()).unwrap().E_dBuVm;
        assert_eq!(result, expected);
    }

    #[test]
    fn calc_field_strength_for_line_at_km_with_one_sea_segment_cut_short() {
        let distance = 523.5;
        let shorter_distance = 235.74;
        let segment = LineSegment::with_length(Terrain::Sea.parameters(), distance);
        let line = Line::with_segments(0.0, once(segment));
        let result = calc_field_strength_for_line_at_km(&line, shorter_distance).unwrap();
        let mut parameters = segment.lfmf_parameters();
        parameters.d__km = shorter_distance;
        let expected = calc_LFMF(parameters).unwrap().E_dBuVm;
        assert_eq!(result, expected);
    }

    #[test]
    fn calc_field_strength_for_line_at_km_with_two_segments() {
        let distance = 193.9203;
        let segments = [
            LineSegment::with_length(Terrain::Ground.parameters(), 50.0),
            LineSegment::with_length(Terrain::Sea.parameters(), 200.0),
        ];
        let line = Line::with_segments(0.0, segments);
        let result = calc_field_strength_for_line_at_km(&line, distance).unwrap();
        let expected_field_strength = {
            let mut ground_parameters = Terrain::Ground.parameters();
            let mut sea_parameters = Terrain::Sea.parameters();

            let mut field_strength1 = 0.0;
            ground_parameters.d__km = 50.0;
            field_strength1 += calc_LFMF(ground_parameters).unwrap().E_dBuVm;
            sea_parameters.d__km = 50.0;
            field_strength1 -= calc_LFMF(sea_parameters).unwrap().E_dBuVm;
            sea_parameters.d__km = distance;
            field_strength1 += calc_LFMF(sea_parameters).unwrap().E_dBuVm;

            let mut field_strength2 = 0.0;
            sea_parameters.d__km = distance - 50.0;
            field_strength2 += calc_LFMF(sea_parameters).unwrap().E_dBuVm;
            ground_parameters.d__km = distance - 50.0;
            field_strength2 -= calc_LFMF(ground_parameters).unwrap().E_dBuVm;
            ground_parameters.d__km = distance;
            field_strength2 += calc_LFMF(ground_parameters).unwrap().E_dBuVm;

            (field_strength1 + field_strength2) / 2.0
        };
        assert_eq!(
            result, expected_field_strength,
            "Expected field strength of {} dBuVm, got {} dBuVm.",
            expected_field_strength, result
        );
    }

    #[test]
    fn calc_field_strength_for_line_at_km_with_three_segments() {
        let distance = 180.0;
        let segments = [
            LineSegment::with_length(Terrain::Ground.parameters(), 100.0),
            LineSegment::with_length(Terrain::Sea.parameters(), 50.0),
            LineSegment::with_length(Terrain::Ground.parameters(), 200.0),
        ];
        let line = Line::with_segments(0.0, segments);
        let result = calc_field_strength_for_line_at_km(&line, distance).unwrap();
        let expected_field_strength = {
            let mut ground_parameters = Terrain::Ground.parameters();
            let mut sea_parameters = Terrain::Sea.parameters();

            let mut field_strength1 = 0.0;
            ground_parameters.d__km = 100.0;
            field_strength1 += calc_LFMF(ground_parameters).unwrap().E_dBuVm;
            sea_parameters.d__km = 100.0;
            field_strength1 -= calc_LFMF(sea_parameters).unwrap().E_dBuVm;
            sea_parameters.d__km = 150.0;
            field_strength1 += calc_LFMF(sea_parameters).unwrap().E_dBuVm;
            ground_parameters.d__km = 150.0;
            field_strength1 -= calc_LFMF(ground_parameters).unwrap().E_dBuVm;
            ground_parameters.d__km = 180.0;
            field_strength1 += calc_LFMF(ground_parameters).unwrap().E_dBuVm;

            let mut field_strength2 = 0.0;
            ground_parameters.d__km = 30.0;
            field_strength2 += calc_LFMF(ground_parameters).unwrap().E_dBuVm;
            sea_parameters.d__km = 30.0;
            field_strength2 -= calc_LFMF(sea_parameters).unwrap().E_dBuVm;
            sea_parameters.d__km = 80.0;
            field_strength2 += calc_LFMF(sea_parameters).unwrap().E_dBuVm;
            ground_parameters.d__km = 80.0;
            field_strength2 -= calc_LFMF(ground_parameters).unwrap().E_dBuVm;
            ground_parameters.d__km = 180.0;
            field_strength2 += calc_LFMF(ground_parameters).unwrap().E_dBuVm;

            (field_strength1 + field_strength2) / 2.0
        };
        assert_eq!(
            result, expected_field_strength,
            "Expected field strength of {} dBuVm, got {} dBuVm.",
            expected_field_strength, result
        );
    }

    #[test]
    fn find_max_distance_for_old_min_strength_on_small_ground_line() {
        let segment = LineSegment::with_length(Terrain::Ground.parameters(), 100.0);
        let line = Line::with_segments(0.0, once(segment));
        let max_distance = find_max_distance_for_line(61.9, &line).unwrap();
        let expected = 48.7;
        let error = max_distance - expected;
        assert!(
            error.abs() < DISTANCE_TOLERANCE,
            "Expected max distance of {} +-{} km, got {} km",
            expected,
            DISTANCE_TOLERANCE,
            max_distance
        );
    }

    #[test]
    fn find_max_distance_for_old_min_strength_on_medium_ground_line() {
        let segment = LineSegment::with_length(Terrain::Ground.parameters(), 500.0);
        let line = Line::with_segments(0.0, once(segment));
        let max_distance = find_max_distance_for_line(61.9, &line).unwrap();
        let expected = 48.7;
        let error = max_distance - expected;
        assert!(
            error.abs() < DISTANCE_TOLERANCE,
            "Expected max distance of {} +-{} km, got {} km",
            expected,
            DISTANCE_TOLERANCE,
            max_distance
        );
    }

    #[test]
    fn find_max_distance_for_old_min_strength_on_biggest_ground_line() {
        let segment = LineSegment::with_length(Terrain::Ground.parameters(), 10000.0);
        let line = Line::with_segments(0.0, once(segment));
        let max_distance = find_max_distance_for_line(61.9, &line).unwrap();
        let expected = 48.7;
        let error = max_distance - expected;
        assert!(
            error.abs() < DISTANCE_TOLERANCE,
            "Expected max distance of {} +-{} km, got {} km",
            expected,
            DISTANCE_TOLERANCE,
            max_distance
        );
    }

    #[test]
    fn find_max_distance_for_old_min_strength_on_small_sea_line() {
        let segment = LineSegment::with_length(Terrain::Sea.parameters(), 100.0);
        let line = Line::with_segments(0.0, once(segment));
        let max_distance = find_max_distance_for_line(61.9, &line);
        assert!(max_distance.is_err());
    }

    #[test]
    fn find_max_distance_for_old_min_strength_on_medium_sea_line() {
        let segment = LineSegment::with_length(Terrain::Sea.parameters(), 500.0);
        let line = Line::with_segments(0.0, once(segment));
        let max_distance = find_max_distance_for_line(61.9, &line).unwrap();
        let expected = 353.830;
        let error = max_distance - expected;
        assert!(
            error.abs() < DISTANCE_TOLERANCE,
            "Expected max distance of {} +-{} km, got {} km",
            expected,
            DISTANCE_TOLERANCE,
            max_distance
        );
    }

    #[test]
    fn find_max_distance_for_old_min_strength_on_biggest_sea_line() {
        let segment = LineSegment::with_length(Terrain::Sea.parameters(), 10000.0);
        let line = Line::with_segments(0.0, once(segment));
        let max_distance = find_max_distance_for_line(61.9, &line).unwrap();
        let expected = 353.830;
        let error = max_distance - expected;
        assert!(
            error.abs() < DISTANCE_TOLERANCE,
            "Expected max distance of {} +-{} km, got {} km",
            expected,
            DISTANCE_TOLERANCE,
            max_distance
        );
    }

    #[test]
    fn find_max_distance_for_new_min_strength_on_small_ground_line() {
        let segment = LineSegment::with_length(Terrain::Ground.parameters(), 100.0);
        let line = Line::with_segments(0.0, once(segment));
        let max_distance = find_max_distance_for_line(43.2, &line);
        assert!(max_distance.is_err());
    }

    #[test]
    fn find_max_distance_for_new_min_strength_on_medium_ground_line() {
        let segment = LineSegment::with_length(Terrain::Ground.parameters(), 500.0);
        let line = Line::with_segments(0.0, once(segment));
        let max_distance = find_max_distance_for_line(43.2, &line).unwrap();
        let expected = 123.283;
        let error = max_distance - expected;
        assert!(
            error.abs() < DISTANCE_TOLERANCE,
            "Expected max distance of {} +-{} km, got {} km",
            expected,
            DISTANCE_TOLERANCE,
            max_distance
        );
    }

    #[test]
    fn find_max_distance_for_new_min_strength_on_biggest_ground_line() {
        let segment = LineSegment::with_length(Terrain::Ground.parameters(), 10000.0);
        let line = Line::with_segments(0.0, once(segment));
        let max_distance = find_max_distance_for_line(43.2, &line).unwrap();
        let expected = 123.283;
        let error = max_distance - expected;
        assert!(
            error.abs() < DISTANCE_TOLERANCE,
            "Expected max distance of {} +-{} km, got {} km",
            expected,
            DISTANCE_TOLERANCE,
            max_distance
        );
    }

    #[test]
    fn find_max_distance_for_new_min_strength_on_small_sea_line() {
        let segment = LineSegment::with_length(Terrain::Sea.parameters(), 100.0);
        let line = Line::with_segments(0.0, once(segment));
        let max_distance = find_max_distance_for_line(43.2, &line);
        assert!(max_distance.is_err());
    }

    #[test]
    fn find_max_distance_for_new_min_strength_on_medium_sea_line() {
        let segment = LineSegment::with_length(Terrain::Sea.parameters(), 500.0);
        let line = Line::with_segments(0.0, once(segment));
        let max_distance = find_max_distance_for_line(43.2, &line);
        assert!(max_distance.is_err());
    }

    #[test]
    fn find_max_distance_for_new_min_strength_on_biggest_sea_line() {
        let segment = LineSegment::with_length(Terrain::Sea.parameters(), 10000.0);
        let line = Line::with_segments(0.0, once(segment));
        let max_distance = find_max_distance_for_line(43.2, &line).unwrap();
        let expected = 749.350;
        let error = max_distance - expected;
        assert!(
            error.abs() < DISTANCE_TOLERANCE,
            "Expected max distance of {} +-{} km, got {} km",
            expected,
            DISTANCE_TOLERANCE,
            max_distance
        );
    }

    #[test]
    fn find_max_distance_for_ground_sea_line_far() {
        let segments = [
            LineSegment::with_length(Terrain::Ground.parameters(), 50.0),
            LineSegment::with_length(Terrain::Sea.parameters(), 200.0),
        ];
        let line = Line::with_segments(0.0, segments);
        let expected_distance = 193.9203;
        let field_strength = calc_field_strength_for_line_at_km(&line, expected_distance).unwrap();
        let max_distance = find_max_distance_for_line(field_strength, &line).expect("The distance it should find is within the bounds and has already been calculated to be valid.");
        let error = max_distance - expected_distance;
        assert!(
            error.abs() < DISTANCE_TOLERANCE,
            "Expected max distance of {} +-{} km, got {} km",
            expected_distance,
            DISTANCE_TOLERANCE,
            max_distance
        );
    }

    #[test]
    fn find_max_distance_for_ground_sea_line_near() {
        let segments = [
            LineSegment::with_length(Terrain::Ground.parameters(), 50.0),
            LineSegment::with_length(Terrain::Sea.parameters(), 200.0),
        ];
        let line = Line::with_segments(0.0, segments);
        let expected_distance = 42.356;
        let field_strength = calc_field_strength_for_line_at_km(&line, expected_distance).unwrap();
        let max_distance = find_max_distance_for_line(field_strength, &line).expect("The distance it should find is within the bounds and has already been calculated to be valid.");
        let error = max_distance - expected_distance;
        assert!(
            error.abs() < DISTANCE_TOLERANCE,
            "Expected max distance of {} +-{} km, got {} km",
            expected_distance,
            DISTANCE_TOLERANCE,
            max_distance
        );
    }

    #[test]
    fn find_max_distance_for_ground_sea_ground_line_far() {
        let segments = [
            LineSegment::with_length(Terrain::Ground.parameters(), 100.0),
            LineSegment::with_length(Terrain::Sea.parameters(), 50.0),
            LineSegment::with_length(Terrain::Ground.parameters(), 200.0),
        ];
        let line = Line::with_segments(0.0, segments);
        let expected_distance = 180.0;
        let field_strength = calc_field_strength_for_line_at_km(&line, expected_distance).unwrap();
        let max_distance = find_max_distance_for_line(field_strength, &line)
            .expect("The distance it should find is within the bounds and has already been calculated to be valid.");
        let error = max_distance - expected_distance;
        assert!(
            error.abs() < DISTANCE_TOLERANCE,
            "Expected max distance of {} +-{} km, got {} km",
            expected_distance,
            DISTANCE_TOLERANCE,
            max_distance
        );
    }

    #[test]
    fn find_max_distance_for_ground_sea_ground_line_medium() {
        let segments = [
            LineSegment::with_length(Terrain::Ground.parameters(), 100.0),
            LineSegment::with_length(Terrain::Sea.parameters(), 50.0),
            LineSegment::with_length(Terrain::Ground.parameters(), 200.0),
        ];
        let line = Line::with_segments(0.0, segments);
        // Fun fact: you can find the same field strength for this distance at ~115 km.
        // That's why this test isn't in the sea segment.
        let expected_distance = 82.096;
        let field_strength = calc_field_strength_for_line_at_km(&line, expected_distance).unwrap();
        let max_distance = find_max_distance_for_line(field_strength, &line)
            .expect("The distance it should find is within the bounds and has already been calculated to be valid.");
        let error = max_distance - expected_distance;
        assert!(
            error.abs() < DISTANCE_TOLERANCE,
            "Expected max distance of {} +-{} km, got {} km",
            expected_distance,
            DISTANCE_TOLERANCE,
            max_distance
        );
    }

    #[test]
    fn find_max_distance_for_ground_sea_ground_line_near() {
        let segments = [
            LineSegment::with_length(Terrain::Ground.parameters(), 100.0),
            LineSegment::with_length(Terrain::Sea.parameters(), 50.0),
            LineSegment::with_length(Terrain::Ground.parameters(), 200.0),
        ];
        let line = Line::with_segments(0.0, segments);
        let expected_distance = 32.637;
        let field_strength = calc_field_strength_for_line_at_km(&line, expected_distance).unwrap();
        let max_distance = find_max_distance_for_line(field_strength, &line)
            .expect("The distance it should find is within the bounds and has already been calculated to be valid.");
        let error = max_distance - expected_distance;
        assert!(
            error.abs() < DISTANCE_TOLERANCE,
            "Expected max distance of {} +-{} km, got {} km",
            expected_distance,
            DISTANCE_TOLERANCE,
            max_distance
        );
    }

    #[test]
    fn find_max_distance_for_long_4_segment_line() {
        let segments = [
            LineSegment::with_length(Terrain::Ground.parameters(), 100.0),
            LineSegment::with_length(Terrain::Sea.parameters(), 300.0),
            LineSegment::with_length(Terrain::Ground.parameters(), 100.0),
            LineSegment::with_length(Terrain::Sea.parameters(), 500.0),
        ];
        let line = Line::with_segments(0.0, segments);
        let expected_distance = 932.2594;
        let field_strength = calc_field_strength_for_line_at_km(&line, expected_distance).unwrap();
        let max_distance = find_max_distance_for_line(field_strength, &line)
            .expect("The distance it should find is within the bounds and has already been calculated to be valid.");
        let error = max_distance - expected_distance;
        assert!(
            error.abs() < DISTANCE_TOLERANCE,
            "Expected max distance of {} +-{} km, got {} km",
            expected_distance,
            DISTANCE_TOLERANCE,
            max_distance
        );
    }

    #[test]
    fn find_max_distance_for_empty_line() {
        let line = Line::new(0.0);
        let max_distance = find_max_distance_for_line(61.9, &line);
        assert!(max_distance.is_err());
    }
}

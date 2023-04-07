use crate::lfmf::{calc_LFMF, LFMF_Parameters};
use std::error::Error;

/// Find the maximum distance between the transmitter and the receiver for a given minimum usable field strength in dB(uV)/m.
///
/// # Arguments
/// * `min_usable_field_strength` - Minimum usable field strength in dB(uV)/m.
/// * `lfmf_parameters` - LFMF parameters. The distance field is ignored as the function uses its own values for it.
/// * `max_search_distance` - Maximum search distance in km.
///
/// # Returns
/// A result of either the maximum distance in km where the field strength doesn't fall below the minimum or an error.
pub fn find_max_distance(
    min_usable_field_strength: f64,
    mut lfmf_parameters: LFMF_Parameters,
    max_search_distance: f64,
) -> Result<f64, Box<dyn Error>> {
    const FIELD_STRENGTH_DB_TOLERANCE: f64 = 0.0001;
    const MINIMUM_STEP: f64 = 0.0001; // An accuracy of 10 cm is way more than enough given how approximated the results of LFMF are
    let lower_bound = min_usable_field_strength;
    let upper_bound = min_usable_field_strength + FIELD_STRENGTH_DB_TOLERANCE;

    lfmf_parameters.d__km = max_search_distance;
    let mut current_field_strength = calc_LFMF(lfmf_parameters)?.E_dBuVm;
    if current_field_strength > upper_bound {
        return Err("The distance where the minimum field strength value is at is greater than the maximum search distance".into());
    }

    // First we'll try to do a "binary search" for the position where the field strength is at using the tolerance.
    let mut step = lfmf_parameters.d__km / 2.0;
    while current_field_strength < lower_bound || upper_bound < current_field_strength {
        if current_field_strength < lower_bound {
            lfmf_parameters.d__km -= step;
        } else {
            lfmf_parameters.d__km += step;
        }
        current_field_strength = calc_LFMF(lfmf_parameters)?.E_dBuVm;
        step /= 2.0;
        if step < MINIMUM_STEP {
            break; // Binary search failed probably due to the discontinuity at the point where LFMF switches calculation methods.
        }
    }

    // Check if the binary search failed and adjust closer to the transmitter until the minimum field strength is reached.
    while current_field_strength < lower_bound {
        lfmf_parameters.d__km -= MINIMUM_STEP;
        current_field_strength = calc_LFMF(lfmf_parameters)?.E_dBuVm;
    }

    Ok(lfmf_parameters.d__km)
}

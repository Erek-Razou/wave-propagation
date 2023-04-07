use crate::terrain::defaults::Terrain;
use crate::terrain::{Line, LineSegment};
mod cli;
mod find_max_distance;
mod lfmf;
mod terrain;

fn main() {
    let cli = cli::parse();

    println!("Hello, world!");

    let min_e = cli.min_field_strength;
    let max_search_d = 10000.0;

    let segments = vec![
        LineSegment {
            lfmf_parameters: Terrain::Ground.parameters(),
            end_distance_km: 20.0,
        },
        LineSegment {
            lfmf_parameters: Terrain::Sea.parameters(),
            end_distance_km: 100.0,
        },
        LineSegment {
            lfmf_parameters: Terrain::Ground.parameters(),
            end_distance_km: 400.0,
        },
    ];
    let line = Line::from_segments(0.0, segments);
    println!("{line:?}");

    let test_parameters = Terrain::Sea.parameters();

    let result = find_max_distance::find_max_distance(min_e, test_parameters, max_search_d);

    if let Ok(max_d) = result {
        println!("Maximum usable distance is {max_d} km.");
        let mut parameters = test_parameters;
        parameters.d__km = max_d;
        println!(
            "Field strength is {} dB(uV)/m.",
            lfmf::calc_LFMF(parameters)
                .expect("It has already been calculated as a valid value")
                .E_dBuVm
        );
    } else {
        println!("{result:?}");
    }
}

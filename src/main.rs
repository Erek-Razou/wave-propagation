use crate::lfmf::LFMF_Parameters;

mod cli;
mod find_max_distance;
mod lfmf;

const GROUND_PARAMETERS: LFMF_Parameters = LFMF_Parameters {
    h_tx__meter: 10.0,
    h_rx__meter: 10.0,
    f__mhz: 1.0,
    P_tx__watt: 10000.0,
    N_s: 300.0,
    d__km: 300.0,
    epsilon: 22.0,
    sigma: 0.003,
    pol: 1,
};

const SEA_PARAMETERS: LFMF_Parameters = LFMF_Parameters {
    h_tx__meter: 10.0,
    h_rx__meter: 10.0,
    f__mhz: 1.0,
    P_tx__watt: 10000.0,
    N_s: 300.0,
    d__km: 300.0,
    epsilon: 70.0,
    sigma: 5.0,
    pol: 1,
};

fn main() {
    let cli = cli::parse();

    println!("Hello, world!");

    let min_e = cli.min_field_strength;
    let max_search_d = cli.max_search_distance;
    let test_parameters = GROUND_PARAMETERS;

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

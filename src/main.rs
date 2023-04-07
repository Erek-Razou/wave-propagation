mod cli;
mod defaults_for_lfmf;
mod find_max_distance;
mod lfmf;

fn main() {
    let cli = cli::parse();

    println!("Hello, world!");

    let min_e = cli.min_field_strength;
    let max_search_d = cli.max_search_distance;
    let test_parameters = defaults_for_lfmf::GROUND_PARAMETERS;

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

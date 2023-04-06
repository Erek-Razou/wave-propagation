use crate::lfmf::LFMF_Parameters;

mod lfmf;

fn main() {
    println!("Hello, world!");

    let lfmf_parameters = LFMF_Parameters {
        h_tx__meter: 10.0,
        h_rx__meter: 10.0,
        f__mhz: 1.0,
        P_tx__watt: 1000.0,
        N_s: 300.0,
        d__km: 300.0,
        epsilon: 70.0,
        sigma: 5.0,
        pol: 1,
    };

    let result = lfmf::calc_LFMF(lfmf_parameters);
    println!("{:?}", result);
}

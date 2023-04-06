mod LFMF;

fn main() {
    println!("Hello, world!");
    let h_tx__meter = 10.0;
    let h_rx__meter = 10.0;
    let f__mhz = 1.0;
    let P_tx__watt = 1000.0;
    let N_s = 300.0;
    let d__km = 300.0;
    let epsilon = 70.0;
    let sigma = 5.0;
    let pol = 1;

    let result = LFMF::calc_LFMF(
        h_tx__meter,
        h_rx__meter,
        f__mhz,
        P_tx__watt,
        N_s,
        d__km,
        epsilon,
        sigma,
        pol,
    );
    println!("{:?}", result);
}

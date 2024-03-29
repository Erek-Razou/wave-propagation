use crate::lfmf::LFMF_Parameters;

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

#[derive(Debug, Copy, Clone)]
pub enum Terrain {
    Ground,
    Sea,
}

impl Terrain {
    pub const fn parameters(&self) -> LFMF_Parameters {
        match self {
            Self::Ground => GROUND_PARAMETERS,
            Self::Sea => SEA_PARAMETERS,
        }
    }
}

use crate::lfmf::LFMF_Parameters;

#[derive(Debug, Clone, Copy)]
pub struct LineSegment {
    /// In contained is the distance in km that the segment spans.
    lfmf_parameters: LFMF_Parameters,
}

impl LineSegment {
    pub fn new(lfmf_parameters: LFMF_Parameters) -> Self {
        Self { lfmf_parameters }
    }
    pub fn with_length(mut lfmf_parameters: LFMF_Parameters, length_km: f64) -> Self {
        lfmf_parameters.d__km = length_km;
        Self { lfmf_parameters }
    }

    pub fn lfmf_parameters(&self) -> LFMF_Parameters {
        self.lfmf_parameters
    }
    pub fn length_km(&self) -> f64 {
        self.lfmf_parameters.d__km
    }
    pub fn set_length_km(&mut self, distance_km: f64) {
        self.lfmf_parameters.d__km = distance_km;
    }
}

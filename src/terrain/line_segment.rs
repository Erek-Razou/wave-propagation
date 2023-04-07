use crate::lfmf::LFMF_Parameters;

#[derive(Debug, Clone, Copy)]
pub struct LineSegment {
    pub lfmf_parameters: LFMF_Parameters,
    /// The distance in km from the transmitter to where the segment ends.
    /// It's specified in such a way for easier definition for a human when combining multiple segments to a line.
    pub end_distance_km: f64,
}
impl PartialEq for LineSegment {
    fn eq(&self, other: &Self) -> bool {
        self.end_distance_km.eq(&other.end_distance_km)
    }
}
impl PartialOrd for LineSegment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.end_distance_km.partial_cmp(&other.end_distance_km)
    }
}
impl Eq for LineSegment {}
impl Ord for LineSegment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.end_distance_km.total_cmp(&other.end_distance_km)
    }
}

use crate::terrain::line_segment::LineSegment;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct Line {
    angle: f64,
    segments: BTreeSet<LineSegment>,
}

impl Line {
    pub fn new(angle: f64) -> Self {
        Self {
            angle,
            segments: BTreeSet::new(),
        }
    }

    pub fn from_segments<I: IntoIterator<Item = LineSegment>>(
        angle: f64,
        segments_iter: I,
    ) -> Self {
        Self {
            angle,
            segments: BTreeSet::from_iter(segments_iter),
        }
    }
}

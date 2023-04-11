use crate::terrain::line_segment::LineSegment;

#[derive(Debug, Clone)]
pub struct Line {
    angle: f64,
    segments: Vec<LineSegment>,
}

impl Line {
    pub fn new(angle: f64) -> Self {
        Self {
            angle,
            segments: Vec::new(),
        }
    }
    pub fn with_segments<I: IntoIterator<Item = LineSegment>>(
        angle: f64,
        segments_iter: I,
    ) -> Self {
        Self {
            angle,
            segments: Vec::from_iter(segments_iter),
        }
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }

    pub fn segments(&self) -> impl Iterator<Item = &LineSegment> {
        self.segments.iter()
    }

    pub fn add_segment(&mut self, segment: LineSegment) {
        self.segments.push(segment);
    }

    pub fn max_distance(&self) -> f64 {
        self.segments.iter().map(LineSegment::length_km).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    pub fn segments_until(&self, end_distance_km: f64) -> Option<Vec<LineSegment>> {
        let mut segments = Vec::new();
        let mut last_distance = 0.0;
        let mut new_distance = 0.0;
        for segment in &self.segments {
            new_distance += segment.length_km();
            segments.push(*segment);
            if new_distance >= end_distance_km {
                break;
            }
            last_distance = new_distance;
        }
        if new_distance < end_distance_km {
            return None;
        } else if new_distance > end_distance_km {
            segments
                .last_mut()?
                .set_length_km(end_distance_km - last_distance);
        }
        Some(segments)
    }
}

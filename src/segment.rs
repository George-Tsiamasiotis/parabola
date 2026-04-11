//! A line segment in the 2-dimensional Cartesian space

use crate::Point;

/// A line segment in the 2-dimensional Cartesian space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment {
    /// The segment's start point.
    pub start: Point,
    /// The segment's end point.
    pub end: Point,
}

impl Segment {
    /// Calculates the length of the segment.
    ///
    /// # Example
    /// ```
    /// # use parabola::*;
    /// let segment = Segment {
    ///     start: Point { x: 0.0, y: 0.0 },
    ///     end: Point { x: 1.0, y: 1.0 },
    /// };
    /// approx::assert_relative_eq!(segment.length(), 2.0f64.sqrt());
    /// ```
    #[inline]
    #[must_use]
    pub fn length(&self) -> f64 {
        ((self.end.x - self.start.x).powi(2) + (self.end.y - self.start.y).powi(2)).sqrt()
    }
}

#[cfg(test)]
mod test_segment {
    use super::*;
    use approx::assert_relative_eq;

    const EPS: f64 = 1e-30;

    #[test]
    fn length() {
        let seg = Segment {
            start: Point { x: 0.0, y: 0.0 },
            end: Point { x: 0.0, y: 1.0 },
        };
        assert_relative_eq!(seg.length(), 1.0, epsilon = EPS);
        let seg = Segment {
            start: Point { x: 0.0, y: 0.0 },
            end: Point { x: 1.0, y: 1.0 },
        };
        assert_relative_eq!(seg.length(), 2.0f64.sqrt(), epsilon = EPS);
        let seg = Segment {
            start: Point {
                x: 1234.0,
                y: 5678.0,
            },
            end: Point {
                x: 8765.0,
                y: 4321.0,
            },
        };
        assert_relative_eq!(seg.length(), 7652.281359176491, epsilon = EPS);
    }
}

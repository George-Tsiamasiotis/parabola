//! Representation of a straight line of the form `ax + b`.

use crate::Point;

/// Representation of a straight line of the form `ax + b`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    /// The line's slope.
    pub slope: f64,
    /// The y-coordinate of the line's intercept with the y-axis
    pub intercept: f64,
}

/// Intercepts of a [`Parabola`](crate::Parabola) with a [`Line`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineIntercepts {
    /// Line does not intercept the Parabola.
    NoIntercepts,
    /// Line intercepts the Parabola at a single point.
    OneIntercept(Point),
    /// Line intercepts the Parabola at two points.
    TwoIntercepts(Point, Point),
}

impl Line {
    /// Creates a new line from its slope and its y-axis intercept.
    ///
    /// # Example
    /// ```
    /// # use parabola::*;
    /// // y = 2x + 5
    /// let line = Line::from_slope_intercept(2.0, 5.0);
    /// ```
    #[inline]
    #[must_use]
    pub fn from_slope_intercept(slope: f64, intercept: f64) -> Self {
        Self { slope, intercept }
    }

    /// Creates a new line from its slope and its y-axis intercept.
    ///
    /// # Example
    /// ```
    /// # use parabola::*;
    /// // y = 2x + 5
    /// let slope = 5.0;
    /// let point = Point{ x: 1.0, y: 7.0 };
    /// let line = Line::from_slope_point(2.0, point);
    /// ```
    #[inline]
    #[must_use]
    pub fn from_slope_point(slope: f64, point: Point) -> Self {
        Self {
            slope,
            intercept: point.y - slope * point.x,
        }
    }

    /// Creates a new line from 2 points.
    ///
    /// # Example
    /// ```
    /// # use parabola::*;
    /// // y = 2x + 5
    /// let point1 = Point{ x: 0.0, y: 5.0 };
    /// let point2 = Point{ x: 1.0, y: 7.0 };
    /// let line = Line::from_points(point1, point2);
    /// ```
    #[inline]
    #[must_use]
    pub fn from_points(point1: Point, point2: Point) -> Self {
        let slope = (point2.y - point1.y) / (point2.x - point1.x);
        Self {
            slope,
            intercept: point1.y - slope * point1.x,
        }
    }
}

impl Line {
    /// Evaluates the parabola at a specific `x`.
    ///
    /// # Example
    /// ```
    /// # use parabola::*;
    /// // y = 2x + 5
    /// let line = Line::from_slope_intercept(2.0, 5.0);
    /// approx::assert_relative_eq!(line.eval(-1.0), 3.0);
    /// ```
    #[inline]
    #[must_use]
    pub fn eval(&self, x: f64) -> f64 {
        self.slope * x + self.intercept
    }
}

#[cfg(test)]
mod test_line {
    use super::*;
    use approx::assert_relative_eq;

    const EPS: f64 = 1e-30;

    #[test]
    fn from_slope_intercept() {
        // y = 2x + 1
        let line = Line::from_slope_intercept(2.0, 1.0);
        assert_eq!(line.slope, 2.0);
        assert_eq!(line.intercept, 1.0);
        assert_relative_eq!(line.eval(0.5), 2.0, epsilon = EPS);
    }

    #[test]
    fn from_slope_point() {
        // y = 2x + 1
        let point = Point { x: 1.0, y: 3.0 };
        let line = Line::from_slope_point(2.0, point);
        assert_eq!(line.slope, 2.0);
        assert_eq!(line.intercept, 1.0);
        assert_relative_eq!(line.eval(0.5), 2.0, epsilon = EPS);
    }

    #[test]
    fn from_points() {
        // y = 2x + 1
        let point1 = Point { x: 0.0, y: 1.0 };
        let point2 = Point { x: 1.0, y: 3.0 };
        let line = Line::from_points(point1, point2);
        assert_eq!(line.slope, 2.0);
        assert_eq!(line.intercept, 1.0);
        assert_relative_eq!(line.eval(0.5), 2.0, epsilon = EPS);
    }
}

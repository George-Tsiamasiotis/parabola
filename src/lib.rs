//! ## Parabola
//!
//! Representation of a [`Parabola`] of the form `ax² + bx + c`.
//!
//! Provides methods for evaluation, calculation of critical points, point classification and point
//! projection
//!
//! The helper struct [`Line`], representing a straight line of the form `ax + b`, is also
//! provided.

use core::cmp::Ordering;

mod line;

pub use line::Line;

/// Representation of a parabola of the form `ax² + bx + c`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Parabola {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

/// Representation of a parabolas real roots, calculated from the quadratic formula.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Roots {
    /// Parabola has no real roots (Δ<0).
    NoRoots,
    /// Parabola has one real root (Δ=0).
    One(f64),
    /// Parabola has two distinct real roots (Δ>0).
    Two(f64, f64),
}

/// Representation of a point in the 2-dimensional Cartesian space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    /// The point's abscissa.
    pub x: f64,
    /// The point's ordinate.
    pub y: f64,
}

impl Parabola {
    /// Evaluates the parabola at a specific `x`.
    ///
    /// # Example
    ///
    /// ```
    /// # use parabola::*;
    /// let parabola = Parabola {
    ///     a: 1.0,
    ///     b: 4.0,
    ///     c: 3.0,
    /// };
    /// assert_eq!(parabola.eval(3.0), 24.0);
    /// ```
    #[inline]
    #[must_use]
    pub fn eval(&self, x: f64) -> f64 {
        // Faster
        x * (self.a * x + self.b) + self.c
    }

    /// Evaluates the parabola's first derivative at a specific `x`.
    ///
    /// # Example
    ///
    /// ```
    /// # use parabola::*;
    /// let parabola = Parabola {
    ///     a: 1.0,
    ///     b: 4.0,
    ///     c: 3.0,
    /// };
    /// assert_eq!(parabola.eval_deriv(3.0), 10.0);
    /// ```
    #[inline]
    #[must_use]
    pub fn eval_deriv(&self, x: f64) -> f64 {
        2.0 * self.a * x + self.b
    }

    /// Returns the parabola's constant second derivative.
    ///
    /// # Example
    ///
    /// ```
    /// # use parabola::*;
    /// let parabola = Parabola {
    ///     a: 1.0,
    ///     b: 4.0,
    ///     c: 3.0,
    /// };
    /// assert_eq!(parabola.deriv2(), 2.0);
    /// ```
    #[inline]
    #[must_use]
    #[doc(alias = "eval_deriv2")]
    pub fn deriv2(&self) -> f64 {
        2.0 * self.a
    }

    /// Calculates the roots (x-axis intercepts) of the parabola.
    ///
    /// If `Δ>=0`, the roots are returned in increasing order.
    ///
    /// + If `a` is zero, the root of `bx + c = 0` is returned.
    /// + If both `a` and `b` are zero, `c` is ignored and `NoRoots` is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use parabola::*;
    /// let parabola = Parabola {
    ///     a: 1.0,
    ///     b: 4.0,
    ///     c: 3.0,
    /// };
    /// let expected_roots = Roots::Two(-3.0, -1.0);
    /// assert_eq!(parabola.roots(), expected_roots);
    /// ```
    #[must_use]
    #[doc(alias = "x_intercepts")]
    pub fn roots(&self) -> Roots {
        let (a, b, c) = (self.a, self.b, self.c);
        if a == 0.0 {
            if b == 0.0 {
                // Constant case, either `c=0` or not
                return Roots::NoRoots;
            }
            // Linear case
            return Roots::One(-c / b);
        }

        // Quadratic formula
        let disc = b.powi(2) - 4.0 * a * c;
        match disc.total_cmp(&0.0) {
            Ordering::Less => Roots::NoRoots,
            Ordering::Greater => {
                if b == 0.0 {
                    // Two opposite roots case
                    // `-c/a` is strictly positive here
                    debug_assert!((-c / a).is_sign_positive());
                    let root = (-c / a).sqrt();
                    Roots::Two(-root, root)
                } else {
                    // Two roots case
                    let signb = b.signum();
                    let temp = -b.midpoint(signb * disc.sqrt());
                    let (root1, root2) = (temp / a, c / temp);
                    if root1 < root2 {
                        Roots::Two(root1, root2)
                    } else {
                        Roots::Two(root2, root1)
                    }
                }
            }
            Ordering::Equal => Roots::One((-b) / (2.0 * a)),
        }
    }

    /// Calculates the x-position of the parabola's axis of symmetry, if it exists.
    ///
    /// The axis of symmetry is only defined in the case `a!=0`.
    ///
    /// This value coincides with the abscissa of the parabola's extremum.
    ///
    /// # Example
    ///
    /// ```
    /// # use parabola::*;
    /// let parabola = Parabola {
    ///     a: 1.0,
    ///     b: 4.0,
    ///     c: 3.0,
    /// };
    /// approx::assert_relative_eq!(
    ///     parabola.axis().unwrap(),
    ///     -2.0,
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn axis(&self) -> Option<f64> {
        if self.a == 0.0 {
            None
        } else {
            Some(-self.b / (2.0 * self.a))
        }
    }

    /// Calculates the y-axis intercept.
    ///
    /// # Example
    ///
    /// ```
    /// # use parabola::*;
    /// let parabola = Parabola {
    ///     a: 1.0,
    ///     b: 4.0,
    ///     c: 3.0,
    /// };
    /// approx::assert_relative_eq!(
    ///     parabola.y_intercept(),
    ///     3.0,
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn y_intercept(&self) -> f64 {
        self.c
    }

    /// Calculates the total minimum of the parabola, if it exists.
    ///
    /// For the minimum to exist, `a` must be strictly positive.
    ///
    /// # Example
    ///
    /// ```
    /// # use parabola::*;
    /// let parabola = Parabola {
    ///     a: 1.0,
    ///     b: 4.0,
    ///     c: 3.0,
    /// };
    /// approx::assert_relative_eq!(
    ///     parabola.minimum().unwrap(),
    ///     -1.0,
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn minimum(&self) -> Option<f64> {
        if self.a > 0.0 {
            self.axis().map(|axis| self.eval(axis))
        } else {
            None
        }
    }

    /// Calculates the total maximum of the parabola, if it exists.
    ///
    /// For the minimum to exist, `a` must be strictly negative.
    ///
    /// # Example
    ///
    /// ```
    /// # use parabola::*;
    /// let parabola = Parabola {
    ///     a: -2.0,
    ///     b: 3.0,
    ///     c: 5.0,
    /// };
    /// approx::assert_relative_eq!(
    ///     parabola.maximum().unwrap(),
    ///     6.125,
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn maximum(&self) -> Option<f64> {
        if self.a < 0.0 {
            self.axis().map(|axis| self.eval(axis))
        } else {
            None
        }
    }

    /// Calculates the parabola's vertex point.
    ///
    /// The vertex point is the point where the parabola intersects its axis of symmetry, and is
    /// the point where the parabola is most sharply curved. It is not defined in the case `a=0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use parabola::*;
    /// let parabola = Parabola {
    ///     a: -2.0,
    ///     b: 3.0,
    ///     c: 5.0,
    /// };
    ///
    /// let vertex= parabola.vertex().unwrap();
    /// approx::assert_relative_eq!(vertex.x, 3.0 / 4.0);
    /// approx::assert_relative_eq!(vertex.y, 49.0 / 8.0);
    /// ```
    #[inline]
    #[must_use]
    pub fn vertex(&self) -> Option<Point> {
        self.axis().map(|axis| Point {
            x: axis,
            y: {
                let (a, b, c) = (self.a, self.b, self.c);
                (4.0 * a * c - b.powi(2)) / (4.0 * a)
            },
        })
    }

    /// Calculates the parabola's focus point.
    ///
    /// The focus point, along with the [`directix`] define the parabola as the set of points that
    /// are equidistant from both the focus point and the directix. It is not defined in the case
    /// `a=0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use parabola::*;
    /// let parabola = Parabola {
    ///     a: 2.0,
    ///     b: 8.0,
    ///     c: 6.0,
    /// };
    ///
    /// let focus = parabola.focus().unwrap();
    /// approx::assert_relative_eq!(focus.x, -2.0);
    /// approx::assert_relative_eq!(focus.y, -15.0 / 8.0);
    /// ```
    ///
    /// [`directix`]: Parabola::directix
    #[inline]
    #[must_use]
    pub fn focus(&self) -> Option<Point> {
        self.axis().map(|axis| Point {
            x: axis,
            y: {
                let (a, b, c) = (self.a, self.b, self.c);
                (4.0 * a * c - b.powi(2) + 1.0) / (4.0 * a)
            },
        })
    }

    /// Calculates the parabola's directix, defined as a `y=const` line.
    ///
    /// The directix, along with the [`focus point`], define the parabola as the set of points that
    /// are equidistant from both the focus point and the directix. It is not defined in the
    /// case `a=0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use parabola::*;
    /// let parabola = Parabola {
    ///     a: 2.0,
    ///     b: 4.0,
    ///     c: 5.0,
    /// };
    ///
    /// let directix = parabola.directix().unwrap();
    /// assert_eq!(directix.slope, 0.0);
    /// approx::assert_relative_eq!(directix.intercept, 23.0 / 8.0);
    /// ```
    ///
    /// [`focus point`]: Parabola::focus
    #[inline]
    #[must_use]
    pub fn directix(&self) -> Option<Line> {
        if self.a == 0.0 {
            None
        } else {
            let (a, b, c) = (self.a, self.b, self.c);
            Some(Line {
                slope: 0.0,
                intercept: (4.0 * a * c - b.powi(2) - 1.0) / (4.0 * a),
            })
        }
    }

    /// Calculates the parabola's focal length.
    ///
    /// The focal length is defined as the distance between the parabola's [`vertex`] and
    /// [`focus`] points.
    ///
    /// # Example
    ///
    /// ```
    /// # use parabola::*;
    /// let parabola = Parabola {
    ///     a: 2.0,
    ///     b: 8.0,
    ///     c: 6.0,
    /// };
    ///
    /// let focal_length = parabola.focal_length().unwrap();
    /// approx::assert_relative_eq!(focal_length, 1.0 / 8.0);
    /// ```
    ///
    /// [`vertex`]: Parabola::vertex
    /// [`focus`]: Parabola::focus
    #[inline]
    #[must_use]
    pub fn focal_length(&self) -> Option<f64> {
        if self.a == 0.0 {
            None
        } else {
            Some((1.0 / (4.0 * self.a)).abs())
        }
    }

    /// Calculates the vertical projection of a point onto the parabola.
    ///
    /// # Example
    ///
    /// ```
    /// # use parabola::*;
    /// let parabola = Parabola {
    ///     a: 1.0,
    ///     b: 0.0,
    ///     c: 5.0,
    /// };
    ///
    /// let projection = parabola.project(Point{ x: 1.0, y: 12.0 });
    /// approx::assert_relative_eq!(projection.x, 1.0);
    /// approx::assert_relative_eq!(projection.y, 6.0);
    /// ```
    #[inline]
    #[must_use]
    pub fn project(&self, point: Point) -> Point {
        Point {
            x: point.x,
            y: self.eval(point.x),
        }
    }

    /// Returns `true` if the `point` is inside the parabola's opening.
    ///
    /// Whether or not a point is contained in a parabola's opening is decided by calculating the
    /// point's vertical projection onto the curve and comparing their y-coordinates.
    ///
    /// # Panics
    ///
    /// Panics if the parabola's `a` coefficient is zero.
    ///
    /// # Example
    ///
    /// ```
    /// # use parabola::*;
    /// let parabola = Parabola {
    ///     a: 1.0,
    ///     b: 0.0,
    ///     c: 5.0,
    /// };
    ///
    /// assert!(parabola.contains(Point{ x: 0.0, y: 6.0 }));
    /// assert!(!parabola.contains(Point{ x: 1.0, y: -2.0 }));
    /// ```
    #[inline]
    #[must_use]
    pub fn contains(&self, point: Point) -> bool {
        match self.a.total_cmp(&0.0) {
            Ordering::Equal => panic!("Zero 'a' coefficient encountered"),
            Ordering::Less => {
                // Downward opening case
                let proj = self.project(point);
                point.y <= proj.y
            }
            Ordering::Greater => {
                // Upward opening case
                let proj = self.project(point);
                point.y >= proj.y
            }
        }
    }

    /// Returns the tangent line to the parabola at `x`.
    ///
    /// If the parabola's `a` coefficient is zero, the returned line is equivalent to the parabola.
    ///
    /// # Example
    ///
    /// ```
    /// # use parabola::*;
    /// let parabola = Parabola {
    ///     a: 2.0,
    ///     b: 8.0,
    ///     c: 6.0,
    /// };
    /// let tangent = parabola.tangent(-4.0);
    /// approx::assert_relative_eq!(tangent.slope, -8.0);
    /// approx::assert_relative_eq!(tangent.intercept, -26.0);
    /// ```
    #[inline]
    #[must_use]
    pub fn tangent(&self, x: f64) -> Line {
        let point = Point { x, y: self.eval(x) };
        Line::from_slope_point(self.eval_deriv(x), point)
    }
}

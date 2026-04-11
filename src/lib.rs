#![doc = include_str!("../README.md")]

use core::cmp::Ordering;

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
    /// assert_eq!(parabola.axis(), Some(-2.0));
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
    /// assert_eq!(parabola.y_intercept(), 3.0);
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
    /// assert_eq!(parabola.minimum(), Some(-1.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn minimum(&self) -> Option<f64> {
        if self.a > 0.0 {
            match self.axis() {
                Some(axis) => Some(self.eval(axis)),
                None => unreachable!("'a' is nonzero"),
            }
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
    /// assert_eq!(parabola.maximum(), Some(6.125));
    /// ```
    #[inline]
    #[must_use]
    pub fn maximum(&self) -> Option<f64> {
        if self.a < 0.0 {
            match self.axis() {
                Some(axis) => Some(self.eval(axis)),
                None => unreachable!("'a' is nonzero"),
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test_parabola {
    use super::*;
    use approx::assert_relative_eq;

    const EPS: f64 = 1e-30;

    #[test]
    fn desmos_2root_up_parabola() {
        let p = Parabola {
            a: 2.0,
            b: 8.0,
            c: 6.0,
        };
        assert_relative_eq!(p.eval(3.0), 48.0, epsilon = EPS);
        match p.roots() {
            Roots::Two(root1, root2) => {
                assert_relative_eq!(root1, -3.0, epsilon = EPS);
                assert_relative_eq!(root2, -1.0, epsilon = EPS);
            }
            _ => panic!("Expected 2 roots."),
        }
        assert_relative_eq!(p.axis().unwrap(), -2.0, epsilon = EPS);
        assert_relative_eq!(p.y_intercept(), 6.0, epsilon = EPS);
        assert_relative_eq!(p.minimum().unwrap(), -2.0, epsilon = EPS);
        assert!(p.maximum().is_none());
    }

    #[test]
    fn desmos_2root_down_parabola() {
        let p = Parabola {
            a: -2.0,
            b: 3.0,
            c: 5.0,
        };
        assert_relative_eq!(p.eval(3.0), -4.0, epsilon = EPS);
        match p.roots() {
            Roots::Two(root1, root2) => {
                assert_relative_eq!(root1, -1.0, epsilon = EPS);
                assert_relative_eq!(root2, 2.5, epsilon = EPS);
            }
            _ => panic!("Expected 2 roots."),
        }
        assert_relative_eq!(p.axis().unwrap(), 0.75, epsilon = EPS);
        assert_relative_eq!(p.y_intercept(), 5.0, epsilon = EPS);
        assert_relative_eq!(p.maximum().unwrap(), 6.125, epsilon = EPS);
        assert!(p.minimum().is_none());
    }

    #[test]
    fn symmetric_2root_parabola() {
        let p = Parabola {
            a: 1.0,
            b: 0.0,
            c: -4.0,
        };
        assert_relative_eq!(p.eval(3.0), 5.0, epsilon = EPS);
        match p.roots() {
            Roots::Two(root1, root2) => {
                assert_relative_eq!(root1, -2.0, epsilon = EPS);
                assert_relative_eq!(root2, 2.0, epsilon = EPS);
            }
            _ => panic!("Expected 2 roots."),
        }
        assert_relative_eq!(p.axis().unwrap(), 0.0, epsilon = EPS);
        assert_relative_eq!(p.y_intercept(), -4.0, epsilon = EPS);
        assert_relative_eq!(p.minimum().unwrap(), -4.0, epsilon = EPS);
        assert!(p.maximum().is_none());
    }

    #[test]
    fn single_root_parabola() {
        let p = Parabola {
            a: 1.0,
            b: 4.0,
            c: 4.0,
        };
        assert_relative_eq!(p.eval(3.0), 25.0, epsilon = EPS);
        assert_eq!(p.roots(), Roots::One(-2.0));
        match p.roots() {
            Roots::One(root) => {
                assert_relative_eq!(root, -2.0, epsilon = EPS);
            }
            _ => panic!("Expected 1 root."),
        }
        assert_relative_eq!(p.axis().unwrap(), -2.0, epsilon = EPS);
        assert_relative_eq!(p.y_intercept(), 4.0, epsilon = EPS);
        assert_relative_eq!(p.minimum().unwrap(), 0.0, epsilon = EPS);
        assert!(p.maximum().is_none());
    }

    #[test]
    fn non_intercepting_up_parabola() {
        let p = Parabola {
            a: 1.0,
            b: 2.0,
            c: 100.0,
        };
        assert_relative_eq!(p.eval(3.0), 115.0, epsilon = EPS);
        assert_eq!(p.roots(), Roots::NoRoots);
        assert_relative_eq!(p.axis().unwrap(), -1.0, epsilon = EPS);
        assert_relative_eq!(p.y_intercept(), 100.0, epsilon = EPS);
        assert_relative_eq!(p.minimum().unwrap(), 99.0, epsilon = EPS);
        assert!(p.maximum().is_none());
    }

    #[test]
    fn non_intercepting_down_parabola() {
        let p = Parabola {
            a: -1.0,
            b: -2.0,
            c: -100.0,
        };
        assert_relative_eq!(p.eval(3.0), -115.0, epsilon = EPS);
        assert_eq!(p.roots(), Roots::NoRoots);
        assert_relative_eq!(p.axis().unwrap(), -1.0, epsilon = EPS);
        assert_relative_eq!(p.y_intercept(), -100.0, epsilon = EPS);
        assert_relative_eq!(p.maximum().unwrap(), -99.0, epsilon = EPS);
        assert!(p.minimum().is_none());
    }

    #[test]
    fn zero_a_parabola() {
        let p = Parabola {
            a: 0.0,
            b: 2.0,
            c: 3.0,
        };
        assert_relative_eq!(p.eval(3.0), 9.0, epsilon = EPS);
        assert_eq!(p.roots(), Roots::One(-3.0 / 2.0));
        assert!(p.axis().is_none());
        assert_relative_eq!(p.y_intercept(), 3.0, epsilon = EPS);
        assert!(p.minimum().is_none());
        assert!(p.maximum().is_none());
    }

    #[test]
    fn zero_ab_parabola() {
        let p = Parabola {
            a: 0.0,
            b: 0.0,
            c: 3.0,
        };
        assert_relative_eq!(p.eval(3.0), 3.0, epsilon = EPS);
        assert_eq!(p.roots(), Roots::NoRoots);
        assert!(p.axis().is_none());
        assert_relative_eq!(p.y_intercept(), 3.0, epsilon = EPS);
        assert!(p.minimum().is_none());
        assert!(p.maximum().is_none());
    }

    #[test]
    fn zero_abc_parabola() {
        let p = Parabola {
            a: 0.0,
            b: 0.0,
            c: 0.0,
        };
        assert_relative_eq!(p.eval(3.0), 0.0, epsilon = EPS);
        assert_eq!(p.roots(), Roots::NoRoots);
        assert!(p.axis().is_none());
        assert_relative_eq!(p.y_intercept(), 0.0, epsilon = EPS);
        assert!(p.minimum().is_none());
        assert!(p.maximum().is_none());
    }
}

//! GNU Scientific Library's `gsl_poly_solve_quadratic()` tests.
//!
//! Located in `gsl/poly/test.c`.

use approx::assert_relative_eq;
use parabola::*;

const EPS: f64 = 1e-30;

#[test]
fn quadratic1() {
    let parabola = Parabola {
        a: 4.0,
        b: -20.0,
        c: 26.0,
    };
    assert_eq!(parabola.roots(), Roots::NoRoots);
}

#[test]
fn quadratic2() {
    let parabola = Parabola {
        a: 4.0,
        b: -20.0,
        c: 25.0,
    };
    match parabola.roots() {
        Roots::One(root) => assert_relative_eq!(root, 2.5, epsilon = EPS),
        _ => panic!("Expected 1 root."),
    }
}

#[test]
fn quadratic3() {
    let parabola = Parabola {
        a: 4.0,
        b: -20.0,
        c: 21.0,
    };
    match parabola.roots() {
        Roots::Two(root1, root2) => {
            assert_relative_eq!(root1, 1.5, epsilon = EPS);
            assert_relative_eq!(root2, 3.5, epsilon = EPS);
        }
        _ => panic!("Expected 2 roots."),
    }
}

#[test]
fn quadratic4() {
    let parabola = Parabola {
        a: 4.0,
        b: 7.0,
        c: 0.0,
    };
    match parabola.roots() {
        Roots::Two(root1, root2) => {
            assert_relative_eq!(root1, -1.75, epsilon = EPS);
            assert_relative_eq!(root2, 0.0, epsilon = EPS);
        }
        _ => panic!("Expected 2 roots."),
    }
}

#[test]
fn quadratic5() {
    let parabola = Parabola {
        a: 0.0,
        b: 3.0,
        c: -21.0,
    };
    match parabola.roots() {
        Roots::One(root) => assert_relative_eq!(root, 7.0, epsilon = EPS),
        _ => panic!("Expected 2 roots."),
    }
}

#[test]
fn quadratic6() {
    let parabola = Parabola {
        a: 0.0,
        b: 0.0,
        c: 1.0,
    };
    assert_eq!(parabola.roots(), Roots::NoRoots);
}

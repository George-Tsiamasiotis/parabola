//! Value testing of all Parabola's methods.

use approx::assert_relative_eq;
use parabola::*;

const EPS: f64 = 1e-30;

#[test]
fn desmos_2root_up_parabola() {
    let p = Parabola {
        a: 2.0,
        b: 8.0,
        c: 6.0,
    };
    assert_relative_eq!(p.eval(3.0), 48.0, epsilon = EPS);
    assert_relative_eq!(p.eval_deriv(3.0), 20.0, epsilon = EPS);
    assert_relative_eq!(p.deriv2(), 4.0, epsilon = EPS);
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
    match p.vertex() {
        Some(vertex) => {
            assert_relative_eq!(vertex.x, -2.0, epsilon = EPS);
            assert_relative_eq!(vertex.y, -2.0, epsilon = EPS);
        }
        None => panic!("Vertex point is well-defined"),
    }
    match p.focus() {
        Some(focus) => {
            assert_relative_eq!(focus.x, -2.0, epsilon = EPS);
            assert_relative_eq!(focus.y, -15.0 / 8.0, epsilon = EPS);
        }
        None => panic!("Focus point is well-defined"),
    }
    assert_eq!(p.directix().unwrap().slope, 0.0);
    assert_relative_eq!(p.directix().unwrap().intercept, -17.0 / 8.0, epsilon = EPS);
    assert_relative_eq!(p.focal_length().unwrap(), 1.0 / 8.0, epsilon = EPS);
    {
        let projection = p.project(Point { x: -1.0, y: 3.0 });
        assert_relative_eq!(projection.x, -1.0, epsilon = EPS);
        assert_relative_eq!(projection.y, 0.0, epsilon = EPS);
    }
    assert!(p.contains(Point { x: -3.0, y: 3.0 }));
    assert!(!p.contains(Point { x: -3.0, y: -5.0 }));
    {
        let tangent = p.tangent(-4.0);
        assert_relative_eq!(tangent.slope, -8.0, epsilon = EPS);
        assert_relative_eq!(tangent.intercept, -26.0, epsilon = EPS);
    }
}

#[test]
fn desmos_2root_down_parabola() {
    let p = Parabola {
        a: -2.0,
        b: 3.0,
        c: 5.0,
    };
    assert_relative_eq!(p.eval(3.0), -4.0, epsilon = EPS);
    assert_relative_eq!(p.eval_deriv(3.0), -9.0, epsilon = EPS);
    assert_relative_eq!(p.deriv2(), -4.0, epsilon = EPS);
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
    match p.vertex() {
        Some(vertex) => {
            assert_relative_eq!(vertex.x, 3.0 / 4.0, epsilon = EPS);
            assert_relative_eq!(vertex.y, 49.0 / 8.0, epsilon = EPS);
        }
        None => panic!("Vertex point is well-defined"),
    }
    match p.focus() {
        Some(focus) => {
            assert_relative_eq!(focus.x, 3.0 / 4.0, epsilon = EPS);
            assert_relative_eq!(focus.y, 6.0, epsilon = EPS);
        }
        None => panic!("Focus point is well-defined"),
    }
    assert_eq!(p.directix().unwrap().slope, 0.0);
    assert_relative_eq!(p.directix().unwrap().intercept, 25.0 / 4.0, epsilon = EPS);
    assert_relative_eq!(p.focal_length().unwrap(), 1.0 / 8.0, epsilon = EPS);
    {
        let projection = p.project(Point { x: -1.0, y: 3.0 });
        assert_relative_eq!(projection.x, -1.0, epsilon = EPS);
        assert_relative_eq!(projection.y, 0.0, epsilon = EPS);
    }
    assert!(p.contains(Point { x: 1.0, y: 3.0 }));
    assert!(!p.contains(Point { x: -3.0, y: 3.0 }));
    {
        let tangent = p.tangent(0.0);
        assert_relative_eq!(tangent.slope, 3.0, epsilon = EPS);
        assert_relative_eq!(tangent.intercept, 5.0, epsilon = EPS);
    }
}

#[test]
fn symmetric_2root_parabola() {
    let p = Parabola {
        a: 1.0,
        b: 0.0,
        c: -4.0,
    };
    assert_relative_eq!(p.eval(3.0), 5.0, epsilon = EPS);
    assert_relative_eq!(p.eval_deriv(3.0), 6.0, epsilon = EPS);
    assert_relative_eq!(p.deriv2(), 2.0, epsilon = EPS);
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
    match p.vertex() {
        Some(vertex) => {
            assert_relative_eq!(vertex.x, 0.0, epsilon = EPS);
            assert_relative_eq!(vertex.y, -4.0, epsilon = EPS);
        }
        None => panic!("Vertex point is well-defined"),
    }
    match p.focus() {
        Some(focus) => {
            assert_relative_eq!(focus.x, 0.0, epsilon = EPS);
            assert_relative_eq!(focus.y, -15.0 / 4.0, epsilon = EPS);
        }
        None => panic!("Focus point is well-defined"),
    }
    assert_eq!(p.directix().unwrap().slope, 0.0);
    assert_relative_eq!(p.directix().unwrap().intercept, -17.0 / 4.0, epsilon = EPS);
    assert_relative_eq!(p.focal_length().unwrap(), 1.0 / 4.0, epsilon = EPS);
    {
        let projection = p.project(Point { x: -1.0, y: 3.0 });
        assert_relative_eq!(projection.x, -1.0, epsilon = EPS);
        assert_relative_eq!(projection.y, -3.0, epsilon = EPS);
    }
    assert!(p.contains(Point { x: 1.0, y: 3.0 }));
    assert!(!p.contains(Point { x: 4.0, y: 3.0 }));
    {
        let tangent = p.tangent(2.0);
        assert_relative_eq!(tangent.slope, 4.0, epsilon = EPS);
        assert_relative_eq!(tangent.intercept, -8.0, epsilon = EPS);
    }
}

#[test]
fn single_root_parabola() {
    let p = Parabola {
        a: 1.0,
        b: 4.0,
        c: 4.0,
    };
    assert_relative_eq!(p.eval(3.0), 25.0, epsilon = EPS);
    assert_relative_eq!(p.eval_deriv(3.0), 10.0, epsilon = EPS);
    assert_relative_eq!(p.deriv2(), 2.0, epsilon = EPS);
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
    match p.vertex() {
        Some(vertex) => {
            assert_relative_eq!(vertex.x, -2.0, epsilon = EPS);
            assert_relative_eq!(vertex.y, 0.0, epsilon = EPS);
        }
        None => panic!("Vertex point is well-defined"),
    }
    match p.focus() {
        Some(focus) => {
            assert_relative_eq!(focus.x, -2.0, epsilon = EPS);
            assert_relative_eq!(focus.y, 1.0 / 4.0, epsilon = EPS);
        }
        None => panic!("Focus point is well-defined"),
    }
    assert_eq!(p.directix().unwrap().slope, 0.0);
    assert_relative_eq!(p.directix().unwrap().intercept, -1.0 / 4.0, epsilon = EPS);
    assert_relative_eq!(p.focal_length().unwrap(), 1.0 / 4.0, epsilon = EPS);
    {
        let projection = p.project(Point { x: -1.0, y: 3.0 });
        assert_relative_eq!(projection.x, -1.0, epsilon = EPS);
        assert_relative_eq!(projection.y, 1.0, epsilon = EPS);
    }
    assert!(p.contains(Point { x: -1.0, y: 3.0 }));
    assert!(!p.contains(Point { x: 4.0, y: 3.0 }));
    {
        let tangent = p.tangent(0.0);
        assert_relative_eq!(tangent.slope, 4.0, epsilon = EPS);
        assert_relative_eq!(tangent.intercept, 4.0, epsilon = EPS);
    }
}

#[test]
fn non_intercepting_up_parabola() {
    let p = Parabola {
        a: 1.0,
        b: 2.0,
        c: 100.0,
    };
    assert_relative_eq!(p.eval(3.0), 115.0, epsilon = EPS);
    assert_relative_eq!(p.eval_deriv(3.0), 8.0, epsilon = EPS);
    assert_relative_eq!(p.deriv2(), 2.0, epsilon = EPS);
    assert_eq!(p.roots(), Roots::NoRoots);
    assert_relative_eq!(p.axis().unwrap(), -1.0, epsilon = EPS);
    assert_relative_eq!(p.y_intercept(), 100.0, epsilon = EPS);
    assert_relative_eq!(p.minimum().unwrap(), 99.0, epsilon = EPS);
    assert!(p.maximum().is_none());
    match p.vertex() {
        Some(vertex) => {
            assert_relative_eq!(vertex.x, -1.0, epsilon = EPS);
            assert_relative_eq!(vertex.y, 99.0, epsilon = EPS);
        }
        None => panic!("Vertex point is well-defined"),
    }
    match p.focus() {
        Some(focus) => {
            assert_relative_eq!(focus.x, -1.0, epsilon = EPS);
            assert_relative_eq!(focus.y, 397.0 / 4.0, epsilon = EPS);
        }
        None => panic!("Focus point is well-defined"),
    }
    assert_eq!(p.directix().unwrap().slope, 0.0);
    assert_relative_eq!(p.directix().unwrap().intercept, 395.0 / 4.0, epsilon = EPS);
    assert_relative_eq!(p.focal_length().unwrap(), 1.0 / 4.0, epsilon = EPS);
    {
        let projection = p.project(Point { x: -1.0, y: 3.0 });
        assert_relative_eq!(projection.x, -1.0, epsilon = EPS);
        assert_relative_eq!(projection.y, 99.0, epsilon = EPS);
    }
    assert!(p.contains(Point { x: -1.0, y: 120.0 }));
    assert!(!p.contains(Point { x: 5.0, y: 120.0 }));
    {
        let tangent = p.tangent(3.0);
        assert_relative_eq!(tangent.slope, 8.0, epsilon = EPS);
        assert_relative_eq!(tangent.intercept, 91.0, epsilon = EPS);
    }
}

#[test]
fn non_intercepting_down_parabola() {
    let p = Parabola {
        a: -1.0,
        b: -2.0,
        c: -100.0,
    };
    assert_relative_eq!(p.eval(3.0), -115.0, epsilon = EPS);
    assert_relative_eq!(p.eval_deriv(3.0), -8.0, epsilon = EPS);
    assert_relative_eq!(p.deriv2(), -2.0, epsilon = EPS);
    assert_eq!(p.roots(), Roots::NoRoots);
    assert_relative_eq!(p.axis().unwrap(), -1.0, epsilon = EPS);
    assert_relative_eq!(p.y_intercept(), -100.0, epsilon = EPS);
    assert_relative_eq!(p.maximum().unwrap(), -99.0, epsilon = EPS);
    assert!(p.minimum().is_none());
    match p.vertex() {
        Some(vertex) => {
            assert_relative_eq!(vertex.x, -1.0, epsilon = EPS);
            assert_relative_eq!(vertex.y, -99.0, epsilon = EPS);
        }
        None => panic!("Vertex point is well-defined"),
    }
    match p.focus() {
        Some(focus) => {
            assert_relative_eq!(focus.x, -1.0, epsilon = EPS);
            assert_relative_eq!(focus.y, -397.0 / 4.0, epsilon = EPS);
        }
        None => panic!("Focus point is well-defined"),
    }
    assert_eq!(p.directix().unwrap().slope, 0.0);
    assert_relative_eq!(p.directix().unwrap().intercept, -395.0 / 4.0, epsilon = EPS);
    assert_relative_eq!(p.focal_length().unwrap(), 1.0 / 4.0, epsilon = EPS);
    {
        let projection = p.project(Point { x: -1.0, y: 3.0 });
        assert_relative_eq!(projection.x, -1.0, epsilon = EPS);
        assert_relative_eq!(projection.y, -99.0, epsilon = EPS);
    }
    assert!(p.contains(Point { x: -1.0, y: -120.0 }));
    assert!(!p.contains(Point { x: 5.0, y: -120.0 }));
    {
        let tangent = p.tangent(3.0);
        assert_relative_eq!(tangent.slope, -8.0, epsilon = EPS);
        assert_relative_eq!(tangent.intercept, -91.0, epsilon = EPS);
    }
}

#[test]
fn zero_a_parabola() {
    let p = Parabola {
        a: 0.0,
        b: 2.0,
        c: 3.0,
    };
    assert_relative_eq!(p.eval(3.0), 9.0, epsilon = EPS);
    assert_relative_eq!(p.eval_deriv(3.0), 2.0, epsilon = EPS);
    assert_relative_eq!(p.deriv2(), 0.0, epsilon = EPS);
    assert_eq!(p.roots(), Roots::One(-3.0 / 2.0));
    assert!(p.axis().is_none());
    assert_relative_eq!(p.y_intercept(), 3.0, epsilon = EPS);
    assert!(p.minimum().is_none());
    assert!(p.maximum().is_none());
    assert!(p.vertex().is_none());
    assert!(p.focus().is_none());
    {
        let projection = p.project(Point { x: -1.0, y: 3.0 });
        assert_relative_eq!(projection.x, -1.0, epsilon = EPS);
        assert_relative_eq!(projection.y, 1.0, epsilon = EPS);
    }
    assert!(p.directix().is_none());
    assert!(p.focal_length().is_none());
    {
        let tangent = p.tangent(3.0);
        assert_relative_eq!(tangent.slope, p.b, epsilon = EPS);
        assert_relative_eq!(tangent.intercept, p.c, epsilon = EPS);
    }
}

#[test]
fn zero_ab_parabola() {
    let p = Parabola {
        a: 0.0,
        b: 0.0,
        c: 3.0,
    };
    assert_relative_eq!(p.eval(3.0), 3.0, epsilon = EPS);
    assert_relative_eq!(p.eval_deriv(3.0), 0.0, epsilon = EPS);
    assert_relative_eq!(p.deriv2(), 0.0, epsilon = EPS);
    assert_eq!(p.roots(), Roots::NoRoots);
    assert!(p.axis().is_none());
    assert_relative_eq!(p.y_intercept(), 3.0, epsilon = EPS);
    assert!(p.minimum().is_none());
    assert!(p.maximum().is_none());
    assert!(p.vertex().is_none());
    assert!(p.focus().is_none());
    {
        let projection = p.project(Point { x: -1.0, y: 4.0 });
        assert_relative_eq!(projection.x, -1.0, epsilon = EPS);
        assert_relative_eq!(projection.y, 3.0, epsilon = EPS);
    }
    assert!(p.directix().is_none());
    assert!(p.focal_length().is_none());
    {
        let tangent = p.tangent(3.0);
        assert_relative_eq!(tangent.slope, p.b, epsilon = EPS);
        assert_relative_eq!(tangent.intercept, p.c, epsilon = EPS);
    }
}

#[test]
fn zero_abc_parabola() {
    let p = Parabola {
        a: 0.0,
        b: 0.0,
        c: 0.0,
    };
    assert_relative_eq!(p.eval(3.0), 0.0, epsilon = EPS);
    assert_relative_eq!(p.eval_deriv(3.0), 0.0, epsilon = EPS);
    assert_relative_eq!(p.deriv2(), 0.0, epsilon = EPS);
    assert_eq!(p.roots(), Roots::NoRoots);
    assert!(p.axis().is_none());
    assert_relative_eq!(p.y_intercept(), 0.0, epsilon = EPS);
    assert!(p.minimum().is_none());
    assert!(p.maximum().is_none());
    assert!(p.vertex().is_none());
    assert!(p.focus().is_none());
    {
        let projection = p.project(Point { x: -1.0, y: 4.0 });
        assert_relative_eq!(projection.x, -1.0, epsilon = EPS);
        assert_relative_eq!(projection.y, 0.0, epsilon = EPS);
    }
    assert!(p.directix().is_none());
    assert!(p.focal_length().is_none());
    {
        let tangent = p.tangent(3.0);
        assert_relative_eq!(tangent.slope, p.b, epsilon = EPS);
        assert_relative_eq!(tangent.intercept, p.c, epsilon = EPS);
    }
}

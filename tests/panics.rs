//! Test Parabola's panic cases.

use parabola::*;

#[test]
#[should_panic]
fn zero_a_contains() {
    let _ = Parabola {
        a: 0.0,
        b: 2.0,
        c: 3.0,
    }
    .contains(Point { x: 1.0, y: 2.0 });
}

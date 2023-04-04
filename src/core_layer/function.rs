pub fn solve_quadratic(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
    let discr = b * b - 4.0 * a * c;
    if discr < 0.0 || a == 0.0 {
        return None;
    } else if discr == 0.0 {
        return Some((-0.5 * b / a, -0.5 * b / a));
    }
    let q = if b > 0.0 {
        -0.5 * (b + discr.sqrt())
    } else {
        -0.5 * (b - discr.sqrt())
    };
    let x0 = q / a;
    let x1 = c / q;
    if x0 < x1 {
        Some((x0, x1))
    } else {
        Some((x1, x0))
    }
}

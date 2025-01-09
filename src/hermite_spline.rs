use glam::Vec3;

pub fn hermite_spline(t: f32, p0: Vec3, m0: Vec3, t1: Vec3, p1: Vec3) -> Vec3 {
    let t2 = t * t;
    let t3 = t2 * t;
    let h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
    let h10 = t3 - 2.0 * t2 + t;
    let h01 = -2.0 * t3 + 3.0 * t2;
    let m1 = t3 - t2;
    h00 * p0 + h10 * m0 + h01 * p1 + m1 * t1
}
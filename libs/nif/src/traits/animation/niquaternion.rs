// http://number-none.com/product/Hacking%20Quaternions/

type Quat = nalgebra::Quaternion<f32>;

fn isqrt_approx_in_neighborhood(s: f32) -> f32 {
    const NEIGHBORHOOD: f32 = 0.959066;
    const NEIGHBORHOOD_SQRT: f32 = 0.979319;
    const SCALE: f32 = 1.000311;
    const ADDITIVE_CONSTANT: f32 = SCALE / NEIGHBORHOOD_SQRT;
    const FACTOR: f32 = SCALE * (-0.5 / (NEIGHBORHOOD * NEIGHBORHOOD_SQRT));
    (s - NEIGHBORHOOD).mul_add(FACTOR, ADDITIVE_CONSTANT)
}

fn fast_normalize(q: &mut Quat) {
    let s = q.dot(q);
    let mut k = isqrt_approx_in_neighborhood(s);
    if s <= 0.915212 {
        k *= isqrt_approx_in_neighborhood(k * k * s);
        if s <= 0.6521197 {
            k *= isqrt_approx_in_neighborhood(k * k * s);
        }
    }
    q.coords.scale_mut(k);
}

fn counter_warp(t: f32, cos: f32) -> f32 {
    const ATTENUATION: f32 = 0.8227969;
    const WORST_SLOPE: f32 = 0.5854922;

    let mut f = ATTENUATION.mul_add(-cos, 1.0);
    f *= f;
    f *= WORST_SLOPE;

    // t * (f * t * (2.0 * t - 3.0) + 1.0 + f)
    t * (f * t).mul_add(2f32.mul_add(t, -3.0), 1.0 + f)
}

pub fn slerp(q0: Quat, q1: Quat, t: f32) -> Quat {
    let cos = q0.dot(&q1);

    let t = {
        if t > 0.5 {
            1.0 - counter_warp(1.0 - t, cos)
        } else {
            counter_warp(t, cos)
        }
    };

    let mut q = q0.lerp(&q1, t);
    fast_normalize(&mut q);

    q
}

pub fn intermediate(prev: Quat, this: Quat, next: Quat) -> Quat {
    let inv = this.conjugate();
    let mut q = (inv * prev).ln() + (inv * next).ln();
    q.coords.scale_mut(-0.25);
    this * q.exp()
}

pub fn squad(q0: Quat, i0: Quat, i1: Quat, q1: Quat, t: f32) -> Quat {
    slerp(slerp(q0, q1, t), slerp(i0, i1, t), 2.0 * t * (1.0 - t))
}

use super::ndf::NDF;
use crate::core_layer::constants::INV_PI;
use crate::function_layer::V3f;
use cgmath::Vector2;

pub struct BeckmannDistribution;

impl BeckmannDistribution {
    fn get_g1(w_local: V3f, alpha: f32) -> f32 {
        let cos_jv = w_local.y;
        let tan_jv = (1.0 / (cos_jv * cos_jv) - 1.0).sqrt();
        let inv_a = alpha * tan_jv;
        let a = 1.0 / inv_a;
        if a < 1.6 {
            (3.535 + 2.181 * a) / (inv_a + 2.276 + 2.577 * a)
        } else {
            1.0
        }
    }
}

impl NDF for BeckmannDistribution {
    fn get_d(&self, wh_local: V3f, alpha: Vector2<f32>) -> f32 {
        let cos_j2 = wh_local.y * wh_local.y;
        let tan_j2 = 1.0 / cos_j2 - 1.0;
        let alpha2 = alpha.x * alpha.x;
        (-tan_j2 / alpha2).exp() * INV_PI / (alpha2 * cos_j2 * cos_j2)
    }

    fn get_g(&self, wo_local: V3f, wi_local: V3f, alpha: Vector2<f32>) -> f32 {
        BeckmannDistribution::get_g1(wo_local, alpha.x)
            * BeckmannDistribution::get_g1(wi_local, alpha.x)
    }

    fn pdf(&self, _wo_local: V3f, wh_local: V3f, alpha: Vector2<f32>) -> f32 {
        self.get_d(wh_local, alpha) * wh_local.y
    }

    fn sample_wh(&self, _wo_local: V3f, alpha: Vector2<f32>, sample: Vector2<f32>) -> V3f {
        let a = alpha.x;
        let tan_theta_2 = -a * a * (1.0 - sample.x).ln();
        let phi = sample.y * 2.0 * std::f32::consts::PI;

        let cos_theta = (1.0 / (1.0 + tan_theta_2)).sqrt();
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        V3f::new(sin_theta * phi.cos(), sin_theta * phi.sin(), cos_theta)
    }
}

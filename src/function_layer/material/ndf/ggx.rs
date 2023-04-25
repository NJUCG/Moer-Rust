use cgmath::Vector2;
use crate::core_layer::constants::INV_PI;
use crate::function_layer::V3f;
use super::ndf::NDF;

pub struct GGXDistribution;

impl GGXDistribution {
    fn get_g1(w_local: V3f, alpha: f32) -> f32 {
        let cos_jv = w_local.y;
        let tan_jv = (1.0 - cos_jv * cos_jv).sqrt() / cos_jv;
        let inv_a = alpha * tan_jv;
        2.0 / (1.0 + (1.0 + inv_a * inv_a).sqrt())
    }
}

impl NDF for GGXDistribution {
    fn get_d(&self, wh_local: V3f, alpha: Vector2<f32>) -> f32 {
        let cos_j = wh_local.y;
        let tan_j = (1.0 - cos_j * cos_j).sqrt() / cos_j;
        let d_sqrt = alpha.x / (cos_j * cos_j * (alpha.x + tan_j * tan_j));
        d_sqrt * d_sqrt * INV_PI
    }

    fn get_g(&self, wo_local: V3f, wi_local: V3f, alpha: Vector2<f32>) -> f32 {
        GGXDistribution::get_g1(wo_local, alpha.x) *
            GGXDistribution::get_g1(wi_local, alpha.x)
    }

    fn pdf(&self, _wo_local: V3f, wh_local: V3f, alpha: Vector2<f32>) -> f32 {
        self.get_d(wh_local, alpha) * wh_local.y
    }

    fn sample_wh(&self, _wo_local: V3f, alpha: Vector2<f32>, sample: Vector2<f32>) -> V3f {
        let a = alpha.x;
        let tan_theta_2 = a * a * sample.x / (1.0 - sample.x);
        let phi = sample.y * 2.0 * std::f32::consts::PI;

        let cos_theta = (1.0 / (1.0 + tan_theta_2)).sqrt();
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        V3f::new(sin_theta * phi.cos(), sin_theta * phi.sin(), cos_theta)
    }
}
use super::medium::{Medium, MediumInteraction};
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::medium::medium::HenyeyGreenstein;
use crate::function_layer::{MediumInterface, Ray, Sampler};
use cgmath::Array;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct HomogeneousMedium {
    // sigma_a: SpectrumRGB,
    sigma_s: SpectrumRGB,
    sigma_t: SpectrumRGB,
    g: f32,
}

impl Medium for HomogeneousMedium {
    fn tr(&self, ray: &Ray, _sampler: Rc<RefCell<dyn Sampler>>) -> SpectrumRGB {
        (self.sigma_t * -f32::MAX.min(ray.t_max)).exp()
    }

    fn sample(
        &self,
        ray: &Ray,
        sampler: Rc<RefCell<dyn Sampler>>,
        mi: &mut MediumInteraction,
    ) -> SpectrumRGB {
        let channel = ThreadRng::default().gen_range(0..3usize);
        let dist = -(1.0 - sampler.borrow_mut().next_1d()).ln() / self.sigma_t.rgb()[channel];
        let t = dist.min(ray.t_max);
        let sampled_medium = t < ray.t_max;
        if sampled_medium {
            mi.position = ray.at(t);
            mi.wo = -ray.direction;
            mi.time = ray.t;
            mi.medium_interface =
                MediumInterface::new(Some(Rc::new(self.clone())), Some(Rc::new(self.clone())));
            mi.phase = Some(Box::new(HenyeyGreenstein::new(self.g)));
        }
        let tr = (self.sigma_t * -f32::MAX.min(t)).exp();
        let density = if sampled_medium {
            self.sigma_t * tr
        } else {
            tr
        };
        let mut pdf = density.rgb().sum() / 3.0;
        if pdf == 0.0 {
            pdf = 1.0;
        }
        if sampled_medium {
            tr * self.sigma_s / pdf
        } else {
            tr / pdf
        }
    }
}

impl HomogeneousMedium {
    pub fn new(sigma_a: SpectrumRGB, sigma_s: SpectrumRGB, g: f32) -> Self {
        let sigma_t = sigma_s + sigma_a;
        Self {
            // sigma_a,
            sigma_s,
            sigma_t,
            g,
        }
    }
}

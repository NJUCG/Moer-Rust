#![allow(dead_code)]

use crate::core_layer::colorspace::SpectrumRGB;
use crate::core_layer::function::lerp;
use crate::core_layer::transform::Transform;
use crate::function_layer::medium::medium::{HenyeyGreenstein, Medium, MediumInteraction};
use crate::function_layer::{Bounds3, Ray, Sampler, V3f};
use cgmath::{Point3, Vector3};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct GridDensityMedium {
    sigma_a: SpectrumRGB,
    sigma_s: SpectrumRGB,
    g: f32,
    nx: usize,
    ny: usize,
    nz: usize,
    medium2world: Transform,
    density: Vec<f32>,
    sigma_t: f32,
    inv_max_density: f32,
}

impl GridDensityMedium {
    pub fn new(
        sigma_a: SpectrumRGB,
        sigma_s: SpectrumRGB,
        g: f32,
        nx: usize,
        ny: usize,
        nz: usize,
        medium2world: Transform,
        density: Vec<f32>,
    ) -> Self {
        let sigma_t = (sigma_a + sigma_s).rgb().x;
        let mut density = density;
        density.resize(nx * ny * nz, 0.0);
        let inv_max_density = 1.0 / density.iter().max_by(|&a, &b| a.total_cmp(b)).unwrap();
        Self {
            sigma_a,
            sigma_s,
            g,
            nx,
            ny,
            nz,
            medium2world,
            density,
            sigma_t,
            inv_max_density,
        }
    }

    pub fn d(&self, p: Point3<usize>) -> f32 {
        if p.x >= self.nx || p.y >= self.ny || p.z >= self.nz {
            return 0.0;
        }
        self.density[(p.z * self.ny + p.y) * self.nx + p.x]
    }

    pub fn density(&self, p: Point3<f32>) -> f32 {
        let p_samples = Point3::new(
            p.x * self.nx as f32 - 0.5,
            p.y * self.ny as f32 - 0.5,
            p.z * self.nz as f32 - 0.5,
        );
        let pi = Point3::new(
            p_samples.x.floor(),
            p_samples.y.floor(),
            p_samples.z.floor(),
        );
        let d = p_samples - pi;
        let pi = Point3::new(pi.x as usize, pi.y as usize, pi.z as usize);
        let d00 = lerp(d.x, self.d(pi), self.d(pi + Vector3::new(1, 0, 0)));
        let d10 = lerp(
            d.x,
            self.d(pi + Vector3::new(0, 1, 0)),
            self.d(pi + Vector3::new(1, 1, 0)),
        );
        let d01 = lerp(
            d.x,
            self.d(pi + Vector3::new(0, 0, 1)),
            self.d(pi + Vector3::new(1, 0, 1)),
        );
        let d11 = lerp(
            d.x,
            self.d(pi + Vector3::new(0, 1, 1)),
            self.d(pi + Vector3::new(1, 1, 1)),
        );
        let d0 = lerp(d.y, d00, d10);
        let d1 = lerp(d.y, d01, d11);
        lerp(d.z, d0, d1)
    }
}

impl Medium for GridDensityMedium {
    fn tr(&self, ray: &Ray, sampler: Rc<RefCell<dyn Sampler>>) -> SpectrumRGB {
        let ray = self.medium2world.local_ray(ray);
        (0..256)
            .map(|_| sampler.borrow_mut().next_1d())
            .map(|p| ray.at(ray.t_max * p))
            .map(|p| self.density(p))
            .map(|p| SpectrumRGB::same(p))
            .fold(SpectrumRGB::same(0.0), |a, b| a + b)
    }

    fn sample(
        &self,
        ray: &Ray,
        sampler: Rc<RefCell<dyn Sampler>>,
        mi: &mut MediumInteraction,
    ) -> SpectrumRGB {
        let local_ray = self.medium2world.local_ray(ray);
        let b = Bounds3::new(V3f::from([0.0; 3]), V3f::from([1.0; 3]));
        let (t_near, t_far) = b.intersect_t(ray);
        if t_near > t_far {
            return SpectrumRGB::same(1.0);
        }
        let mut t = t_near;
        loop {
            t -= (1.0 - sampler.borrow_mut().next_1d()).ln() * self.inv_max_density / self.sigma_t;
            if t >= t_far {
                break;
            }
            if self.density(local_ray.at(t)) * self.inv_max_density > sampler.borrow_mut().next_1d()
            {
                let phase = HenyeyGreenstein::new(self.g);
                *mi = MediumInteraction::new(
                    ray.at(t),
                    ray.t,
                    -ray.direction,
                    Rc::new(self.clone()),
                    Some(Box::new(phase)),
                );
                return self.sigma_s / self.sigma_t;
            }
        }
        SpectrumRGB::same(1.0)
    }
}

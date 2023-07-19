use crate::core_layer::colorspace::SpectrumRGB;
use crate::core_layer::function::{coordinate_system, spherical_direction};
use crate::function_layer::{fetch_v3f, Interaction, Ray, Sampler, V3f};
use cgmath::{InnerSpace, Matrix4, Point3, SquareMatrix, Vector2, Zero};
use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;
use serde_json::Value;
use crate::core_layer::transform::Transform;
use super::{grid_density::GridDensityMedium, homogeneous::HomogeneousMedium};


pub trait Medium {
    fn tr(&self, ray: &Ray, sampler: Rc<RefCell<dyn Sampler>>) -> SpectrumRGB;
    fn sample(
        &self,
        ray: &Ray,
        sampler: Rc<RefCell<dyn Sampler>>,
        mi: &mut MediumInteraction,
    ) -> SpectrumRGB;
    fn sigma_s(&self) -> SpectrumRGB;
}

pub struct MediumInteraction {
    pub position: Point3<f32>,
    pub time: f32,
    pub p_error: V3f,
    pub wo: V3f,
    pub normal: V3f,
    pub medium_interface: MediumInterface,
    pub phase: Option<Box<dyn PhaseFunction>>,
}

impl Interaction for MediumInteraction {
    fn is_medium_interaction(&self) -> bool {
        true
    }

    fn f(&self, wo: V3f, wi: V3f) -> SpectrumRGB {
        let p = self.phase.as_ref().unwrap().p(wo, wi);
        SpectrumRGB::same(p)
    }

    fn p(&self) -> Point3<f32> {
        self.position
    }
}

impl Default for MediumInteraction {
    fn default() -> Self {
        Self {
            position: Point3::new(0.0, 0.0, 0.0),
            time: 0.0,
            p_error: V3f::new(0.0, 0.0, 0.0),
            wo: V3f::new(0.0, 0.0, 0.0),
            normal: V3f::new(0.0, 0.0, 0.0),
            medium_interface: Default::default(),
            phase: None,
        }
    }
}

impl MediumInteraction {
    pub fn new(
        p: Point3<f32>,
        time: f32,
        wo: V3f,
        medium_interface: Rc<dyn Medium>,
        phase: Option<Box<dyn PhaseFunction>>,
    ) -> Self {
        Self {
            position: p,
            time,
            p_error: V3f::from([0.0; 3]),
            wo,
            normal: V3f::from([0.0; 3]),
            medium_interface: MediumInterface::new(Some(medium_interface), None),
            phase,
        }
    }
    pub fn is_valid(&self) -> bool {
        self.phase.is_some()
    }
}

pub trait PhaseFunction {
    fn p(&self, wo: V3f, wi: V3f) -> f32;
    fn sample_p(&self, wo: V3f, wi: &mut V3f, u: Vector2<f32>) -> f32;
}

#[inline]
fn phase_hg(cos_theta: f32, g: f32) -> f32 {
    let denom = 1.0 + g * g + 2.0 * g * cos_theta;
    std::f32::consts::FRAC_PI_4 * (1.0 - g * g) / (denom * denom.sqrt())
}

pub struct HenyeyGreenstein {
    g: f32,
}

impl HenyeyGreenstein {
    pub fn new(g: f32) -> Self {
        Self { g }
    }
}

impl PhaseFunction for HenyeyGreenstein {
    fn p(&self, wo: V3f, wi: V3f) -> f32 {
        phase_hg(wo.dot(wi), self.g)
    }

    fn sample_p(&self, wo: V3f, wi: &mut V3f, u: Vector2<f32>) -> f32 {
        let g = self.g;
        let cos_theta = if g.abs() < 1e-3 {
            1.0 - 2.0 * u.x
        } else {
            let sqr_term = (1.0 - g * g) / (1.0 + g - 2.0 * g * u[0]);
            -(1.0 + g * g - sqr_term * sqr_term) / (2.0 * g)
        };
        let sin_theta = (1.0 - cos_theta * cos_theta).max(0.0).sqrt();
        let phi = PI * 2.0 * u.y;
        let (mut v1, mut v2) = (V3f::zero(), V3f::zero());
        coordinate_system(wo, &mut v1, &mut v2);
        *wi = spherical_direction(sin_theta, cos_theta, phi, v1, v2, wo);
        phase_hg(cos_theta, g)
    }
}

#[derive(Default, Clone)]
pub struct MediumInterface {
    inside: Option<Rc<dyn Medium>>,
    outside: Option<Rc<dyn Medium>>,
}

impl MediumInterface {
    pub fn new(inside: Option<Rc<dyn Medium>>, outside: Option<Rc<dyn Medium>>) -> Self {
        Self { inside, outside }
    }
    pub fn is_medium_transition(&self) -> bool {
        // inside != outside
        !match (&self.inside, &self.outside) {
            (None, None) => true,
            (Some(m1), Some(m2)) => Rc::ptr_eq(m1, m2),
            _ => false,
        }
    }

    pub fn inside(&self) -> Option<Rc<dyn Medium>> {
        self.inside.clone()
    }
    pub fn outside(&self) -> Option<Rc<dyn Medium>> {
        self.outside.clone()
    }
}

pub fn construct_medium(json: &Value) -> Option<Rc<dyn Medium>> {
    let g = json["g"].as_f64().unwrap_or(0.0) as f32;
    let medium = json["medium"].as_str().unwrap();
    let (sig_a, sig_s) = match SUBSURFACE_PARAMETER_TABLE.iter().position(|&x| x.0 == medium) {
        Some(pos) => {
            let r = SUBSURFACE_PARAMETER_TABLE[pos];
            (r.2, r.1)
        }
        None => (V3f::new(0.0011, 0.0024, 0.014),
                 V3f::new(2.55, 3.21, 3.77))
    };
    let (sig_a, sig_s) = (SpectrumRGB::from_rgb(sig_a), SpectrumRGB::from_rgb(sig_s));
    match json["type"].as_str().unwrap() {
        "homogeneous" => Some(Rc::new(HomogeneousMedium::new(sig_a, sig_s, g))),
        "gridDensity" => {
            let density = match json.get("density") {
                Some(val) => serde_json::from_value::<Vec<f32>>(val.clone()).unwrap(),
                None => {
                    eprintln!("No density given for grid density medium");
                    return None;
                }
            };
            let nx = json["nx"].as_u64().unwrap_or(1) as usize;
            let ny = json["ny"].as_u64().unwrap_or(1) as usize;
            let nz = json["nz"].as_u64().unwrap_or(1) as usize;
            if density.len() != nx * ny * nz {
                eprintln!("GridDensityMedium has {} density values; expected nx*ny*nz = {}",
                          density.len(), nx * ny * nz);
                return None;
            }
            let p0 = fetch_v3f(json, "p0", V3f::zero());
            let p1 = fetch_v3f(json, "p1", V3f::zero());
            let translate = Transform::translation(p0);
            let rotate = Matrix4::identity();
            let scale = Transform::scalation(p1 - p0);
            let medium2world = Transform::new(translate, rotate, scale);
            Some(Rc::new(GridDensityMedium::new(
                sig_a, sig_s, g,
                nx, ny, nz, medium2world, density, )))
        }
        tp => panic!("Invalid medium type: {}!", tp)
    }
}

// (Medium name, sigma_s, sigma_a)
static SUBSURFACE_PARAMETER_TABLE: &[(&'static str, V3f, V3f)] = &[
    ("Salt Powder", V3f::new(0.027333, 0.032451, 0.031979), V3f::new(0.28415, 0.3257, 0.34148)),
    ("Orange Powder", V3f::new(0.00015617, 0.00017482, 0.0001762), V3f::new(0.001449, 0.003441, 0.007863)),
    ("Sugar Powder", V3f::new(0.00022272, 0.00025513, 0.000271), V3f::new(0.012638, 0.031051, 0.050124)),
    ("Head & Shoulders Shampoo", V3f::new(0.023805, 0.028804, 0.034306), V3f::new(0.084621, 0.15688, 0.20365)),
    ("Lemon Tea Powder", V3f::new(0.040224, 0.045264, 0.051081), V3f::new(2.4288, 4.5757, 7.2127)),
    ("Grape Juice", V3f::new(5.382e-05, 0.0, 0.0), V3f::new(0.10404, 0.23958, 0.29325)),
    ("Lowfat Milk", V3f::new(0.89187, 1.5136, 2.532), V3f::new(0.002875, 0.00575, 0.0115)),
    ("Coke", V3f::new(8.9053e-05, 8.372e-05, 0.0), V3f::new(0.10014, 0.16503, 0.2468)),
    ("Mint Mocha Coffee", V3f::new(0.31602, 0.38538, 0.48131), V3f::new(3.772, 5.8228, 7.82)),
    ("Cream", V3f::new(7.38, 5.47, 3.15), V3f::new(0.0002, 0.0028, 0.0163)),
    ("Reduced Milk", V3f::new(2.4858, 3.1669, 4.5214), V3f::new(0.0025556, 0.0051111, 0.012778)),
    ("Fog", V3f::new(0.01, 0.01, 0.01), V3f::new(0.001, 0.001, 0.001)),
    // (, V3f::new(), V3f::new()),
];
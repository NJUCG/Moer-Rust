use super::light::{InfiniteLight, Light, LightSampleResult, LightType};
use crate::core_layer::{colorspace::SpectrumRGB, constants::INV_PI, distribution::Distribution};
use crate::function_layer::texture::TextureCoord;
use crate::function_layer::{construct_texture, Intersection, Ray, Texture, V3f};
use cgmath::Vector2;
use cgmath::{InnerSpace, Zero};
use serde_json::Value;
use std::any::Any;
use std::f32::consts::PI;
use std::rc::Rc;

#[derive(Clone)]
pub struct EnvironmentLight {
    environment_map: Rc<dyn Texture<SpectrumRGB>>,
    energy_distribution: Distribution<Vector2<usize>>,
}

fn direction2uv(direction: V3f) -> Vector2<f32> {
    let (mut u, mut v): (f32, f32);
    let cos_theta = direction[1];
    v = cos_theta.acos();
    if direction[2].abs() < 1e-8 {
        u = if direction[0] > 0.0 {
            PI * 0.5
        } else {
            PI * 1.5
        }
    } else {
        let tan_phi = direction[0] / (direction[2] + 1e-8);
        u = tan_phi.atan();
        if direction[0] < 0.0 && direction[2] < 0.0 {
            u += PI;
        } else if direction[0] < 0.0 && direction[2] > 0.0 {
            u += 2.0 * PI;
        } else if direction[0] > 0.0 && direction[2] < 0.0 {
            u += PI; // TODO ????
        }
    }
    u *= 0.5 * INV_PI;
    v *= INV_PI;
    Vector2::new(u, v)
}

impl EnvironmentLight {
    pub fn from_json(json: &Value) -> Self {
        if json.get("texture").is_none() {
            panic!("EnvironmentLight must specify texture!\n")
        }
        let environment_map = construct_texture::<SpectrumRGB>(&json["texture"]);
        let Vector2 {
            x: width,
            y: height,
        } = environment_map.size();
        let mut indices: Vec<Vector2<usize>> = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                indices.push(Vector2::new(x, y));
            }
        }
        let inv_width = 1.0 / width as f32;
        let inv_height = 1.0 / height as f32;

        let weight_func = |index: Vector2<usize>| {
            let u = index.x as f32 * inv_width;
            let v = index.y as f32 * inv_height;
            let sin_theta = (PI * (index.y as f32 + 0.5) * inv_height).sin();
            let tex_coord = TextureCoord {
                coord: Vector2::new(u, v),
                duv_dx: Vector2::zero(),
                duv_dy: Vector2::zero(),
            };
            let s = environment_map.evaluate_coord(&tex_coord);
            s.rgb().dot(V3f::new(0.212671, 0.715160, 0.072169)) * sin_theta
        };
        let energy_distribution = Distribution::new(indices, weight_func);
        Self {
            environment_map,
            energy_distribution,
        }
    }
}

impl Light for EnvironmentLight {
    fn evaluate_emission(&self, _intersection: &Intersection, _wo: &V3f) -> SpectrumRGB {
        panic!("This shouldn't be invoked!\n");
    }

    fn sample(&self, _shading_point: &Intersection, sample: Vector2<f32>) -> LightSampleResult {
        let sz = self.environment_map.size();
        let inv_width = 1.0 / sz.x as f32;
        let inv_height = 1.0 / sz.y as f32;
        let mut pdf = 0.0;
        let index = self.energy_distribution.sample(sample.x, &mut pdf).unwrap();
        let u = (index.x as f32 * inv_width) % 1.0;
        let v = (index.y as f32 * inv_height) % 2.0;
        let phi = u * 2.0 * PI;
        let theta = v * PI;
        let sin_theta = fastapprox::fast::sin(theta);
        let cos_theta = fastapprox::fast::cos(theta);
        let sin_phi = fastapprox::fast::sinfull(phi);
        let cos_phi = fastapprox::fast::cosfull(phi);

        let x = sin_theta * sin_phi;
        let y = cos_theta;
        let z = sin_theta * cos_phi;
        let tex_coord = TextureCoord {
            coord: Vector2::new(u, v),
            duv_dx: Vector2::zero(),
            duv_dy: Vector2::zero(),
        };
        let energy = self.environment_map.evaluate_coord(&tex_coord);
        pdf *= (sz[0] * sz[1]) as f32 * INV_PI * INV_PI * 0.5 / sin_theta;
        LightSampleResult {
            energy,
            direction: V3f::new(x, y, z),
            distance: f32::INFINITY,
            normal: V3f::zero(),
            pdf,
            is_delta: false,
            light_type: LightType::EnvironmentLight,
        }
    }

    fn light_type(&self) -> LightType {
        LightType::EnvironmentLight
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn do_equal(&self, rhs: &dyn Light) -> bool {
        if rhs.light_type() != LightType::EnvironmentLight {
            return false;
        }
        let other = rhs.as_any().downcast_ref::<Self>().unwrap();
        self.energy_distribution == other.energy_distribution
            && Rc::ptr_eq(&self.environment_map, &other.environment_map)
    }
}

impl InfiniteLight for EnvironmentLight {
    fn evaluate_emission_ray(&self, ray: &Ray) -> SpectrumRGB {
        let uv = direction2uv(ray.direction);
        self.environment_map.evaluate_coord(&TextureCoord {
            coord: uv,
            duv_dx: Vector2::zero(),
            duv_dy: Vector2::zero(),
        })
    }
}

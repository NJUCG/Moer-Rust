use std::rc::Rc;

use cgmath::{EuclideanSpace, Point3};
use serde_json::Value;

use crate::core_layer::distribution::Distribution;
use crate::function_layer::{
    Acceleration, construct_light, construct_shape, create_acceleration, Light, Ray, RR,
    set_acc_type, SurfaceInteraction,
};
use crate::function_layer::light::{
    area_light::AreaLight, environment_light::EnvironmentLight, light::LightType,
};
use crate::function_layer::material::MaterialType;

pub struct Scene {
    pub infinite_lights: Vec<Rc<EnvironmentLight>>,
    acceleration: Box<dyn Acceleration>,
    light_distribution: Distribution<RR<dyn Light>>,
    black_hole_centers: Vec<Point3<f32>>,
}

impl Scene {
    pub fn from_json(json: &Value) -> Self {
        let mut geom_id = 0;
        let acc = json["acceleration"].as_str().unwrap_or("bvh");
        set_acc_type(acc);
        let mut acceleration = create_acceleration();
        let shapes = json["shapes"].as_array().unwrap();
        let mut black_hole_centers = vec![];
        for shape in shapes {
            let shape = construct_shape(shape);
            if let Some(mat) = shape.borrow().material().as_ref() {
                if mat.mat_type() == MaterialType::BlackHole {
                    black_hole_centers.push(Point3::from_vec(shape.borrow().get_bounds().centroid()));
                }
            }
            shape.borrow_mut().set_geometry_id(geom_id);
            geom_id += 1;
            acceleration.attach_shape(shape);
        }
        let mut infinite_lights = vec![];

        let lights = json["lights"].as_array().unwrap().to_vec();
        let mut light_v = vec![];
        for light in lights {
            let light = construct_light(&light);
            let ltp = light.borrow().light_type();
            match ltp {
                // 如果是环境光源，不加入光源分布
                LightType::EnvironmentLight => {
                    let light = light
                        .borrow()
                        .as_any()
                        .downcast_ref::<EnvironmentLight>()
                        .unwrap()
                        .clone();
                    infinite_lights.push(Rc::new(light));
                    continue;
                }
                // 如果是面光源，将其shape也加入加速结构
                LightType::AreaLight => {
                    let mut l = light.borrow_mut();
                    let al = l.as_any_mut().downcast_mut::<AreaLight>().unwrap();
                    let mut shape = al.shape.as_mut().unwrap().borrow_mut();
                    shape.set_light(light.clone());
                    shape.set_geometry_id(geom_id);
                    drop(shape);
                    geom_id += 1;
                    acceleration.attach_shape(al.shape.as_ref().unwrap().clone());
                }
                LightType::SpotLight => (),
            }
            light_v.push(light);
        }
        let light_distribution = Distribution::new(light_v, |_light| 1.0);
        acceleration.build();
        Self {
            infinite_lights,
            acceleration,
            light_distribution,
            black_hole_centers,
        }
    }

    pub fn ray_intersect(&self, ray: &mut Ray) -> Option<SurfaceInteraction> {
        self.acceleration.get_intersect(ray)
    }

    pub fn sample_light(&self, sample: f32, pdf: &mut f32) -> Option<RR<dyn Light>> {
        self.light_distribution.sample(sample, pdf)
    }

    pub fn black_hole_centers(&self) -> &Vec<Point3<f32>> {
        &self.black_hole_centers
    }
}

#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;
use serde_json::Value;
use crate::core_layer::distribution::Distribution;
use crate::function_layer::acceleration::BVHAccel;
use crate::function_layer::light::{light::{construct_light, LightType},
                                   environment_light::EnvironmentLight,
                                   area_light::AreaLight};
use crate::function_layer::{Ray, Light, construct_shape, Intersection};
use super::acceleration::Acceleration;

pub struct Scene {
    pub infinite_lights: Option<Rc<EnvironmentLight>>,
    acceleration: Rc<dyn Acceleration>,
    light_distribution: Distribution<Rc<RefCell<dyn Light>>>,
}

impl Scene {
    pub fn from_json(json: &Value) -> Self {
        let mut acceleration = BVHAccel::default();
        let shapes = json["shapes"].as_array().unwrap();
        for shape in shapes {
            let shape = construct_shape(shape);
            acceleration.attach_shape(shape);
        }
        let mut infinite_lights: Option<Rc<EnvironmentLight>> = None;

        let lights = json["lights"].as_array().unwrap().to_vec();
        let mut light_v = vec![];
        for light in lights {
            let light = construct_light(&light);
            match light.borrow().light_type() {
                LightType::EnvironmentLight => {
                    let light = EnvironmentLight::copy_constr(light.borrow().as_any().downcast_ref::<EnvironmentLight>().unwrap());
                    infinite_lights = Some(Rc::new(light));
                    continue;
                }
                LightType::AreaLight => {
                    let mut l = light.borrow_mut();
                    let al = l.as_any_mut().downcast_mut::<AreaLight>().unwrap();
                    let mut shape = al.shape.as_mut().unwrap().borrow_mut();
                    shape.set_light(light.clone());
                    drop(shape);
                    acceleration.attach_shape(al.shape.as_ref().unwrap().clone());
                }
                LightType::SpotLight => ()
            }
            light_v.push(light);
        }
        let light_distribution = Distribution::new(light_v,
                                                   |_light| 1.0);
        acceleration.build();
        Self {
            infinite_lights,
            acceleration: Rc::new(acceleration),
            light_distribution,
        }
    }

    pub fn ray_intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.acceleration.ray_intersect(ray)
    }

    pub fn sample_light(&self, sample: f32, pdf: &mut f32) -> Rc<RefCell<dyn Light>> {
        self.light_distribution.sample(sample, pdf)
    }
}
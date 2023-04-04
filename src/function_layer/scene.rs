use crate::core_layer::distribution::Distribution;
use crate::function_layer::light::{
    area_light::AreaLight, environment_light::EnvironmentLight, light::LightType,
};
use crate::function_layer::{
    construct_light, construct_shape, create_acceleration, set_acc_type, Acceleration,
    Intersection, Light, Ray, RR,
};
use serde_json::Value;
use std::rc::Rc;

pub struct Scene {
    pub infinite_lights: Vec<Rc<EnvironmentLight>>,
    acceleration: Box<dyn Acceleration>,
    light_distribution: Distribution<RR<dyn Light>>,
}

impl Scene {
    pub fn from_json(json: &Value) -> Self {
        let mut geom_id = 0;
        let acc = json["acceleration"].as_str().unwrap_or("bvh");
        set_acc_type(acc);
        let mut acceleration = create_acceleration();
        let shapes = json["shapes"].as_array().unwrap();
        for shape in shapes {
            let shape = construct_shape(shape);
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
        }
    }

    pub fn ray_intersect(&self, ray: &mut Ray) -> Option<Intersection> {
        self.acceleration.get_intersect(ray)
    }

    pub fn sample_light(&self, sample: f32, pdf: &mut f32) -> Option<RR<dyn Light>> {
        self.light_distribution.sample(sample, pdf)
    }
}

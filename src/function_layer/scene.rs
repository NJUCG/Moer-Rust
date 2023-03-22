use std::rc::Rc;
use serde_json::Value;
use crate::core_layer::distribution::Distribution;
use crate::function_layer::light::{light::LightType,
                                   environment_light::EnvironmentLight,
                                   area_light::AreaLight};
use crate::function_layer::{Ray, Light, Intersection, Acceleration, construct_shape, construct_light, RR, create_acceleration, set_acc_type};

pub struct Scene {
    pub infinite_lights: Option<Rc<EnvironmentLight>>,
    acceleration: RR<dyn Acceleration>,
    light_distribution: Distribution<RR<dyn Light>>,
}

impl Scene {
    pub fn from_json(json: &Value) -> Self {
        let mut geom_id = 0;
        let acc = json["acceleration"].as_str().unwrap_or("bvh");
        set_acc_type(acc);
        let acceleration = create_acceleration();
        let shapes = json["shapes"].as_array().unwrap();
        for shape in shapes {
            let shape = construct_shape(shape);
            shape.borrow_mut().set_geometry_id(geom_id);
            geom_id += 1;
            acceleration.borrow_mut().attach_shape(shape);
        }
        let mut infinite_lights: Option<Rc<EnvironmentLight>> = None;

        let lights = json["lights"].as_array().unwrap().to_vec();
        let mut light_v = vec![];
        for light in lights {
            let light = construct_light(&light);
            match light.borrow().light_type() {
                // 如果是环境光源，不加入光源分布
                LightType::EnvironmentLight => {
                    let light = EnvironmentLight::copy_constr(light.borrow().as_any().downcast_ref::<EnvironmentLight>().unwrap());
                    infinite_lights = Some(Rc::new(light));
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
                    acceleration.borrow_mut().attach_shape(al.shape.as_ref().unwrap().clone());
                }
                LightType::SpotLight => ()
            }
            light_v.push(light);
        }
        let light_distribution = Distribution::new(light_v,
                                                   |_light| 1.0);
        acceleration.borrow_mut().build();
        Self {
            infinite_lights,
            acceleration,
            light_distribution,
        }
    }

    pub fn ray_intersect(&self, ray: &mut Ray) -> Option<Intersection> {
        self.acceleration.borrow().get_intersect(ray)
    }

    pub fn sample_light(&self, sample: f32, pdf: &mut f32) -> RR<dyn Light> {
        self.light_distribution.sample(sample, pdf)
    }
}
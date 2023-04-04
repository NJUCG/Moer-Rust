use super::bxdf::{specular::SpecularReflection, BSDF};
use crate::function_layer::texture::normal_texture::NormalTexture;
use crate::function_layer::{Intersection, Material, V3f};
use cgmath::Zero;
use serde_json::Value;
use std::rc::Rc;

pub struct MirrorMaterial {
    pub normal_map: Option<Rc<NormalTexture>>,
}

impl MirrorMaterial {
    pub fn from_json(json: &Value) -> Self {
        let normal_map = if json["normalmap"].is_null() {
            None
        } else {
            Some(Rc::new(NormalTexture::from_json(&json["normalmap"])))
        };
        Self { normal_map }
    }
}

impl Material for MirrorMaterial {
    fn normal_map(&self) -> Option<Rc<NormalTexture>> {
        self.normal_map.clone()
    }

    fn compute_bsdf(&self, intersection: &Intersection) -> Rc<dyn BSDF> {
        let mut normal = V3f::zero();
        let mut tangent = V3f::zero();
        let mut bitangent = V3f::zero();

        self.compute_shading_geometry(intersection, &mut normal, &mut tangent, &mut bitangent);
        Rc::new(SpecularReflection {
            normal,
            tangent,
            bitangent,
        })
    }
}

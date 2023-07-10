use super::bxdf::{specular::SpecularReflection, BSDF};
use super::material::fetch_normal_map;
use crate::function_layer::texture::normal_texture::NormalTexture;
use crate::function_layer::{SurfaceInteraction, Material, V3f};
use cgmath::Zero;
use serde_json::Value;
use std::rc::Rc;
use crate::function_layer::material::bxdf::bsdf::BSDFBase;


pub struct MirrorMaterial {
    pub normal_map: Option<Rc<NormalTexture>>,
}

impl MirrorMaterial {
    pub fn from_json(json: &Value) -> Self {
        let normal_map = fetch_normal_map(json);
        Self { normal_map }
    }
}

impl Material for MirrorMaterial {
    fn normal_map(&self) -> Option<Rc<NormalTexture>> {
        self.normal_map.clone()
    }

    fn compute_bsdf(&self, intersection: &SurfaceInteraction) -> Box<dyn BSDF> {
        let mut normal = V3f::zero();
        let mut tangent = V3f::zero();
        let mut bitangent = V3f::zero();

        self.compute_shading_geometry(intersection, &mut normal, &mut tangent, &mut bitangent);
        Box::new(SpecularReflection {
            bsdf: BSDFBase {
                normal,
                tangent,
                bitangent,
            }
        })
    }
}

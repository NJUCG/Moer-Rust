use super::bxdf::bsdf::BSDFBase;
use super::bxdf::transparent::TransparentBSDF;
use crate::function_layer::material::material::fetch_normal_map;
use crate::function_layer::texture::normal_texture::NormalTexture;
use crate::function_layer::{Material, SurfaceInteraction, BSDF};
use serde_json::Value;
use std::rc::Rc;

pub struct TransparentMaterial {
    normal_map: Option<Rc<NormalTexture>>,
    ior: f32,
}

impl TransparentMaterial {
    pub fn from_json(json: &Value) -> Self {
        let normal_map = fetch_normal_map(json);
        let ior = json["ior"].as_f64().unwrap() as f32;
        Self { normal_map, ior }
    }
}

impl Material for TransparentMaterial {
    fn normal_map(&self) -> Option<Rc<NormalTexture>> {
        self.normal_map.clone()
    }

    fn compute_bsdf(&self, intersection: &SurfaceInteraction) -> Box<dyn BSDF> {
        let (normal, tangent, bitangent) = self.compute_shading_geometry(intersection);
        Box::new(TransparentBSDF {
            bsdf: BSDFBase {
                normal,
                tangent,
                bitangent,
            },
            ior: self.ior,
        })
    }
}

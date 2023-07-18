use std::rc::Rc;
use cgmath::Zero;
use serde_json::Value;
use crate::function_layer::{BSDF, Material, SurfaceInteraction, V3f};
use crate::function_layer::material::material::fetch_normal_map;
use super::bxdf::transparent::TransparentBSDF;
use super::bxdf::bsdf::BSDFBase;
use crate::function_layer::texture::normal_texture::NormalTexture;

pub struct TransparentMaterial {
    normal_map: Option<Rc<NormalTexture>>,
    ior: f32,
}

impl TransparentMaterial {
    pub fn from_json(json: &Value) -> Self {
        let normal_map = fetch_normal_map(json);
        let ior = json["ior"].as_f64().unwrap() as f32;
        Self {
            normal_map,
            ior,
        }
    }
}

impl Material for TransparentMaterial {
    fn normal_map(&self) -> Option<Rc<NormalTexture>> {
        self.normal_map.clone()
    }

    fn compute_bsdf(&self, intersection: &SurfaceInteraction) -> Box<dyn BSDF> {
        let mut normal = V3f::zero();
        let mut tangent = V3f::zero();
        let mut bitangent = V3f::zero();

        self.compute_shading_geometry(intersection, &mut normal, &mut tangent, &mut bitangent);

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
use super::{bxdf::bsdf::BSDF, matte::MatteMaterial, mirror::MirrorMaterial};
use crate::function_layer::texture::normal_texture::NormalTexture;
use crate::function_layer::{Intersection, Texture, V3f};
use cgmath::InnerSpace;
use serde_json::Value;
use std::rc::Rc;

pub trait Material {
    fn normal_map(&self) -> Option<Rc<NormalTexture>>;
    fn compute_bsdf(&self, intersection: &Intersection) -> Rc<dyn BSDF>;
    fn compute_shading_geometry(
        &self,
        intersection: &Intersection,
        normal: &mut V3f,
        tangent: &mut V3f,
        bitangent: &mut V3f,
    ) {
        match self.normal_map() {
            None => {
                *normal = intersection.normal;
                *tangent = intersection.tangent;
                *bitangent = intersection.bitangent;
            }
            Some(normal_map) => {
                let local_normal = normal_map.evaluate(intersection);
                *normal = (local_normal.x * intersection.tangent
                    + local_normal.y * intersection.bitangent
                    + local_normal.z * intersection.normal)
                    .normalize();
                *tangent = intersection.tangent;
                *bitangent = tangent.cross(*normal).normalize();
            }
        }
    }
}

pub fn construct_material(json: &Value) -> Rc<dyn Material> {
    match json["type"].as_str().expect("No material type annotation!") {
        "matte" => Rc::new(MatteMaterial::from_json(json)),
        "mirror" => Rc::new(MirrorMaterial::from_json(json)),
        tp => panic!("Invalid type: {}", tp),
    }
}

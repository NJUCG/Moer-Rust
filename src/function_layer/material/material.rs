use super::{bxdf::bsdf::BSDF, matte::MatteMaterial, mirror::MirrorMaterial};
use crate::function_layer::texture::normal_texture::NormalTexture;
use crate::function_layer::{Intersection, Texture, V3f};
use cgmath::InnerSpace;
use serde_json::Value;
use std::rc::Rc;
use crate::function_layer::material::phong_material::PhongMaterial;

pub trait Material {
    fn normal_map(&self) -> Option<Rc<NormalTexture>>; // self.normal_map.clone()
    fn compute_bsdf(&self, intersection: &Intersection) -> Box<dyn BSDF>;
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
pub fn fetch_normal_map(json: &Value) -> Option<Rc<NormalTexture>> {
    if json["normalmap"].is_null() {
        None
    } else {
        Some(Rc::new(NormalTexture::from_json(&json["normalmap"])))
    }
}
pub fn construct_material(json: &Value) -> Rc<dyn Material> {
    match json["type"].as_str().expect("No material type annotation!") {
        "matte" => Rc::new(MatteMaterial::from_json(json)),
        "mirror" => Rc::new(MirrorMaterial::from_json(json)),
        "phong" => Rc::new(PhongMaterial::from_json(json)),
        tp => panic!("Invalid type: {}", tp),
    }
}

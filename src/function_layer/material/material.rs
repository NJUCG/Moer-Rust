use std::rc::Rc;
use nalgebra::Vector3;
use serde_json::Value;
use super::bxdf::bsdf::BSDF;
use super::matte::MatteMaterial;
use crate::function_layer::{
    shape::intersection::Intersection,
    texture::{normal_texture::NormalTexture, texture::Texture},
};

pub trait Material {
    fn normal_map(&self) -> Option<Rc<NormalTexture>>;
    fn compute_bsdf(&self, intersection: &Intersection) -> Rc<dyn BSDF>;
    fn compute_shading_geometry(&self, intersection: &Intersection,
                                normal: &mut Vector3<f32>, tangent: &mut Vector3<f32>, bitangent: &mut Vector3<f32>) {
        match self.normal_map() {
            None => {
                *normal = intersection.normal;
                *tangent = intersection.tangent;
                *bitangent = intersection.bitangent;
            }
            Some(normal_map) => {
                let local_normal = normal_map.evaluate(intersection);
                *normal = (local_normal.x * intersection.tangent +
                    local_normal.y * intersection.bitangent +
                    local_normal.z * intersection.normal).normalize();
                *tangent = intersection.tangent;
                *bitangent = tangent.cross(normal).normalize();
            }
        }
    }
}

pub fn construct_material(json: &Value) -> Rc<dyn Material> {
    match json["type"].as_str().expect("No material type annotation!") {
        "matte" => Rc::new(MatteMaterial::from_json(json)),
        _ => panic!("Invalid type")
    }
}
use super::bxdf::{lambert::LambertReflection, BSDF};
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::texture::{
    constant_texture::ConstantTexture, normal_texture::NormalTexture, texture::construct_texture,
};
use crate::function_layer::{fetch_v3f, SurfaceInteraction, Texture, V3f};
use cgmath::Zero;
use serde_json::Value;
use std::rc::Rc;
use super::material::fetch_normal_map;

use super::Material;

pub struct MatteMaterial {
    normal_map: Option<Rc<NormalTexture>>,
    albedo: Rc<dyn Texture<SpectrumRGB>>,
}

impl MatteMaterial {
    pub fn new() -> Self {
        let albedo = Rc::new(ConstantTexture::new(&SpectrumRGB::same(0.5)));
        Self {
            normal_map: None,
            albedo,
        }
    }

    pub fn from_json(json: &Value) -> Self {
        let normal_map = fetch_normal_map(json);
        let albedo = if json["albedo"].is_object() {
            construct_texture::<SpectrumRGB>(json)
        } else if json["albedo"].is_array() {
            let rgb = fetch_v3f(json, "albedo", V3f::zero());
            let s = SpectrumRGB::from_rgb(rgb.unwrap());
            Rc::new(ConstantTexture::new(&s))
        } else {
            panic!("Error in albedo format!");
        };
        Self { normal_map, albedo }
    }
}

impl Material for MatteMaterial {
    fn normal_map(&self) -> Option<Rc<NormalTexture>> {
        self.normal_map.clone()
    }

    fn compute_bsdf(&self, intersection: &SurfaceInteraction) -> Box<dyn BSDF> {
        let [mut normal, mut tangent, mut bitangent] = [V3f::zero(); 3];
        self.compute_shading_geometry(intersection, &mut normal, &mut tangent, &mut bitangent);
        let s = self.albedo.evaluate(intersection);
        Box::new(LambertReflection::new(s, normal, tangent, bitangent))
    }
}

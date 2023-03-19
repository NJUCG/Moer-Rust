#![allow(dead_code)]

use std::rc::Rc;
use nalgebra::Vector3;
use serde_json::Value;
use crate::core_layer::colorspace::SpectrumRGB;
use super::bxdf::BSDF;
use super::bxdf::lambert::LambertReflection;
use crate::function_layer::{
    Intersection,
    Texture,
    texture::{
        constant_texture::ConstantTexture,
        normal_texture::NormalTexture,
        texture::construct_texture,
    },
};

use super::Material;

pub struct MatteMaterial {
    normal_map: Option<Rc<NormalTexture>>,
    albedo: Rc<dyn Texture<SpectrumRGB>>,
}

impl MatteMaterial {
    pub fn new() -> Self {
        let albedo = Rc::new(ConstantTexture::new(&SpectrumRGB::same(0.5)));
        Self { normal_map: None, albedo }
    }

    pub fn from_json(json: &Value) -> Self {
        let normal_map = if json["normalmap"].is_null() {
            None
        } else {
            Some(Rc::new(NormalTexture::from_json(&json["normalmap"])))
        };
        let albedo = if json["albedo"].is_object() {
            construct_texture::<SpectrumRGB>(json)
        } else if json["albedo"].is_array() {
            let arr: Vec<f32> = serde_json::from_value(json["albedo"].clone()).unwrap();
            let s = SpectrumRGB::from_rgb(Vector3::new(arr[0], arr[1], arr[2]));
            Rc::new(ConstantTexture::new(&s))
        } else {
            panic!("Error in albedo format!");
        };
        Self {
            normal_map,
            albedo,
        }
    }
}

impl Material for MatteMaterial {
    fn normal_map(&self) -> Option<Rc<NormalTexture>> {
        self.normal_map.clone()
    }

    fn compute_bsdf(&self, intersection: &Intersection) -> Rc<dyn BSDF> {
        let [mut normal, mut tangent, mut bitangent] = [Vector3::<f32>::zeros(); 3];
        self.compute_shading_geometry(intersection, &mut normal, &mut tangent, &mut bitangent);
        let s = self.albedo.evaluate(intersection);
        Rc::new(LambertReflection::new(s, normal, tangent, bitangent))
    }
}
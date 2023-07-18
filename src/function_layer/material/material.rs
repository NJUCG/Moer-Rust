use super::ndf::{beckmann::BeckmannDistribution, ggx::GGXDistribution};
use super::{
    bxdf::bsdf::BSDF, dielectric::DielectricMaterial, matte::MatteMaterial, mirror::MirrorMaterial,
    oren_nayar::OrenNayarMaterial, phong::PhongMaterial,
};
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::material::conductor::ConductorMaterial;
use crate::function_layer::texture::constant_texture::ConstantTexture;
use crate::function_layer::texture::normal_texture::NormalTexture;
use crate::function_layer::{construct_texture, fetch_v3f, SurfaceInteraction, Texture, V3f, NDF};
use cgmath::{InnerSpace, Vector2, Zero};
use serde_json::Value;
use std::rc::Rc;
use crate::function_layer::material::transparent::TransparentMaterial;

pub trait Material {
    fn normal_map(&self) -> Option<Rc<NormalTexture>>;
    // self.normal_map.clone()
    fn compute_bsdf(&self, intersection: &SurfaceInteraction) -> Box<dyn BSDF>;
    fn compute_shading_geometry(
        &self,
        intersection: &SurfaceInteraction,
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

pub fn fetch_albedo(json: &Value) -> Rc<dyn Texture<SpectrumRGB>> {
    fetch_spectrum(json, "albedo")
}

pub fn fetch_roughness(json: &Value) -> Vector2<f32> {
    let rn = &json["roughness"];
    let roughness = if rn.is_number() {
        let r = rn.as_f64().unwrap() as f32;
        Vector2::new(r, r)
    } else if rn.is_array() {
        Vector2::from(serde_json::from_value::<[f32; 2]>(rn.clone()).unwrap())
    } else {
        panic!("Error in roughness format!");
    };
    roughness
}

pub fn fetch_ndf(json: &Value) -> Rc<dyn NDF> {
    if !json["ndf"].is_null() && json["ndf"].as_str().unwrap() == "ggx" {
        Rc::new(GGXDistribution {})
    } else {
        Rc::new(BeckmannDistribution {})
    }
}

pub fn fetch_spectrum(json: &Value, field: &str) -> Rc<dyn Texture<SpectrumRGB>> {
    if json[field].is_object() {
        construct_texture::<SpectrumRGB>(&json[field])
    } else {
        let s = fetch_v3f(json, field, V3f::zero());
        Rc::new(ConstantTexture::new(&SpectrumRGB::from_rgb(s)))
    }
}

pub fn construct_material(json: &Value) -> Rc<dyn Material> {
    match json["type"].as_str().expect("No material type annotation!") {
        "matte" => Rc::new(MatteMaterial::from_json(json)),
        "mirror" => Rc::new(MirrorMaterial::from_json(json)),
        "phong" => Rc::new(PhongMaterial::from_json(json)),
        "oren-nayar" => Rc::new(OrenNayarMaterial::from_json(json)),
        "dielectric" => Rc::new(DielectricMaterial::from_json(json)),
        "conductor" => Rc::new(ConductorMaterial::from_json(json)),
        "transparent" => Rc::new(TransparentMaterial::from_json(json)),
        tp => panic!("Invalid type: {}", tp),
    }
}

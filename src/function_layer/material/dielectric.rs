use super::bxdf::{bsdf::BSDFBase, rough_dielectric::RoughDielectricBSDF};
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::texture::normal_texture::NormalTexture;
use crate::function_layer::{Material, SurfaceInteraction, Texture, BSDF, NDF};
use cgmath::Vector2;
use serde_json::Value;
use std::rc::Rc;

use super::material::{fetch_albedo, fetch_ndf, fetch_normal_map, fetch_roughness};

pub struct DielectricMaterial {
    normal_map: Option<Rc<NormalTexture>>,
    albedo: Rc<dyn Texture<SpectrumRGB>>,
    ndf: Rc<dyn NDF>,
    eta: f32,
    roughness: Vector2<f32>,
}

impl DielectricMaterial {
    pub fn from_json(json: &Value) -> Self {
        let albedo = fetch_albedo(json);
        let normal_map = fetch_normal_map(json);
        let roughness = fetch_roughness(json);
        let ndf: Rc<dyn NDF> = fetch_ndf(json);
        let eta = json["eta"].as_f64().unwrap() as f32;
        Self {
            normal_map,
            albedo,
            ndf,
            eta,
            roughness,
        }
    }
}

impl Material for DielectricMaterial {
    fn normal_map(&self) -> Option<Rc<NormalTexture>> {
        self.normal_map.clone()
    }

    fn compute_bsdf(&self, intersection: &SurfaceInteraction) -> Box<dyn BSDF> {
        let (normal, tangent, bitangent) = self.compute_shading_geometry(intersection);
        let s = self.albedo.evaluate(intersection);
        let bsdf = BSDFBase {
            normal,
            tangent,
            bitangent,
        };
        Box::new(RoughDielectricBSDF::new(
            bsdf,
            s,
            self.roughness,
            self.eta,
            Some(self.ndf.clone()),
        ))
    }
}

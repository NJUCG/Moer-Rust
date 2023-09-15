use std::rc::Rc;

use cgmath::{Vector2, Zero};
use serde_json::Value;

use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::{BSDF, fetch_v3f, Material, NDF, SurfaceInteraction, Texture, V3f};
use crate::function_layer::material::bxdf::rough_conductor::RoughConductorBSDF;
use crate::function_layer::texture::normal_texture::NormalTexture;

use super::bxdf::bsdf::BSDFBase;
use super::material::{fetch_albedo, fetch_ndf, fetch_normal_map, fetch_roughness};

pub struct ConductorMaterial {
    normal_map: Option<Rc<NormalTexture>>,
    albedo: Rc<dyn Texture<SpectrumRGB>>,
    ndf: Rc<dyn NDF>,
    eta: V3f,
    k: V3f,
    roughness: Vector2<f32>,
}

impl ConductorMaterial {
    pub fn from_json(json: &Value) -> Self {
        let albedo = fetch_albedo(json);
        let normal_map = fetch_normal_map(json);
        let dft = V3f::zero();
        let eta = fetch_v3f(json, "eta", dft);
        let k = fetch_v3f(json, "k", dft);
        let roughness = fetch_roughness(json);
        let ndf = fetch_ndf(json);
        Self {
            normal_map,
            albedo,
            ndf,
            eta,
            k,
            roughness,
        }
    }
}

impl Material for ConductorMaterial {
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
        Box::new(RoughConductorBSDF::new(
            bsdf,
            s,
            self.roughness,
            self.eta,
            self.k,
            Some(self.ndf.clone()),
        ))
    }
}

use std::rc::Rc;
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::{BSDF, Material, SurfaceInteraction};
use crate::function_layer::material::bxdf::lambert::LambertReflection;
use crate::function_layer::texture::normal_texture::NormalTexture;

struct BlackHole;

impl Material for BlackHole {
    fn normal_map(&self) -> Option<Rc<NormalTexture>> {
        None
    }

    fn compute_bsdf(&self, intersection: &SurfaceInteraction) -> Box<dyn BSDF> {
        let (normal, tangent, bitangent) = self.compute_shading_geometry(intersection);
        Box::new(LambertReflection::new(SpectrumRGB::same(0.0), normal, tangent, bitangent))
    }
}
use nalgebra::Vector2;
use rand::Rng;
use rand::rngs::ThreadRng;
use serde_json::Value;
use crate::function_layer::Sampler;

pub struct IndependentSampler {
    pub x_samples: usize,
    pub y_samples: usize,
    rng: ThreadRng,
}

impl IndependentSampler {
    pub fn from_json(json: &Value) -> Self {
        let x_samples = json["xSamples"].as_u64().unwrap() as usize;
        let y_samples = json["ySamples"].as_u64().unwrap() as usize;
        let rng = ThreadRng::default();
        Self {
            x_samples,
            y_samples,
            rng,
        }
    }
}

impl Sampler for IndependentSampler {
    fn xsp(&self) -> usize {
        self.x_samples
    }

    fn ysp(&self) -> usize {
        self.y_samples
    }

    fn next_1d(&mut self) -> f32 {
        self.rng.gen()
    }

    fn next_2d(&mut self) -> Vector2<f32> {
        Vector2::new(self.rng.gen(), self.rng.gen())
    }
}
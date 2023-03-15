#![allow(dead_code)]

#[derive(Clone, PartialEq)]
pub struct Distribution<T: Clone> {
    data: Vec<T>,
    cdf: Vec<f32>,
}

impl<T: Clone + PartialEq> Distribution<T> {
    pub fn new(data: &Vec<T>, weight_function: fn(T) -> f32) -> Self {
        let mut cdf: Vec<f32> = Vec::with_capacity(data.len() + 1);
        cdf.push(0.0);

        for i in 0..data.len() {
            let w = weight_function(data[i].clone());
            cdf.push(w + cdf.last().unwrap());
        }
        let inv_total = 1.0 / cdf.last().unwrap();
        for e in cdf.iter_mut() { *e *= inv_total; }
        Self {
            data: data.clone(),
            cdf,
        }
    }

    pub fn sample(&self, sample: f32, pdf: &mut f32) -> T {
        let idx = self.cdf.binary_search_by(|probe| probe.partial_cmp(&sample).unwrap());
        let idx = match idx { Ok(i) | Err(i) => i, } - 1;
        *pdf = self.cdf[idx + 1] - self.cdf[idx];
        self.data[idx.min(self.cdf.len() - 2)].clone()
    }

    pub fn pdf(&self, sampled: T) -> f32 {
        let entry = self.data.iter().position(move |x| *x == sampled);
        if let Some(idx) = entry {
            self.cdf[idx + 1] - self.cdf[idx]
        } else {
            0.0
        }
    }
}
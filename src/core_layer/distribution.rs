use std::cmp::Ordering;

#[derive(Clone, PartialEq)]
pub struct Distribution<T: Clone> {
    data: Vec<T>,
    cdf: Vec<f32>,
}

impl<T: Clone + PartialEq> Distribution<T> {
    pub fn new(data: Vec<T>, weight_function: impl Fn(T) -> f32) -> Self {
        let mut cdf: Vec<f32> = Vec::with_capacity(data.len() + 1);
        cdf.push(0.0);

        for i in 0..data.len() {
            let w = weight_function(data[i].clone());
            cdf.push(w + cdf.last().unwrap());
        }
        *cdf.last_mut().unwrap() = 1.0;
        let inv_total = 1.0 / cdf.last().unwrap();
        for e in cdf.iter_mut() {
            *e *= inv_total;
        }
        Self { data, cdf }
    }

    pub fn sample(&self, sample: f32, pdf: &mut f32) -> Option<T> {
        if self.cdf.len() == 1 {
            *pdf = 0.0;
            return None;
        }
        let idx =
            self.cdf
                .binary_search_by(|probe: &f32| match probe.partial_cmp(&sample).unwrap() {
                    Ordering::Equal => Ordering::Greater,
                    ord => ord,
                });
        let idx = idx.err().unwrap() - 1;
        *pdf = self.cdf[idx + 1] - self.cdf[idx];
        Some(self.data[idx.min(self.cdf.len() - 2)].clone())
    }

    #[allow(dead_code)]
    pub fn pdf(&self, sampled: T) -> f32 {
        let entry = self.data.iter().position(move |x| *x == sampled);
        if let Some(idx) = entry {
            self.cdf[idx + 1] - self.cdf[idx]
        } else {
            0.0
        }
    }
}

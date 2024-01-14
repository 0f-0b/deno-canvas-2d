fn gaussian(x: f64, s: f64) -> f64 {
    let sr = s.recip();
    0.3989422804014327 * sr * (-0.5 * (x * x) * (sr * sr)).exp()
}

pub struct GaussianBlur {
    kernel: Vec<f32>,
}

impl GaussianBlur {
    pub fn new(s: f32) -> Option<Self> {
        let r = (3.0 * s).ceil() as usize;
        (r != 0).then(|| Self {
            kernel: (0..=r)
                .map(|x| gaussian(x as f64, s as f64) as f32)
                .collect(),
        })
    }

    pub fn radius(&self) -> usize {
        self.kernel.len() - 1
    }

    pub fn apply(&self, data: &mut [f32], width: usize) {
        let height = data.len() / width;
        assert!(data.len() == width * height);
        let r = self.radius();
        assert!(width >= 2 * r);
        assert!(height >= 2 * r);
        let kernel = self.kernel.as_slice();
        for y in 0..height {
            for x in r..width - r {
                let mut sum = data[y * width + x] * kernel[0];
                for i in 1..=r {
                    sum += (data[y * width + (x - i)] + data[y * width + (x + i)]) * kernel[i];
                }
                data[y * width + (x - r)] = sum;
            }
        }
        for y in r..height - r {
            for x in 0..width - 2 * r {
                let mut sum = data[y * width + x] * kernel[0];
                for i in 1..=r {
                    sum += (data[(y - i) * width + x] + data[(y + i) * width + x]) * kernel[i];
                }
                data[(y - r) * width + x] = sum;
            }
        }
    }
}

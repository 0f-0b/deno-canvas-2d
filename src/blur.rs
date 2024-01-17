fn gaussian(x: f32, sigma: f32) -> f32 {
    let inv_sigma = sigma.recip();
    0.3989423 * inv_sigma * (-0.5 * (x * x) * (inv_sigma * inv_sigma)).exp()
}

fn sampled_gaussian_kernel(sigma: f32) -> Option<Vec<f32>> {
    let m = (3.0 * sigma).ceil() as usize;
    if m == 0 {
        return None;
    }
    let mut v = vec![0.0; m * 2 + 1];
    for i in 0..=m {
        let w = gaussian(i as f32, sigma);
        v[m + i] = w;
        v[m - i] = w;
    }
    let scale = v.iter().sum::<f32>().recip();
    for w in &mut v {
        *w *= scale;
    }
    Some(v)
}

pub struct GaussianBlur {
    kernel: Vec<f32>,
}

impl GaussianBlur {
    pub fn new(sigma: f32) -> Option<Self> {
        sampled_gaussian_kernel(sigma).map(|kernel| Self { kernel })
    }

    pub fn extend_len(&self) -> usize {
        self.kernel.len() - 1
    }

    pub fn apply(&self, data: &mut [f32], width: usize) {
        let height = data.len() / width;
        assert!(data.len() == width * height);
        let n = self.extend_len();
        assert!(width >= n && height >= n);
        let kernel = self.kernel.as_slice();
        for y in 0..height {
            for x in 0..width - n {
                data[y * width + x] = kernel
                    .iter()
                    .enumerate()
                    .map(|(i, w)| data[y * width + (x + i)] * w)
                    .sum();
            }
        }
        for y in 0..height - n {
            for x in 0..width - n {
                data[y * width + x] = kernel
                    .iter()
                    .enumerate()
                    .map(|(i, w)| data[(y + i) * width + x] * w)
                    .sum();
            }
        }
    }
}

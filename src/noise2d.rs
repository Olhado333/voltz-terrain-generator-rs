use {libnoise::prelude::*, rand::prelude::*};

pub struct Noise2D {
    generator: Perlin<2>,
}

impl Noise2D {
    // Creates a new Noise2D object with a provided seed.
    pub fn from_seed(seed: u64) -> Self {
        Self {
            generator: Source::perlin(seed),
        }
    }

    pub fn from_rng(random: &mut SmallRng) -> Self {
        Noise2D::from_seed(random.random())
    }

    // Returns a noise value between scale and -scale.
    pub fn noise(&self, x: f64, y: f64, scale: f64) -> f64 {
        self.generator
            .sample([x / scale + 10000000.0, y / scale + 10000000.0])
    }

    // Returns a noise value between 0 and scale.
    pub fn unit_noise(&self, x: f64, y: f64, scale: f64) -> f64 {
        let raw_noise = self.noise(x, y, scale);
        (raw_noise + 1.0) / 2.0
    }

    // Returns true if the noise value is between range and -range.
    pub fn range(&self, x: f64, y: f64, scale: f64, range: f64) -> bool {
        let raw_noise = self.noise(x, y, scale);
        raw_noise.abs() < range
    }

    // Returns true if the unit noise value is between 0 and range.
    pub fn edge_range(&self, x: f64, y: f64, scale: f64, range: f64) -> bool {
        let unit_noise = self.unit_noise(x, y, scale);
        unit_noise < range
    }
}

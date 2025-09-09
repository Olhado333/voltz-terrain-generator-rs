use crate::noise2d::Noise2D;
use rand::{SeedableRng, rngs::SmallRng};

pub struct TerrainRandomizers {
    // Island
    pub island_gaps: Noise2D,
    pub island_depth: Noise2D,

    // Starter Island
    pub starter_island_radius: Noise2D,

    // Crystal
    pub crystal: SmallRng,

    // Spire
    pub spire_cities: Noise2D,
    pub spire_buildings: Noise2D,
    pub spire_alleys: Noise2D,
    pub spire_height: Noise2D,

    // Grass
    pub grass: Noise2D,

    // Support
    pub support_radius: SmallRng,
    pub support_circular: SmallRng,
    pub support_well: SmallRng,
    pub support_ladder: SmallRng,
}

impl TerrainRandomizers {
    pub fn from_seed(seed: u64) -> Self {
        let mut random = SmallRng::seed_from_u64(seed);

        Self {
            // Island
            island_gaps: Noise2D::from_rng(&mut random),
            island_depth: Noise2D::from_rng(&mut random),
            starter_island_radius: Noise2D::from_rng(&mut random),

            // Crystal
            crystal: SmallRng::from_rng(&mut random),

            // Spire
            spire_cities: Noise2D::from_rng(&mut random),
            spire_buildings: Noise2D::from_rng(&mut random),
            spire_alleys: Noise2D::from_rng(&mut random),
            spire_height: Noise2D::from_rng(&mut random),

            // Grass
            grass: Noise2D::from_rng(&mut random),

            // Support
            support_radius: SmallRng::from_rng(&mut random),
            support_circular: SmallRng::from_rng(&mut random),
            support_well: SmallRng::from_rng(&mut random),
            support_ladder: SmallRng::from_rng(&mut random),
        }
    }
}

use {
    crate::{
        config::TerrainConfig, sources::terrain_randomizers::TerrainRandomizers, vector3::Vector3,
    },
    std::collections::HashMap,
};

pub fn generate_mainland(
    config: &TerrainConfig,
    randomizers: &TerrainRandomizers,
) -> HashMap<Vector3<i32>, i32> {
    // Config
    let mainland_config = &config.mainland;

    // Randomizers
    let gap_noise = &randomizers.island_gaps;
    let depth_noise = &randomizers.island_depth;

    // Output
    let mut mainland_grid: HashMap<Vector3<i32>, i32> = HashMap::new();

    for x in -mainland_config.size..=mainland_config.size {
        for z in -mainland_config.size..=mainland_config.size {
            // Circle
            if (x * x + z * z).isqrt() > mainland_config.size {
                continue;
            }

            // Gaps
            if gap_noise.range(
                x as f64,
                z as f64,
                mainland_config.gaps.scale,
                mainland_config.gaps.weight,
            ) {
                continue;
            }

            // Depth
            let depth = (depth_noise.unit_noise(x as f64, z as f64, mainland_config.depth.scale)
                * mainland_config.depth.weight)
                .floor() as i32;

            // Grid
            mainland_grid.insert(Vector3::new(x, 0, z), depth);
        }
    }

    mainland_grid
}

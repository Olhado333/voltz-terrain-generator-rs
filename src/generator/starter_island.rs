use {
    crate::{Noise2D, TerrainConfig, TerrainRandomizers, Vector3},
    std::collections::HashMap,
};

fn generate_starter_island(
    position: &Vector3<i32>,
    config: &TerrainConfig,
    island_radius_noise: &Noise2D,
    island_depth_noise: &Noise2D,
) -> HashMap<Vector3<i32>, i32> {
    // Config
    let island_config = &config.mainland;
    let starter_island_config = &config.starter_island;

    let starter_island_size = starter_island_config.island_size;

    // Noise values
    let radius_variation = &starter_island_config.radius_variation;

    // Output
    let mut island_grid: HashMap<Vector3<i32>, i32> = HashMap::new();

    let generation_range = starter_island_size + radius_variation.weight.ceil() as i32;

    for x in -generation_range..=generation_range {
        for z in -generation_range..=generation_range {
            // Coordinates adjusted for real world position
            let real_x = x + position.x;
            let real_z = z + position.z;

            // Noisy radius
            let island_radius = starter_island_size
                + (island_radius_noise.unit_noise(
                    real_x as f64,
                    real_z as f64,
                    radius_variation.scale,
                ) * radius_variation.weight)
                    .floor() as i32;

            // Circle
            if (x.pow(2) + z.pow(2)).isqrt() > island_radius {
                continue;
            }

            // Generate depth
            let depth = (island_depth_noise.unit_noise(
                real_x as f64,
                real_z as f64,
                island_config.depth.scale,
            ) * island_config.depth.weight)
                .floor() as i32;

            // Insert into grid
            island_grid.insert(Vector3::new(x, 0, z), depth);
        }
    }

    island_grid
}

// Starter island
pub fn generate_starter_islands(
    config: &TerrainConfig,
    randomizers: &TerrainRandomizers,
    starter_island_point_grid: &Vec<Vector3<i32>>,
) -> HashMap<Vector3<i32>, HashMap<Vector3<i32>, i32>> {
    // Randomizers
    let island_radius_noise = &randomizers.starter_island_radius;
    let island_depth_noise = &randomizers.island_depth;

    // Output
    let mut starter_islands: HashMap<Vector3<i32>, HashMap<Vector3<i32>, i32>> = HashMap::new();

    for position in starter_island_point_grid {
        starter_islands.insert(
            position.clone(),
            generate_starter_island(position, config, island_radius_noise, island_depth_noise),
        );
    }

    starter_islands
}

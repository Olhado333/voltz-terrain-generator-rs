use {
    crate::{TerrainConfig, TerrainRandomizers, Vector3},
    std::collections::{HashMap, HashSet},
};

pub fn generate_grass(
    config: &TerrainConfig,
    randomizers: &TerrainRandomizers,
    island_grid: &HashMap<Vector3<i32>, i32>,
) -> HashSet<Vector3<i32>> {
    // Config
    let grass_config = &config.grass;

    // Randomizers
    let grass_noise = &randomizers.grass;

    // Output
    let mut grass_grid: HashSet<Vector3<i32>> = HashSet::new();

    for position in island_grid.keys() {
        // Grass Noise
        if grass_noise.edge_range(
            position.x as f64,
            position.z as f64,
            grass_config.grass.scale,
            grass_config.grass.weight,
        ) {
            grass_grid.insert(position.clone());
        }
    }

    grass_grid
}

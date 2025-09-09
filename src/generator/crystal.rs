use {
    crate::{TerrainConfig, TerrainRandomizers, Vector3},
    rand::Rng,
    std::collections::{HashMap, HashSet},
};

pub fn generate_crystal(
    config: &TerrainConfig,
    randomizers: &mut TerrainRandomizers,
    island_grid: &HashMap<Vector3<i32>, i32>,
    spire_grid: &HashMap<Vector3<i32>, i32>,
) -> HashSet<Vector3<i32>> {
    // config
    let crystal_config = &config.crystal;

    // Randomizers
    let crystal_random = &mut randomizers.crystal;

    // output
    let mut crystal_grid: HashSet<Vector3<i32>> = HashSet::new();

    for position in island_grid.keys() {
        // Spire Avoidance
        if spire_grid.contains_key(&position) {
            continue;
        }

        // Crystal Randomization
        if crystal_random.random::<f64>() < crystal_config.spawn_rate {
            crystal_grid.insert(position.clone());
        }
    }

    crystal_grid
}

use {
    crate::{Noise2D, SpireConfig, TerrainConfig, TerrainRandomizers, Vector3},
    std::collections::HashMap,
};

fn should_generate(
    position: &Vector3<i32>,
    city_noise: &Noise2D,
    building_noise: &Noise2D,
    alley_noise: &Noise2D,
    spire_config: &SpireConfig,
) -> bool {
    let is_city = city_noise.edge_range(
        position.x as f64,
        position.z as f64,
        spire_config.cities.scale,
        spire_config.cities.weight,
    );
    let is_building = building_noise.edge_range(
        position.x as f64,
        position.z as f64,
        spire_config.buildings.scale,
        spire_config.buildings.weight,
    );
    let is_alley = alley_noise.edge_range(
        position.x as f64,
        position.z as f64,
        spire_config.alleys.scale,
        spire_config.alleys.weight,
    );

    is_city && is_building && !is_alley
}

pub fn generate_spire(
    config: &TerrainConfig,
    randomizers: &mut TerrainRandomizers,
    island_grid: &HashMap<Vector3<i32>, i32>,
) -> HashMap<Vector3<i32>, i32> {
    // Config
    let spire_config = &config.spire;

    // Randomizers
    let city_noise = &randomizers.spire_cities;
    let building_noise = &randomizers.spire_buildings;
    let alley_noise = &randomizers.spire_alleys;
    let height_noise = &randomizers.spire_height;

    // Output
    let mut spire_grid: HashMap<Vector3<i32>, i32> = HashMap::new();

    for position in island_grid.keys() {
        if !should_generate(
            position,
            city_noise,
            building_noise,
            alley_noise,
            spire_config,
        ) {
            continue;
        }

        let spire_height = (height_noise.unit_noise(
            position.x as f64,
            position.z as f64,
            spire_config.height.scale,
        ) * spire_config.height.weight)
            .floor();

        if spire_height == 0.0 {
            continue;
        }

        spire_grid.insert(position.clone(), spire_height as i32);
    }

    spire_grid
}

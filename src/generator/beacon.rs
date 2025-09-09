use {
    crate::{TerrainConfig, Vector3},
    std::collections::HashSet,
};

pub fn generate_beacon(terrain_config: &TerrainConfig) -> HashSet<Vector3<i32>> {
    // Config
    let beacon_config = &terrain_config.beacon;

    // Output
    let mut beacon_grid: HashSet<Vector3<i32>> = HashSet::new();

    if beacon_config.control_beacon {
        beacon_grid.insert(Vector3::new(0, 0, 0));
    }

    let beacon_angle_difference = (360.0 / beacon_config.beacon_count as f64).to_radians();

    for i in 1..=beacon_config.beacon_count {
        let unit_x = (beacon_angle_difference * i as f64).cos();
        let unit_y = (beacon_angle_difference * i as f64).sin();

        let scaled_x = (unit_x * beacon_config.offset as f64).round() as i32;
        let scaled_y = (unit_y * beacon_config.offset as f64).round() as i32;

        beacon_grid.insert(Vector3::new(scaled_x, 0, scaled_y));
    }

    beacon_grid
}

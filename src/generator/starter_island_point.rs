use crate::{TerrainConfig, Vector3};

pub fn starter_island_point(config: &TerrainConfig) -> Vec<Vector3<i32>> {
    // Config
    let island_config = &config.mainland;
    let starter_island_config = &config.starter_island;

    // Output
    let mut starter_island_point_grid: Vec<Vector3<i32>> = Vec::new();

    let island_angle_difference = (360.0 / starter_island_config.island_count as f64).to_radians();

    for i in 1..=starter_island_config.island_count {
        let unit_x = (island_angle_difference * i as f64).cos();
        let unit_y = (island_angle_difference * i as f64).sin();

        let scaled_x = (unit_x * (island_config.size as f64 + starter_island_config.island_offset))
            .round() as i32;
        let scaled_y = (unit_y * (island_config.size as f64 + starter_island_config.island_offset))
            .round() as i32;

        starter_island_point_grid.push(Vector3::new(scaled_x, 0, scaled_y));
    }

    starter_island_point_grid
}

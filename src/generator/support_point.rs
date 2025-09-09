use {
    crate::{ChunkGrid, TerrainConfig, Vector3},
    std::collections::{HashMap, HashSet},
};

pub fn generate_mainland_support_points(
    config: &TerrainConfig,
    starter_island_grid: &HashMap<Vector3<i32>, i32>,
) -> HashSet<Vector3<i32>> {
    // Config
    let support_config = &config.support;

    let mut mainland_support_points = ChunkGrid::new();

    for island_point in starter_island_grid.keys() {
        if !mainland_support_points.is_in_range(island_point, support_config.support_spacing as f64)
        {
            mainland_support_points.insert(island_point.clone());
        }
    }

    mainland_support_points.elements().into_iter().collect()
}

pub fn generate_starter_support_points(
    config: &TerrainConfig,
    island_grid: &HashMap<Vector3<i32>, HashMap<Vector3<i32>, i32>>,
) -> HashSet<Vector3<i32>> {
    // Config
    let support_config = &config.support;

    let mut starter_support_points = ChunkGrid::new();

    for (island_center, island) in island_grid {
        for island_offset in island.keys() {
            let real_position = island_center.clone() + island_offset.clone();

            if !starter_support_points
                .is_in_range(&real_position, support_config.support_spacing as f64)
            {
                starter_support_points.insert(real_position);
            }
        }
    }

    starter_support_points.elements().into_iter().collect()
}

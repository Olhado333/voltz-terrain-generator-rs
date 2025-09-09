use {
    crate::{SupportConfig, TerrainConfig, TerrainRandomizers, Vector3},
    rand::{Rng, rngs::SmallRng},
    std::collections::{HashMap, HashSet},
};

#[derive(Debug)]
pub enum SupportColumnType {
    Fill,
    Well,
    Ladder,
}

// Generate Supports
pub fn generate_supports(
    config: &TerrainConfig,
    randomizers: &mut TerrainRandomizers,
    mainland_point_grid: &HashSet<Vector3<i32>>,
    starter_point_grid: &HashSet<Vector3<i32>>,
) -> HashMap<Vector3<i32>, HashMap<Vector3<i32>, SupportColumnType>> {
    // Config
    let support_config = &config.support;

    // Randomizers
    let radius_random = &mut randomizers.support_radius;
    let circular_random = &mut randomizers.support_circular;
    let well_random = &mut randomizers.support_well;
    let ladder_random = &mut randomizers.support_ladder;

    // Supports output
    let mut support_grids: HashMap<Vector3<i32>, HashMap<Vector3<i32>, SupportColumnType>> =
        HashMap::new();

    for point in mainland_point_grid {
        support_grids.insert(
            point.clone(),
            generate_mainland_support(
                radius_random.random_range(support_config.min_radius..=support_config.max_radius),
                circular_random.random_bool(support_config.circular_chance),
                support_config,
                well_random,
                ladder_random,
            ),
        );
    }

    for point in starter_point_grid {
        support_grids.insert(point.clone(), generate_starter_support());
    }

    support_grids
}

// Generate Support
fn generate_mainland_support(
    radius: i32,
    is_circular: bool,
    support_config: &SupportConfig,
    well_random: &mut SmallRng,
    ladder_random: &mut SmallRng,
) -> HashMap<Vector3<i32>, SupportColumnType> {
    // Collumn subgrid
    let mut mainland_support_grid: HashMap<Vector3<i32>, SupportColumnType> = HashMap::new();

    for x in -radius..=radius {
        for z in -radius..=radius {
            if is_circular && ((x * x) as f64 + (z * z) as f64) > radius as f64 {
                continue;
            }

            if ladder_random.random_range(0.0..=1.0) < support_config.ladder_chance {
                mainland_support_grid.insert(Vector3::new(x, 0, z), SupportColumnType::Ladder);
            } else if well_random.random_range(0.0..=1.0) < support_config.well_chance {
                mainland_support_grid.insert(Vector3::new(x, 0, z), SupportColumnType::Well);
            } else {
                mainland_support_grid.insert(Vector3::new(x, 0, z), SupportColumnType::Fill);
            }
        }
    }

    mainland_support_grid
}

// Generate starter support
fn generate_starter_support() -> HashMap<Vector3<i32>, SupportColumnType> {
    // Column map
    let mut starter_support_grid: HashMap<Vector3<i32>, SupportColumnType> = HashMap::new();

    for x in -1..=1 {
        for z in -1..=1 {
            if ((x * x) as f64 + (z * z) as f64).sqrt() > 1.0 {
                continue;
            }

            if x == 0 && z == 0 {
                starter_support_grid.insert(Vector3::new(x, 0, z), SupportColumnType::Well);
            } else {
                starter_support_grid.insert(Vector3::new(x, 0, z), SupportColumnType::Fill);
            }
        }
    }

    starter_support_grid
}

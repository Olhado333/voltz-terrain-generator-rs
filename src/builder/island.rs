use {
    crate::Vector3,
    std::collections::{HashMap, HashSet},
};

pub enum SurfaceType {
    Grass,
    Fill,
}

pub fn build_mainland(
    mainland_grid: &HashMap<Vector3<i32>, i32>,
    grass_grid: &HashSet<Vector3<i32>>,
) -> HashMap<Vector3<i32>, SurfaceType> {
    // Mainland Output
    let mut mainland_writes: HashMap<Vector3<i32>, SurfaceType> = HashMap::new();

    for (position, depth) in mainland_grid {
        for y in -depth..=0 {
            mainland_writes.insert(
                position.clone() + Vector3::new(0, y, 0),
                if y == 0 && grass_grid.contains(&position) {
                    SurfaceType::Grass
                } else {
                    SurfaceType::Fill
                },
            );
        }
    }

    mainland_writes
}

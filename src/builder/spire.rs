use {
    crate::Vector3,
    std::collections::{HashMap, HashSet},
};

pub enum SpireType {
    Grass,
    Top,
    Fill,
}

pub fn build_spire(
    spire_grid: &HashMap<Vector3<i32>, i32>,
    grass_grid: &HashSet<Vector3<i32>>,
) -> HashMap<Vector3<i32>, SpireType> {
    let mut spire_writes: HashMap<Vector3<i32>, SpireType> = HashMap::new();

    for (position, &height) in spire_grid {
        for y in 1..=height {
            if y != height {
                spire_writes.insert(position.clone() + Vector3::new(0, y, 0), SpireType::Fill);
                continue;
            }

            if grass_grid.contains(position) {
                spire_writes.insert(position.clone() + Vector3::new(0, y, 0), SpireType::Grass);
            } else {
                spire_writes.insert(position.clone() + Vector3::new(0, y, 0), SpireType::Top);
            }
        }
    }

    spire_writes
}

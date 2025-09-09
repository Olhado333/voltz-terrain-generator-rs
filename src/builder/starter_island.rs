use {
    crate::Vector3,
    std::collections::{HashMap, HashSet},
};

pub fn build_starter_island(starter_island: &HashMap<Vector3<i32>, i32>) -> HashSet<Vector3<i32>> {
    let mut island_write: HashSet<Vector3<i32>> = HashSet::new();

    for (offset, &depth) in starter_island {
        for y in -depth..=0 {
            island_write.insert(offset.clone() + Vector3::new(0, y, 0));
        }
    }

    island_write
}

pub fn build_starter_islands(
    starter_island_grids: &HashMap<Vector3<i32>, HashMap<Vector3<i32>, i32>>,
) -> HashMap<Vector3<i32>, HashSet<Vector3<i32>>> {
    // Output grids
    let mut starter_island_writes: HashMap<Vector3<i32>, HashSet<Vector3<i32>>> = HashMap::new();

    for (position, starter_island) in starter_island_grids {
        starter_island_writes.insert(position.clone(), build_starter_island(starter_island));
    }

    starter_island_writes
}
